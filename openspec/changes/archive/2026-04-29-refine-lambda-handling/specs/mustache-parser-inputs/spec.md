## MODIFIED Requirements

### Requirement: Extended Parser Inputs

The system SHALL accept parser inputs for partial mappings, structured
lambda/helper specifications, context JSON Schema, and caller-provided source
position metadata.

#### Scenario: Parser accepts partial mappings

- **WHEN** a caller provides partial mappings as `{ name, path }` values
- **THEN** the parser accepts those mappings as part of the parser input.

#### Scenario: Parser accepts lambda specifications

- **WHEN** a caller provides structured lambda/helper specifications
- **THEN** the parser accepts those specifications as part of the parser input.

#### Scenario: Parser accepts context schema

- **WHEN** a caller provides a JSON Schema for template context
- **THEN** the parser accepts that schema as part of the parser input.

#### Scenario: Parser accepts body source position metadata

- **WHEN** a caller provides a body byte offset and body starting line number
- **THEN** the parser uses that metadata when calculating source locations for
  diagnostics.

### Requirement: Lambda Recognition

The system SHALL recognize references to configured lambda/helper definitions.

#### Scenario: Configured variable lambda is recognized

- **WHEN** a template references a name provided in the lambda specifications
  and that definition allows variable usage
- **THEN** the parser records that variable reference as lambda-recognized in
  parser state.

#### Scenario: Configured section lambda is recognized

- **WHEN** a template references a name provided in the lambda specifications
  and that definition allows section usage
- **THEN** the parser records that section reference as lambda-recognized in
  parser state.

#### Scenario: Configured both-usage lambda is recognized

- **WHEN** a template references a name provided in the lambda specifications
  and that definition allows both variable and section usage
- **THEN** the parser records either supported form as lambda-recognized in
  parser state.

#### Scenario: Unknown lambda-like reference is not classified

- **WHEN** a template references a name absent from the lambda specifications
- **THEN** the parser does not classify that reference as a configured lambda.

#### Scenario: Lambda shape metadata is preserved for semantic validation

- **WHEN** a caller provides lambda argument, return, or side-effect metadata
- **THEN** the system preserves that metadata for semantic validation without
  executing the lambda.
