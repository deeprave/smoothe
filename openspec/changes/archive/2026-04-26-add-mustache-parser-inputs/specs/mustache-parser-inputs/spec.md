## ADDED Requirements

### Requirement: Extended Parser Inputs

The system SHALL accept parser inputs for partial mappings, lambda/helper
specifications, context JSON Schema, and frontmatter behavior.

#### Scenario: Parser accepts partial mappings

- **WHEN** a caller provides partial mappings as `{ name, path }` values
- **THEN** the parser accepts those mappings as part of the parser input.

#### Scenario: Parser accepts lambda specifications

- **WHEN** a caller provides lambda/helper specifications
- **THEN** the parser accepts those specifications as part of the parser input.

#### Scenario: Parser accepts context schema

- **WHEN** a caller provides a JSON Schema for template context
- **THEN** the parser accepts that schema as part of the parser input.

### Requirement: One-Level Partial Parsing

The system SHALL resolve configured partials relative to the source template
root and parse one level of inclusion.

#### Scenario: Configured partial is resolved

- **WHEN** a template references `{{> header}}` and parser input maps `header`
  to `partials/header.mustache`
- **THEN** the parser resolves that path relative to the source template root.

#### Scenario: Configured partial is parsed

- **WHEN** a resolved partial file is readable
- **THEN** the parser parses the partial source and attaches the parsed partial
  model to parser state.

#### Scenario: Missing partial reports diagnostic

- **WHEN** a template references a partial absent from the partial mapping
- **THEN** the parser emits a diagnostic for the unresolved partial.

#### Scenario: Nested partial is not recursively expanded

- **WHEN** a one-level parsed partial references another partial
- **THEN** the parser records the nested reference but does not parse the nested
  partial source in the same parse operation.

### Requirement: Lambda Recognition

The system SHALL recognize references to configured lambda/helper names.

#### Scenario: Configured lambda is recognized

- **WHEN** a template references a name provided in the lambda specifications
- **THEN** the parser records that reference as lambda-recognized in parser
  state.

#### Scenario: Unknown lambda-like reference is not classified

- **WHEN** a template references a name absent from the lambda specifications
- **THEN** the parser does not classify that reference as a configured lambda.

### Requirement: Frontmatter Context Extensions

The system SHALL parse supported frontmatter formats and expose arbitrary
frontmatter keys as context extensions.

#### Scenario: YAML frontmatter is parsed by default

- **WHEN** a template begins with YAML frontmatter
- **THEN** the parser parses the frontmatter as context extension data.

#### Scenario: JSON frontmatter is parsed when detected

- **WHEN** a template begins with frontmatter that is clearly JSON
- **THEN** the parser parses the frontmatter as JSON context extension data.

#### Scenario: TOML frontmatter is parsed when detected

- **WHEN** a template begins with frontmatter that is clearly TOML
- **THEN** the parser parses the frontmatter as TOML context extension data.

#### Scenario: Frontmatter keys are preserved

- **WHEN** frontmatter contains arbitrary keys
- **THEN** the parser state exposes those keys as context extensions.

### Requirement: Context Schema Warnings

The system SHALL validate referenced context paths against the provided JSON
Schema where practical.

#### Scenario: Existing schema path is accepted

- **WHEN** the template references `{{user.name}}` and the context schema
  defines `user.name`
- **THEN** the parser does not emit a missing-path warning for that reference.

#### Scenario: Missing schema path emits warning

- **WHEN** the template references `{{user.name}}` and the context schema does
  not define `user.name`
- **THEN** the parser emits a warning diagnostic for the missing schema path.

#### Scenario: Missing schema does not block parsing

- **WHEN** no context schema is provided
- **THEN** the parser still parses the template without schema-path warnings.
