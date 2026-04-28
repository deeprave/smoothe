## Context

The `check` command currently loads an optional JSON schema, stores it as a raw
`serde_json::Value`, and validates template paths by traversing `properties`
and reading `type` directly from JSON values. This proves the end-to-end flow,
but the behavior is not explicit enough for reliable diagnostics or predictable
schema support.

The next step is to define a small supported JSON Schema subset and convert it
into an internal model before semantic validation. The model should answer the
questions Mustache validation actually asks: whether a path exists, whether a
path is optional, whether a section can enter a scope, whether scalar traversal
is invalid, whether array items define child scope, and whether an object is
permissive about unknown fields.

## Goals / Non-Goals

**Goals:**

- Define the supported JSON Schema subset for context validation.
- Introduce an explicit internal `ContextShape` model.
- Convert supported JSON Schema keywords into that model before validation.
- Emit schema input warnings for malformed or unsupported constructs.
- Update semantic validation to use the model instead of raw JSON traversal.
- Improve warnings for unknown paths, optional paths, scalar traversal, wrong
  section types, array item scopes, enum-constrained values, and permissive
  object behavior.
- Keep the implementation focused on validating template references, not data
  instances.

**Non-Goals:**

- Do not implement full JSON Schema validation.
- Do not validate a concrete JSON document against the schema.
- Do not resolve `$ref`, `$defs`, or `definitions` in this change.
- Do not implement combinators such as `oneOf`, `anyOf`, or `allOf`.
- Do not support `patternProperties` or complex `additionalProperties` schema
  values in this change.
- Do not couple schema validation to template parsing.

## Decisions

1. Use an internal context model instead of raw JSON Schema during validation.

   The schema loader should parse JSON, recognise the supported subset, and
   produce a model like:

   ```text
   ContextShape
     Object { properties, required, additional_properties }
     Array { items }
     Scalar { kind, enum_values, default_value }
     Any
     Unknown
     Unsupported
   ```

   Semantic validation should only depend on this model and a small resolver
   API. This keeps template validation independent from JSON parsing details.

   Alternative considered: keep walking `serde_json::Value` in the validator.
   That is simpler initially but makes optional fields, unsupported constructs,
   permissive objects, and better diagnostics harder to implement consistently.

2. Support a deliberately small JSON Schema subset.

   The supported keywords are `type`, `properties`, `required`, `items`,
   `enum`, and `additionalProperties`. Supported primitive types are `object`,
   `array`, `string`, `number`, `integer`, `boolean`, and `null`.

   Alternative considered: adopt a JSON Schema validation crate and expose more
   of the specification. That would validate instances, but this utility needs
   schema-shape path resolution for templates.

3. Treat unsupported schema constructs as warnings, not hard failures.

   Unsupported or malformed constructs should produce `SchemaInputError`
   warnings and become `Unsupported` or `Unknown` shapes where possible. The
   checker can still validate the supported portions of the schema.

   Alternative considered: reject the schema entirely on the first unsupported
   construct. That would make adoption brittle and prevent useful partial
   validation.

4. Make `additionalProperties` explicit in object behavior.

   `additionalProperties: false` means unknown object fields should warn. A
   missing value or `true` means the object is permissive and unknown fields
   under that object should not produce missing-path warnings. Schema-valued
   `additionalProperties` is deferred and should warn as unsupported.

   Alternative considered: always warn for unknown properties. That produces
   false positives for intentionally open context maps.

5. Distinguish missing paths from optional paths.

   `required` should be tracked per object shape. A present-but-optional
   property should resolve successfully but may emit a lower-confidence warning
   when a template uses it in a way that assumes presence.

   Alternative considered: treat optional properties as fully valid with no
   diagnostic. That loses useful information, especially for strict templates.

6. Preserve Mustache-specific type rules.

   Object and array sections establish child scope. Boolean sections are valid
   but do not change scope. Scalar sections should warn as unexpected type.
   Dotted traversal through scalar shapes should warn separately from an
   unknown top-level path.

   Alternative considered: only check path existence. That misses important
   Mustache usage errors and makes array item scopes unreliable.

7. Keep enum handling diagnostic-only for now.

   `enum` values should be retained on scalar shapes so diagnostics can mention
   constrained values. The checker should not attempt data-flow analysis or
   branch feasibility from enum values in this change.

   Alternative considered: use enum values to infer conditional reachability.
   That is beyond the current validation scope.

## Risks / Trade-offs

- Partial JSON Schema support can surprise users expecting full compliance.
  Mitigation: document the supported subset and warn for unsupported constructs.
- Optional-property warnings may be noisy. Mitigation: make warning messages
  explicit that the path exists but is not required, and keep the behavior
  table-driven so it can be adjusted later.
- Permissive objects can hide misspelled paths. Mitigation: only suppress
  missing-path warnings inside objects that are explicitly or implicitly open,
  and surface that behavior in tests.
- Introducing a new model can be larger than the current implementation.
  Mitigation: keep the API small: convert schema, resolve path, classify
  section use, and describe known fields for diagnostics.
- Unsupported shapes must not crash validation. Mitigation: represent them as
  `Unsupported` or `Unknown` and continue walking where possible.

## Migration Plan

- Add a context schema model module owned by semantic checking.
- Implement JSON Schema conversion for the supported subset.
- Emit schema diagnostics for invalid type values, malformed `properties`,
  malformed `required`, malformed `items`, malformed `enum`, and unsupported
  `additionalProperties` schema values.
- Replace raw JSON path resolution in `check` semantic validation with the
  model resolver.
- Update validation warnings for unknown fields, optional fields, scalar
  traversal, wrong section types, array item scopes, enum-constrained values,
  and permissive object behavior.
- Add table-driven tests for model conversion and validation behavior.
- Keep existing CLI/config behavior for `--schema`, `[check] schema`, and
  `none` unchanged.

## Open Questions

- Should optional-path diagnostics be warnings by default, or should they be
  surfaced only when a stricter mode is added later?
- Should missing `type` with `properties` be treated as object, and missing
  `type` with `items` be treated as array?
- Should `default` be retained only for diagnostics, or should it suppress
  optional-property warnings for properties with defaults?
