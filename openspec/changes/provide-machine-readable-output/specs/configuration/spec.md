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

### Requirement: Check Diagnostic Level Configuration

The system SHALL allow check diagnostic display level defaults to be configured
in the `[check]` configuration section.

#### Scenario: Config selects diagnostic level

- **WHEN** `[check] diagnostic_level = "info"` is configured
- **THEN** the check command displays diagnostics at info level and above unless
  overridden by the CLI.

#### Scenario: CLI diagnostic level overrides config

- **WHEN** `[check] diagnostic_level = "debug"` is configured and the user runs
  `smoothe check --diagnostic-level error template.mustache`
- **THEN** the check command displays only error diagnostics.

#### Scenario: Invalid configured diagnostic level fails config load

- **WHEN** `[check] diagnostic_level` contains an unsupported value
- **THEN** configuration loading reports a configuration error.
