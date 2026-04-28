## 1. Runner Setup

- [ ] 1.1 Add `trycmd` as a dev dependency.
- [ ] 1.2 Add a behavioral test entry point under `tests/`.
- [ ] 1.3 Configure the behavioral test entry point to discover and run fixture cases.
- [ ] 1.4 Verify behavioral tests run under `cargo nextest run`.
- [ ] 1.5 Document runner limitations discovered during initial `trycmd` integration.

## 2. Fixture Layout

- [ ] 2.1 Create a dedicated behavioral fixture hierarchy separate from implementation-focused fixtures.
- [ ] 2.2 Define the fixture convention for command invocation, expected exit status, stdout, and stderr.
- [ ] 2.3 Define how fixture cases include configuration files.
- [ ] 2.4 Define how fixture cases include template files and partial files.
- [ ] 2.5 Define how fixture cases include schema and lambda definition files.
- [ ] 2.6 Document how to add a new behavioral fixture case.

## 3. Output Comparison

- [ ] 3.1 Add line-ending normalization for behavioral output comparison where needed.
- [ ] 3.2 Add path-prefix normalization for machine-specific absolute paths where needed.
- [ ] 3.3 Preserve meaningful relative paths and line/column values during normalization.
- [ ] 3.4 Evaluate whether `trycmd` can compare JSON output structurally.
- [ ] 3.5 Add `snapbox` helper or thin harness support for structural JSON comparison if `trycmd` alone is insufficient.
- [ ] 3.6 Compare compiler-style output as normalized text.
- [ ] 3.7 Define the explicit workflow for intentionally refreshing expected outputs.

## 4. Initial Fixtures

- [ ] 4.1 Add a minimal successful CLI fixture that runs `smoothe` as a black-box command.
- [ ] 4.2 Add a diagnostic-producing fixture that verifies exit status, stdout, and stderr.
- [ ] 4.3 Add a fixture that uses a configuration file.
- [ ] 4.4 Add a fixture that uses template file inputs.
- [ ] 4.5 Add a JSON-output fixture and compare its output structurally where supported.
- [ ] 4.6 Add a compiler-style diagnostic fixture with path normalization where needed.

## 5. Future Coverage Hooks

- [ ] 5.1 Add fixture slots or examples for schema validation cases.
- [ ] 5.2 Add fixture slots or examples for lambda validation cases.
- [ ] 5.3 Add fixture slots or examples for partial graph cases.
- [ ] 5.4 Add fixture slots or examples for rich diagnostics cases.
- [ ] 5.5 Add fixture slots or examples for machine-readable check output cases.

## 6. Maintenance Integration

- [ ] 6.1 Update maintenance documentation or test inventory notes to include behavioral fixtures.
- [ ] 6.2 Ensure behavioral fixture cases are inventoried separately from implementation-focused tests.
- [ ] 6.3 Ensure behavioral fixture cleanup preserves black-box CLI coverage.
- [ ] 6.4 Add guidance for when to add a behavioral fixture instead of a lower-level test.

## 7. Validation

- [ ] 7.1 Run `cargo fmt --check`.
- [ ] 7.2 Run `cargo clippy --all-targets --all-features -- -D warnings`.
- [ ] 7.3 Run `cargo nextest run`.
- [ ] 7.4 Run `openspec validate add-behavioral-fixture-suite --strict`.
- [ ] 7.5 Run `openspec validate --specs --strict`.
