## 1. Runner Setup

- [x] 1.1 Add fixture-runner dependencies only if needed by the first implementation slice.
- [x] 1.2 Add an explicit `cargo behave` behavioral runner command.
- [x] 1.3 Configure the behavioral runner command to discover and run fixture cases.
- [x] 1.4 Verify behavioral tests do not run as part of `cargo nextest run`.
- [x] 1.5 Document runner limitations discovered during initial `cargo behave` integration.
- [x] 1.6 Explore whether optional library integration would be useful without making it required.
- [x] 1.7 Add runner options for `--list`, `--filter`, and `--update`.

## 2. Fixture Layout

- [x] 2.1 Create a dedicated `behavior/fixtures` hierarchy separate from implementation-focused fixtures.
- [x] 2.2 Define the fixture convention for command invocation, expected exit status, stdout, and stderr.
- [x] 2.3 Define how fixture cases include configuration files.
- [x] 2.4 Define how fixture cases include template files and partial files.
- [x] 2.5 Define how fixture cases include schema and lambda definition files.
- [x] 2.6 Document how to add a new behavioral fixture case.
- [x] 2.7 Document how the behavioral fixture hierarchy remains outside normal Rust test discovery.
- [x] 2.8 Discover fixture cases from `behavior/fixtures/**/case.toml`.
- [x] 2.9 Require each fixture case to live in a directory named after the test case.
- [x] 2.10 Resolve `case.toml` paths relative to the case directory.
- [x] 2.11 Support a fixture-local config file passed to `smoothe` with `--config`.

## 3. Output Comparison

- [x] 3.1 Add line-ending normalization for behavioral output comparison where needed.
- [x] 3.2 Add path-prefix normalization for machine-specific absolute paths where needed.
- [x] 3.3 Preserve meaningful relative paths and line/column values during normalization.
- [x] 3.4 Evaluate whether the initial runner can compare JSON output structurally.
- [x] 3.5 Add `snapbox` helper or thin harness support for structural JSON comparison if the initial runner is insufficient.
- [x] 3.6 Compare compiler-style output as normalized text.
- [x] 3.7 Define the explicit workflow for intentionally refreshing expected outputs.

## 4. Initial Fixtures

- [x] 4.1 Add a minimal successful CLI fixture that runs `smoothe` as a black-box command.
- [x] 4.2 Add a diagnostic-producing fixture that verifies exit status, stdout, and stderr.
- [x] 4.3 Add a fixture that uses a configuration file.
- [x] 4.4 Add a fixture that uses template file inputs.
- [x] 4.5 Add a JSON-output fixture and compare its output structurally where supported.
- [x] 4.6 Add a compiler-style diagnostic fixture with path normalization where needed.
- [x] 4.7 Add a fixture that validates explicit partial mappings from config.
- [x] 4.8 Add a fixture that validates frontmatter `includes` partial mappings.
- [x] 4.9 Add negative CLI fixtures for non-zero exit behavior.
- [x] 4.10 Add a fixture that validates config partial mappings remain template-relative.

## 5. Future Coverage Hooks

- [x] 5.1 Add fixture slots or examples for schema validation cases.
- [x] 5.2 Add fixture slots or examples for lambda validation cases.
- [x] 5.3 Add fixture slots or examples for partial graph cases.
- [x] 5.4 Add fixture slots or examples for rich diagnostics cases.
- [x] 5.5 Add fixture slots or examples for machine-readable check output cases.

## 6. Maintenance Integration

- [x] 6.1 Update maintenance documentation or test inventory notes to include behavioral fixtures.
- [x] 6.2 Ensure behavioral fixture cases are inventoried separately from implementation-focused tests.
- [x] 6.3 Ensure behavioral fixture cleanup preserves black-box CLI coverage.
- [x] 6.4 Add guidance for when to add a behavioral fixture instead of a lower-level test.
- [x] 6.5 Add guidance for when to run the behavioral suite separately from the normal test suite.

## 7. Validation

- [x] 7.1 Run `cargo fmt --check`.
- [x] 7.2 Run `cargo clippy --all-targets --all-features -- -D warnings`.
- [x] 7.3 Run `cargo nextest run` and confirm behavioral fixtures are not included.
- [x] 7.4 Run the dedicated behavioral command.
- [x] 7.5 Run `openspec validate add-behavioral-fixture-suite --strict`.
- [x] 7.6 Run `openspec validate --specs --strict`.
