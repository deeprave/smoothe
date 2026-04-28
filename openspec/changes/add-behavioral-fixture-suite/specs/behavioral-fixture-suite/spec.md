## ADDED Requirements

### Requirement: Black-Box CLI Fixture Suite

The system SHALL provide behavioral tests that exercise `smoothe` as a
command-line utility rather than through internal Rust APIs.

#### Scenario: Behavioral test runs smoothe binary

- **WHEN** a behavioral fixture test executes
- **THEN** it invokes the `smoothe` utility through a command-line process.

#### Scenario: Behavioral test avoids internals

- **WHEN** a behavioral fixture test asserts behavior
- **THEN** it uses process exit status, stdout, stderr, and filesystem inputs
  rather than parser or checker internals.

#### Scenario: Behavioral suite runs under Rust test runner

- **WHEN** the project test suite runs
- **THEN** the behavioral fixture suite is executed by the configured Rust test
  harness.

### Requirement: Trycmd-Based Initial Runner

The system SHALL start the behavioral fixture suite using `trycmd` as the
initial CLI fixture runner.

#### Scenario: Trycmd executes fixture cases

- **WHEN** the behavioral test entry point runs
- **THEN** it uses `trycmd` to discover and execute behavioral fixture cases.

#### Scenario: Runner can be reevaluated

- **WHEN** `trycmd` cannot support required fixture behavior such as
  normalization or structured JSON comparison
- **THEN** the project may replace or supplement it with `snapbox` or a thin
  custom runner without changing the black-box fixture contract.

### Requirement: Behavioral Fixture Layout

The system SHALL define a repeatable fixture layout for black-box CLI cases.

#### Scenario: Fixture defines command behavior

- **WHEN** a behavioral fixture is added
- **THEN** it defines the command invocation, expected exit status, expected
  stdout, and expected stderr.

#### Scenario: Fixture can include config file

- **WHEN** a behavioral fixture needs project configuration
- **THEN** it includes a configuration file consumed by the `smoothe` command.

#### Scenario: Fixture can include template inputs

- **WHEN** a behavioral fixture needs template files
- **THEN** it includes those templates as filesystem inputs to the command.

#### Scenario: Fixture can include semantic inputs

- **WHEN** a behavioral fixture needs schema or lambda definitions
- **THEN** it includes those files as filesystem inputs to the command.

#### Scenario: Fixture can include partials

- **WHEN** a behavioral fixture needs partial templates
- **THEN** it includes those partials as filesystem inputs to the command.

### Requirement: Output Normalization

The behavioral fixture suite SHALL normalize machine-specific output where
needed before comparison.

#### Scenario: Line endings are normalized

- **WHEN** behavioral output is compared
- **THEN** line endings are normalized so platform differences do not fail the
  fixture.

#### Scenario: Path prefixes are normalized

- **WHEN** behavioral output contains machine-specific absolute paths
- **THEN** known path prefixes are normalized before comparison.

#### Scenario: Relative paths are preserved

- **WHEN** behavioral output contains meaningful relative paths
- **THEN** normalization preserves those paths.

### Requirement: JSON Behavioral Comparison

The behavioral fixture suite SHALL compare JSON output structurally where
possible.

#### Scenario: JSON output is compared as data

- **WHEN** a fixture expects JSON output
- **THEN** the suite compares parsed JSON values rather than relying only on raw
  text formatting.

#### Scenario: Invalid JSON output fails fixture

- **WHEN** a fixture expects JSON output and the command emits invalid JSON
- **THEN** the fixture fails.

### Requirement: Compiler-Style Output Comparison

The behavioral fixture suite SHALL compare compiler-style diagnostic output as
text with normalization.

#### Scenario: Compiler-style output is compared

- **WHEN** a fixture expects compiler-style diagnostics
- **THEN** the suite compares stdout and stderr text after configured
  normalization.

#### Scenario: Compiler-style source location is preserved

- **WHEN** compiler-style diagnostic output includes file, line, or column data
- **THEN** the fixture comparison preserves those values except for configured
  path-prefix normalization.

### Requirement: Expected Output Update Workflow

The behavioral fixture suite SHALL provide an intentional workflow for updating
expected outputs.

#### Scenario: Expected output can be refreshed intentionally

- **WHEN** behavior intentionally changes
- **THEN** maintainers can refresh fixture expected outputs through an explicit
  update workflow.

#### Scenario: Unexpected output drift fails

- **WHEN** command output differs from expected output without an explicit
  update workflow
- **THEN** the fixture test fails.

### Requirement: Incremental Behavioral Coverage

The behavioral fixture suite SHALL support incremental coverage from simple CLI
cases to full end-to-end semantic cases.

#### Scenario: Minimal CLI fixture exists

- **WHEN** the behavioral suite is introduced
- **THEN** it includes at least one minimal fixture that runs `smoothe` as a
  black-box command.

#### Scenario: Diagnostic fixture exists

- **WHEN** the behavioral suite is introduced
- **THEN** it includes at least one fixture that verifies exit status, stdout,
  and stderr for a diagnostic-producing command.

#### Scenario: Complex fixtures can be added later

- **WHEN** schema, lambda, partial, diagnostic, and machine-readable output
  capabilities are implemented
- **THEN** behavioral fixtures can cover those features end to end.
