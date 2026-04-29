## ADDED Requirements

### Requirement: Check Output Configuration

The system SHALL allow check output format defaults to be configured in the
`[check]` configuration section.

#### Scenario: Config selects JSON output

- **WHEN** `[check] output = "json"` is configured
- **THEN** the check command uses JSON output unless overridden by the CLI.

#### Scenario: Config selects compiler output

- **WHEN** `[check] output = "compiler"` is configured
- **THEN** the check command uses compiler-style output unless overridden by
  the CLI.

#### Scenario: CLI output overrides config output

- **WHEN** `[check] output = "json"` is configured and the user runs
  `smoothe check --format compiler template.mustache`
- **THEN** the check command uses compiler-style output.

### Requirement: Check Verbosity Configuration

The system SHALL allow check event display verbosity defaults to be configured
in the `[check]` configuration section.

#### Scenario: Config selects verbosity

- **WHEN** `[check] verbosity = "info"` is configured
- **THEN** the check command displays events at info verbosity and above unless
  overridden by the CLI.

#### Scenario: CLI verbosity overrides config

- **WHEN** `[check] verbosity = "debug"` is configured and the user runs
  `smoothe check --verbosity error template.mustache`
- **THEN** the check command displays only error events.

#### Scenario: Invalid configured verbosity fails config load

- **WHEN** `[check] verbosity` contains an unsupported value
- **THEN** configuration loading reports a configuration error.
