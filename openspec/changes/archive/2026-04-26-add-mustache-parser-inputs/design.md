## Context

The core parser stage provides AST, spans, delimiter support, structural
validation, and diagnostics. This stage adds contextual inputs so the parser can
validate templates against the caller's known partials, lambdas/helpers, schema,
and frontmatter-derived context without becoming tied to a specific command.

## Goals / Non-Goals

**Goals:**

- Extend parser inputs with partial mappings, lambda/helper specifications,
  context JSON Schema, and frontmatter options.
- Resolve one level of partial inclusion relative to the source template root.
- Parse resolved partial source using the same core parser model.
- Preserve parsed partial models and unresolved partial diagnostics in parser
  state.
- Recognize configured lambda/helper references in parser state.
- Parse YAML frontmatter by default, with JSON/TOML detection where clear.
- Preserve arbitrary frontmatter keys as context extensions.
- Emit warnings for referenced paths missing from the provided JSON Schema.

**Non-Goals:**

- Execute lambdas.
- Recursively parse partials beyond one level.
- Implement inheritance or dynamic-name support.
- Render templates.
- Provide complete JSON Schema validation beyond reference-path warnings.

## Decisions

### Extend the Parser Input Boundary

The existing parser input structure should gain optional fields for partial
mappings, lambda/helper specs, context schema, and frontmatter behavior rather
than introducing separate parser entry points.

Alternative considered: add separate parser functions for each input type. That
would fragment the API and make input precedence harder to reason about.

### Parse One Partial Level

When a template references a configured partial, the parser should resolve the
partial path relative to the source template root, parse that source, and attach
the parsed partial model to parser state. Partial references inside parsed
partials should be recorded but not expanded.

Alternative considered: only validate that partial names exist. That would not
meet the requirement that partials are included and similarly validated during
parsing.

### Recognize Lambdas Without Executing Them

Lambda/helper specifications should classify references during parsing and
validation. The parser should not execute lambda logic in this stage.

Alternative considered: execute lambdas to understand dynamic output. That would
mix parsing with rendering/runtime behavior and introduce security and
determinism concerns.

### Treat Schema Validation as Warnings

Referenced paths should be checked against the supplied JSON Schema where the
path can be confidently mapped. Missing paths should emit warnings rather than
parse errors.

Alternative considered: make schema misses fatal. That would make the parser too
checker-specific and reduce reuse by non-check callers.

### Detect Frontmatter Conservatively

YAML should be the default frontmatter format. JSON and TOML should be parsed
only when detection is clear, and all parsed keys should be exposed as context
extensions.

Alternative considered: require callers to specify a frontmatter format. That
would be precise but less ergonomic for template files.

## Risks / Trade-offs

- [Risk] Partial loading can broaden filesystem access. -> Mitigation: resolve
  only caller-provided mappings relative to the source template root.
- [Risk] Schema path mapping may be incomplete. -> Mitigation: warn only for
  paths that can be confidently checked.
- [Risk] Frontmatter detection can be ambiguous. -> Mitigation: default to YAML
  and use JSON/TOML only for clearly identifiable content.
- [Risk] Parser input shape can become too broad. -> Mitigation: keep this stage
  limited to concrete inputs required by parser validation.
