## MODIFIED Requirements

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
