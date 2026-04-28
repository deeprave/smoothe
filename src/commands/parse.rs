use std::{fs, process::ExitCode};

use serde::Serialize;
use smoothe::content::{ContentInput, process as process_template};
use smoothe::parser::{Ast, Diagnostic, DiagnosticSeverity, Node, SourceMetadata, TemplateName};

use crate::cli::ParseArgs;

use super::{check::format_diagnostic, read_template_inputs};

pub fn parse(args: ParseArgs) -> ExitCode {
    let inputs = match read_template_inputs(&args.inputs) {
        Ok(inputs) => inputs,
        Err(error) => {
            eprintln!("error: {error}");
            return ExitCode::FAILURE;
        }
    };

    let mut diagnostics_output = String::new();
    let mut ast_output = String::new();
    let mut json_inputs = Vec::new();
    let mut has_error = false;

    for input in inputs {
        let mut source = SourceMetadata::new(&input.name);
        if let Some(root) = input.root {
            source = source.with_root(root);
        }
        let result = process_template(ContentInput::new(source, &input.source));

        if args.json {
            json_inputs.push(JsonInputResult::new(
                input.name.clone(),
                &result.ast,
                &result.state.diagnostics,
            ));
        } else {
            for diagnostic in &result.state.diagnostics {
                diagnostics_output.push_str(&format_diagnostic(diagnostic));
                diagnostics_output.push('\n');
            }

            ast_output.push_str(&format_ast(&input.name, &result.ast));
        }

        has_error |= result
            .state
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.severity == DiagnosticSeverity::Error);
    }

    let output = if args.json {
        match serde_json::to_string_pretty(&JsonParseOutput {
            inputs: json_inputs,
        }) {
            Ok(mut output) => {
                output.push('\n');
                output
            }
            Err(error) => {
                eprintln!("error: failed to serialize JSON output: {error}");
                return ExitCode::FAILURE;
            }
        }
    } else {
        let mut output = String::new();
        output.push_str(&diagnostics_output);
        output.push_str(&ast_output);
        output
    };

    if let Some(path) = args.out {
        if let Err(error) = fs::write(&path, output) {
            eprintln!("error: failed to write {}: {error}", path.display());
            return ExitCode::FAILURE;
        }
    } else {
        if args.json {
            print!("{output}");
        } else {
            eprint!("{diagnostics_output}");
            print!("{ast_output}");
        }
    }

    if has_error {
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

#[derive(Serialize)]
struct JsonParseOutput {
    inputs: Vec<JsonInputResult>,
}

#[derive(Serialize)]
struct JsonInputResult {
    name: String,
    ast: JsonAst,
    errors: Vec<JsonDiagnostic>,
    warnings: Vec<JsonDiagnostic>,
}

impl JsonInputResult {
    fn new(name: String, ast: &Ast, diagnostics: &[Diagnostic]) -> Self {
        Self {
            name,
            ast: JsonAst::from(ast),
            errors: diagnostics
                .iter()
                .filter(|diagnostic| diagnostic.severity == DiagnosticSeverity::Error)
                .map(JsonDiagnostic::from)
                .collect(),
            warnings: diagnostics
                .iter()
                .filter(|diagnostic| diagnostic.severity == DiagnosticSeverity::Warning)
                .map(JsonDiagnostic::from)
                .collect(),
        }
    }
}

#[derive(Serialize)]
struct JsonAst {
    nodes: Vec<JsonNode>,
    template_units: Vec<JsonTemplateUnit>,
}

impl From<&Ast> for JsonAst {
    fn from(ast: &Ast) -> Self {
        Self {
            nodes: ast.nodes.iter().map(JsonNode::from).collect(),
            template_units: ast
                .template_units
                .iter()
                .map(JsonTemplateUnit::from)
                .collect(),
        }
    }
}

#[derive(Serialize)]
struct JsonTemplateUnit {
    id: usize,
    name: String,
    path: String,
    source: String,
    body_offset: usize,
    body_start_line: usize,
    nodes: Vec<JsonNode>,
}

impl From<&smoothe::parser::TemplateUnit> for JsonTemplateUnit {
    fn from(unit: &smoothe::parser::TemplateUnit) -> Self {
        Self {
            id: unit.id,
            name: unit.name.clone(),
            path: unit.path.display().to_string(),
            source: unit.source.name.clone(),
            body_offset: unit.source.body_offset,
            body_start_line: unit.source.body_start_line,
            nodes: unit.nodes.iter().map(JsonNode::from).collect(),
        }
    }
}

#[derive(Serialize)]
struct JsonSpan {
    start: usize,
    end: usize,
}

impl From<&std::ops::Range<usize>> for JsonSpan {
    fn from(span: &std::ops::Range<usize>) -> Self {
        Self {
            start: span.start,
            end: span.end,
        }
    }
}

impl From<smoothe::parser::SourceSpan> for JsonSpan {
    fn from(span: smoothe::parser::SourceSpan) -> Self {
        Self {
            start: span.start,
            end: span.end,
        }
    }
}

#[derive(Serialize)]
struct JsonDiagnostic {
    issue: String,
    source: String,
    line: usize,
    column: usize,
    span: JsonSpan,
    message: String,
}

impl From<&Diagnostic> for JsonDiagnostic {
    fn from(diagnostic: &Diagnostic) -> Self {
        Self {
            issue: diagnostic.issue.as_str().to_owned(),
            source: diagnostic.source_name.clone(),
            line: diagnostic.location.line,
            column: diagnostic.location.column,
            span: JsonSpan::from(diagnostic.span),
            message: diagnostic.message.clone(),
        }
    }
}

#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum JsonNode {
    Text {
        text: String,
        span: JsonSpan,
    },
    EscapedVariable {
        name: String,
        span: JsonSpan,
    },
    LambdaVariable {
        name: String,
        span: JsonSpan,
    },
    UnescapedVariable {
        name: String,
        span: JsonSpan,
    },
    Comment {
        text: String,
        span: JsonSpan,
    },
    Section {
        name: String,
        span: JsonSpan,
        children: Vec<JsonNode>,
    },
    LambdaSection {
        name: String,
        span: JsonSpan,
        children: Vec<JsonNode>,
    },
    InvertedSection {
        name: String,
        span: JsonSpan,
        children: Vec<JsonNode>,
    },
    Partial {
        name: String,
        span: JsonSpan,
    },
    ResolvedPartial {
        name: String,
        span: JsonSpan,
        resolved_path: String,
        template_id: usize,
        recursive: bool,
    },
    DynamicPartial {
        expression: String,
        span: JsonSpan,
    },
    Parent {
        name: JsonTemplateName,
        span: JsonSpan,
        children: Vec<JsonNode>,
    },
    Block {
        name: String,
        span: JsonSpan,
        children: Vec<JsonNode>,
    },
    DelimiterChange {
        open: String,
        close: String,
        span: JsonSpan,
    },
}

impl From<&Node> for JsonNode {
    fn from(node: &Node) -> Self {
        match node {
            Node::Text { text, span } => Self::Text {
                text: text.clone(),
                span: JsonSpan::from(span),
            },
            Node::EscapedVariable { name, span } => Self::EscapedVariable {
                name: name.clone(),
                span: JsonSpan::from(span),
            },
            Node::LambdaVariable { name, span } => Self::LambdaVariable {
                name: name.clone(),
                span: JsonSpan::from(span),
            },
            Node::UnescapedVariable { name, span } => Self::UnescapedVariable {
                name: name.clone(),
                span: JsonSpan::from(span),
            },
            Node::Comment { text, span } => Self::Comment {
                text: text.clone(),
                span: JsonSpan::from(span),
            },
            Node::Section {
                name,
                span,
                children,
            } => Self::Section {
                name: name.clone(),
                span: JsonSpan::from(span),
                children: children.iter().map(JsonNode::from).collect(),
            },
            Node::LambdaSection {
                name,
                span,
                children,
            } => Self::LambdaSection {
                name: name.clone(),
                span: JsonSpan::from(span),
                children: children.iter().map(JsonNode::from).collect(),
            },
            Node::InvertedSection {
                name,
                span,
                children,
            } => Self::InvertedSection {
                name: name.clone(),
                span: JsonSpan::from(span),
                children: children.iter().map(JsonNode::from).collect(),
            },
            Node::Partial { name, span } => Self::Partial {
                name: name.clone(),
                span: JsonSpan::from(span),
            },
            Node::ResolvedPartial {
                name,
                span,
                resolved_path,
                template_id,
                recursive,
            } => Self::ResolvedPartial {
                name: name.clone(),
                span: JsonSpan::from(span),
                resolved_path: resolved_path.display().to_string(),
                template_id: *template_id,
                recursive: *recursive,
            },
            Node::DynamicPartial { expression, span } => Self::DynamicPartial {
                expression: expression.clone(),
                span: JsonSpan::from(span),
            },
            Node::Parent {
                name,
                span,
                children,
            } => Self::Parent {
                name: JsonTemplateName::from(name),
                span: JsonSpan::from(span),
                children: children.iter().map(JsonNode::from).collect(),
            },
            Node::Block {
                name,
                span,
                children,
            } => Self::Block {
                name: name.clone(),
                span: JsonSpan::from(span),
                children: children.iter().map(JsonNode::from).collect(),
            },
            Node::DelimiterChange { open, close, span } => Self::DelimiterChange {
                open: open.clone(),
                close: close.clone(),
                span: JsonSpan::from(span),
            },
        }
    }
}

#[derive(Serialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
enum JsonTemplateName {
    Static(String),
    Dynamic(String),
}

impl From<&TemplateName> for JsonTemplateName {
    fn from(name: &TemplateName) -> Self {
        match name {
            TemplateName::Static(value) => Self::Static(value.clone()),
            TemplateName::Dynamic(value) => Self::Dynamic(value.clone()),
        }
    }
}

fn format_ast(input_name: &str, ast: &Ast) -> String {
    let mut output = format!("input {input_name}\n");

    for node in &ast.nodes {
        format_node(node, 0, &mut output);
    }

    if ast.nodes.is_empty() {
        output.push_str("  <empty>\n");
    }

    for unit in &ast.template_units {
        output.push_str(&format!(
            "  template_unit id={} name={} path={} body_offset={} body_start_line={}\n",
            unit.id,
            quote(&unit.name),
            quote(&unit.path.display().to_string()),
            unit.source.body_offset,
            unit.source.body_start_line
        ));
        for node in &unit.nodes {
            format_node(node, 1, &mut output);
        }
    }

    output
}

fn format_node(node: &Node, depth: usize, output: &mut String) {
    let indent = "  ".repeat(depth + 1);

    match node {
        Node::Text { text, span } => {
            output.push_str(&format!(
                "{indent}text value={} span={}\n",
                quote(text),
                format_span(span)
            ));
        }
        Node::EscapedVariable { name, span } => {
            output.push_str(&format!(
                "{indent}escaped_variable name={} span={}\n",
                quote(name),
                format_span(span)
            ));
        }
        Node::LambdaVariable { name, span } => {
            output.push_str(&format!(
                "{indent}lambda_variable name={} span={}\n",
                quote(name),
                format_span(span)
            ));
        }
        Node::UnescapedVariable { name, span } => {
            output.push_str(&format!(
                "{indent}unescaped_variable name={} span={}\n",
                quote(name),
                format_span(span)
            ));
        }
        Node::Comment { text, span } => {
            output.push_str(&format!(
                "{indent}comment text={} span={}\n",
                quote(text),
                format_span(span)
            ));
        }
        Node::Section {
            name,
            span,
            children,
        } => format_children("section", name, span, children, &indent, depth, output),
        Node::LambdaSection {
            name,
            span,
            children,
        } => format_children(
            "lambda_section",
            name,
            span,
            children,
            &indent,
            depth,
            output,
        ),
        Node::InvertedSection {
            name,
            span,
            children,
        } => format_children(
            "inverted_section",
            name,
            span,
            children,
            &indent,
            depth,
            output,
        ),
        Node::Partial { name, span } => {
            output.push_str(&format!(
                "{indent}partial name={} span={}\n",
                quote(name),
                format_span(span)
            ));
        }
        Node::ResolvedPartial {
            name,
            span,
            resolved_path,
            template_id,
            recursive,
        } => {
            output.push_str(&format!(
                "{indent}resolved_partial name={} path={} template_id={} recursive={} span={}\n",
                quote(name),
                quote(&resolved_path.display().to_string()),
                template_id,
                recursive,
                format_span(span)
            ));
        }
        Node::DynamicPartial { expression, span } => {
            output.push_str(&format!(
                "{indent}dynamic_partial expression={} span={}\n",
                quote(expression),
                format_span(span)
            ));
        }
        Node::Parent {
            name,
            span,
            children,
        } => {
            let (kind, value) = match name {
                TemplateName::Static(value) => ("static", value),
                TemplateName::Dynamic(value) => ("dynamic", value),
            };
            output.push_str(&format!(
                "{indent}parent name={kind}:{} span={} children={}\n",
                quote(value),
                format_span(span),
                children.len()
            ));
            for child in children {
                format_node(child, depth + 1, output);
            }
        }
        Node::Block {
            name,
            span,
            children,
        } => format_children("block", name, span, children, &indent, depth, output),
        Node::DelimiterChange { open, close, span } => {
            output.push_str(&format!(
                "{indent}delimiter_change open={} close={} span={}\n",
                quote(open),
                quote(close),
                format_span(span)
            ));
        }
    }
}

fn format_children(
    kind: &str,
    name: &str,
    span: &std::ops::Range<usize>,
    children: &[Node],
    indent: &str,
    depth: usize,
    output: &mut String,
) {
    output.push_str(&format!(
        "{indent}{kind} name={} span={} children={}\n",
        quote(name),
        format_span(span),
        children.len()
    ));

    for child in children {
        format_node(child, depth + 1, output);
    }
}

fn format_span(span: &std::ops::Range<usize>) -> String {
    format!("{}..{}", span.start, span.end)
}

fn quote(value: &str) -> String {
    format!("{value:?}")
}
