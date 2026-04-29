## ADDED Requirements

### Requirement: Check Output Format CLI Options

The system SHALL provide check command options for selecting diagnostic output
format.

#### Scenario: Check accepts JSON flag

- **WHEN** the user runs `smoothe check --json template.mustache`
- **THEN** the CLI selects JSON output for the check command.

#### Scenario: Check accepts output format option

- **WHEN** the user runs `smoothe check --format json template.mustache`
- **THEN** the CLI selects JSON output for the check command.

#### Scenario: Check accepts compiler output format

- **WHEN** the user runs `smoothe check --format compiler template.mustache`
- **THEN** the CLI selects compiler-style output for the check command.

#### Scenario: Check accepts no-json flag

- **WHEN** the user runs `smoothe check --no-json template.mustache`
- **THEN** the CLI selects compiler-style output as the default for the check
  command.

#### Scenario: JSON flags override configured default output

- **GIVEN** `[check] output = "compiler"` is configured
- **WHEN** the user runs `smoothe check --json template.mustache`
- **THEN** the CLI selects JSON output for the check command.
- **GIVEN** `[check] output = "json"` is configured
- **WHEN** the user runs `smoothe check --no-json template.mustache`
- **THEN** the CLI selects compiler-style output for the check command.

#### Scenario: Explicit format overrides JSON default flags

- **WHEN** the user runs `smoothe check --json --format compiler template.mustache`
- **THEN** the CLI selects compiler-style output for the check command.

#### Scenario: JSON default flags conflict with each other

- **WHEN** the user runs `smoothe check --json --no-json template.mustache`
- **THEN** the CLI rejects the conflicting default output selection.

### Requirement: Check Verbosity CLI Option

The system SHALL provide a check command option for selecting the minimum
displayed event verbosity.

#### Scenario: Check accepts error verbosity

- **WHEN** the user runs `smoothe check --verbosity error template.mustache`
- **THEN** the CLI selects error-only event display.

#### Scenario: Check accepts warning verbosity

- **WHEN** the user runs `smoothe check --verbosity warning template.mustache`
- **THEN** the CLI selects warning-and-error event display.

#### Scenario: Check accepts info verbosity

- **WHEN** the user runs `smoothe check --verbosity info template.mustache`
- **THEN** the CLI selects error, warning, and info event display.

#### Scenario: Check accepts debug verbosity

- **WHEN** the user runs `smoothe check --verbosity debug template.mustache`
- **THEN** the CLI selects error, warning, info, and debug event display.

#### Scenario: Check accepts trace verbosity

- **WHEN** the user runs `smoothe check --verbosity trace template.mustache`
- **THEN** the CLI selects error, warning, info, debug, and trace event display.

#### Scenario: Invalid verbosity is rejected

- **WHEN** the user runs `smoothe check --verbosity verbose template.mustache`
- **THEN** the CLI rejects the invalid verbosity.
