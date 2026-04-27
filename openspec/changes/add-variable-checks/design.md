## Context

The `check` command currently reads template inputs, processes frontmatter and partial includes through the content layer, parses templates into an AST, and reports parser diagnostics. `ContentInput` and `ParserInput` already carry optional `lambdas` and `context_schema` fields, but the CLI does not yet load these inputs and the parser remains responsible only for syntax and structural parsing.

This change adds a semantic validation phase for `check`. The validator should consume the parsed AST, the effective partial mappings derived by content processing, an optional context-shape schema, and optional lambda definitions. It should report warnings through the same diagnostic model used by parse/check today.

## Goals / Non-Goals

**Goals:**

- Let `check` accept an optional JSON schema describing the expected template context shape through `--schema` or `[check] schema`.
- Let `check` accept optional lambda definitions that describe known lambdas and their allowed usage through `--lambdas` or `[check] lambdas`.
- Treat `none` case-insensitively as an explicit disable value for schema and lambda checking.
- Validate schema input enough to ensure it is valid JSON and recognisable as a context-shape schema.
- Convert schema input into an internal model that can answer whether Mustache paths are valid in the current scope.
- Validate variable references, sections, inverted sections, and dotted paths against the current schema scope.
- Validate lambda references against known lambda definitions and report unsupported inverted lambda usage.
- Keep partial mapping discovery in the content/frontmatter layer, with explicit partial input remaining optional.

**Non-Goals:**

- Do not validate a concrete JSON data instance against the schema.
- Do not execute lambdas.
- Do not make parsing depend on semantic schema or lambda validation.
- Do not require schema or lambda inputs for existing syntax-only `check` behavior, which remains the default.
- Do not attempt full JSON Schema compliance beyond the subset needed to model context shape for templates.

## Decisions

1. Add semantic validation after content processing and parsing.

   The parser should continue producing AST and syntax diagnostics. The `check` command should run semantic validation after `process_template` returns, then merge semantic diagnostics with parser/content diagnostics for reporting and exit-code decisions. This preserves the parser as a syntax layer and lets semantic validation evolve independently.

   Alternative considered: classify all schema and lambda issues during parsing. That would couple parser behavior to optional `check` inputs and make the parser responsible for concerns outside AST construction.

2. Represent the supplied JSON schema as an internal context model.

   The schema loader should first parse the file as JSON, then recognise a supported schema shape and migrate it into a resolver-oriented model. The validator needs operations such as resolving `user.name` from the current scope, identifying whether a section can enter an object or array item scope, and understanding whether `.` is meaningful.

   Alternative considered: call a JSON Schema validator directly. That does not answer template path-resolution questions because the check is against template references, not a JSON instance.

3. Treat semantic validation diagnostics as warnings.

   Unknown variables, unexpected variable usage, unknown lambdas, incompatible lambda usage, and inverted lambda sections should be warnings. This matches the proposal and avoids failing existing syntax checks when users incrementally introduce schemas or lambda definitions.

   Alternative considered: make missing schema/lambda entries errors. That is stricter but would make adoption harder and can be too strong for partial schemas.

4. Keep partial mappings as effective content state.

   Partials can come from document frontmatter and should not require a separate validation input. The semantic validator should receive the effective mapping produced by content processing, including any explicit caller-supplied mappings if those exist.

   Alternative considered: require all partial mappings to be passed to `check` separately. That duplicates frontmatter behavior and makes the common case noisier.

5. Expand lambda definitions beyond names.

   `LambdaSpec` should become a structured definition that can distinguish variable and section usage, argument type, and return type. Lambdas accept a single argument and return a single value. Section lambdas are valid for `{{#lambda}}...{{/lambda}}`; inverted sections such as `{{^lambda}}...{{/lambda}}` are unsupported and should warn when the name resolves to a lambda.

   Alternative considered: keep lambda definitions as names only. That can detect unknown lambdas but cannot validate usage or type compatibility.

6. Resolve semantic input paths according to where they are declared.

   CLI paths passed with `--schema` and `--lambdas` are resolved relative to the current working directory, matching normal CLI path expectations. Configuration paths in `[check] schema` and `[check] lambdas` are resolved relative to the configuration file that declared them. The string `none`, matched case-insensitively, is not treated as a path and disables the corresponding checker.

   Alternative considered: resolve all paths relative to the current working directory. That would make configuration files less portable and would be surprising when the config file lives outside the command's working directory.

## Risks / Trade-offs

- Partial schema support may produce false positives when schemas intentionally omit dynamic data. Mitigation: report semantic findings as warnings and allow checks to run without schema input.
- JSON Schema has broad semantics that are not all useful for Mustache path resolution. Mitigation: document and implement a supported subset focused on object properties, arrays, scalar types, required/optional shape, and dotted-path traversal.
- Mustache scope resolution can be subtle, especially sections and `.`. Mitigation: implement validation as a scoped AST walk and add table-driven tests for object sections, array item sections, inverted sections, dotted paths, and current-context references.
- Lambda names can overlap with context variable names. Mitigation: define deterministic resolution rules in the spec, preferring explicit lambda definitions when validating lambda-specific usage and producing clear diagnostics for ambiguous usage if needed.
- Existing parser lambda classification may not be sufficient once lambda definitions include usage/type metadata. Mitigation: keep parser classification minimal and let the semantic validator make final usage decisions from AST nodes and definitions.

## Migration Plan

- Add CLI/configuration inputs for schema and lambda definition files without changing existing required operands.
- Implement explicit `none` handling for CLI and configuration inputs before loading schema or lambda files.
- Add schema and lambda loaders that produce diagnostics for invalid or unrecognisable inputs.
- Add an internal context resolver and semantic validator used by `check`.
- Thread loaded schema and lambda definitions through `ContentInput` where useful, but keep validation after AST creation.
- Add focused tests for schema loading, variable validation, lambda validation, unsupported inverted lambdas, and unchanged syntax-only checks.
- Existing users who do not pass schema or lambda inputs keep current `check` behavior.

## Open Questions

- How should ambiguous names be handled when a name exists as both a context variable and a lambda definition?
- Which JSON Schema keywords are in the supported subset for the first implementation?
