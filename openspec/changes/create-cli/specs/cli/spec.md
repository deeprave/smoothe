## ADDED Requirements

### Requirement: Global CLI Options

The system SHALL provide a `clap`-backed command-line parser with global
`--help`, `--version`, and color control options.

#### Scenario: Help output is available

- **WHEN** the user runs `smoothe --help`
- **THEN** the CLI displays help text and exits successfully.

#### Scenario: Version output is available

- **WHEN** the user runs `smoothe --version`
- **THEN** the CLI displays version text and exits successfully.

#### Scenario: Color option accepts US spelling

- **WHEN** the user runs `smoothe --color <value> check`
- **THEN** the CLI parses the color option before dispatching the `check`
  command.

#### Scenario: Color option accepts UK spelling

- **WHEN** the user runs `smoothe --colour <value> check`
- **THEN** the CLI parses the color option as the same internal setting used by
  `--color` before dispatching the `check` command.

### Requirement: Command Dispatch

The system SHALL dispatch parsed subcommands through an explicit command
dispatcher rather than embedding command behavior directly in argument parsing.

#### Scenario: Check command is dispatched

- **WHEN** the user runs `smoothe check`
- **THEN** the dispatcher invokes the check command handler.

### Requirement: Check Command Stub

The system SHALL provide an initial `check` command with a dedicated argument
type and a stub handler function.

#### Scenario: Check command succeeds as stub

- **WHEN** the user runs `smoothe check`
- **THEN** the check handler returns success without performing Mustache parsing
  or semantic validation.

#### Scenario: Check command can grow arguments later

- **WHEN** future check-specific arguments are added
- **THEN** they can be represented on the existing check command argument type
  without changing the top-level command dispatch shape.
