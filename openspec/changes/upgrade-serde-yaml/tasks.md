## 1. Compatibility Coverage

- [ ] 1.1 Add a focused `prepare_source` test for the YAML frontmatter subset used by `smoothe`.
- [ ] 1.2 Assert the parsed frontmatter remains exposed as `serde_json::Value`.
- [ ] 1.3 Keep invalid YAML coverage focused on warning diagnostics and continued body parsing.

## 2. Dependency Migration

- [ ] 2.1 Replace `serde_yaml` with `serde_norway` in `Cargo.toml`.
- [ ] 2.2 Update `src/source_prepare.rs` to parse YAML through the replacement crate.
- [ ] 2.3 Update `Cargo.lock` and inspect transitive dependency changes.
- [ ] 2.4 Confirm the replacement does not require changes outside the YAML parsing branch.

## 3. Documentation

- [ ] 3.1 Update dependency or maintenance notes if they mention YAML parser maintenance.
- [ ] 3.2 Document any deliberate limits of supported YAML frontmatter if discovered during migration.

## 4. Validation

- [ ] 4.1 Run `cargo fmt --check`.
- [ ] 4.2 Run `cargo clippy --all-targets --all-features -- -D warnings`.
- [ ] 4.3 Run `cargo nextest run`.
- [ ] 4.4 Run `cargo behave`.
- [ ] 4.5 Run `openspec validate upgrade-serde-yaml --strict`.
