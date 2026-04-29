## ADDED Requirements

### Requirement: Check Semantic Validation Inputs

The system SHALL provide check command options for optional context schema and lambda definition inputs.

#### Scenario: Check accepts context schema input

- **WHEN** the user runs `smoothe check --schema schema.json template.mustache`
- **THEN** the CLI parses the schema path before dispatching the check command.

#### Scenario: Check accepts lambda definitions input

- **WHEN** the user runs `smoothe check --lambdas lambdas.json template.mustache`
- **THEN** the CLI parses the lambda definitions path before dispatching the check command.

#### Scenario: Check remains valid without semantic inputs

- **WHEN** the user runs `smoothe check` with template operands and no schema or lambda definitions options
- **THEN** the CLI dispatches the check command without requiring semantic validation paths.

#### Scenario: Check accepts explicit none values

- **WHEN** the user runs `smoothe check --schema none --lambdas none template.mustache`
- **THEN** the CLI dispatches the check command with schema and lambda checking disabled.

### Requirement: Check Semantic Validation Configuration

The system SHALL provide `[check]` configuration values for optional context schema and lambda definition inputs.

#### Scenario: Config schema path is resolved relative to config file

- **WHEN** `[check] schema` is configured as a path in a configuration file outside the current working directory
- **THEN** the system resolves the schema path relative to that configuration file.

#### Scenario: Config lambda path is resolved relative to config file

- **WHEN** `[check] lambdas` is configured as a path in a configuration file outside the current working directory
- **THEN** the system resolves the lambda definitions path relative to that configuration file.

#### Scenario: CLI schema overrides config schema

- **WHEN** both `[check] schema` and `--schema` are provided
- **THEN** the system uses the CLI schema value.

#### Scenario: CLI lambdas overrides config lambdas

- **WHEN** both `[check] lambdas` and `--lambdas` are provided
- **THEN** the system uses the CLI lambdas value.

#### Scenario: Config none disables semantic inputs

- **WHEN** `[check] schema = "none"` and `[check] lambdas = "none"` are configured
- **THEN** the system disables schema and lambda checking without loading files named `none`.
