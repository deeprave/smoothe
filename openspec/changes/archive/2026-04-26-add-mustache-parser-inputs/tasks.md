## 1. Parser Input Extensions

- [x] 1.1 Extend parser input types with partial mappings.
- [x] 1.2 Extend parser input types with lambda/helper specifications.
- [x] 1.3 Extend parser input types with optional context JSON Schema.
- [x] 1.4 Extend parser input types with frontmatter parsing options.

## 2. Partial Handling

- [x] 2.1 Resolve configured partial paths relative to the source template root.
- [x] 2.2 Parse one level of configured partial inclusion.
- [x] 2.3 Attach parsed partial models to parser state.
- [x] 2.4 Emit diagnostics for unresolved partials.
- [x] 2.5 Record nested partial references without recursively expanding them.

## 3. Context Inputs

- [x] 3.1 Recognize references to configured lambda/helper names.
- [x] 3.2 Parse YAML frontmatter by default.
- [x] 3.3 Parse JSON frontmatter when clearly detected.
- [x] 3.4 Parse TOML frontmatter when clearly detected.
- [x] 3.5 Preserve arbitrary frontmatter keys as context extensions.
- [x] 3.6 Warn for referenced paths missing from the provided JSON Schema where
  practical.

## 4. Verification

- [x] 4.1 Add tests for partial mapping input and one-level partial parsing.
- [x] 4.2 Add tests for unresolved partial diagnostics.
- [x] 4.3 Add tests for lambda/helper recognition.
- [x] 4.4 Add tests for YAML, JSON, and TOML frontmatter parsing.
- [x] 4.5 Add tests for JSON Schema missing-path warnings.
- [x] 4.6 Run `cargo fmt --check`.
- [x] 4.7 Run `cargo clippy --all-targets --all-features -- -D warnings`.
- [x] 4.8 Run `cargo nextest run`.
