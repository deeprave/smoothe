## ADDED Requirements

### Requirement: Semantic Diagnostic Context

The system SHALL include expected/found/source context in semantic diagnostics
when that context is known.

#### Scenario: Schema semantic diagnostic includes context

- **WHEN** semantic validation emits a schema diagnostic with known expected and
  found shape information
- **THEN** the diagnostic includes that information as structured detail.

#### Scenario: Lambda semantic diagnostic includes context

- **WHEN** semantic validation emits a lambda diagnostic with known expected and
  actual usage information
- **THEN** the diagnostic includes that information as structured detail.

#### Scenario: Unknown context avoids speculative detail

- **WHEN** semantic validation lacks reliable expected or found information
- **THEN** the diagnostic does not include speculative expected/found detail.

### Requirement: Semantic Diagnostic Suggestions

The system SHALL include local near-hit suggestions in semantic diagnostics
when useful candidate sets are available.

#### Scenario: Variable path diagnostic suggests known fields

- **WHEN** semantic validation emits a missing-path diagnostic and the current
  schema scope has nearby known fields
- **THEN** the diagnostic includes those nearby field names as suggestions.

#### Scenario: Lambda diagnostic suggests known lambda names

- **WHEN** semantic validation emits an unknown-lambda diagnostic and nearby
  lambda definitions are known
- **THEN** the diagnostic includes those nearby lambda names as suggestions.

#### Scenario: Suggestions are omitted without candidates

- **WHEN** semantic validation has no useful local candidate set
- **THEN** the diagnostic omits suggestions.
