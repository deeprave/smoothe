use std::{cell::RefCell, rc::Rc};

use smoothe::parser::{
    Diagnostic, DiagnosticSeverity, IssueKind, Node, ParseEvent, ParserInput, SourceMetadata, parse,
};

fn parse_template(source: &str) -> smoothe::parser::ParseResult {
    parse(ParserInput::new(
        SourceMetadata::new("template.mustache"),
        source,
    ))
}

#[test]
fn parses_core_nodes_with_source_spans() {
    let result = parse_template("hello {{name}} {{{raw}}} {{& other}} {{! note }} {{> header}}");

    assert!(result.state.diagnostics.is_empty());
    assert_eq!(
        result.ast.nodes,
        vec![
            Node::text("hello ", 0..6),
            Node::escaped_variable("name", 6..14),
            Node::text(" ", 14..15),
            Node::unescaped_variable("raw", 15..24),
            Node::text(" ", 24..25),
            Node::unescaped_variable("other", 25..36),
            Node::text(" ", 36..37),
            Node::comment("note", 37..48),
            Node::text(" ", 48..49),
            Node::partial("header", 49..61),
        ]
    );
}

#[test]
fn parses_nested_sections_and_inverted_sections() {
    let result = parse_template("{{#items}}{{name}}{{^empty}}x{{/empty}}{{/items}}");

    assert!(result.state.diagnostics.is_empty());
    assert_eq!(
        result.ast.nodes,
        vec![Node::section(
            "items",
            0..49,
            vec![
                Node::escaped_variable("name", 10..18),
                Node::inverted_section("empty", 18..39, vec![Node::text("x", 28..29)]),
            ],
        )]
    );
}

#[test]
fn delimiter_change_affects_later_tags() {
    let result = parse_template("{{=<% %>=}}<%name%>");

    assert!(result.state.diagnostics.is_empty());
    assert_eq!(
        result.ast.nodes,
        vec![
            Node::delimiter_change("<%", "%>", 0..11),
            Node::escaped_variable("name", 11..19),
        ]
    );
    assert_eq!(result.state.delimiters.open, "<%");
    assert_eq!(result.state.delimiters.close, "%>");
}

#[test]
fn reports_unclosed_and_mismatched_sections_with_locations() {
    let unclosed = parse_template("one\n{{#items}}");

    assert_eq!(unclosed.state.diagnostics.len(), 1);
    assert_eq!(
        unclosed.state.diagnostics[0].severity,
        DiagnosticSeverity::Error
    );
    assert_eq!(
        unclosed.state.diagnostics[0].issue,
        IssueKind::UnclosedSection
    );
    assert_eq!(unclosed.state.diagnostics[0].location.line, 2);
    assert_eq!(unclosed.state.diagnostics[0].location.column, 1);
    assert_eq!(
        unclosed.state.diagnostics[0].source_name,
        "template.mustache"
    );

    let mismatched = parse_template("{{#items}}{{/users}}");

    assert_eq!(mismatched.state.diagnostics.len(), 1);
    assert_eq!(
        mismatched.state.diagnostics[0].issue,
        IssueKind::MismatchedClosingTag
    );
}

#[test]
fn reports_malformed_tags_and_returns_partial_ast() {
    let result = parse_template("hello {{name");

    assert_eq!(result.ast.nodes, vec![Node::text("hello ", 0..6)]);
    assert_eq!(result.state.diagnostics.len(), 1);
    assert_eq!(result.state.diagnostics[0].issue, IssueKind::MalformedTag);
    assert_eq!(result.state.diagnostics[0].location.column, 7);
}

#[test]
fn routes_diagnostics_to_feedback_handlers() {
    let diagnostics = Rc::new(RefCell::new(Vec::<Diagnostic>::new()));
    let received = Rc::clone(&diagnostics);
    let mut input = ParserInput::new(SourceMetadata::new("broken.mustache"), "{{#items}}");
    input.feedback.on_error = Some(Box::new(move |event| {
        received.borrow_mut().push(event.diagnostic.clone());
    }));

    let result = parse(input);

    assert_eq!(result.state.diagnostics.len(), 1);
    assert_eq!(diagnostics.borrow().len(), 1);
    assert_eq!(
        diagnostics.borrow()[0].issue,
        result.state.diagnostics[0].issue
    );
}

#[test]
fn exposes_feedback_event_metadata() {
    let events = Rc::new(RefCell::new(Vec::<ParseEvent>::new()));
    let received = Rc::clone(&events);
    let mut input = ParserInput::new(SourceMetadata::new("broken.mustache"), "{{/items}}");
    input.feedback.on_error = Some(Box::new(move |event| {
        received.borrow_mut().push(event.clone());
    }));

    parse(input);

    assert_eq!(events.borrow().len(), 1);
    assert_eq!(
        events.borrow()[0].diagnostic.issue,
        IssueKind::UnmatchedClosingTag
    );
}
