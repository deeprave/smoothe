## Why

The current context schema handling is intentionally approximate and validates
template paths by inspecting raw JSON Schema values directly. To make `check`
useful as a dependable utility, the supported schema subset needs to be
explicit and converted into an internal model that can answer Mustache path and
type questions predictably.

## What Changes

- Define the supported JSON Schema subset for template context validation:
  `type`, `properties`, `required`, `items`, `enum`, and
  `additionalProperties`.
- Support JSON Schema primitive types: `object`, `array`, `string`, `number`,
  `integer`, `boolean`, and `null`.
- Treat `additionalProperties: false` as closed-object validation and treat
  `additionalProperties: true` or a missing value as permissive.
- Defer `$defs`, `definitions`, `$ref`, `oneOf`, `anyOf`, `allOf`,
  `patternProperties`, and full JSON Schema validation.
- Introduce an explicit internal context model with object, array, scalar, any,
  unknown, and unsupported shapes.
- Convert loaded JSON Schema input into the internal model before semantic
  validation.
- Emit warnings for malformed or unsupported schema constructs during schema
  conversion.
- Update context validation to use the internal model instead of ad hoc
  `serde_json::Value` traversal.
- Improve schema-aware warnings for unknown paths, optional paths, scalar
  traversal, wrong section types, array item scope, enum-constrained values, and
  permissive objects.
- Add fixture-driven and table-driven coverage for supported schema subset
  behavior.

## Capabilities

### New Capabilities

- `context-schema-model`: Supported JSON Schema subset, internal context-shape
  model, schema conversion, and schema-aware validation behavior.

### Modified Capabilities

- `mustache-parser-inputs`: Context schema validation behavior changes from
  practical raw-schema path checks to validation through the supported
  context-shape model.

## Impact

- Affects `check` semantic validation, schema loading, diagnostics, and tests.
- Introduces an internal context schema model that should be reusable by future
  diagnostics and editor/CI integrations.
- Does not require validating concrete JSON documents against a schema.
- Does not require full JSON Schema support or schema reference resolution in
  this change.
