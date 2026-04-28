## MODIFIED Requirements

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
