use serde_json::json;
use smoothe::context_schema::{
    AdditionalProperties, ContextSchema, ContextShape, PathResolution, ScalarKind, SectionScope,
};
use smoothe::parser::IssueKind;

#[test]
fn converts_supported_primitive_types() {
    let cases = [
        ("string", ScalarKind::String),
        ("number", ScalarKind::Number),
        ("integer", ScalarKind::Integer),
        ("boolean", ScalarKind::Boolean),
        ("null", ScalarKind::Null),
    ];

    for (schema_type, expected) in cases {
        let schema = ContextSchema::from_json(json!({ "type": schema_type }), "context.json");

        assert!(schema.diagnostics().is_empty(), "{schema_type}");
        assert_eq!(
            schema.root(),
            &ContextShape::Scalar {
                kind: expected,
                enum_values: Vec::new(),
                default_value: None
            }
        );
    }
}

#[test]
fn converts_object_required_fields_and_additional_properties() {
    let schema = ContextSchema::from_json(
        json!({
            "type": "object",
            "required": ["name"],
            "additionalProperties": false,
            "properties": {
                "name": { "type": "string" },
                "fullname": { "type": "string", "default": "Anonymous" }
            }
        }),
        "context.json",
    );

    let ContextShape::Object(object) = schema.root() else {
        panic!("expected object shape");
    };

    assert_eq!(object.additional_properties, AdditionalProperties::Closed);
    assert!(object.is_required("name"));
    assert!(!object.is_required("fullname"));
    assert_eq!(
        schema.resolve_path("fullname"),
        PathResolution::Found {
            shape: object.properties.get("fullname").expect("fullname shape"),
            optional: Some("fullname".to_owned())
        }
    );
    assert!(matches!(
        schema.resolve_path("missing"),
        PathResolution::Missing { .. }
    ));
}

#[test]
fn resolves_array_item_scope_and_scalar_traversal() {
    let schema = ContextSchema::from_json(
        json!({
            "type": "object",
            "required": ["items", "user"],
            "additionalProperties": false,
            "properties": {
                "items": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "required": ["title"],
                        "additionalProperties": false,
                        "properties": {
                            "title": { "type": "string" }
                        }
                    }
                },
                "user": {
                    "type": "object",
                    "required": ["name"],
                    "additionalProperties": false,
                    "properties": {
                        "name": { "type": "string" }
                    }
                }
            }
        }),
        "context.json",
    );

    match schema.section_scope("items") {
        SectionScope::Changed { shape, .. } => {
            assert!(matches!(
                shape.resolve_path("title"),
                PathResolution::Found { .. }
            ));
        }
        other => panic!("expected changed array item scope, got {other:?}"),
    }

    assert_eq!(
        schema.resolve_path("user.name.first"),
        PathResolution::InvalidTraversal {
            traversed_path: "user.name".to_owned(),
            shape: schema
                .resolve_path("user.name")
                .shape()
                .expect("user.name shape")
        }
    );
}

#[test]
fn warns_for_unsupported_schema_constructs() {
    let schema = ContextSchema::from_json(
        json!({
            "type": "object",
            "$ref": "#/$defs/User",
            "oneOf": [],
            "additionalProperties": { "type": "string" }
        }),
        "context.json",
    );

    let diagnostics = schema.diagnostics();

    assert_eq!(diagnostics.len(), 3);
    assert!(diagnostics.iter().all(|diagnostic| {
        diagnostic.issue == IssueKind::SchemaInputError && diagnostic.source_name == "context.json"
    }));
}

#[test]
fn warns_for_malformed_supported_keywords() {
    let schema = ContextSchema::from_json(
        json!({
            "type": "object",
            "properties": [],
            "required": "name",
            "additionalProperties": "sometimes"
        }),
        "context.json",
    );

    let messages = schema
        .diagnostics()
        .iter()
        .map(|diagnostic| diagnostic.message.as_str())
        .collect::<Vec<_>>();

    assert!(
        messages
            .iter()
            .any(|message| message.contains("properties"))
    );
    assert!(messages.iter().any(|message| message.contains("required")));
    assert!(
        messages
            .iter()
            .any(|message| message.contains("additionalProperties"))
    );

    let schema = ContextSchema::from_json(
        json!({
            "type": "array",
            "items": []
        }),
        "context.json",
    );

    assert!(
        schema
            .diagnostics()
            .iter()
            .any(|diagnostic| diagnostic.message.contains("items"))
    );

    let schema = ContextSchema::from_json(
        json!({
            "type": "string",
            "enum": "active"
        }),
        "context.json",
    );

    assert!(
        schema
            .diagnostics()
            .iter()
            .any(|diagnostic| diagnostic.message.contains("enum"))
    );
}

#[test]
fn warns_when_enum_has_no_scalar_type() {
    let schema = ContextSchema::from_json(
        json!({
            "enum": ["discussion", "planning"]
        }),
        "context.json",
    );

    assert_eq!(schema.root(), &ContextShape::Unknown);
    assert!(schema.diagnostics().iter().any(
        |diagnostic| diagnostic.message.contains("enum") && diagnostic.message.contains("type")
    ));
}
