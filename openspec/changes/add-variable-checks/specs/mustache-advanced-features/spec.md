## ADDED Requirements

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
