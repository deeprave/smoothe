# mustache-parser-inputs Specification

## Purpose
Define parser input extensions for partial mappings, lambda/helper recognition,
context schemas, and source position metadata.
## Requirements
### Requirement: Extended Parser Inputs

The system SHALL accept parser inputs for partial mappings, lambda/helper
specifications, context JSON Schema, and caller-provided source position
metadata.

#### Scenario: Parser accepts partial mappings

- **WHEN** a caller provides partial mappings as `{ name, path }` values
- **THEN** the parser accepts those mappings as part of the parser input.

#### Scenario: Parser accepts lambda specifications

- **WHEN** a caller provides lambda/helper specifications
- **THEN** the parser accepts those specifications as part of the parser input.

#### Scenario: Parser accepts context schema

- **WHEN** a caller provides a JSON Schema for template context
- **THEN** the parser accepts that schema as part of the parser input.

#### Scenario: Parser accepts body source position metadata

- **WHEN** a caller provides a body byte offset and body starting line number
- **THEN** the parser uses that metadata when calculating source locations for
  diagnostics.

### Requirement: Lambda Recognition

The system SHALL recognize references to configured lambda/helper names.

#### Scenario: Configured lambda is recognized

- **WHEN** a template references a name provided in the lambda specifications
- **THEN** the parser records that reference as lambda-recognized in parser
  state.

#### Scenario: Unknown lambda-like reference is not classified

- **WHEN** a template references a name absent from the lambda specifications
- **THEN** the parser does not classify that reference as a configured lambda.

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

### Requirement: Full Partial Graph Parsing

The system SHALL resolve configured static partials relative to the source
template root and parse the full reachable static partial graph as separate
template units.

#### Scenario: Configured partial is resolved

- **WHEN** a template references `{{> header}}` and parser input maps `header`
  to `partials/header.mustache`
- **THEN** the parser resolves that path relative to the source template root.

#### Scenario: Configured partial is parsed

- **WHEN** a resolved partial file is readable
- **THEN** the parser parses the partial source as a separate template unit and
  includes that unit in the returned AST graph.

#### Scenario: Missing partial reports diagnostic

- **WHEN** a template references a partial absent from the partial mapping
- **THEN** the parser emits an error diagnostic for the unresolved partial.

#### Scenario: Nested partial is resolved into graph

- **WHEN** a resolved partial references another mapped partial
- **THEN** the parser resolves and parses the nested partial source in the same
  parse operation.

#### Scenario: Partial graph preserves source metadata

- **WHEN** parser input resolves and parses partial files
- **THEN** each parsed partial retains its partial name, resolved path, body
  offset, and body starting line metadata.

#### Scenario: Recursive partial reference preserves graph

- **WHEN** recursive partial parsing detects that a partial path is already in
  the active resolution stack
- **THEN** the parser records a recursive reference to the existing parsed
  template unit without expanding that branch indefinitely.

