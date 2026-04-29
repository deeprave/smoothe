## Why

Diagnostics currently identify the issue and location, but semantic checks are
becoming richer through full partial parsing, context schema modeling, and
structured lambda definitions. To make the engine useful in daily workflows,
diagnostics need to explain what was expected, what was found, where the issue
came from, and what nearby valid choices exist.

## What Changes

- Improve diagnostic messages to report expected values, found values, and the
  source of the expectation.
- Preserve accurate file, line, and column reporting across main templates and
  resolved partials.
- Include schema-aware context for missing paths, such as known fields in the
  current object scope.
- Suppress cascading missing-path diagnostics inside unknown section scopes and
  replace them with a bounded warning that child references could not be fully
  validated because the section scope is unknown.
- Include near-hit suggestions for likely typos in variable paths, schema
  fields, lambda names, and partial names where enough candidates are known.
- Make semantic diagnostics clearer for schema validation, lambda validation,
  and partial graph validation.
- Keep diagnostics structured so CLI text output and future machine-readable
  output can use the same underlying data.
- Preserve existing error/warning severity behavior from the dependent changes.

## Capabilities

### New Capabilities

- `diagnostic-quality`: Rich diagnostic messages, expectation/found/source
  context, near-hit suggestions, and source-accurate reporting across template
  graphs.

### Modified Capabilities

- `mustache-core-parser`: Parser diagnostics include richer context while
  preserving structured issue, severity, source, line, column, and span data.
- `template-content`: Content and partial-source diagnostics preserve accurate
  file and body-offset source locations across main templates and partials.
- `json-parser-output`: JSON diagnostics preserve structured diagnostic data
  needed by improved diagnostics.
- `template-semantic-checks`: Semantic diagnostics for schema and lambda checks
  include expected/found/source context and useful suggestions.
- `context-schema-model`: Schema validation diagnostics expose known fields,
  optionality, enum values, and near-hit information where available.
- `lambda-definition-model`: Lambda validation diagnostics expose expected
  usage, actual usage, type-shape context, and near-hit lambda names where
  available.
- `template-partial-graph`: Partial diagnostics expose reference name, resolved
  path when available, source template location, and near-hit partial names
  where available.

## Impact

- Depends on `add-full-partial-support`, `refine-context-schema-model`, and
  `refine-lambda-handling`.
- Affects diagnostic data construction, text formatting, JSON diagnostic
  projection, and tests.
- May change diagnostic message text while preserving stable issue identifiers.
- Does not introduce new validation rules beyond clearer reporting and
  suggestions.
