# mustache-parser-inputs Specification

## Purpose
Define parser input extensions for partial mappings, lambda/helper recognition,
context schemas, and source position metadata.
## Requirements
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

### Requirement: Context Schema Warnings

The system SHALL validate referenced context paths against the provided JSON
Schema by converting the supported schema subset into an internal context shape
model before semantic validation.

#### Scenario: Existing schema path is accepted

- **WHEN** the template references `{{user.name}}` and the context shape defines
  `user.name`
- **THEN** the parser does not emit a missing-path warning for that reference.

#### Scenario: Missing closed-object schema path emits warning

- **WHEN** the template references `{{user.name}}` and the context shape for
  `user` is closed and does not define `name`
- **THEN** the parser emits a warning diagnostic for the missing schema path.

#### Scenario: Missing permissive-object schema path is accepted

- **WHEN** the template references `{{metadata.anything}}` and the context shape
  for `metadata` permits additional properties
- **THEN** the parser does not emit a missing-path warning for that reference.

#### Scenario: Optional schema path emits warning

- **WHEN** the template references `{{user.fullname}}` and the context shape
  defines `fullname` but does not mark it required
- **THEN** the parser emits a warning diagnostic that the schema path is
  optional.

#### Scenario: Scalar traversal emits warning

- **WHEN** the template references `{{user.name.first}}` and the context shape
  defines `user.name` as a scalar
- **THEN** the parser emits a warning diagnostic for traversing a scalar schema
  path.

#### Scenario: Wrong section type emits warning

- **WHEN** the template uses a section for a schema path whose context shape is
  not valid as a section scope
- **THEN** the parser emits a warning diagnostic for unexpected schema type.

#### Scenario: Array section validates item scope

- **WHEN** the template uses `{{#items}}{{title}}{{/items}}` and the context
  shape defines `items` as an array of objects with `title`
- **THEN** the parser validates `title` against the array item shape.

#### Scenario: Unsupported schema construct emits warning

- **WHEN** context schema conversion encounters an unsupported schema construct
- **THEN** the parser emits a warning diagnostic for the schema input and
  continues validating supported schema portions.

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

