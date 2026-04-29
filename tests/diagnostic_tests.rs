use smoothe::parser::{
    Diagnostic, DiagnosticSeverity, DiagnosticSuggestion, DiagnosticSuggestionKind, IssueKind,
    RelatedLocation, SourceLocation, SourceSpan, near_hit_suggestions,
};

#[test]
fn diagnostic_details_preserve_core_fields() {
    let diagnostic = Diagnostic::new(
        DiagnosticSeverity::Warning,
        IssueKind::MissingSchemaPath,
        "template.mustache",
        SourceLocation { line: 2, column: 3 },
        SourceSpan::new(10, 18),
        "missing schema path `user.nmae`",
    )
    .with_expected("known schema field")
    .with_found("user.nmae")
    .with_expectation_source("context schema")
    .with_note("field names are case-sensitive")
    .with_suggestion(DiagnosticSuggestion::new(
        DiagnosticSuggestionKind::SchemaField,
        "user.name",
    ))
    .with_related_location(RelatedLocation::new(
        "partials/user.mustache",
        SourceLocation { line: 1, column: 1 },
        SourceSpan::new(0, 8),
        "referenced from partial",
    ));

    assert_eq!(diagnostic.severity, DiagnosticSeverity::Warning);
    assert_eq!(diagnostic.issue, IssueKind::MissingSchemaPath);
    assert_eq!(diagnostic.source_name, "template.mustache");
    assert_eq!(diagnostic.location.line, 2);
    assert_eq!(diagnostic.span, SourceSpan::new(10, 18));
    assert_eq!(diagnostic.message, "missing schema path `user.nmae`");
    assert_eq!(
        diagnostic.details.expected.as_deref(),
        Some("known schema field")
    );
    assert_eq!(diagnostic.details.found.as_deref(), Some("user.nmae"));
    assert_eq!(
        diagnostic.details.expectation_source.as_deref(),
        Some("context schema")
    );
    assert_eq!(
        diagnostic.details.notes,
        vec!["field names are case-sensitive".to_owned()]
    );
    assert_eq!(diagnostic.details.suggestions.len(), 1);
    assert_eq!(diagnostic.details.related_locations.len(), 1);
}

#[test]
fn near_hit_suggestions_are_ranked_bounded_and_typed() {
    let candidates = ["email", "name", "nickname", "fullname"]
        .into_iter()
        .map(str::to_owned)
        .collect::<Vec<_>>();

    let suggestions = near_hit_suggestions(
        "nmae",
        &candidates,
        DiagnosticSuggestionKind::SchemaField,
        1,
    );

    assert_eq!(
        suggestions,
        vec![DiagnosticSuggestion::new(
            DiagnosticSuggestionKind::SchemaField,
            "name"
        )]
    );
}

#[test]
fn near_hit_suggestions_are_empty_without_candidates_or_near_hits() {
    assert!(near_hit_suggestions("name", &[], DiagnosticSuggestionKind::SchemaField, 3).is_empty());

    let candidates = ["email", "phase"]
        .into_iter()
        .map(str::to_owned)
        .collect::<Vec<_>>();

    assert!(
        near_hit_suggestions(
            "completely_different",
            &candidates,
            DiagnosticSuggestionKind::SchemaField,
            3,
        )
        .is_empty()
    );
}
