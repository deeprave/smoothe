## 1. Schema and Lambda Inputs

- [x] 1.1 Add check command arguments for an optional context schema path and optional lambda definitions path.
- [x] 1.2 Add tests that check accepts schema and lambda definition options while preserving syntax-only invocation without those options.
- [x] 1.3 Add `[check]` configuration values for schema and lambda definitions, resolving configured paths relative to the configuration file.
- [x] 1.4 Implement case-insensitive `none` handling for CLI and configuration schema/lambda values.
- [x] 1.5 Implement schema file loading that reports diagnostics for unreadable or invalid JSON schema input.
- [x] 1.6 Implement lambda definitions loading that reports diagnostics for unreadable or unrecognisable lambda definition input.

## 2. Semantic Models

- [x] 2.1 Define an internal context-shape model for the supported JSON Schema subset.
- [x] 2.2 Convert recognised JSON schema input into the internal context-shape model.
- [x] 2.3 Extend lambda definitions beyond names to include allowed usage, argument type, and return type.
- [x] 2.4 Add diagnostics for unrecognisable schema shapes and invalid lambda definition shapes.

## 3. AST Semantic Validation

- [x] 3.1 Add a semantic validator that walks the parsed AST after content processing and parsing.
- [x] 3.2 Validate variable references and dotted paths against the current schema scope.
- [x] 3.3 Validate object sections by entering the object scope for child nodes.
- [x] 3.4 Validate array sections by entering the array item scope for child nodes.
- [x] 3.5 Warn when variable or section usage is incompatible with the recognised schema type.
- [x] 3.6 Validate known variable and section lambdas against supplied lambda definitions.
- [x] 3.7 Warn when an identifiable lambda reference has no supplied lambda definition.
- [x] 3.8 Warn when a known lambda is used in an incompatible form or type context.
- [x] 3.9 Warn when an inverted section resolves to a known lambda.

## 4. Check Command Integration

- [x] 4.1 Thread loaded schema and lambda definitions into check command processing.
- [x] 4.2 Merge semantic validation diagnostics with content and parser diagnostics for check output.
- [x] 4.3 Preserve check exit behavior so semantic validation warnings do not fail the command.
- [x] 4.4 Preserve frontmatter-derived partial mapping behavior without requiring explicit partial inputs.

## 5. Verification

- [x] 5.1 Add tests for valid schema loading, invalid JSON schema input, and unrecognisable schema input.
- [x] 5.2 Add tests for known variables, unknown variables, dotted variables, object scopes, array item scopes, and incompatible schema usage.
- [x] 5.3 Add tests for valid lambda usage, unknown lambda references, incompatible lambda usage, and inverted lambda sections.
- [x] 5.4 Add tests that check remains syntax-only when no schema or lambda definitions are supplied.
- [x] 5.5 Add tests for `none` values and config-relative path resolution.
- [x] 5.6 Run `openspec validate add-variable-checks --strict`.
- [x] 5.7 Run `openspec validate --specs --strict`.
- [x] 5.8 Run `cargo fmt --check`.
- [x] 5.9 Run `cargo clippy --all-targets --all-features -- -D warnings`.
- [x] 5.10 Run `cargo nextest run`.
