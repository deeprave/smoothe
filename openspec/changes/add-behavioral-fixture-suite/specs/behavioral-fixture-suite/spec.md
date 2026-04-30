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

#### Scenario: Behavioral suite is separate from normal Rust tests

- **WHEN** the normal Rust test suite runs
- **THEN** the behavioral fixture suite is not executed as part of that suite.

#### Scenario: Behavioral suite runs through explicit command

- **WHEN** a developer wants to run behavioral fixtures
- **THEN** they invoke `cargo behave`.

### Requirement: Behave-Based Initial Runner

The system SHALL start the behavioral fixture suite with a custom `cargo
behave` command.

#### Scenario: Behave executes fixture cases

- **WHEN** the behavioral runner command runs
- **THEN** it discovers and executes behavioral fixture cases.

#### Scenario: Runner may use fixture libraries

- **WHEN** the custom runner is implemented
- **THEN** it may use `trycmd`, `snapbox`, or a thin custom harness internally
  while preserving the `cargo behave` command surface.

#### Scenario: Runner can be reevaluated

- **WHEN** the initial runner cannot support required fixture behavior such as
  normalization or structured JSON comparison
- **THEN** the project may replace or supplement it with `snapbox` or a thin
  custom runner without changing the black-box fixture contract.

### Requirement: Behavioral Fixture Layout

The system SHALL define a repeatable fixture layout for black-box CLI cases.

#### Scenario: Fixture hierarchy avoids Rust test discovery

- **WHEN** the behavioral fixture hierarchy is created
- **THEN** it is not automatically discovered as a standard Rust integration
  test suite.

#### Scenario: Fixture discovery uses case manifests

- **WHEN** the behavioral runner discovers cases
- **THEN** it finds fixture manifests matching `behavior/fixtures/**/case.toml`.

#### Scenario: Fixture case has named directory

- **WHEN** a behavioral fixture is added
- **THEN** the fixture files are stored in a directory named after the test case.

#### Scenario: Fixture paths are case-relative

- **WHEN** `case.toml` references input or expected-output files
- **THEN** those paths are resolved relative to the fixture case directory.

#### Scenario: Fixture defines command behavior

- **WHEN** a behavioral fixture is added
- **THEN** it defines the command invocation, expected exit status, expected
  stdout, and expected stderr.

#### Scenario: Fixture can include config file

- **WHEN** a behavioral fixture needs project configuration
- **THEN** it includes a configuration file consumed by the `smoothe` command.

#### Scenario: Fixture config is passed explicitly

- **WHEN** a behavioral fixture includes a `smoothe` config file
- **THEN** the runner passes that config to `smoothe` explicitly, such as with
  `--config`.

#### Scenario: Fixture can include template inputs

- **WHEN** a behavioral fixture needs template files
- **THEN** it includes those templates as filesystem inputs to the command.

#### Scenario: Fixture can include semantic inputs

- **WHEN** a behavioral fixture needs schema or lambda definitions
- **THEN** it includes those files as filesystem inputs to the command.

#### Scenario: Fixture can include partials

- **WHEN** a behavioral fixture needs partial templates
- **THEN** it includes those partials as filesystem inputs to the command.

#### Scenario: Fixture covers explicit partial mappings

- **WHEN** a behavioral fixture validates configured partials
- **THEN** it provides partial mappings through fixture-local configuration.

#### Scenario: Configured partial mappings remain template-relative

- **WHEN** a behavioral fixture validates configured partials from a config file
  outside the template directory
- **THEN** relative configured partial paths are resolved relative to the
  template file that includes them, not relative to the config file.

#### Scenario: Fixture covers frontmatter partial includes

- **WHEN** a behavioral fixture validates frontmatter partials
- **THEN** it provides template frontmatter `includes` and the corresponding
  partial files.

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

### Requirement: Optional Library Integration Exploration

The behavioral fixture suite SHALL keep the initial behavioral contract
CLI-oriented. The design may explore closer integration with `smoothe` as a
library, but library integration is optional.

#### Scenario: Library integration is evaluated

- **WHEN** the behavioral runner design is refined
- **THEN** maintainers may evaluate whether loading `smoothe` as a library helps
  setup, fixture generation, or performance.

#### Scenario: Library integration is not required

- **WHEN** the initial behavioral suite is implemented
- **THEN** it can run entirely by invoking the `smoothe` command-line utility.
