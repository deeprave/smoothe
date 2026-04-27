## Why

The `check` command currently validates template syntax but cannot verify whether referenced template variables and lambdas are meaningful for the expected rendering context. Adding schema-backed variable validation and lambda definitions will let `check` catch template/context mismatches before rendering.

## What Changes

- Add optional `check` command inputs and `[check]` configuration values for a JSON context schema and known lambda definitions.
- Treat `none` case-insensitively as an explicit disable value for schema and lambda checking; checking is disabled by default.
- Parse and approximately validate the supplied schema before semantic template checks run.
- Convert the supplied schema into an internal context-shape model suitable for resolving Mustache variable paths.
- Warn when template variable references are absent from the schema or appear incompatible with their Mustache usage.
- Warn when lambda references are absent from the supplied lambda definitions or appear incompatible with their usage.
- Warn when lambdas are used through inverted sections, since negative lambda references are unsupported.
- Preserve existing syntax validation behavior when no schema or lambda definitions are supplied.

## Capabilities

### New Capabilities

- `template-semantic-checks`: Semantic validation of parsed templates against context schema and lambda definitions.

### Modified Capabilities

- `cli`: Add `check` command inputs for schema and lambda definition files.
- `mustache-advanced-features`: Clarify lambda usage validation, including unsupported inverted lambda sections.

## Impact

- Affects the `check` command interface, `[check]` configuration, and validation flow.
- Adds schema and lambda-definition loading/parsing before AST semantic validation.
- Introduces diagnostics for schema recognition, unknown variables, incompatible variable usage, unknown lambdas, incompatible lambda usage, and inverted lambda sections.
- Does not require executing lambdas or validating a concrete JSON data instance.
