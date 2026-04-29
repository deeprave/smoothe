## 1. Context Shape Model

- [x] 1.1 Add an internal context schema model for object, array, scalar, any, unknown, and unsupported shapes.
- [x] 1.2 Model object properties, required fields, and additional-properties behavior.
- [x] 1.3 Model array item shape.
- [x] 1.4 Model scalar kind, enum values, and default metadata.
- [x] 1.5 Add a resolver API for dotted Mustache paths, current-context references, section scope classification, and known-field descriptions.

## 2. JSON Schema Conversion

- [x] 2.1 Convert supported primitive `type` values into context shapes.
- [x] 2.2 Convert `properties` into object property shapes.
- [x] 2.3 Convert `required` into object required-field metadata.
- [x] 2.4 Convert `items` into array item shape.
- [x] 2.5 Convert `enum` into scalar diagnostic metadata.
- [x] 2.6 Convert `additionalProperties: false` into closed-object behavior.
- [x] 2.7 Treat missing or `true` `additionalProperties` as permissive-object behavior.
- [x] 2.8 Emit schema warnings for malformed supported keywords.
- [x] 2.9 Emit schema warnings for unsupported constructs such as `$ref`, `$defs`, `definitions`, `oneOf`, `anyOf`, `allOf`, and `patternProperties`.
- [x] 2.10 Keep existing invalid JSON and unrecognisable schema input warnings.

## 3. Semantic Validation Integration

- [x] 3.1 Replace raw `serde_json::Value` schema traversal in `check` with the context shape resolver.
- [x] 3.2 Preserve current behavior when schema checking is disabled or set to `none`.
- [x] 3.3 Validate known paths through object and dotted-path traversal.
- [x] 3.4 Warn for missing paths only when the containing object is closed.
- [x] 3.5 Suppress missing-path warnings for permissive objects.
- [x] 3.6 Warn when a referenced property exists but is optional.
- [x] 3.7 Warn when a dotted path attempts to traverse a scalar shape.
- [x] 3.8 Validate object and array sections using object scope and array item scope.
- [x] 3.9 Accept boolean sections without changing child scope.
- [x] 3.10 Warn for scalar section usage that is incompatible with Mustache section scope.
- [x] 3.11 Include enum allowed values in relevant diagnostics where useful.

## 4. Diagnostics

- [x] 4.1 Add or reuse diagnostic issue kinds for optional schema paths and scalar traversal.
- [x] 4.2 Improve missing-path diagnostics to include useful known fields when available.
- [x] 4.3 Ensure schema conversion warnings identify the schema file and unsupported or malformed keyword.
- [x] 4.4 Keep semantic schema diagnostics as warnings rather than parse/check errors.

## 5. Tests

- [x] 5.1 Add table-driven tests for supported primitive type conversion.
- [x] 5.2 Add table-driven tests for object properties, required fields, and additional-properties behavior.
- [x] 5.3 Add table-driven tests for array item scope conversion and validation.
- [x] 5.4 Add tests for enum metadata in diagnostics.
- [x] 5.5 Add tests for unsupported and malformed schema constructs producing warnings.
- [x] 5.6 Add check command tests for closed-object missing paths and permissive-object unknown paths.
- [x] 5.7 Add check command tests for optional property warnings.
- [x] 5.8 Add check command tests for scalar traversal warnings.
- [x] 5.9 Add check command tests for object, array, boolean, and scalar section behavior.
- [x] 5.10 Add regression tests proving CLI and configuration schema inputs still resolve as before.

## 6. Validation

- [x] 6.1 Run `cargo fmt --check`.
- [x] 6.2 Run `cargo clippy --all-targets --all-features -- -D warnings`.
- [x] 6.3 Run `cargo nextest run`.
- [x] 6.4 Run `openspec validate refine-context-schema-model --strict`.
- [x] 6.5 Run `openspec validate --specs --strict`.
