# mustache-core-parser Specification

## Purpose
Define the core Mustache parser API, AST nodes, delimiter handling, structural
validation, and diagnostics.
## Requirements
### Requirement: Core Parser API

The system SHALL provide a parser module that accepts source metadata, source
text, and feedback handlers, then returns an AST plus parser state.

#### Scenario: Parser accepts source input

- **WHEN** a caller invokes the parser with filename/path metadata and source
  text
- **THEN** the parser accepts those values through a structured parser input.

#### Scenario: Parser returns AST and state

- **WHEN** parsing completes
- **THEN** the parser returns an AST and parser state containing diagnostics and
  collected parser metadata.

#### Scenario: Parser is command independent

- **WHEN** the parser module is used
- **THEN** it does not depend on CLI argument parsing or the `check` command.

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

### Requirement: Delimiter Handling

The system SHALL track active Mustache delimiters during tokenization.

#### Scenario: Default delimiters parse tags

- **WHEN** the parser receives `{{name}}`
- **THEN** the parser recognizes the tag using the default `{{` and `}}`
  delimiters.

#### Scenario: Delimiter change affects later tags

- **WHEN** the parser receives `{{=<% %>=}}<%name%>`
- **THEN** the parser recognizes `<%name%>` as a variable tag using the updated
  delimiters.

### Requirement: Structural Validation

The system SHALL validate core Mustache structure while parsing.

#### Scenario: Balanced section succeeds

- **WHEN** the parser receives `{{#items}}{{/items}}`
- **THEN** parsing completes without a section-balance error.

#### Scenario: Unclosed section reports diagnostic

- **WHEN** the parser receives `{{#items}}`
- **THEN** the parser emits an error diagnostic for the unclosed section.

#### Scenario: Mismatched section reports diagnostic

- **WHEN** the parser receives `{{#items}}{{/users}}`
- **THEN** the parser emits an error diagnostic for the mismatched closing tag.

#### Scenario: Malformed tag reports diagnostic

- **WHEN** the parser receives a malformed Mustache tag
- **THEN** the parser emits an error diagnostic with filename, line, column,
  issue type, and message.

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

