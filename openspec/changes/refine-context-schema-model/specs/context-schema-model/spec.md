## ADDED Requirements

### Requirement: Supported Context Schema Subset

The system SHALL define a supported JSON Schema subset for template context
validation.

#### Scenario: Supported primitive types are accepted

- **WHEN** a context schema uses `type` with `object`, `array`, `string`,
  `number`, `integer`, `boolean`, or `null`
- **THEN** the schema converter recognises the type as supported.

#### Scenario: Supported object keywords are accepted

- **WHEN** a context schema object uses `properties`, `required`, or
  `additionalProperties`
- **THEN** the schema converter recognises those keywords for context-shape
  validation.

#### Scenario: Supported array keyword is accepted

- **WHEN** a context schema array uses `items`
- **THEN** the schema converter recognises the array item schema for child
  scope validation.

#### Scenario: Supported enum keyword is accepted

- **WHEN** a context schema uses `enum`
- **THEN** the schema converter preserves the allowed literal values for
  diagnostics.

#### Scenario: Unsupported schema constructs warn

- **WHEN** a context schema uses unsupported constructs such as `$ref`,
  `$defs`, `definitions`, `oneOf`, `anyOf`, `allOf`, or `patternProperties`
- **THEN** the schema converter emits a warning diagnostic and continues with
  supported schema portions where possible.

### Requirement: Internal Context Shape Model

The system SHALL convert supported JSON Schema input into an internal context
shape model before template semantic validation.

#### Scenario: Object schema converts to object shape

- **WHEN** a schema has object type or object properties
- **THEN** the converter creates an object shape with named properties,
  required fields, and additional-properties behavior.

#### Scenario: Array schema converts to array shape

- **WHEN** a schema has array type or array items
- **THEN** the converter creates an array shape with an item shape.

#### Scenario: Scalar schema converts to scalar shape

- **WHEN** a schema has scalar type
- **THEN** the converter creates a scalar shape with the scalar kind and any
  enum or default metadata recognised by the converter.

#### Scenario: Permissive schema converts to any shape

- **WHEN** a schema position permits any value
- **THEN** the converter creates an any shape.

#### Scenario: Malformed supported keyword converts safely

- **WHEN** a supported keyword has a malformed value
- **THEN** the converter emits a warning diagnostic and represents that schema
  position as unknown or unsupported without panicking.

### Requirement: Additional Properties Semantics

The system SHALL model object additional-properties behavior for context path
validation.

#### Scenario: Closed object rejects unknown fields

- **WHEN** an object schema has `additionalProperties: false`
- **THEN** unknown fields under that object are reported as missing schema
  paths.

#### Scenario: Missing additionalProperties is permissive

- **WHEN** an object schema omits `additionalProperties`
- **THEN** unknown fields under that object are treated as allowed.

#### Scenario: True additionalProperties is permissive

- **WHEN** an object schema has `additionalProperties: true`
- **THEN** unknown fields under that object are treated as allowed.

#### Scenario: Schema-valued additionalProperties warns

- **WHEN** an object schema uses a schema value for `additionalProperties`
- **THEN** the converter emits an unsupported schema warning for that value.

### Requirement: Context Shape Path Resolution

The system SHALL resolve Mustache paths against the internal context shape
model.

#### Scenario: Existing property path resolves

- **WHEN** a template references `{{user.name}}` and the context shape defines
  object property `user.name`
- **THEN** path resolution succeeds for that reference.

#### Scenario: Missing closed-object property warns

- **WHEN** a template references a property absent from a closed object shape
- **THEN** semantic validation emits a missing schema path warning.

#### Scenario: Missing permissive-object property is allowed

- **WHEN** a template references a property absent from a permissive object
  shape
- **THEN** semantic validation does not emit a missing schema path warning for
  that property.

#### Scenario: Scalar traversal warns

- **WHEN** a template references `{{user.name.first}}` and `user.name` resolves
  to a scalar shape
- **THEN** semantic validation emits a warning that the path traverses a scalar
  value.

#### Scenario: Array item scope resolves child path

- **WHEN** a template uses `{{#items}}{{title}}{{/items}}` and `items` resolves
  to an array of objects with property `title`
- **THEN** semantic validation resolves `title` against the array item shape.

### Requirement: Context Shape Usage Diagnostics

The system SHALL use context shape information to produce schema-aware
diagnostics for Mustache usage.

#### Scenario: Optional property usage warns

- **WHEN** a template references a property that exists in the context shape but
  is not listed in that object shape's required fields
- **THEN** semantic validation emits a warning that the path is optional.

#### Scenario: Object section changes scope

- **WHEN** a section references an object shape
- **THEN** semantic validation validates child nodes against that object shape.

#### Scenario: Array section changes scope

- **WHEN** a section references an array shape
- **THEN** semantic validation validates child nodes against the array item
  shape.

#### Scenario: Boolean section preserves scope

- **WHEN** a section references a boolean shape
- **THEN** semantic validation accepts the section and validates child nodes
  against the current scope.

#### Scenario: Scalar section warns

- **WHEN** a section references a string, number, integer, or null scalar shape
- **THEN** semantic validation emits an unexpected schema type warning.

#### Scenario: Enum values appear in diagnostics

- **WHEN** semantic validation emits a warning for a path whose shape has enum
  values
- **THEN** the warning message includes the known allowed values when useful.
