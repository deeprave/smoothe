use std::{
    fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use smoothe::{
    content::{ContentInput, FrontmatterFormat, process},
    parser::{DiagnosticSeverity, IssueKind, Node, PartialMapping, SourceMetadata},
};

fn temp_template_root() -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time")
        .as_nanos();
    let root = std::env::temp_dir().join(format!("smoothe-content-test-{unique}"));
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

fn process_template(source: &str) -> smoothe::content::TemplateContent {
    process(ContentInput::new(
        SourceMetadata::new("template.mustache"),
        source,
    ))
}

#[test]
fn content_result_preserves_raw_data_and_body_position() {
    let result = process_template("---\ntitle: Hello\n---\n{{title}}");

    assert_eq!(result.raw_data, "---\ntitle: Hello\n---\n{{title}}");
    assert_eq!(result.frontmatter.format, Some(FrontmatterFormat::Yaml));
    assert_eq!(result.frontmatter.context["title"], "Hello");
    assert_eq!(result.body_offset, 21);
    assert_eq!(result.body_start_line, 4);
    assert_eq!(
        result.ast.nodes,
        vec![Node::escaped_variable("title", 21..30)]
    );
    assert!(result.state.diagnostics.is_empty());
}

#[test]
fn content_without_frontmatter_starts_at_beginning() {
    let result = process_template("{{title}}");

    assert_eq!(result.body_offset, 0);
    assert_eq!(result.body_start_line, 1);
    assert_eq!(result.frontmatter.format, None);
    assert_eq!(
        result.ast.nodes,
        vec![Node::escaped_variable("title", 0..9)]
    );
}

#[test]
fn invalid_frontmatter_is_skipped_and_reported() {
    let result = process_template("---\n{\"title\":\n---\n{{title}}");

    assert_eq!(result.body_offset, 18);
    assert_eq!(
        result.ast.nodes,
        vec![Node::escaped_variable("title", 18..27)]
    );
    assert_eq!(result.state.diagnostics.len(), 1);
    assert_eq!(
        result.state.diagnostics[0].severity,
        DiagnosticSeverity::Warning
    );
    assert_eq!(
        result.state.diagnostics[0].issue,
        IssueKind::FrontmatterParseError
    );
    assert_eq!(result.state.diagnostics[0].location.line, 1);
    assert_eq!(result.state.diagnostics[0].location.column, 1);
}

#[test]
fn diagnostics_after_frontmatter_use_original_source_locations() {
    let result = process_template("---\ntitle: Hello\n---\n{{#items}}");

    assert_eq!(result.state.diagnostics.len(), 1);
    assert_eq!(
        result.state.diagnostics[0].issue,
        IssueKind::UnclosedSection
    );
    assert_eq!(result.state.diagnostics[0].location.line, 4);
    assert_eq!(result.state.diagnostics[0].location.column, 1);
    assert_eq!(result.state.diagnostics[0].span.start, 21);
}

#[test]
fn derives_partial_mappings_from_frontmatter_includes() {
    let root = temp_template_root();
    write_file(&root, "_partials/_path.mustache", "Hello {{name}}");
    write_file(&root, "_partials/_another-path.mustache", "Other {{value}}");

    let result = process(ContentInput::new(
        SourceMetadata::new("pages/index.mustache").with_root(root.clone()),
        "---\nincludes:\n  - _partials/path.mustache\n  - _partials/another-path.mustache\n---\n{{> path}} {{> another-path}}",
    ));

    assert!(result.state.diagnostics.is_empty());
    assert_eq!(result.state.partials.len(), 2);
    assert_eq!(result.state.partials[0].name, "path");
    assert_eq!(
        result.state.partials[0].path,
        root.join("_partials/_path.mustache")
    );
    assert_eq!(result.state.partials[1].name, "another-path");
    assert_eq!(
        result.state.partials[1].path,
        root.join("_partials/_another-path.mustache")
    );
}

#[test]
fn include_path_does_not_duplicate_existing_underscore_prefix() {
    let root = temp_template_root();
    write_file(&root, "_partials/_path.mustache", "Hello {{name}}");

    let result = process(ContentInput::new(
        SourceMetadata::new("pages/index.mustache").with_root(root.clone()),
        "---\nincludes:\n  - _partials/_path.mustache\n---\n{{> path}}",
    ));

    assert!(result.state.diagnostics.is_empty());
    assert_eq!(result.state.partials.len(), 1);
    assert_eq!(result.state.partials[0].name, "path");
    assert_eq!(
        result.state.partials[0].path,
        root.join("_partials/_path.mustache")
    );
}

#[test]
fn caller_partial_mapping_overrides_frontmatter_mapping() {
    let root = temp_template_root();
    write_file(&root, "_partials/_path.mustache", "Frontmatter {{name}}");
    write_file(&root, "overrides/path.mustache", "Override {{name}}");

    let mut input = ContentInput::new(
        SourceMetadata::new("pages/index.mustache").with_root(root.clone()),
        "---\nincludes:\n  - _partials/path.mustache\n---\n{{> path}}",
    );
    input.partials.push(PartialMapping::new(
        "path",
        PathBuf::from("overrides/path.mustache"),
    ));

    let result = process(input);

    assert!(result.state.diagnostics.is_empty());
    assert_eq!(result.state.partials.len(), 1);
    assert_eq!(
        result.state.partials[0].path,
        root.join("overrides/path.mustache")
    );
}

#[test]
fn unsupported_includes_shapes_report_warnings() {
    let result = process_template("---\nincludes:\n  - 1\n  - valid.mustache\n---\n{{name}}");

    assert_eq!(
        result
            .state
            .diagnostics
            .iter()
            .filter(|diagnostic| diagnostic.issue == IssueKind::UnsupportedIncludes)
            .count(),
        1
    );

    let result = process_template("---\nincludes: invalid\n---\n{{name}}");

    assert_eq!(
        result
            .state
            .diagnostics
            .iter()
            .filter(|diagnostic| diagnostic.issue == IssueKind::UnsupportedIncludes)
            .count(),
        1
    );
}
