use std::{fs, process::ExitCode};

use smoothe::parser::{
    Ast, DiagnosticSeverity, Node, ParserInput, SourceMetadata, TemplateName,
    parse as parse_template,
};

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
    let mut has_error = false;

    for input in inputs {
        let result = parse_template(ParserInput::new(
            SourceMetadata::new(&input.name),
            &input.source,
        ));

        for diagnostic in &result.state.diagnostics {
            diagnostics_output.push_str(&format_diagnostic(diagnostic));
            diagnostics_output.push('\n');
        }

        ast_output.push_str(&format_ast(&input.name, &result.ast));

        has_error |= result
            .state
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.severity == DiagnosticSeverity::Error);
    }

    if let Some(path) = args.out {
        let mut output = String::new();
        output.push_str(&diagnostics_output);
        output.push_str(&ast_output);

        if let Err(error) = fs::write(&path, output) {
            eprintln!("error: failed to write {}: {error}", path.display());
            return ExitCode::FAILURE;
        }
    } else {
        eprint!("{diagnostics_output}");
        print!("{ast_output}");
    }

    if has_error {
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

fn format_ast(input_name: &str, ast: &Ast) -> String {
    let mut output = format!("input {input_name}\n");

    for node in &ast.nodes {
        format_node(node, 0, &mut output);
    }

    if ast.nodes.is_empty() {
        output.push_str("  <empty>\n");
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
