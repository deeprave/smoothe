use std::{
    collections::{HashMap, HashSet},
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::ExitCode,
};

use serde::Serialize;
use smoothe::check_events::{
    CheckEvent, CheckEventBus, CheckEventLevel, CheckEventListener, PartialEvent, ProgressEvent,
};
use smoothe::config::{
    CheckOutputFormat, CheckVerbosity, ResolvedCheckOptions, ResolvedGlobalOptions, SemanticInput,
};
use smoothe::content::{ContentInput, process as process_template};
use smoothe::context_schema::{ContextSchema, ContextShape, PathResolution, SectionScope};
use smoothe::lambda::{LambdaSideEffects, LambdaSpec, LambdaUsage};
use smoothe::parser::{
    Ast, Diagnostic, DiagnosticSeverity, DiagnosticSuggestionKind, IssueKind, Node, SourceLocation,
    SourceMetadata, SourceSpan, TemplateUnit, near_hit_suggestions,
};

use crate::cli::CheckArgs;

use super::read_template_inputs;

pub fn check(
    args: CheckArgs,
    global_options: ResolvedGlobalOptions,
    check_options: ResolvedCheckOptions,
) -> ExitCode {
    let _color = global_options.color;
    let output_options = CheckOutputOptions::resolve(&args, &check_options);
    let mut events = output_options.event_bus();

    let inputs = match read_template_inputs(&args.inputs) {
        Ok(inputs) => inputs,
        Err(error) => {
            eprintln!("error: {error}");
            return ExitCode::FAILURE;
        }
    };

    let semantic_inputs = SemanticInputs::resolve(&args, &check_options);
    let (schema, schema_diagnostics) = load_schema(&semantic_inputs.schema);
    let (lambdas, lambda_diagnostics) = load_lambdas(&semantic_inputs.lambdas);
    let mut has_error = false;

    let _ = events.emit(CheckEvent::run_started(inputs.len()));
    has_error |= emit_diagnostics(&mut events, schema_diagnostics);
    has_error |= emit_diagnostics(&mut events, lambda_diagnostics);

    for mut input in inputs {
        let source_name = std::mem::take(&mut input.name);
        let _ = events.emit(CheckEvent::InputStarted {
            source_name: source_name.clone(),
        });
        let _ = events.emit(CheckEvent::progress(
            CheckEventLevel::Trace,
            ProgressEvent::new(format!("check-input {source_name}")),
        ));
        let mut source = SourceMetadata::new(&source_name);
        if let Some(root) = input.root {
            source = source.with_root(root);
        }
        let mut content_input = ContentInput::new(source, &input.source);
        content_input.lambdas = lambdas.values().cloned().collect();
        let result = process_template(content_input);
        let mut input_has_error = false;

        input_has_error |= emit_diagnostics(&mut events, result.state.diagnostics);
        input_has_error |= validate_semantics_with_events(
            &result.raw_data,
            &source_name,
            &result.ast,
            schema.as_ref(),
            &lambdas,
            &mut events,
        );

        has_error |= input_has_error;
        let _ = events.emit(CheckEvent::InputFinished {
            source_name,
            has_error: input_has_error,
        });
    }
    if let Err(error) = events.emit(CheckEvent::run_finished(has_error)) {
        eprintln!("error: check output listener failed; output may be incomplete: {error}");
        return ExitCode::FAILURE;
    }

    if let Some(error) = events.failure() {
        // Listener failures are output/runtime failures independent of
        // validation diagnostics. Diagnostics may already have been emitted;
        // the command still fails so automation can detect incomplete output.
        eprintln!("error: check output listener failed; output may be incomplete: {error}");
        return ExitCode::FAILURE;
    }

    if has_error {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

fn emit_diagnostics(
    events: &mut CheckEventBus,
    diagnostics: impl IntoIterator<Item = Diagnostic>,
) -> bool {
    let mut has_error = false;
    for diagnostic in diagnostics {
        has_error |= diagnostic.severity == DiagnosticSeverity::Error;
        let _ = events.emit(CheckEvent::diagnostic(diagnostic));
    }
    has_error
}

struct CheckOutputOptions {
    output: CheckOutputFormat,
    verbosity: CheckVerbosity,
}

impl CheckOutputOptions {
    fn resolve(args: &CheckArgs, options: &ResolvedCheckOptions) -> Self {
        Self {
            output: resolve_check_output_format(args, options),
            verbosity: args.verbosity.unwrap_or(options.verbosity),
        }
    }

    fn event_bus(&self) -> CheckEventBus {
        let mut bus = CheckEventBus::new();
        match self.output {
            CheckOutputFormat::Compiler => {
                bus.add_listener(CompilerListener::new(verbosity_level(self.verbosity)));
            }
            CheckOutputFormat::Json => {
                bus.add_listener(JsonListener::new(verbosity_level(self.verbosity)));
            }
        }
        bus
    }
}

fn resolve_check_output_format(
    args: &CheckArgs,
    options: &ResolvedCheckOptions,
) -> CheckOutputFormat {
    // Precedence is intentionally explicit: --format is the exact selector,
    // --json/--no-json only change the configured default.
    if let Some(format) = args.format {
        return format;
    }
    if args.json {
        return CheckOutputFormat::Json;
    }
    if args.no_json {
        return CheckOutputFormat::Compiler;
    }
    options.output
}

fn verbosity_level(value: CheckVerbosity) -> CheckEventLevel {
    match value {
        CheckVerbosity::Error => CheckEventLevel::Error,
        CheckVerbosity::Warning => CheckEventLevel::Warning,
        CheckVerbosity::Info => CheckEventLevel::Info,
        CheckVerbosity::Debug => CheckEventLevel::Debug,
        CheckVerbosity::Trace => CheckEventLevel::Trace,
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
    Diagnostic::new(
        DiagnosticSeverity::Warning,
        issue,
        path.display().to_string(),
        SourceLocation { line: 1, column: 1 },
        SourceSpan::new(0, 0),
        message,
    )
}

struct CompilerListener {
    verbosity: CheckEventLevel,
}

impl CompilerListener {
    fn new(verbosity: CheckEventLevel) -> Self {
        Self { verbosity }
    }
}

impl CheckEventListener for CompilerListener {
    fn on_event(&mut self, event: &CheckEvent) -> Result<(), String> {
        if !event.level().visible_at(self.verbosity) {
            return Ok(());
        }
        match event {
            CheckEvent::Diagnostic(event) => {
                eprintln!("{}", format_compiler_diagnostic(&event.diagnostic));
            }
            CheckEvent::PartialStarted(event) => {
                eprintln!("{}", format_partial_event("partial-started", event));
            }
            CheckEvent::PartialFinished(event) => {
                eprintln!("{}", format_partial_event("partial-finished", event));
            }
            CheckEvent::PartialSkipped(event) => {
                eprintln!("{}", format_partial_event("partial-skipped", event));
            }
            CheckEvent::Progress(event) | CheckEvent::Trace(event) => {
                eprintln!("{}: {}", event.level.as_str(), event.message);
            }
            CheckEvent::RunStarted { .. }
            | CheckEvent::InputStarted { .. }
            | CheckEvent::InputFinished { .. }
            | CheckEvent::RunFinished { .. } => {}
        }
        Ok(())
    }
}

fn format_partial_event(kind: &str, event: &PartialEvent) -> String {
    let path = event
        .path
        .as_ref()
        .map(|path| path.display().to_string())
        .unwrap_or_else(|| "<unresolved>".to_owned());
    format!(
        "partial: {}: {kind}: {} {path}",
        event.level.as_str(),
        event.name
    )
}

fn format_compiler_diagnostic(diagnostic: &Diagnostic) -> String {
    let mut output = format!(
        "{}:{}:{}: {}: {}: {}",
        diagnostic.source_name,
        diagnostic.location.line,
        diagnostic.location.column,
        severity_label(diagnostic.severity),
        diagnostic.issue.as_str(),
        diagnostic.message
    );
    append_diagnostic_details(&mut output, diagnostic);
    output
}

struct JsonListener {
    verbosity: CheckEventLevel,
    diagnostics: JsonDiagnosticGroups,
    events: Vec<JsonCheckEvent>,
    inputs: Vec<JsonCheckInput>,
    current_input: Option<JsonCheckInput>,
    finished: bool,
    writer: Box<dyn Write>,
}

impl JsonListener {
    fn new(verbosity: CheckEventLevel) -> Self {
        Self::with_writer(std::io::stdout(), verbosity)
    }

    fn with_writer(writer: impl Write + 'static, verbosity: CheckEventLevel) -> Self {
        Self {
            verbosity,
            diagnostics: JsonDiagnosticGroups::default(),
            events: Vec::new(),
            inputs: Vec::new(),
            current_input: None,
            finished: false,
            writer: Box::new(writer),
        }
    }

    fn current_input_mut(&mut self) -> Option<&mut JsonCheckInput> {
        self.current_input.as_mut()
    }
}

impl CheckEventListener for JsonListener {
    fn on_event(&mut self, event: &CheckEvent) -> Result<(), String> {
        // A JSON listener is single-use. This also prevents a second
        // RunFinished event from serializing an empty document.
        if self.finished {
            return Err("JSON check listener received an event after run-finished".to_owned());
        }
        match event {
            CheckEvent::InputStarted { source_name } => {
                if self.current_input.is_some() {
                    return Err("JSON check listener received input-started before finishing the previous input".to_owned());
                }
                self.current_input = Some(JsonCheckInput::new(source_name.clone()));
            }
            CheckEvent::InputFinished { has_error, .. } => {
                let Some(mut input) = self.current_input.take() else {
                    return Err(
                        "JSON check listener received input-finished without input-started"
                            .to_owned(),
                    );
                };
                input.has_error = *has_error;
                self.inputs.push(input);
            }
            CheckEvent::Diagnostic(event) => {
                if !event.level.visible_at(self.verbosity) {
                    return Ok(());
                }
                let diagnostic = JsonCheckDiagnostic::from(&event.diagnostic, event.level);
                if let Some(input) = self.current_input_mut() {
                    input
                        .diagnostics
                        .add_diagnostic(event.diagnostic.severity, diagnostic);
                } else {
                    self.diagnostics
                        .add_diagnostic(event.diagnostic.severity, diagnostic);
                }
            }
            CheckEvent::PartialStarted(event) => self.push_event(event.level, || {
                JsonCheckEvent::partial("partial-started", event)
            }),
            CheckEvent::PartialFinished(event) => {
                self.push_event(event.level, || {
                    JsonCheckEvent::partial("partial-finished", event)
                });
            }
            CheckEvent::PartialSkipped(event) => self.push_event(event.level, || {
                JsonCheckEvent::partial("partial-skipped", event)
            }),
            CheckEvent::Progress(event) => self.push_event(event.level, || {
                JsonCheckEvent::message("progress", event.level, event.message.clone())
            }),
            CheckEvent::Trace(event) => self.push_event(event.level, || {
                JsonCheckEvent::message("trace", event.level, event.message.clone())
            }),
            CheckEvent::RunFinished { has_error } => {
                if self.current_input.is_some() {
                    return Err(
                        "JSON check listener received run-finished while an input was still in progress"
                            .to_owned(),
                    );
                }
                self.finished = true;
                let output = JsonCheckOutput {
                    has_error: *has_error,
                    diagnostics: std::mem::take(&mut self.diagnostics),
                    events: std::mem::take(&mut self.events),
                    inputs: std::mem::take(&mut self.inputs),
                };
                serde_json::to_writer_pretty(&mut self.writer, &output)
                    .map_err(|error| format!("failed to serialize JSON output: {error}"))?;
                writeln!(&mut self.writer)
                    .map_err(|error| format!("failed to write JSON output: {error}"))?;
            }
            CheckEvent::RunStarted { .. } => {}
        }
        Ok(())
    }
}

impl JsonListener {
    fn push_event(&mut self, level: CheckEventLevel, build: impl FnOnce() -> JsonCheckEvent) {
        if !level.visible_at(self.verbosity) {
            return;
        }
        if let Some(input) = self.current_input_mut() {
            input.events.push(build());
        } else {
            self.events.push(build());
        }
    }
}

#[derive(Serialize)]
struct JsonCheckOutput {
    has_error: bool,
    #[serde(flatten)]
    diagnostics: JsonDiagnosticGroups,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    events: Vec<JsonCheckEvent>,
    inputs: Vec<JsonCheckInput>,
}

#[derive(Serialize)]
struct JsonCheckInput {
    source: String,
    has_error: bool,
    #[serde(flatten)]
    diagnostics: JsonDiagnosticGroups,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    events: Vec<JsonCheckEvent>,
}

impl JsonCheckInput {
    fn new(source: String) -> Self {
        Self {
            source,
            has_error: false,
            diagnostics: JsonDiagnosticGroups::default(),
            events: Vec::new(),
        }
    }
}

#[derive(Default, Serialize)]
struct JsonDiagnosticGroups {
    errors: Vec<JsonCheckDiagnostic>,
    warnings: Vec<JsonCheckDiagnostic>,
    info: Vec<JsonCheckDiagnostic>,
    debug: Vec<JsonCheckDiagnostic>,
}

impl JsonDiagnosticGroups {
    fn add_diagnostic(&mut self, severity: DiagnosticSeverity, diagnostic: JsonCheckDiagnostic) {
        match severity {
            DiagnosticSeverity::Error => self.errors.push(diagnostic),
            DiagnosticSeverity::Warning => self.warnings.push(diagnostic),
            DiagnosticSeverity::Info => self.info.push(diagnostic),
            DiagnosticSeverity::Debug => self.debug.push(diagnostic),
        }
    }
}

#[derive(Serialize)]
struct JsonCheckDiagnostic {
    level: String,
    severity: String,
    issue: String,
    source: String,
    line: usize,
    column: usize,
    span: JsonSpan,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    expected: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    found: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expectation_source: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    notes: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    suggestions: Vec<JsonSuggestion>,
}

impl JsonCheckDiagnostic {
    fn from(diagnostic: &Diagnostic, level: CheckEventLevel) -> Self {
        Self {
            level: level.as_str().to_owned(),
            severity: severity_label(diagnostic.severity).to_owned(),
            issue: diagnostic.issue.as_str().to_owned(),
            source: diagnostic.source_name.clone(),
            line: diagnostic.location.line,
            column: diagnostic.location.column,
            span: JsonSpan::from(diagnostic.span),
            message: diagnostic.message.clone(),
            expected: diagnostic.details.expected.clone(),
            found: diagnostic.details.found.clone(),
            expectation_source: diagnostic.details.expectation_source.clone(),
            notes: diagnostic.details.notes.clone(),
            suggestions: diagnostic
                .details
                .suggestions
                .iter()
                .map(|suggestion| JsonSuggestion {
                    kind: suggestion.kind.as_str().to_owned(),
                    value: suggestion.value.clone(),
                })
                .collect(),
        }
    }
}

#[derive(Serialize)]
struct JsonSpan {
    start: usize,
    end: usize,
}

impl From<SourceSpan> for JsonSpan {
    fn from(span: SourceSpan) -> Self {
        Self {
            start: span.start,
            end: span.end,
        }
    }
}

#[derive(Serialize)]
struct JsonSuggestion {
    kind: String,
    value: String,
}

#[derive(Serialize)]
struct JsonCheckEvent {
    kind: String,
    level: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<String>,
}

impl JsonCheckEvent {
    fn message(kind: &str, level: CheckEventLevel, message: String) -> Self {
        Self {
            kind: kind.to_owned(),
            level: level.as_str().to_owned(),
            message: Some(message),
            name: None,
            path: None,
        }
    }

    fn partial(kind: &str, event: &PartialEvent) -> Self {
        Self {
            kind: kind.to_owned(),
            level: event.level.as_str().to_owned(),
            message: None,
            name: Some(event.name.clone()),
            path: event.path.as_ref().map(|path| path.display().to_string()),
        }
    }
}

fn validate_semantics_with_events(
    source: &str,
    source_name: &str,
    ast: &Ast,
    schema: Option<&ContextSchema>,
    lambdas: &HashMap<String, LambdaSpec>,
    events: &mut CheckEventBus,
) -> bool {
    let mut validation = SemanticValidation {
        template_units: &ast.template_units,
        scope_schema: schema.map(ContextSchema::root),
        lambdas,
        events,
        has_error: false,
        in_progress_template_units: HashSet::new(),
    };
    validation.validate_nodes(source, source_name, &ast.nodes);
    validation.has_error
}

struct SemanticValidation<'a, 'events> {
    template_units: &'a [TemplateUnit],
    scope_schema: Option<&'a ContextShape>,
    lambdas: &'a HashMap<String, LambdaSpec>,
    events: &'events mut CheckEventBus,
    has_error: bool,
    in_progress_template_units: HashSet<usize>,
}

impl SemanticValidation<'_, '_> {
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
                        self.emit_diagnostic(source_diagnostic_with_severity(
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
                Node::ResolvedPartial {
                    name,
                    span,
                    resolved_path,
                    template_id,
                    recursive,
                } => {
                    let partial_event = PartialEvent::new(name.clone())
                        .with_path(resolved_path.clone())
                        .with_referrer(
                            source_name.to_owned(),
                            SourceSpan::new(span.start, span.end),
                        )
                        .recursive(*recursive);
                    if *recursive {
                        let _ = self
                            .events
                            .emit(CheckEvent::PartialSkipped(partial_event.clone()));
                        self.emit_recursive_partial_skipped(source, source_name, span, name);
                        continue;
                    }
                    if !self.in_progress_template_units.insert(*template_id) {
                        let _ = self
                            .events
                            .emit(CheckEvent::PartialSkipped(partial_event.recursive(true)));
                        self.emit_recursive_partial_skipped(source, source_name, span, name);
                        continue;
                    }
                    let Some(unit) = self.template_units.get(*template_id) else {
                        self.emit_diagnostic(source_diagnostic(
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
                    let _ = self
                        .events
                        .emit(CheckEvent::PartialStarted(partial_event.clone()));
                    self.validate_nodes(&unit.raw_data, &unit.source.name, &unit.nodes);
                    let _ = self.events.emit(CheckEvent::PartialFinished(partial_event));
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
                self.warn_invalid_lambda_usage(source, source_name, name, span, lambda, "variable");
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
                self.warn_invalid_lambda_usage(source, source_name, name, span, lambda, "section");
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
                    self.emit_diagnostic(
                        source_diagnostic(
                            IssueKind::UnexpectedSchemaType,
                            source,
                            source_name,
                            span,
                            invalid_section_message(name, shape),
                        )
                        .with_expected("object, array, boolean, lambda, or permissive section")
                        .with_found(shape_description(shape))
                        .with_expectation_source("context schema")
                        .with_suggestions(enum_value_suggestions(shape)),
                    );
                }
                SectionScope::Missing {
                    missing_path,
                    known_fields,
                } => {
                    self.warn_missing_path_with_note(
                        source,
                        source_name,
                        span,
                        &missing_path,
                        known_fields,
                        format!(
                            "references inside unknown section `{name}` were not fully validated"
                        ),
                    );
                    return;
                }
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

    fn emit_recursive_partial_skipped(
        &mut self,
        source: &str,
        source_name: &str,
        span: &std::ops::Range<usize>,
        name: &str,
    ) {
        self.emit_diagnostic(source_diagnostic(
            IssueKind::PartialSkipped,
            source,
            source_name,
            span,
            format!(
                "recursive partial `{name}` detected; body was not validated to avoid infinite recursion"
            ),
        ));
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
                self.warn_invalid_lambda_usage(source, source_name, name, span, lambda, "variable");
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
                self.warn_invalid_lambda_usage(source, source_name, name, span, lambda, "section");
            }
            self.warn_incompatible_section_shapes(source, source_name, name, span, lambda);
        }
    }

    fn warn_invalid_lambda_usage(
        &mut self,
        source: &str,
        source_name: &str,
        name: &str,
        span: &std::ops::Range<usize>,
        lambda: &LambdaSpec,
        actual_usage: &str,
    ) {
        self.emit_diagnostic(
            source_diagnostic(
                IssueKind::InvalidLambdaUsage,
                source,
                source_name,
                span,
                format!("lambda `{name}` is not valid as a {actual_usage}"),
            )
            .with_expected(format!("{} lambda usage", lambda.usage.as_str()))
            .with_found(format!("{actual_usage} lambda usage"))
            .with_expectation_source("lambda definitions")
            .with_note(format!("side effects: {}", lambda.side_effects.as_str())),
        );
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
        self.emit_diagnostic(
            source_diagnostic(
                IssueKind::LambdaTypeMismatch,
                source,
                source_name,
                span,
                format!(
                    "lambda `{name}` return shape is incompatible with variable usage: {}",
                    shape_description(returns)
                ),
            )
            .with_expected("scalar-renderable return shape")
            .with_found(shape_description(returns))
            .with_expectation_source("lambda definitions")
            .with_note(format!("side effects: {}", lambda.side_effects.as_str())),
        );
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
            self.emit_diagnostic(
                source_diagnostic(
                    IssueKind::LambdaTypeMismatch,
                    source,
                    source_name,
                    span,
                    format!(
                        "lambda `{name}` argument shape is incompatible with section usage: {}",
                        shape_description(argument)
                    ),
                )
                .with_expected("string section argument")
                .with_found(shape_description(argument))
                .with_expectation_source("lambda definitions")
                .with_note(format!("side effects: {}", lambda.side_effects.as_str())),
            );
        }

        let Some(returns) = &lambda.returns else {
            return;
        };
        if shape_renders_as_scalar(returns) {
            return;
        }
        self.emit_diagnostic(
            source_diagnostic(
                IssueKind::LambdaTypeMismatch,
                source,
                source_name,
                span,
                format!(
                    "lambda `{name}` return shape is incompatible with section usage: {}",
                    shape_description(returns)
                ),
            )
            .with_expected("scalar-renderable return shape")
            .with_found(shape_description(returns))
            .with_expectation_source("lambda definitions")
            .with_note(format!("side effects: {}", lambda.side_effects.as_str())),
        );
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
        self.emit_diagnostic(
            source_diagnostic(
                IssueKind::MissingSchemaPath,
                source,
                source_name,
                span,
                message,
            )
            .with_expected(schema_fields_expectation(&known_fields))
            .with_found(missing_path)
            .with_expectation_source("context schema")
            .with_suggestions(schema_field_suggestions(missing_path, &known_fields)),
        );
    }

    fn warn_missing_path_with_note(
        &mut self,
        source: &str,
        source_name: &str,
        span: &std::ops::Range<usize>,
        missing_path: &str,
        known_fields: Vec<String>,
        note: String,
    ) {
        let mut message = format!("missing schema path `{missing_path}`");
        if !known_fields.is_empty() {
            message.push_str(&format!("; known fields: {}", known_fields.join(", ")));
        }
        self.emit_diagnostic(
            source_diagnostic(
                IssueKind::MissingSchemaPath,
                source,
                source_name,
                span,
                message,
            )
            .with_expected(schema_fields_expectation(&known_fields))
            .with_found(missing_path)
            .with_expectation_source("context schema")
            .with_note(note)
            .with_suggestions(schema_field_suggestions(missing_path, &known_fields)),
        );
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
        self.emit_diagnostic(
            source_diagnostic(
                IssueKind::OptionalSchemaPath,
                source,
                source_name,
                span,
                format!("schema path `{name}` depends on optional field `{optional_path}`"),
            )
            .with_expected("required schema path")
            .with_found(format!("optional schema path `{optional_path}`"))
            .with_expectation_source("context schema"),
        );
    }

    fn warn_invalid_traversal(
        &mut self,
        source: &str,
        source_name: &str,
        span: &std::ops::Range<usize>,
        traversed_path: &str,
        shape: &ContextShape,
    ) {
        self.emit_diagnostic(
            source_diagnostic(
                IssueKind::InvalidSchemaTraversal,
                source,
                source_name,
                span,
                format!(
                    "schema path `{traversed_path}` cannot be traversed because it is {}",
                    shape_description(shape)
                ),
            )
            .with_expected("traversable object path")
            .with_found(shape_description(shape))
            .with_expectation_source("context schema"),
        );
    }

    fn emit_diagnostic(&mut self, diagnostic: Diagnostic) {
        self.has_error |= diagnostic.severity == DiagnosticSeverity::Error;
        let _ = self.events.emit(CheckEvent::diagnostic(diagnostic));
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
    Diagnostic::new(
        severity,
        issue,
        source_name,
        SourceLocation::for_offset(source, span.start),
        SourceSpan::new(span.start, span.end),
        message,
    )
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
    let mut output = format!(
        "{} {:?} at {}:{}:{}: {}",
        severity_label(diagnostic.severity),
        diagnostic.issue,
        diagnostic.source_name,
        diagnostic.location.line,
        diagnostic.location.column,
        diagnostic.message
    );
    append_diagnostic_details(&mut output, diagnostic);
    output
}

fn append_diagnostic_details(output: &mut String, diagnostic: &Diagnostic) {
    if let Some(expected) = &diagnostic.details.expected {
        output.push_str(&format!("; expected: {expected}"));
    }
    if let Some(found) = &diagnostic.details.found {
        output.push_str(&format!("; found: {found}"));
    }
    if let Some(source) = &diagnostic.details.expectation_source {
        output.push_str(&format!("; source: {source}"));
    }
    if !diagnostic.details.notes.is_empty() {
        output.push_str(&format!("; notes: {}", diagnostic.details.notes.join("; ")));
    }
    if !diagnostic.details.suggestions.is_empty() {
        let suggestions = diagnostic
            .details
            .suggestions
            .iter()
            .map(|suggestion| suggestion.value.as_str())
            .collect::<Vec<_>>()
            .join(", ");
        output.push_str(&format!("; suggestions: {suggestions}"));
    }
}

fn schema_fields_expectation(known_fields: &[String]) -> String {
    if known_fields.is_empty() {
        "defined schema path".to_owned()
    } else {
        format!("one of schema fields {}", known_fields.join(", "))
    }
}

fn schema_field_suggestions(
    missing_path: &str,
    known_fields: &[String],
) -> Vec<smoothe::parser::DiagnosticSuggestion> {
    let target = missing_path.rsplit('.').next().unwrap_or(missing_path);
    near_hit_suggestions(
        target,
        known_fields,
        DiagnosticSuggestionKind::SchemaField,
        3,
    )
}

fn enum_value_suggestions(shape: &ContextShape) -> Vec<smoothe::parser::DiagnosticSuggestion> {
    let ContextShape::Scalar { enum_values, .. } = shape else {
        return Vec::new();
    };

    enum_values
        .iter()
        .map(serde_json::Value::to_string)
        .map(|value| {
            smoothe::parser::DiagnosticSuggestion::new(DiagnosticSuggestionKind::SchemaValue, value)
        })
        .collect()
}

fn severity_label(severity: DiagnosticSeverity) -> &'static str {
    match severity {
        DiagnosticSeverity::Error => "error",
        DiagnosticSeverity::Warning => "warning",
        DiagnosticSeverity::Info => "info",
        DiagnosticSeverity::Debug => "debug",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_listener_rejects_run_finished_while_input_is_in_progress() {
        let mut listener = JsonListener::with_writer(Vec::new(), CheckEventLevel::Warning);

        listener
            .on_event(&CheckEvent::InputStarted {
                source_name: "template.mustache".to_owned(),
            })
            .expect("input start");
        let error = listener
            .on_event(&CheckEvent::run_finished(false))
            .expect_err("run-finished should fail while input is active");

        assert!(error.contains("input was still in progress"));
    }
}
