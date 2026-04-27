## ADDED Requirements

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
