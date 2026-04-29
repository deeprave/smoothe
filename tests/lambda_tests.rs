use serde_json::json;
use smoothe::context_schema::{ContextShape, ScalarKind};
use smoothe::lambda::{LambdaSideEffects, LambdaSpec, LambdaUsage};

#[test]
fn lambda_spec_models_usage_shapes_and_side_effects() {
    let spec = LambdaSpec::new("markdown")
        .with_usage(LambdaUsage::Both)
        .with_argument(ContextShape::Scalar {
            kind: ScalarKind::String,
            enum_values: Vec::new(),
            default_value: None,
        })
        .with_returns(ContextShape::Scalar {
            kind: ScalarKind::String,
            enum_values: Vec::new(),
            default_value: Some(json!("")),
        })
        .with_side_effects(LambdaSideEffects::Declared);

    assert_eq!(spec.name, "markdown");
    assert!(spec.usage.allows_variable());
    assert!(spec.usage.allows_section());
    assert!(spec.argument.is_some());
    assert!(spec.returns.is_some());
    assert_eq!(spec.side_effects, LambdaSideEffects::Declared);
}
