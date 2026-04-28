## 1. Context Shape Model

- [ ] 1.1 Add an internal context schema model for object, array, scalar, any, unknown, and unsupported shapes.
- [ ] 1.2 Model object properties, required fields, and additional-properties behavior.
- [ ] 1.3 Model array item shape.
- [ ] 1.4 Model scalar kind, enum values, and default metadata.
- [ ] 1.5 Add a resolver API for dotted Mustache paths, current-context references, section scope classification, and known-field descriptions.

## 2. JSON Schema Conversion

- [ ] 2.1 Convert supported primitive `type` values into context shapes.
- [ ] 2.2 Convert `properties` into object property shapes.
- [ ] 2.3 Convert `required` into object required-field metadata.
- [ ] 2.4 Convert `items` into array item shape.
- [ ] 2.5 Convert `enum` into scalar diagnostic metadata.
- [ ] 2.6 Convert `additionalProperties: false` into closed-object behavior.
- [ ] 2.7 Treat missing or `true` `additionalProperties` as permissive-object behavior.
- [ ] 2.8 Emit schema warnings for malformed supported keywords.
- [ ] 2.9 Emit schema warnings for unsupported constructs such as `$ref`, `$defs`, `definitions`, `oneOf`, `anyOf`, `allOf`, and `patternProperties`.
- [ ] 2.10 Keep existing invalid JSON and unrecognisable schema input warnings.

## 3. Semantic Validation Integration

- [ ] 3.1 Replace raw `serde_json::Value` schema traversal in `check` with the context shape resolver.
- [ ] 3.2 Preserve current behavior when schema checking is disabled or set to `none`.
- [ ] 3.3 Validate known paths through object and dotted-path traversal.
- [ ] 3.4 Warn for missing paths only when the containing object is closed.
- [ ] 3.5 Suppress missing-path warnings for permissive objects.
- [ ] 3.6 Warn when a referenced property exists but is optional.
- [ ] 3.7 Warn when a dotted path attempts to traverse a scalar shape.
- [ ] 3.8 Validate object and array sections using object scope and array item scope.
- [ ] 3.9 Accept boolean sections without changing child scope.
- [ ] 3.10 Warn for scalar section usage that is incompatible with Mustache section scope.
- [ ] 3.11 Include enum allowed values in relevant diagnostics where useful.

## 4. Diagnostics

- [ ] 4.1 Add or reuse diagnostic issue kinds for optional schema paths and scalar traversal.
- [ ] 4.2 Improve missing-path diagnostics to include useful known fields when available.
- [ ] 4.3 Ensure schema conversion warnings identify the schema file and unsupported or malformed keyword.
- [ ] 4.4 Keep semantic schema diagnostics as warnings rather than parse/check errors.

## 5. Tests

- [ ] 5.1 Add table-driven tests for supported primitive type conversion.
- [ ] 5.2 Add table-driven tests for object properties, required fields, and additional-properties behavior.
- [ ] 5.3 Add table-driven tests for array item scope conversion and validation.
- [ ] 5.4 Add tests for enum metadata in diagnostics.
- [ ] 5.5 Add tests for unsupported and malformed schema constructs producing warnings.
- [ ] 5.6 Add check command tests for closed-object missing paths and permissive-object unknown paths.
- [ ] 5.7 Add check command tests for optional property warnings.
- [ ] 5.8 Add check command tests for scalar traversal warnings.
- [ ] 5.9 Add check command tests for object, array, boolean, and scalar section behavior.
- [ ] 5.10 Add regression tests proving CLI and configuration schema inputs still resolve as before.

## 6. Validation

- [ ] 6.1 Run `cargo fmt --check`.
- [ ] 6.2 Run `cargo clippy --all-targets --all-features -- -D warnings`.
- [ ] 6.3 Run `cargo nextest run`.
- [ ] 6.4 Run `openspec validate refine-context-schema-model --strict`.
- [ ] 6.5 Run `openspec validate --specs --strict`.
