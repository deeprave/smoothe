use std::{
    collections::{HashMap, HashSet},
    fs,
    path::{Path, PathBuf},
    process::ExitCode,
};

use serde::Deserialize;
use smoothe::config::{ResolvedCheckOptions, ResolvedGlobalOptions, SemanticInput};
use smoothe::content::{ContentInput, process as process_template};
use smoothe::parser::{
    Ast, Diagnostic, DiagnosticSeverity, IssueKind, Node, SourceLocation, SourceMetadata,
    SourceSpan, TemplateUnit,
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
            &result.ast,
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
    ast: &Ast,
    schema: Option<&ContextSchema>,
    lambdas: &HashMap<String, LambdaDefinition>,
) -> Vec<Diagnostic> {
    let mut validation = SemanticValidation {
        template_units: &ast.template_units,
        scope_schema: schema.map(ContextSchema::root),
        lambdas,
        diagnostics: Vec::new(),
        in_progress_template_units: HashSet::new(),
    };
    validation.validate_nodes(source, source_name, &ast.nodes);
    validation.diagnostics
}

struct SemanticValidation<'a> {
    template_units: &'a [TemplateUnit],
    scope_schema: Option<&'a serde_json::Value>,
    lambdas: &'a HashMap<String, LambdaDefinition>,
    diagnostics: Vec<Diagnostic>,
    in_progress_template_units: HashSet<usize>,
}

impl SemanticValidation<'_> {
    fn validate_nodes(&mut self, source: &str, source_name: &str, nodes: &[Node]) {
        for node in nodes {
            match node {
                Node::EscapedVariable { name, span } | Node::UnescapedVariable { name, span } => {
                    self.validate_variable(source, source_name, name, span);
                }
                Node::Section {
                    name,
                    span,
                    children,
                } => self.validate_section(source, source_name, name, span, children),
                Node::InvertedSection {
                    name,
                    span,
                    children,
                } => {
                    if self.lambdas.contains_key(name) {
                        self.diagnostics.push(source_diagnostic(
                            IssueKind::InvalidLambdaUsage,
                            source,
                            source_name,
                            span,
                            format!("inverted lambda section `{name}` is unsupported"),
                        ));
                    } else {
                        self.validate_schema_path(source, source_name, name, span);
                    }
                    self.validate_nodes(source, source_name, children);
                }
                Node::LambdaVariable { name, span } => {
                    if !self.lambdas.contains_key(name) {
                        self.diagnostics.push(source_diagnostic(
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
                    if !self.lambdas.contains_key(name) {
                        self.diagnostics.push(source_diagnostic(
                            IssueKind::InvalidLambdaUsage,
                            source,
                            source_name,
                            span,
                            format!("lambda `{name}` is not defined"),
                        ));
                    }
                    self.validate_nodes(source, source_name, children);
                }
                Node::Parent { children, .. } | Node::Block { children, .. } => {
                    self.validate_nodes(source, source_name, children);
                }
                Node::ResolvedPartial { template_id, .. } => {
                    if !self.in_progress_template_units.insert(*template_id) {
                        continue;
                    }
                    let Some(unit) = self.template_units.get(*template_id) else {
                        self.diagnostics.push(source_diagnostic(
                            IssueKind::UnresolvedPartial,
                            source,
                            source_name,
                            &(0..0),
                            format!(
                                "resolved partial references missing template unit `{template_id}`"
                            ),
                        ));
                        self.in_progress_template_units.remove(template_id);
                        continue;
                    };
                    self.validate_nodes(&unit.raw_data, &unit.source.name, &unit.nodes);
                    self.in_progress_template_units.remove(template_id);
                }
                Node::Text { .. }
                | Node::Comment { .. }
                | Node::Partial { .. }
                | Node::DynamicPartial { .. }
                | Node::DelimiterChange { .. } => {}
            }
        }
    }

    fn validate_variable(
        &mut self,
        source: &str,
        source_name: &str,
        name: &str,
        span: &std::ops::Range<usize>,
    ) {
        if let Some(lambda) = self.lambdas.get(name) {
            if lambda.usage != LambdaUsage::Variable {
                self.diagnostics.push(source_diagnostic(
                    IssueKind::InvalidLambdaUsage,
                    source,
                    source_name,
                    span,
                    format!("lambda `{name}` is not valid as a variable"),
                ));
            }
            return;
        }
        self.validate_schema_path(source, source_name, name, span);
    }

    fn validate_section(
        &mut self,
        source: &str,
        source_name: &str,
        name: &str,
        span: &std::ops::Range<usize>,
        children: &[Node],
    ) {
        if let Some(lambda) = self.lambdas.get(name) {
            if lambda.usage != LambdaUsage::Section {
                self.diagnostics.push(source_diagnostic(
                    IssueKind::InvalidLambdaUsage,
                    source,
                    source_name,
                    span,
                    format!("lambda `{name}` is not valid as a section"),
                ));
            }
            self.validate_nodes(source, source_name, children);
            return;
        }

        let resolved_scope = resolve_schema_path(self.scope_schema, name);
        if resolved_scope.is_none() {
            self.validate_schema_path(source, source_name, name, span);
        } else if !supports_section_scope(resolved_scope) {
            self.diagnostics.push(source_diagnostic(
                IssueKind::UnexpectedSchemaType,
                source,
                source_name,
                span,
                format!("schema path `{name}` is not valid as a section"),
            ));
        }

        let previous_scope = self.scope_schema;
        self.scope_schema = section_scope(resolved_scope.or(self.scope_schema));
        self.validate_nodes(source, source_name, children);
        self.scope_schema = previous_scope;
    }

    fn validate_schema_path(
        &mut self,
        source: &str,
        source_name: &str,
        name: &str,
        span: &std::ops::Range<usize>,
    ) {
        let Some(schema) = self.scope_schema else {
            return;
        };
        if name == "." || resolve_schema_path(Some(schema), name).is_some() {
            return;
        }
        self.diagnostics.push(source_diagnostic(
            IssueKind::MissingSchemaPath,
            source,
            source_name,
            span,
            format!("missing schema path `{name}`"),
        ));
    }
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
