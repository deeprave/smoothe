## MODIFIED Requirements

### Requirement: Core AST Nodes

The system SHALL parse core Mustache syntax into AST nodes with source spans,
including resolved static partial graph nodes when partial mappings allow
resolution.

#### Scenario: Text node is parsed

- **WHEN** the parser receives `hello`
- **THEN** the AST contains a text node spanning the source text.

#### Scenario: Escaped variable node is parsed

- **WHEN** the parser receives `{{name}}`
- **THEN** the AST contains an escaped variable node for `name` with a source
  span.

#### Scenario: Unescaped variable node is parsed

- **WHEN** the parser receives `{{{name}}}` or `{{& name}}`
- **THEN** the AST contains an unescaped variable node for `name` with a source
  span.

#### Scenario: Comment node is parsed

- **WHEN** the parser receives `{{! comment }}`
- **THEN** the AST records a comment node or comment event with a source span.

#### Scenario: Section node is parsed

- **WHEN** the parser receives `{{#items}}{{name}}{{/items}}`
- **THEN** the AST contains a section node named `items` with nested child
  nodes.

#### Scenario: Inverted section node is parsed

- **WHEN** the parser receives `{{^items}}empty{{/items}}`
- **THEN** the AST contains an inverted section node named `items` with nested
  child nodes.

#### Scenario: Partial reference node is parsed

- **WHEN** the parser receives `{{> header}}` and no resolved partial content is
  available
- **THEN** the AST contains a partial reference node named `header` with a
  source span.

#### Scenario: Resolved partial node is parsed

- **WHEN** the parser receives `{{> header}}` and partial mapping resolves
  `header` to readable template content
- **THEN** the AST contains a resolved partial node named `header` with the
  original source span and a link to the separately parsed partial template
  unit.

#### Scenario: Section boundaries do not cross partials

- **WHEN** a section is opened in one template unit and closed only from a
  resolved partial template unit
- **THEN** the parser reports an unclosed section diagnostic for the opening
  template unit.

### Requirement: Parser Diagnostics

The system SHALL produce structured diagnostics and safe partial results for
recoverable parse errors and incomplete partial graph resolution.

#### Scenario: Diagnostics include source location

- **WHEN** the parser emits a diagnostic for source text with filename metadata
- **THEN** the diagnostic includes filename, line, column, issue type, severity,
  and message.

#### Scenario: Feedback handler receives diagnostics

- **WHEN** parser input includes feedback handlers
- **THEN** parser diagnostics are sent to the corresponding handler based on
  severity.

#### Scenario: Recoverable error returns partial state

- **WHEN** parsing encounters recoverable syntax errors after some nodes were
  parsed
- **THEN** the parser returns safe parsed AST fragments and diagnostics.

#### Scenario: Unresolved static partial reports error

- **WHEN** the parser encounters a static partial reference that cannot be
  resolved through the effective partial mapping
- **THEN** the parser emits an error diagnostic with filename, line, column,
  issue type, and message.

#### Scenario: Partial parse diagnostic uses partial file

- **WHEN** parsing a resolved partial produces a diagnostic
- **THEN** the diagnostic source location refers to the partial file rather than
  the referring template.
