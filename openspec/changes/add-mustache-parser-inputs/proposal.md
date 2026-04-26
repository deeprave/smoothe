## Why

After the core parser exists, `smoothe` needs configurable parser inputs so
templates can be validated in the context they actually use: partial files,
known lambdas/helpers, context schemas, and frontmatter-derived context.

## What Changes

- Extend parser inputs with partial mappings, lambda/helper specifications,
  context JSON Schema, and frontmatter options.
- Resolve partial paths relative to the source template root.
- Parse and attach one level of partial inclusion to parser state.
- Recognize references to configured lambdas/helpers.
- Parse YAML frontmatter by default and preserve arbitrary keys as context
  extensions.
- Support JSON and TOML frontmatter when the format can be clearly detected.
- Validate referenced context paths against the provided JSON Schema where
  practical, emitting warnings for missing paths.

## Capabilities

### New Capabilities

- `mustache-parser-inputs`: Configurable parser inputs for partials, lambdas,
  context schema validation, and frontmatter context extensions.

### Modified Capabilities

None.

## Impact

- Affected code: parser input types, partial resolution, frontmatter parsing,
  context reference collection, JSON Schema path checking, parser state, and
  tests/spec fixtures.
- Dependencies: may add YAML, TOML, JSON, and JSON Schema support.
- APIs: extends the parser input/result boundary established by the core parser.
