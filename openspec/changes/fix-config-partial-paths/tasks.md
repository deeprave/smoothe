## 1. Partial Path Resolution

- [x] 1.1 Add shared partial path normalization that prefixes `_` to the filename basename when needed.
- [x] 1.2 Update frontmatter include mapping to use the shared normalization helper without changing template-relative base semantics.
- [x] 1.3 Update config partial mapping resolution to join relative paths to the loaded config file directory.
- [x] 1.4 Apply shared underscore filename normalization to config partial mappings after config-directory resolution.
- [x] 1.5 Preserve absolute config partial paths while still applying filename normalization.

## 2. Test Coverage

- [x] 2.1 Add configuration tests for explicit and discovered config partial paths resolving relative to the config directory.
- [x] 2.2 Add content or parser tests for config and frontmatter underscore filename normalization.
- [x] 2.3 Add or update behavioral fixtures covering config-relative partials.

## 3. Documentation

- [x] 3.1 Update user-facing configuration documentation to state config partial paths are config-file-relative.
- [x] 3.2 Update behavioral fixture guidance if it currently describes config partials as template-relative.

## 4. Validation

- [x] 4.1 Run `cargo fmt --check`.
- [x] 4.2 Run `cargo clippy --all-targets --all-features -- -D warnings`.
- [x] 4.3 Run `cargo nextest run`.
- [x] 4.4 Run `cargo behave`.
- [x] 4.5 Run `openspec validate fix-config-partial-paths --strict`.
