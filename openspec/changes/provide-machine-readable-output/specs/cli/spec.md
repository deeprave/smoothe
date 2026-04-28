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

#### Scenario: JSON flag conflicts with incompatible format

- **WHEN** the user runs `smoothe check --json --format compiler template.mustache`
- **THEN** the CLI rejects the conflicting output format selection.

### Requirement: Check Diagnostic Level CLI Option

The system SHALL provide a check command option for selecting the minimum
displayed diagnostic severity.

#### Scenario: Check accepts error diagnostic level

- **WHEN** the user runs `smoothe check --diagnostic-level error template.mustache`
- **THEN** the CLI selects error-only diagnostic display.

#### Scenario: Check accepts warning diagnostic level

- **WHEN** the user runs `smoothe check --diagnostic-level warning template.mustache`
- **THEN** the CLI selects warning-and-error diagnostic display.

#### Scenario: Check accepts info diagnostic level

- **WHEN** the user runs `smoothe check --diagnostic-level info template.mustache`
- **THEN** the CLI selects error, warning, and info diagnostic display.

#### Scenario: Check accepts debug diagnostic level

- **WHEN** the user runs `smoothe check --diagnostic-level debug template.mustache`
- **THEN** the CLI selects error, warning, info, and debug diagnostic display.

#### Scenario: Invalid diagnostic level is rejected

- **WHEN** the user runs `smoothe check --diagnostic-level verbose template.mustache`
- **THEN** the CLI rejects the invalid diagnostic level.
