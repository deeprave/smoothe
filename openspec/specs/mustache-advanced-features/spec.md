# mustache-advanced-features Specification

## Purpose
Define parser support for advanced Mustache constructs such as lambdas,
inheritance, and dynamic names.
## Requirements
### Requirement: Lambda Syntax Modeling

The system SHALL model lambda-related Mustache structures without executing
lambda code, using structured lambda definitions when they are available.

#### Scenario: Lambda section is represented

- **WHEN** the parser receives a section whose name is a configured lambda that
  allows section usage
- **THEN** the AST/state represents that section as lambda-recognized without
  executing the lambda.

#### Scenario: Lambda variable is represented

- **WHEN** the parser receives a variable whose name is a configured lambda that
  allows variable usage
- **THEN** the AST/state represents that variable as lambda-recognized without
  executing the lambda.

#### Scenario: Both-usage lambda is represented in either form

- **WHEN** the parser receives a variable or section whose name is a configured
  lambda that allows both usage forms
- **THEN** the AST/state represents that reference as lambda-recognized without
  executing the lambda.

#### Scenario: Inverted lambda section is unsupported

- **WHEN** semantic validation sees an inverted section whose name resolves to a
  known lambda
- **THEN** the system emits an error diagnostic because inverted lambda
  sections are unsupported.

#### Scenario: Lambda metadata does not execute code

- **WHEN** the system models a lambda reference using structured lambda
  metadata
- **THEN** it does not execute the lambda.

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

### Requirement: Lambda Semantic Usage Constraints

The system SHALL enforce semantic validation constraints for known lambda references without executing lambda code.

#### Scenario: Positive lambda section is supported

- **WHEN** semantic validation sees a positive section whose name resolves to a known section lambda
- **THEN** the system treats the lambda section form as supported.

#### Scenario: Inverted lambda section is unsupported

- **WHEN** semantic validation sees an inverted section whose name resolves to a known lambda
- **THEN** the system emits a warning that inverted lambda sections are unsupported.

#### Scenario: Lambda execution is not required

- **WHEN** semantic validation checks a lambda variable or lambda section
- **THEN** the system validates the lambda reference and declared usage without executing the lambda.

