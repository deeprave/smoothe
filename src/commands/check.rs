use std::{
    collections::{HashMap, HashSet},
    fs,
    path::{Path, PathBuf},
    process::ExitCode,
};

use smoothe::config::{ResolvedCheckOptions, ResolvedGlobalOptions, SemanticInput};
use smoothe::content::{ContentInput, process as process_template};
use smoothe::context_schema::{ContextSchema, ContextShape, PathResolution, SectionScope};
use smoothe::lambda::{LambdaSideEffects, LambdaSpec, LambdaUsage};
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
        let mut content_input = ContentInput::new(source, &input.source);
        content_input.lambdas = lambdas.values().cloned().collect();
        let result = process_template(content_input);

        for diagnostic in &result.state.diagnostics {
            eprintln!("{}", format_diagnostic(diagnostic));
        }
        let semantic_diagnostics = validate_semantics(
            &result.raw_data,
            &input.name,
            &result.ast,
            schema.as_ref(),
            &lambdas,
        );
        for diagnostic in &semantic_diagnostics {
            eprintln!("{}", format_diagnostic(diagnostic));
        }

        has_error |= result
            .state
            .diagnostics
            .iter()
            .chain(semantic_diagnostics.iter())
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
            let schema = ContextSchema::from_json(schema, path.display().to_string());
            let diagnostics = schema.diagnostics().to_vec();
            (Some(schema), diagnostics)
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
    schema.as_object().is_some_and(|object| {
        object.contains_key("type")
            || object.contains_key("properties")
            || object.contains_key("items")
            || object.contains_key("enum")
            || object.contains_key("additionalProperties")
            || object.contains_key("$ref")
            || object.contains_key("$defs")
            || object.contains_key("definitions")
            || object.contains_key("oneOf")
            || object.contains_key("anyOf")
            || object.contains_key("allOf")
            || object.contains_key("patternProperties")
    })
}

fn load_lambdas(input: &SemanticInput) -> (HashMap<String, LambdaSpec>, Vec<Diagnostic>) {
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

    match serde_json::from_str::<serde_json::Value>(&source) {
        Ok(value) => parse_lambda_file(value, path),
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

fn parse_lambda_file(
    value: serde_json::Value,
    path: &Path,
) -> (HashMap<String, LambdaSpec>, Vec<Diagnostic>) {
    let Some(root) = value.as_object() else {
        return (
            HashMap::new(),
            vec![input_diagnostic(
                IssueKind::LambdaInputError,
                path,
                format!("unrecognisable lambda definitions {}", path.display()),
            )],
        );
    };
    let Some(lambdas) = root.get("lambdas").and_then(serde_json::Value::as_object) else {
        return (
            HashMap::new(),
            vec![input_diagnostic(
                IssueKind::LambdaInputError,
                path,
                format!("unrecognisable lambda definitions {}", path.display()),
            )],
        );
    };

    let mut definitions = HashMap::new();
    let mut diagnostics = Vec::new();
    for (name, definition) in lambdas {
        let Some(object) = definition.as_object() else {
            diagnostics.push(input_diagnostic(
                IssueKind::LambdaInputError,
                path,
                format!("lambda `{name}` definition must be an object"),
            ));
            continue;
        };

        let Some(usage) = object
            .get("usage")
            .and_then(serde_json::Value::as_str)
            .and_then(parse_lambda_usage)
        else {
            diagnostics.push(input_diagnostic(
                IssueKind::LambdaInputError,
                path,
                format!("lambda `{name}` has invalid or missing `usage`"),
            ));
            continue;
        };

        let mut spec = LambdaSpec::new(name.clone()).with_usage(usage);
        if let Some(argument) = object.get("argument")
            && let Some(shape) = lambda_shape(argument, path, name, "argument", &mut diagnostics)
        {
            spec = spec.with_argument(shape);
        }
        if let Some(returns) = object.get("returns")
            && let Some(shape) = lambda_shape(returns, path, name, "returns", &mut diagnostics)
        {
            spec = spec.with_returns(shape);
        }
        if let Some(side_effects) = object.get("side_effects") {
            if let Some(side_effects) = side_effects.as_str().and_then(parse_lambda_side_effects) {
                spec = spec.with_side_effects(side_effects);
            } else {
                diagnostics.push(input_diagnostic(
                    IssueKind::LambdaInputError,
                    path,
                    format!("lambda `{name}` has invalid `side_effects`"),
                ));
            }
        }
        definitions.insert(name.clone(), spec);
    }

    (definitions, diagnostics)
}

fn parse_lambda_usage(value: &str) -> Option<LambdaUsage> {
    match value {
        "variable" => Some(LambdaUsage::Variable),
        "section" => Some(LambdaUsage::Section),
        "both" => Some(LambdaUsage::Both),
        _ => None,
    }
}

fn parse_lambda_side_effects(value: &str) -> Option<LambdaSideEffects> {
    match value {
        "none" => Some(LambdaSideEffects::None),
        "declared" => Some(LambdaSideEffects::Declared),
        "unknown" => Some(LambdaSideEffects::Unknown),
        _ => None,
    }
}

fn lambda_shape(
    value: &serde_json::Value,
    path: &Path,
    name: &str,
    field: &str,
    diagnostics: &mut Vec<Diagnostic>,
) -> Option<ContextShape> {
    if !value.is_object() {
        diagnostics.push(input_diagnostic(
            IssueKind::LambdaInputError,
            path,
            format!("lambda `{name}` `{field}` shape must be an object"),
        ));
        return None;
    }

    let schema = ContextSchema::from_json(value.clone(), path.display().to_string());
    diagnostics.extend(schema.diagnostics().iter().map(|diagnostic| {
        input_diagnostic(
            IssueKind::LambdaInputError,
            path,
            format!("lambda `{name}` `{field}` shape: {}", diagnostic.message),
        )
    }));
    Some(schema.root().clone())
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
    lambdas: &HashMap<String, LambdaSpec>,
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
    scope_schema: Option<&'a ContextShape>,
    lambdas: &'a HashMap<String, LambdaSpec>,
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
                        self.diagnostics.push(source_diagnostic_with_severity(
                            DiagnosticSeverity::Error,
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
                    self.validate_lambda_variable(source, source_name, name, span);
                }
                Node::LambdaSection {
                    name,
                    span,
                    children,
                } => {
                    self.validate_lambda_section(source, source_name, name, span);
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
            if !lambda.usage.allows_variable() {
                self.diagnostics.push(source_diagnostic(
                    IssueKind::InvalidLambdaUsage,
                    source,
                    source_name,
                    span,
                    format!("lambda `{name}` is not valid as a variable"),
                ));
            }
            self.warn_incompatible_variable_return(source, source_name, name, span, lambda);
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
            if !lambda.usage.allows_section() {
                self.diagnostics.push(source_diagnostic(
                    IssueKind::InvalidLambdaUsage,
                    source,
                    source_name,
                    span,
                    format!("lambda `{name}` is not valid as a section"),
                ));
            }
            self.warn_incompatible_section_shapes(source, source_name, name, span, lambda);
            self.validate_nodes(source, source_name, children);
            return;
        }

        let previous_scope = self.scope_schema;
        if let Some(schema) = self.scope_schema {
            match schema.section_scope(name) {
                SectionScope::Changed {
                    shape: next_scope,
                    optional,
                } => {
                    self.warn_optional_path(source, source_name, name, span, optional);
                    self.scope_schema = Some(next_scope);
                }
                SectionScope::Current { optional } => {
                    self.warn_optional_path(source, source_name, name, span, optional);
                }
                SectionScope::Invalid { shape, optional } => {
                    self.warn_optional_path(source, source_name, name, span, optional);
                    self.diagnostics.push(source_diagnostic(
                        IssueKind::UnexpectedSchemaType,
                        source,
                        source_name,
                        span,
                        invalid_section_message(name, shape),
                    ));
                }
                SectionScope::Missing {
                    missing_path,
                    known_fields,
                } => self.warn_missing_path(source, source_name, span, &missing_path, known_fields),
                SectionScope::Permissive { .. } => {}
                SectionScope::InvalidTraversal {
                    traversed_path,
                    shape,
                } => self.warn_invalid_traversal(source, source_name, span, &traversed_path, shape),
            }
        }
        self.validate_nodes(source, source_name, children);
        self.scope_schema = previous_scope;
    }

    fn validate_lambda_variable(
        &mut self,
        source: &str,
        source_name: &str,
        name: &str,
        span: &std::ops::Range<usize>,
    ) {
        if let Some(lambda) = self.lambdas.get(name) {
            if !lambda.usage.allows_variable() {
                self.diagnostics.push(source_diagnostic(
                    IssueKind::InvalidLambdaUsage,
                    source,
                    source_name,
                    span,
                    format!("lambda `{name}` is not valid as a variable"),
                ));
            }
            self.warn_incompatible_variable_return(source, source_name, name, span, lambda);
        }
    }

    fn validate_lambda_section(
        &mut self,
        source: &str,
        source_name: &str,
        name: &str,
        span: &std::ops::Range<usize>,
    ) {
        if let Some(lambda) = self.lambdas.get(name) {
            if !lambda.usage.allows_section() {
                self.diagnostics.push(source_diagnostic(
                    IssueKind::InvalidLambdaUsage,
                    source,
                    source_name,
                    span,
                    format!("lambda `{name}` is not valid as a section"),
                ));
            }
            self.warn_incompatible_section_shapes(source, source_name, name, span, lambda);
        }
    }

    fn warn_incompatible_variable_return(
        &mut self,
        source: &str,
        source_name: &str,
        name: &str,
        span: &std::ops::Range<usize>,
        lambda: &LambdaSpec,
    ) {
        let Some(returns) = &lambda.returns else {
            return;
        };
        if shape_renders_as_scalar(returns) {
            return;
        }
        self.diagnostics.push(source_diagnostic(
            IssueKind::LambdaTypeMismatch,
            source,
            source_name,
            span,
            format!(
                "lambda `{name}` return shape is incompatible with variable usage: {}",
                shape_description(returns)
            ),
        ));
    }

    fn warn_incompatible_section_shapes(
        &mut self,
        source: &str,
        source_name: &str,
        name: &str,
        span: &std::ops::Range<usize>,
        lambda: &LambdaSpec,
    ) {
        if let Some(argument) = &lambda.argument
            && !shape_accepts_section_argument(argument)
        {
            self.diagnostics.push(source_diagnostic(
                IssueKind::LambdaTypeMismatch,
                source,
                source_name,
                span,
                format!(
                    "lambda `{name}` argument shape is incompatible with section usage: {}",
                    shape_description(argument)
                ),
            ));
        }

        let Some(returns) = &lambda.returns else {
            return;
        };
        if shape_renders_as_scalar(returns) {
            return;
        }
        self.diagnostics.push(source_diagnostic(
            IssueKind::LambdaTypeMismatch,
            source,
            source_name,
            span,
            format!(
                "lambda `{name}` return shape is incompatible with section usage: {}",
                shape_description(returns)
            ),
        ));
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
        match schema.resolve_path(name) {
            PathResolution::Found { optional, .. } => {
                self.warn_optional_path(source, source_name, name, span, optional);
            }
            PathResolution::Missing {
                missing_path,
                known_fields,
            } => self.warn_missing_path(source, source_name, span, &missing_path, known_fields),
            PathResolution::Permissive { .. } => {}
            PathResolution::InvalidTraversal {
                traversed_path,
                shape,
            } => self.warn_invalid_traversal(source, source_name, span, &traversed_path, shape),
        }
    }

    fn warn_missing_path(
        &mut self,
        source: &str,
        source_name: &str,
        span: &std::ops::Range<usize>,
        missing_path: &str,
        known_fields: Vec<String>,
    ) {
        let mut message = format!("missing schema path `{missing_path}`");
        if !known_fields.is_empty() {
            message.push_str(&format!("; known fields: {}", known_fields.join(", ")));
        }
        self.diagnostics.push(source_diagnostic(
            IssueKind::MissingSchemaPath,
            source,
            source_name,
            span,
            message,
        ));
    }

    fn warn_optional_path(
        &mut self,
        source: &str,
        source_name: &str,
        name: &str,
        span: &std::ops::Range<usize>,
        optional: Option<String>,
    ) {
        let Some(optional_path) = optional else {
            return;
        };
        self.diagnostics.push(source_diagnostic(
            IssueKind::OptionalSchemaPath,
            source,
            source_name,
            span,
            format!("schema path `{name}` depends on optional field `{optional_path}`"),
        ));
    }

    fn warn_invalid_traversal(
        &mut self,
        source: &str,
        source_name: &str,
        span: &std::ops::Range<usize>,
        traversed_path: &str,
        shape: &ContextShape,
    ) {
        self.diagnostics.push(source_diagnostic(
            IssueKind::InvalidSchemaTraversal,
            source,
            source_name,
            span,
            format!(
                "schema path `{traversed_path}` cannot be traversed because it is {}",
                shape_description(shape)
            ),
        ));
    }
}

fn source_diagnostic(
    issue: IssueKind,
    source: &str,
    source_name: &str,
    span: &std::ops::Range<usize>,
    message: String,
) -> Diagnostic {
    source_diagnostic_with_severity(
        DiagnosticSeverity::Warning,
        issue,
        source,
        source_name,
        span,
        message,
    )
}

fn source_diagnostic_with_severity(
    severity: DiagnosticSeverity,
    issue: IssueKind,
    source: &str,
    source_name: &str,
    span: &std::ops::Range<usize>,
    message: String,
) -> Diagnostic {
    Diagnostic {
        severity,
        issue,
        source_name: source_name.to_owned(),
        location: SourceLocation::for_offset(source, span.start),
        span: SourceSpan::new(span.start, span.end),
        message,
    }
}

fn invalid_section_message(name: &str, shape: &ContextShape) -> String {
    let mut message = format!(
        "schema path `{name}` is not valid as a section because it is {}",
        shape_description(shape)
    );
    if let ContextShape::Scalar { enum_values, .. } = shape
        && !enum_values.is_empty()
    {
        let values = enum_values
            .iter()
            .map(serde_json::Value::to_string)
            .collect::<Vec<_>>()
            .join(", ");
        message.push_str(&format!("; allowed values: {values}"));
    }
    message
}

fn shape_description(shape: &ContextShape) -> String {
    match shape {
        ContextShape::Object(_) => "object".to_owned(),
        ContextShape::Array(_) => "array".to_owned(),
        ContextShape::Scalar { kind, .. } => format!("{} scalar", kind.as_str()),
        ContextShape::Any => "any value".to_owned(),
        ContextShape::Unknown => "unknown schema shape".to_owned(),
        ContextShape::Unsupported => "unsupported schema shape".to_owned(),
    }
}

fn shape_accepts_section_argument(shape: &ContextShape) -> bool {
    matches!(
        shape,
        ContextShape::Scalar {
            kind: smoothe::context_schema::ScalarKind::String,
            ..
        } | ContextShape::Any
            | ContextShape::Unknown
            | ContextShape::Unsupported
    )
}

fn shape_renders_as_scalar(shape: &ContextShape) -> bool {
    matches!(
        shape,
        ContextShape::Scalar { .. }
            | ContextShape::Any
            | ContextShape::Unknown
            | ContextShape::Unsupported
    )
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
