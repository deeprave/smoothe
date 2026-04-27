use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    process::ExitCode,
};

use serde::Deserialize;
use smoothe::config::{ResolvedCheckOptions, ResolvedGlobalOptions, SemanticInput};
use smoothe::content::{ContentInput, process as process_template};
use smoothe::parser::{
    Diagnostic, DiagnosticSeverity, IssueKind, Node, SourceLocation, SourceMetadata, SourceSpan,
};

use crate::cli::CheckArgs;

use super::read_template_inputs;

pub fn check(
    args: CheckArgs,
    global_options: ResolvedGlobalOptions,
    _check_options: ResolvedCheckOptions,
) -> ExitCode {
    let _color = global_options.color;

    let inputs = match read_template_inputs(&args.inputs) {
        Ok(inputs) => inputs,
        Err(error) => {
            eprintln!("error: {error}");
            return ExitCode::FAILURE;
        }
    };

    let semantic_inputs = SemanticInputs::resolve(&args, &_check_options);
    let (schema, schema_diagnostics) = load_schema(&semantic_inputs.schema);
    let (lambdas, lambda_diagnostics) = load_lambdas(&semantic_inputs.lambdas);
    let mut has_error = false;

    for diagnostic in schema_diagnostics.iter().chain(lambda_diagnostics.iter()) {
        eprintln!("{}", format_diagnostic(diagnostic));
    }

    for input in inputs {
        let mut source = SourceMetadata::new(&input.name);
        if let Some(root) = input.root {
            source = source.with_root(root);
        }
        let result = process_template(ContentInput::new(source, &input.source));

        for diagnostic in &result.state.diagnostics {
            eprintln!("{}", format_diagnostic(diagnostic));
        }
        for diagnostic in validate_semantics(
            &result.raw_data,
            &input.name,
            &result.ast.nodes,
            schema.as_ref(),
            &lambdas,
        ) {
            eprintln!("{}", format_diagnostic(&diagnostic));
        }

        has_error |= result
            .state
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.severity == DiagnosticSeverity::Error);
    }

    if has_error {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

struct SemanticInputs {
    schema: SemanticInput,
    lambdas: SemanticInput,
}

impl SemanticInputs {
    fn resolve(args: &CheckArgs, options: &ResolvedCheckOptions) -> Self {
        Self {
            schema: semantic_input(args.schema.as_deref())
                .unwrap_or_else(|| options.schema.clone()),
            lambdas: semantic_input(args.lambdas.as_deref())
                .unwrap_or_else(|| options.lambdas.clone()),
        }
    }
}

fn semantic_input(value: Option<&str>) -> Option<SemanticInput> {
    value.map(|value| {
        if value.eq_ignore_ascii_case("none") {
            SemanticInput::Disabled
        } else {
            SemanticInput::Path(PathBuf::from(value))
        }
    })
}

#[derive(Debug)]
struct ContextSchema {
    root: serde_json::Value,
}

impl ContextSchema {
    fn new(root: serde_json::Value) -> Self {
        Self { root }
    }

    fn root(&self) -> &serde_json::Value {
        &self.root
    }
}

fn load_schema(input: &SemanticInput) -> (Option<ContextSchema>, Vec<Diagnostic>) {
    let SemanticInput::Path(path) = input else {
        return (None, Vec::new());
    };

    let source = match fs::read_to_string(path) {
        Ok(source) => source,
        Err(error) => {
            return (
                None,
                vec![input_diagnostic(
                    IssueKind::SchemaInputError,
                    path,
                    format!("failed to read schema {}: {error}", path.display()),
                )],
            );
        }
    };

    match serde_json::from_str::<serde_json::Value>(&source) {
        Ok(schema) if is_recognisable_schema(&schema) => {
            (Some(ContextSchema::new(schema)), Vec::new())
        }
        Ok(_) => (
            None,
            vec![input_diagnostic(
                IssueKind::SchemaInputError,
                path,
                format!("unrecognisable context schema {}", path.display()),
            )],
        ),
        Err(error) => (
            None,
            vec![input_diagnostic(
                IssueKind::SchemaInputError,
                path,
                format!("failed to parse schema {}: {error}", path.display()),
            )],
        ),
    }
}

fn is_recognisable_schema(schema: &serde_json::Value) -> bool {
    schema
        .as_object()
        .is_some_and(|object| object.contains_key("type") || object.contains_key("properties"))
}

#[derive(Debug, Deserialize)]
struct LambdaFile {
    lambdas: HashMap<String, LambdaDefinition>,
}

#[derive(Debug, Deserialize)]
struct LambdaDefinition {
    usage: LambdaUsage,
    argument: Option<TypeDefinition>,
    returns: Option<TypeDefinition>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
enum LambdaUsage {
    Variable,
    Section,
}

#[derive(Debug, Deserialize)]
struct TypeDefinition {
    #[serde(rename = "type")]
    kind: String,
}

fn load_lambdas(input: &SemanticInput) -> (HashMap<String, LambdaDefinition>, Vec<Diagnostic>) {
    let SemanticInput::Path(path) = input else {
        return (HashMap::new(), Vec::new());
    };

    let source = match fs::read_to_string(path) {
        Ok(source) => source,
        Err(error) => {
            return (
                HashMap::new(),
                vec![input_diagnostic(
                    IssueKind::LambdaInputError,
                    path,
                    format!("failed to read lambdas {}: {error}", path.display()),
                )],
            );
        }
    };

    match serde_json::from_str::<LambdaFile>(&source) {
        Ok(file) => {
            let _type_names_are_present = file
                .lambdas
                .values()
                .flat_map(|lambda| [lambda.argument.as_ref(), lambda.returns.as_ref()])
                .flatten()
                .all(|definition| !definition.kind.is_empty());
            (file.lambdas, Vec::new())
        }
        Err(error) => (
            HashMap::new(),
            vec![input_diagnostic(
                IssueKind::LambdaInputError,
                path,
                format!("failed to parse lambdas {}: {error}", path.display()),
            )],
        ),
    }
}

fn input_diagnostic(issue: IssueKind, path: &Path, message: String) -> Diagnostic {
    Diagnostic {
        severity: DiagnosticSeverity::Warning,
        issue,
        source_name: path.display().to_string(),
        location: SourceLocation { line: 1, column: 1 },
        span: SourceSpan::new(0, 0),
        message,
    }
}

fn validate_semantics(
    source: &str,
    source_name: &str,
    nodes: &[Node],
    schema: Option<&ContextSchema>,
    lambdas: &HashMap<String, LambdaDefinition>,
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    validate_nodes(
        source,
        source_name,
        nodes,
        schema.map(ContextSchema::root),
        lambdas,
        &mut diagnostics,
    );
    diagnostics
}

fn validate_nodes(
    source: &str,
    source_name: &str,
    nodes: &[Node],
    scope_schema: Option<&serde_json::Value>,
    lambdas: &HashMap<String, LambdaDefinition>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    for node in nodes {
        match node {
            Node::EscapedVariable { name, span } | Node::UnescapedVariable { name, span } => {
                if let Some(lambda) = lambdas.get(name) {
                    if lambda.usage != LambdaUsage::Variable {
                        diagnostics.push(source_diagnostic(
                            IssueKind::InvalidLambdaUsage,
                            source,
                            source_name,
                            span,
                            format!("lambda `{name}` is not valid as a variable"),
                        ));
                    }
                    continue;
                }
                validate_schema_path(source, source_name, scope_schema, name, span, diagnostics);
            }
            Node::Section {
                name,
                span,
                children,
            } => {
                if let Some(lambda) = lambdas.get(name) {
                    if lambda.usage != LambdaUsage::Section {
                        diagnostics.push(source_diagnostic(
                            IssueKind::InvalidLambdaUsage,
                            source,
                            source_name,
                            span,
                            format!("lambda `{name}` is not valid as a section"),
                        ));
                    }
                    validate_nodes(
                        source,
                        source_name,
                        children,
                        scope_schema,
                        lambdas,
                        diagnostics,
                    );
                    continue;
                }
                let resolved_scope = resolve_schema_path(scope_schema, name);
                if resolved_scope.is_none() {
                    validate_schema_path(
                        source,
                        source_name,
                        scope_schema,
                        name,
                        span,
                        diagnostics,
                    );
                } else if !supports_section_scope(resolved_scope) {
                    diagnostics.push(source_diagnostic(
                        IssueKind::UnexpectedSchemaType,
                        source,
                        source_name,
                        span,
                        format!("schema path `{name}` is not valid as a section"),
                    ));
                }
                validate_nodes(
                    source,
                    source_name,
                    children,
                    section_scope(resolved_scope.or(scope_schema)),
                    lambdas,
                    diagnostics,
                );
            }
            Node::InvertedSection {
                name,
                span,
                children,
            } => {
                if lambdas.contains_key(name) {
                    diagnostics.push(source_diagnostic(
                        IssueKind::InvalidLambdaUsage,
                        source,
                        source_name,
                        span,
                        format!("inverted lambda section `{name}` is unsupported"),
                    ));
                } else {
                    validate_schema_path(
                        source,
                        source_name,
                        scope_schema,
                        name,
                        span,
                        diagnostics,
                    );
                }
                validate_nodes(
                    source,
                    source_name,
                    children,
                    scope_schema,
                    lambdas,
                    diagnostics,
                );
            }
            Node::LambdaVariable { name, span } => {
                if !lambdas.contains_key(name) {
                    diagnostics.push(source_diagnostic(
                        IssueKind::InvalidLambdaUsage,
                        source,
                        source_name,
                        span,
                        format!("lambda `{name}` is not defined"),
                    ));
                }
            }
            Node::LambdaSection {
                name,
                span,
                children,
            } => {
                if !lambdas.contains_key(name) {
                    diagnostics.push(source_diagnostic(
                        IssueKind::InvalidLambdaUsage,
                        source,
                        source_name,
                        span,
                        format!("lambda `{name}` is not defined"),
                    ));
                }
                validate_nodes(
                    source,
                    source_name,
                    children,
                    scope_schema,
                    lambdas,
                    diagnostics,
                );
            }
            Node::Parent { children, .. } | Node::Block { children, .. } => {
                validate_nodes(
                    source,
                    source_name,
                    children,
                    scope_schema,
                    lambdas,
                    diagnostics,
                );
            }
            Node::Text { .. }
            | Node::Comment { .. }
            | Node::Partial { .. }
            | Node::DynamicPartial { .. }
            | Node::DelimiterChange { .. } => {}
        }
    }
}

fn validate_schema_path(
    source: &str,
    source_name: &str,
    schema: Option<&serde_json::Value>,
    name: &str,
    span: &std::ops::Range<usize>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let Some(schema) = schema else {
        return;
    };
    if name == "." || resolve_schema_path(Some(schema), name).is_some() {
        return;
    }
    diagnostics.push(source_diagnostic(
        IssueKind::MissingSchemaPath,
        source,
        source_name,
        span,
        format!("missing schema path `{name}`"),
    ));
}

fn resolve_schema_path<'a>(
    schema: Option<&'a serde_json::Value>,
    path: &str,
) -> Option<&'a serde_json::Value> {
    let mut current = schema?;
    for segment in path.split('.') {
        if segment.is_empty() {
            return None;
        }
        current = property_schema(current, segment)?;
    }
    Some(current)
}

fn property_schema<'a>(
    schema: &'a serde_json::Value,
    property: &str,
) -> Option<&'a serde_json::Value> {
    schema
        .get("properties")
        .and_then(serde_json::Value::as_object)
        .and_then(|properties| properties.get(property))
}

fn section_scope(schema: Option<&serde_json::Value>) -> Option<&serde_json::Value> {
    let schema = schema?;
    match schema.get("type").and_then(serde_json::Value::as_str) {
        Some("array") => schema.get("items").or(Some(schema)),
        Some("object") => Some(schema),
        _ => Some(schema),
    }
}

fn supports_section_scope(schema: Option<&serde_json::Value>) -> bool {
    let Some(schema) = schema else {
        return true;
    };
    matches!(
        schema.get("type").and_then(serde_json::Value::as_str),
        Some("object") | Some("array") | Some("boolean")
    )
}

fn source_diagnostic(
    issue: IssueKind,
    source: &str,
    source_name: &str,
    span: &std::ops::Range<usize>,
    message: String,
) -> Diagnostic {
    Diagnostic {
        severity: DiagnosticSeverity::Warning,
        issue,
        source_name: source_name.to_owned(),
        location: SourceLocation::for_offset(source, span.start),
        span: SourceSpan::new(span.start, span.end),
        message,
    }
}

pub(crate) fn format_diagnostic(diagnostic: &Diagnostic) -> String {
    format!(
        "{} {:?} at {}:{}:{}: {}",
        severity_label(diagnostic.severity),
        diagnostic.issue,
        diagnostic.source_name,
        diagnostic.location.line,
        diagnostic.location.column,
        diagnostic.message
    )
}

fn severity_label(severity: DiagnosticSeverity) -> &'static str {
    match severity {
        DiagnosticSeverity::Error => "error",
        DiagnosticSeverity::Warning => "warning",
        DiagnosticSeverity::Info => "info",
        DiagnosticSeverity::Debug => "debug",
    }
}
