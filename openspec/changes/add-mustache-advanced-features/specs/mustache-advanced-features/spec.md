## ADDED Requirements

### Requirement: Lambda Syntax Modeling

The system SHALL model lambda-related Mustache structures without executing
lambda code.

#### Scenario: Lambda section is represented

- **WHEN** the parser receives a section whose name is a configured lambda
- **THEN** the AST/state represents that section as lambda-recognized without
  executing the lambda.

#### Scenario: Lambda variable is represented

- **WHEN** the parser receives a variable whose name is a configured lambda
- **THEN** the AST/state represents that variable as lambda-recognized without
  executing the lambda.

### Requirement: Inheritance Syntax

The system SHALL represent Mustache inheritance syntax explicitly.

#### Scenario: Parent template reference is parsed

- **WHEN** the parser receives a Mustache parent/inheritance tag
- **THEN** the AST/state records the parent reference and its source span.

#### Scenario: Block definition is parsed

- **WHEN** the parser receives a Mustache inheritance block definition
- **THEN** the AST/state records the block name, children, and source span.

#### Scenario: Malformed inheritance emits diagnostic

- **WHEN** inheritance syntax is malformed
- **THEN** the parser emits a structured diagnostic for the issue.

### Requirement: Dynamic Names

The system SHALL represent dynamic-name syntax explicitly.

#### Scenario: Dynamic partial name is parsed

- **WHEN** the parser receives a dynamic partial name
- **THEN** the AST/state records the dynamic expression and source span.

#### Scenario: Dynamic parent name is parsed

- **WHEN** the parser receives a dynamic parent name
- **THEN** the AST/state records the dynamic expression and source span.

#### Scenario: Malformed dynamic name emits diagnostic

- **WHEN** dynamic-name syntax is malformed
- **THEN** the parser emits a structured diagnostic for the issue.

### Requirement: Advanced Fixture Coverage

The system SHALL include tests or fixtures for supported advanced Mustache
features.

#### Scenario: Supported upstream lambda fixtures are covered

- **WHEN** the advanced feature tests run
- **THEN** they cover supported lambda cases from the upstream Mustache spec.

#### Scenario: Supported upstream inheritance fixtures are covered

- **WHEN** the advanced feature tests run
- **THEN** they cover supported inheritance cases from the upstream Mustache
  spec.

#### Scenario: Supported upstream dynamic name fixtures are covered

- **WHEN** the advanced feature tests run
- **THEN** they cover supported dynamic-name cases from the upstream Mustache
  spec.
