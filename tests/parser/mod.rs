use std::{
    cell::RefCell,
    fs,
    path::{Path, PathBuf},
    rc::Rc,
    time::{SystemTime, UNIX_EPOCH},
};

use smoothe::lambda::LambdaUsage;
use smoothe::parser::{
    Diagnostic, DiagnosticSeverity, DiagnosticSuggestionKind, IssueKind, LambdaSpec, Node,
    ParseEvent, ParserInput, PartialMapping, SourceMetadata, TemplateName, parse,
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

#[cfg(unix)]
#[test]
fn normalized_partial_mapping_leaves_non_utf8_basename_unchanged() {
    use std::os::unix::ffi::{OsStrExt, OsStringExt};

    let mut path = PathBuf::from("partials");
    path.push(std::ffi::OsString::from_vec(vec![0x66, 0x80, b'.', b'm']));

    let mapping = PartialMapping::from_partial_path("header", path);
    let file_name = mapping
        .path
        .file_name()
        .expect("normalized filename")
        .as_bytes();

    assert_eq!(file_name, &[0x66, 0x80, b'.', b'm']);
}

#[test]
fn normalized_partial_mapping_prefixes_full_basename_before_extension() {
    let mapping = PartialMapping::from_partial_path("header", "partials/header.mustache");

    assert_eq!(mapping.path, PathBuf::from("partials/_header.mustache"));
}

#[test]
fn normalized_partial_mapping_prefixes_dotfile_basename() {
    let mapping = PartialMapping::from_partial_path("hidden", "partials/.mustache");

    assert_eq!(mapping.path, PathBuf::from("partials/_.mustache"));
}

#[test]
fn parses_core_nodes_with_source_spans() {
    let result = parse_template("hello {{name}} {{{raw}}} {{& other}} {{! note }}");

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
fn reports_locations_relative_to_body_start_line() {
    let result = parse(ParserInput::new(
        SourceMetadata::new("template.mustache").with_body_start(0, 4),
        "{{#items}}",
    ));

    assert_eq!(result.state.diagnostics.len(), 1);
    assert_eq!(
        result.state.diagnostics[0].issue,
        IssueKind::UnclosedSection
    );
    assert_eq!(result.state.diagnostics[0].location.line, 4);
    assert_eq!(result.state.diagnostics[0].location.column, 1);
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
fn parses_configured_partials_into_graph_units() {
    let root = temp_template_root();
    write_file(
        &root,
        "partials/header.mustache",
        "Header {{title}} {{> nested}}",
    );
    write_file(&root, "partials/nested.mustache", "Nested");

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
    assert_eq!(result.ast.template_units.len(), 2);
    assert_eq!(result.ast.template_units[0].name, "header");
    assert_eq!(result.ast.template_units[1].name, "nested");
}

#[test]
fn resolves_static_partials_into_ast_graph() {
    let root = temp_template_root();
    write_file(&root, "partials/header.mustache", "Header {{title}}");

    let mut input = ParserInput::new(
        SourceMetadata::new("pages/index.mustache").with_root(root.clone()),
        "Start {{> header}}",
    );
    input.partials.push(PartialMapping::new(
        "header",
        PathBuf::from("partials/header.mustache"),
    ));

    let result = parse(input);

    assert!(result.state.diagnostics.is_empty());
    assert_eq!(result.ast.template_units.len(), 1);
    assert_eq!(result.ast.template_units[0].name, "header");
    assert_eq!(
        result.ast.template_units[0].path,
        root.join("partials/header.mustache")
    );
    assert_eq!(
        result.ast.template_units[0].nodes,
        vec![
            Node::text("Header ", 0..7),
            Node::escaped_variable("title", 7..16),
        ]
    );
    assert_eq!(
        result.ast.nodes,
        vec![
            Node::text("Start ", 0..6),
            Node::resolved_partial(
                "header",
                6..18,
                root.join("partials/header.mustache"),
                0,
                false,
            ),
        ]
    );
}

#[test]
fn resolves_nested_partials_into_ast_graph() {
    let root = temp_template_root();
    write_file(&root, "partials/header.mustache", "Header {{> nested}}");
    write_file(&root, "partials/nested.mustache", "Nested {{title}}");

    let mut input = ParserInput::new(
        SourceMetadata::new("pages/index.mustache").with_root(root.clone()),
        "{{> header}}",
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
    assert_eq!(result.ast.template_units.len(), 2);
    assert_eq!(result.ast.template_units[0].name, "header");
    assert_eq!(result.ast.template_units[1].name, "nested");
    assert_eq!(
        result.ast.template_units[0].nodes,
        vec![
            Node::text("Header ", 0..7),
            Node::resolved_partial(
                "nested",
                7..19,
                root.join("partials/nested.mustache"),
                1,
                false,
            ),
        ]
    );
}

#[test]
fn preserves_recursive_partial_reference_without_error() {
    let root = temp_template_root();
    write_file(&root, "partials/self.mustache", "Self {{> self}}");

    let mut input = ParserInput::new(
        SourceMetadata::new("pages/index.mustache").with_root(root.clone()),
        "{{> self}}",
    );
    input.partials.push(PartialMapping::new(
        "self",
        PathBuf::from("partials/self.mustache"),
    ));

    let result = parse(input);

    assert!(result.state.diagnostics.is_empty());
    assert_eq!(result.ast.template_units.len(), 1);
    assert_eq!(
        result.ast.template_units[0].nodes,
        vec![
            Node::text("Self ", 0..5),
            Node::resolved_partial("self", 5..15, root.join("partials/self.mustache"), 0, true,),
        ]
    );
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
        DiagnosticSeverity::Error
    );
    assert_eq!(
        result.state.diagnostics[0].issue,
        IssueKind::UnresolvedPartial
    );
}

#[test]
fn routes_unresolved_partial_errors_to_feedback_handlers() {
    let diagnostics = Rc::new(RefCell::new(Vec::<Diagnostic>::new()));
    let received = Rc::clone(&diagnostics);
    let mut input = ParserInput::new(SourceMetadata::new("template.mustache"), "{{> missing}}");
    input.feedback.on_error = Some(Box::new(move |event| {
        received.borrow_mut().push(event.diagnostic.clone());
    }));

    let result = parse(input);

    assert_eq!(result.state.diagnostics.len(), 1);
    assert_eq!(diagnostics.borrow().len(), 1);
    assert_eq!(diagnostics.borrow()[0].issue, IssueKind::UnresolvedPartial);
}

#[test]
fn reports_unreadable_mapped_partials_as_errors() {
    let root = temp_template_root();
    let mut input = ParserInput::new(
        SourceMetadata::new("pages/index.mustache").with_root(root),
        "{{> missing}}",
    );
    input.partials.push(PartialMapping::new(
        "missing",
        PathBuf::from("partials/missing.mustache"),
    ));

    let result = parse(input);

    assert_eq!(result.state.diagnostics.len(), 1);
    assert_eq!(
        result.state.diagnostics[0].severity,
        DiagnosticSeverity::Error
    );
    assert_eq!(
        result.state.diagnostics[0].issue,
        IssueKind::UnresolvedPartial
    );
}

#[test]
fn skips_frontmatter_in_resolved_partials_and_preserves_body_line() {
    let root = temp_template_root();
    write_file(
        &root,
        "partials/header.mustache",
        "---\ntitle: Partial\n---\n{{#title}}",
    );

    let mut input = ParserInput::new(
        SourceMetadata::new("pages/index.mustache").with_root(root.clone()),
        "{{> header}}",
    );
    input.partials.push(PartialMapping::new(
        "header",
        PathBuf::from("partials/header.mustache"),
    ));

    let result = parse(input);

    assert_eq!(result.ast.template_units.len(), 1);
    assert_eq!(result.ast.template_units[0].source.body_offset, 23);
    assert_eq!(result.ast.template_units[0].source.body_start_line, 4);
    assert_eq!(
        result.ast.template_units[0].nodes,
        vec![Node::section("title", 23..33, vec![])]
    );
    assert_eq!(result.state.diagnostics.len(), 1);
    assert_eq!(
        result.state.diagnostics[0].issue,
        IssueKind::UnclosedSection
    );
    assert_eq!(
        result.state.diagnostics[0].source_name,
        root.join("partials/header.mustache").display().to_string()
    );
    assert_eq!(result.state.diagnostics[0].location.line, 4);
    assert!(result.state.recovered);
}

#[test]
fn dynamic_partials_remain_runtime_references_without_static_resolution_error() {
    let result = parse_template("{{>* runtime_partial}}");

    assert!(result.state.diagnostics.is_empty());
    assert_eq!(
        result.ast.nodes,
        vec![Node::dynamic_partial("runtime_partial", 0..22)]
    );
}

#[test]
fn sections_do_not_balance_across_partial_boundaries() {
    let root = temp_template_root();
    write_file(&root, "partials/close.mustache", "{{/items}}");

    let mut input = ParserInput::new(
        SourceMetadata::new("pages/index.mustache").with_root(root.clone()),
        "{{#items}}{{> close}}",
    );
    input.partials.push(PartialMapping::new(
        "close",
        PathBuf::from("partials/close.mustache"),
    ));

    let result = parse(input);

    assert_eq!(
        result
            .state
            .diagnostics
            .iter()
            .filter(|diagnostic| diagnostic.issue == IssueKind::UnclosedSection)
            .count(),
        1
    );
    assert_eq!(
        result
            .state
            .diagnostics
            .iter()
            .filter(|diagnostic| diagnostic.issue == IssueKind::UnmatchedClosingTag)
            .count(),
        1
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
fn upstream_lambda_fixture_cases_are_modeled_without_execution() {
    let mut input = ParserInput::new(
        SourceMetadata::new("template.mustache"),
        "{{lambda_value}} {{#lambda_section}}{{name}}{{/lambda_section}}",
    );
    input.lambdas.push(LambdaSpec::new("lambda_value"));
    input.lambdas.push(LambdaSpec::new("lambda_section"));

    let result = parse(input);

    assert!(result.state.diagnostics.is_empty());
    assert_eq!(
        result.ast.nodes,
        vec![
            Node::lambda_variable("lambda_value", 0..16),
            Node::text(" ", 16..17),
            Node::lambda_section(
                "lambda_section",
                17..63,
                vec![Node::escaped_variable("name", 36..44)],
            ),
        ]
    );
    assert_eq!(result.state.lambda_references.len(), 2);
}

#[test]
fn recognizes_configured_lambdas_only_for_allowed_usage_forms() {
    let mut input = ParserInput::new(
        SourceMetadata::new("template.mustache"),
        "{{var_only}} {{#var_only}}x{{/var_only}} {{section_only}} {{#section_only}}x{{/section_only}} {{both}} {{#both}}x{{/both}}",
    );
    input
        .lambdas
        .push(LambdaSpec::new("var_only").with_usage(LambdaUsage::Variable));
    input
        .lambdas
        .push(LambdaSpec::new("section_only").with_usage(LambdaUsage::Section));
    input
        .lambdas
        .push(LambdaSpec::new("both").with_usage(LambdaUsage::Both));

    let result = parse(input);

    assert!(matches!(result.ast.nodes[0], Node::LambdaVariable { .. }));
    assert!(matches!(result.ast.nodes[2], Node::Section { .. }));
    assert!(matches!(result.ast.nodes[4], Node::EscapedVariable { .. }));
    assert!(matches!(result.ast.nodes[6], Node::LambdaSection { .. }));
    assert!(matches!(result.ast.nodes[8], Node::LambdaVariable { .. }));
    assert!(matches!(result.ast.nodes[10], Node::LambdaSection { .. }));
    assert_eq!(result.state.lambda_references.len(), 4);
}

#[test]
fn upstream_inheritance_fixture_cases_are_preserved() {
    let result = parse_template("{{< layout}}{{$title}}Default{{/title}}{{/layout}}");

    assert!(result.state.diagnostics.is_empty());
    assert_eq!(
        result.ast.nodes,
        vec![Node::parent(
            TemplateName::Static("layout".to_owned()),
            0..50,
            vec![Node::block(
                "title",
                12..39,
                vec![Node::text("Default", 22..29)]
            )],
        )]
    );
    assert_eq!(result.state.parent_references.len(), 1);
    assert_eq!(
        result.state.parent_references[0].name,
        TemplateName::Static("layout".to_owned())
    );
    assert_eq!(result.state.block_definitions.len(), 1);
    assert_eq!(result.state.block_definitions[0].name, "title");
}

#[test]
fn upstream_dynamic_name_fixture_cases_are_preserved() {
    let result = parse_template("{{>* partial_name}} {{<* parent_name}}{{/parent_name}}");

    assert!(result.state.diagnostics.is_empty());
    assert_eq!(
        result.ast.nodes,
        vec![
            Node::dynamic_partial("partial_name", 0..19),
            Node::text(" ", 19..20),
            Node::parent(
                TemplateName::Dynamic("parent_name".to_owned()),
                20..54,
                vec![]
            ),
        ]
    );
    assert_eq!(result.state.dynamic_names.len(), 2);
    assert_eq!(
        result.state.dynamic_names[0].name,
        TemplateName::Dynamic("partial_name".to_owned())
    );
    assert_eq!(
        result.state.dynamic_names[1].name,
        TemplateName::Dynamic("parent_name".to_owned())
    );
}

#[test]
fn documents_unsupported_advanced_fixture_cases_as_diagnostics() {
    let inheritance = parse_template("{{< }}");

    assert_eq!(inheritance.state.diagnostics.len(), 1);
    assert_eq!(
        inheritance.state.diagnostics[0].issue,
        IssueKind::MalformedInheritance
    );

    let dynamic_name = parse_template("{{>* }}");

    assert_eq!(dynamic_name.state.diagnostics.len(), 1);
    assert_eq!(
        dynamic_name.state.diagnostics[0].issue,
        IssueKind::MalformedDynamicName
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
        "required": ["user"],
        "additionalProperties": false,
        "properties": {
            "user": {
                "type": "object",
                "required": ["name"],
                "additionalProperties": false,
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
        "missing schema path `user.email`; known fields: name"
    );
    assert_eq!(
        result.state.diagnostics[0].details.expected.as_deref(),
        Some("one of schema fields name")
    );
    assert_eq!(
        result.state.diagnostics[0].details.found.as_deref(),
        Some("user.email")
    );
    assert_eq!(
        result.state.diagnostics[0]
            .details
            .expectation_source
            .as_deref(),
        Some("context schema")
    );
}

#[test]
fn warns_for_optional_paths_from_context_schema() {
    let mut input = ParserInput::new(
        SourceMetadata::new("template.mustache"),
        "{{user.fullname}}",
    );
    input.context_schema = Some(serde_json::json!({
        "type": "object",
        "required": ["user"],
        "additionalProperties": false,
        "properties": {
            "user": {
                "type": "object",
                "required": ["name"],
                "additionalProperties": false,
                "properties": {
                    "name": { "type": "string" },
                    "fullname": { "type": "string" }
                }
            }
        }
    }));

    let result = parse(input);

    assert_eq!(result.state.diagnostics.len(), 1);
    assert_eq!(
        result.state.diagnostics[0].issue,
        IssueKind::OptionalSchemaPath
    );
    assert_eq!(
        result.state.diagnostics[0].message,
        "schema path `user.fullname` depends on optional field `user.fullname`"
    );
}

#[test]
fn warns_for_scalar_traversal_from_context_schema() {
    let mut input = ParserInput::new(
        SourceMetadata::new("template.mustache"),
        "{{user.name.first}}",
    );
    input.context_schema = Some(serde_json::json!({
        "type": "object",
        "required": ["user"],
        "additionalProperties": false,
        "properties": {
            "user": {
                "type": "object",
                "required": ["name"],
                "additionalProperties": false,
                "properties": {
                    "name": { "type": "string" }
                }
            }
        }
    }));

    let result = parse(input);

    assert_eq!(result.state.diagnostics.len(), 1);
    assert_eq!(
        result.state.diagnostics[0].issue,
        IssueKind::InvalidSchemaTraversal
    );
    assert_eq!(
        result.state.diagnostics[0].message,
        "schema path `user.name` cannot be traversed because it is string scalar"
    );
}

#[test]
fn warns_for_invalid_section_type_from_context_schema() {
    let mut input = ParserInput::new(
        SourceMetadata::new("template.mustache"),
        "{{#name}}x{{/name}}",
    );
    input.context_schema = Some(serde_json::json!({
        "type": "object",
        "required": ["name"],
        "additionalProperties": false,
        "properties": {
            "name": {
                "type": "string",
                "enum": ["discussion", "planning"]
            }
        }
    }));

    let result = parse(input);

    assert_eq!(result.state.diagnostics.len(), 1);
    assert_eq!(
        result.state.diagnostics[0].issue,
        IssueKind::UnexpectedSchemaType
    );
    assert!(
        result.state.diagnostics[0]
            .message
            .contains("allowed values: \"discussion\", \"planning\"")
    );
    assert_eq!(
        result.state.diagnostics[0].details.expected.as_deref(),
        Some("object, array, boolean, lambda, or permissive section")
    );
    assert_eq!(
        result.state.diagnostics[0].details.found.as_deref(),
        Some("string scalar")
    );
    assert!(
        result.state.diagnostics[0]
            .details
            .suggestions
            .iter()
            .all(|suggestion| suggestion.kind == DiagnosticSuggestionKind::SchemaValue)
    );
}

#[test]
fn parser_context_schema_suggests_nearby_schema_fields() {
    let mut input = ParserInput::new(SourceMetadata::new("template.mustache"), "{{user.nmae}}");
    input.context_schema = Some(serde_json::json!({
        "type": "object",
        "required": ["user"],
        "additionalProperties": false,
        "properties": {
            "user": {
                "type": "object",
                "additionalProperties": false,
                "properties": {
                    "name": { "type": "string" },
                    "email": { "type": "string" }
                }
            }
        }
    }));

    let result = parse(input);
    let diagnostic = &result.state.diagnostics[0];

    assert_eq!(diagnostic.issue, IssueKind::MissingSchemaPath);
    assert_eq!(diagnostic.details.found.as_deref(), Some("user.nmae"));
    assert!(diagnostic.details.suggestions.iter().any(|suggestion| {
        suggestion.kind == DiagnosticSuggestionKind::SchemaField && suggestion.value == "name"
    }));
}

#[test]
fn parser_context_schema_suppresses_unknown_section_child_cascades() {
    let mut input = ParserInput::new(
        SourceMetadata::new("template.mustache"),
        "{{#user}}{{name}}{{email}}{{/user}}",
    );
    input.context_schema = Some(serde_json::json!({
        "type": "object",
        "additionalProperties": false,
        "properties": {
            "account": {
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
        result.state.diagnostics[0].issue,
        IssueKind::MissingSchemaPath
    );
    assert_eq!(
        result.state.diagnostics[0].details.notes,
        vec!["references inside unknown section `user` were not fully validated".to_owned()]
    );
}
