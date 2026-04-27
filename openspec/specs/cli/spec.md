# cli Specification

## Purpose
Define the command-line interface shell, global options, and command dispatch
contract.

## Requirements
### Requirement: Global CLI Options

The system SHALL provide a `clap`-backed command-line parser with global
`--help`, `--version`, and color control options.

#### Scenario: Help output is available

- **WHEN** the user runs `smoothe --help`
- **THEN** the CLI displays help text and exits successfully.

#### Scenario: Help output is available with short alias

- **WHEN** the user runs `smoothe -h`
- **THEN** the CLI displays help text and exits successfully.

#### Scenario: Version output is available

- **WHEN** the user runs `smoothe --version`
- **THEN** the CLI displays version text and exits successfully.

#### Scenario: Version output is available with short alias

- **WHEN** the user runs `smoothe -V`
- **THEN** the CLI displays version text and exits successfully.

#### Scenario: Color option defaults to automatic behavior

- **WHEN** the user runs `smoothe check` without a color option or color
  environment override
- **THEN** the CLI uses `clap`'s `ColorChoice::Auto` setting.

#### Scenario: Color option accepts US spelling

- **WHEN** the user runs `smoothe --color <value> check`
- **THEN** the CLI parses the color option before dispatching the `check`
  command.

#### Scenario: Color option accepts UK spelling

- **WHEN** the user runs `smoothe --colour <value> check`
- **THEN** the CLI parses the color option as the same internal setting used by
  `--color` before dispatching the `check` command.

#### Scenario: Color option accepts short alias

- **WHEN** the user runs `smoothe -c <value> check`
- **THEN** the CLI parses the color option as the same internal setting used by
  `--color` before dispatching the `check` command.

#### Scenario: Color output can be disabled with flag

- **WHEN** the user runs `smoothe --no-color check`
- **THEN** the CLI parses the color setting as disabled before dispatching the
  `check` command.

#### Scenario: Color output can be disabled with environment

- **WHEN** the user runs `smoothe check` with `NOCOLOR` set in the environment
- **THEN** the CLI uses color-disabled behavior unless an explicit color option
  overrides it.

### Requirement: Command Dispatch

The system SHALL dispatch parsed subcommands through an explicit command
dispatcher rather than embedding command behavior directly in argument parsing.

#### Scenario: Check command is dispatched

- **WHEN** the user runs `smoothe check`
- **THEN** the dispatcher invokes the check command handler.

#### Scenario: No default command is dispatched

- **WHEN** the user runs `smoothe` without a subcommand
- **THEN** the CLI does not invoke the check command handler.

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

### Requirement: Config CLI Option

The system SHALL provide a global CLI option for selecting an explicit
configuration file path.

#### Scenario: Long config option is accepted

- **WHEN** the user runs `smoothe --config smoothe.toml check template.mustache`
- **THEN** the CLI parses `smoothe.toml` as the explicit configuration path
  before dispatching the `check` command.

#### Scenario: Short config option is accepted

- **WHEN** the user runs `smoothe -C smoothe.toml check template.mustache`
- **THEN** the CLI parses `smoothe.toml` as the explicit configuration path
  before dispatching the `check` command.

### Requirement: Effective Options Dispatch

The system SHALL dispatch commands with resolved global options and resolved
command-specific options after configuration, environment, and CLI overrides
have been applied.

#### Scenario: Check receives resolved options

- **WHEN** the user runs `smoothe check template.mustache`
- **THEN** the dispatcher invokes the check handler with resolved global options
  and resolved check options.

#### Scenario: CLI color override is preserved

- **WHEN** the user runs `smoothe --color always check template.mustache` and
  configuration or environment values also set color behavior
- **THEN** the dispatcher invokes the check handler with color enabled from the
  CLI override.
