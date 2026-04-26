use std::{
    cell::RefCell,
    fs,
    path::{Path, PathBuf},
    rc::Rc,
    time::{SystemTime, UNIX_EPOCH},
};

use smoothe::parser::{
    Diagnostic, DiagnosticSeverity, FrontmatterFormat, FrontmatterOptions, IssueKind, LambdaSpec,
    Node, ParseEvent, ParserInput, PartialMapping, SourceMetadata, parse,
};

fn parse_template(source: &str) -> smoothe::parser::ParseResult {
    parse(ParserInput::new(
        SourceMetadata::new("template.mustache"),
        source,
    ))
}

fn temp_template_root() -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time")
        .as_nanos();
    let root = std::env::temp_dir().join(format!("smoothe-parser-test-{unique}"));
    fs::create_dir_all(&root).expect("create template root");
    root
}

fn write_file(root: &Path, relative: &str, source: &str) {
    let path = root.join(relative);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("create parent dir");
    }
    fs::write(path, source).expect("write template file");
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

#[test]
fn parses_one_level_of_configured_partials() {
    let root = temp_template_root();
    write_file(
        &root,
        "partials/header.mustache",
        "Header {{title}} {{> nested}}",
    );

    let mut input = ParserInput::new(
        SourceMetadata::new("pages/index.mustache").with_root(root.clone()),
        "Start {{> header}}",
    );
    input.partials.push(PartialMapping::new(
        "header",
        PathBuf::from("partials/header.mustache"),
    ));
    input.partials.push(PartialMapping::new(
        "nested",
        PathBuf::from("partials/nested.mustache"),
    ));

    let result = parse(input);

    assert!(result.state.diagnostics.is_empty());
    assert_eq!(result.state.partials.len(), 1);
    assert_eq!(result.state.partials[0].name, "header");
    assert_eq!(
        result.state.partials[0].path,
        root.join("partials/header.mustache")
    );
    assert_eq!(
        result.state.partials[0].ast.nodes,
        vec![
            Node::text("Header ", 0..7),
            Node::escaped_variable("title", 7..16),
            Node::text(" ", 16..17),
            Node::partial("nested", 17..29),
        ]
    );
    assert_eq!(result.state.nested_partials.len(), 1);
    assert_eq!(result.state.nested_partials[0].name, "nested");
}

#[test]
fn reports_unresolved_partials() {
    let mut input = ParserInput::new(SourceMetadata::new("template.mustache"), "{{> missing}}");
    input.partials.push(PartialMapping::new(
        "header",
        PathBuf::from("partials/header.mustache"),
    ));

    let result = parse(input);

    assert_eq!(result.state.diagnostics.len(), 1);
    assert_eq!(
        result.state.diagnostics[0].severity,
        DiagnosticSeverity::Warning
    );
    assert_eq!(
        result.state.diagnostics[0].issue,
        IssueKind::UnresolvedPartial
    );
}

#[test]
fn recognizes_configured_lambda_references() {
    let mut input = ParserInput::new(
        SourceMetadata::new("template.mustache"),
        "{{#resource}}name{{/resource}} {{plain}}",
    );
    input.lambdas.push(LambdaSpec::new("resource"));

    let result = parse(input);

    assert_eq!(result.state.lambda_references.len(), 1);
    assert_eq!(result.state.lambda_references[0].name, "resource");
}

#[test]
fn parses_yaml_json_and_toml_frontmatter_context_extensions() {
    let yaml = parse(ParserInput::new(
        SourceMetadata::new("template.mustache"),
        "---\ntitle: Hello\ncount: 2\n---\n{{title}}",
    ));
    assert_eq!(yaml.state.frontmatter.format, Some(FrontmatterFormat::Yaml));
    assert_eq!(yaml.state.frontmatter.context["title"], "Hello");
    assert_eq!(
        yaml.ast.nodes,
        vec![Node::escaped_variable("title", 30..39)]
    );

    let json = parse(ParserInput::new(
        SourceMetadata::new("template.mustache"),
        "---\n{\"title\":\"Hello\"}\n---\n{{title}}",
    ));
    assert_eq!(json.state.frontmatter.format, Some(FrontmatterFormat::Json));
    assert_eq!(json.state.frontmatter.context["title"], "Hello");

    let toml = parse(ParserInput::new(
        SourceMetadata::new("template.mustache"),
        "---\ntitle = \"Hello\"\n---\n{{title}}",
    ));
    assert_eq!(toml.state.frontmatter.format, Some(FrontmatterFormat::Toml));
    assert_eq!(toml.state.frontmatter.context["title"], "Hello");
}

#[test]
fn can_disable_frontmatter_parsing() {
    let mut input = ParserInput::new(
        SourceMetadata::new("template.mustache"),
        "---\ntitle: Hello\n---\n{{title}}",
    );
    input.frontmatter = FrontmatterOptions::disabled();

    let result = parse(input);

    assert_eq!(result.state.frontmatter.format, None);
    assert_eq!(
        result.ast.nodes[0],
        Node::text("---\ntitle: Hello\n---\n", 0..21)
    );
}

#[test]
fn warns_for_referenced_paths_missing_from_context_schema() {
    let mut input = ParserInput::new(
        SourceMetadata::new("template.mustache"),
        "{{user.name}} {{user.email}}",
    );
    input.context_schema = Some(serde_json::json!({
        "type": "object",
        "properties": {
            "user": {
                "type": "object",
                "properties": {
                    "name": { "type": "string" }
                }
            }
        }
    }));

    let result = parse(input);

    assert_eq!(result.state.diagnostics.len(), 1);
    assert_eq!(
        result.state.diagnostics[0].severity,
        DiagnosticSeverity::Warning
    );
    assert_eq!(
        result.state.diagnostics[0].issue,
        IssueKind::MissingSchemaPath
    );
    assert_eq!(
        result.state.diagnostics[0].message,
        "missing schema path `user.email`"
    );
}
