## MODIFIED Requirements

### Requirement: Frontmatter Is Handled Outside Parser

The system SHALL extract and parse frontmatter before invoking the Mustache
parser for primary templates, and SHALL prepare partial template bodies by
detecting and skipping frontmatter before partial parsing. The parser SHALL
parse only template bodies using caller-provided source position metadata.

#### Scenario: Frontmatter is skipped during template parsing

- **WHEN** a template starts with frontmatter followed by Mustache content
- **THEN** the AST contains nodes only for the Mustache content after the
  frontmatter block.

#### Scenario: Diagnostics use original source locations

- **WHEN** the parser reports a diagnostic for content after a frontmatter block
- **THEN** the diagnostic line and column are calculated relative to the
  original raw input, not relative to a stripped body string.

#### Scenario: Parser state does not expose frontmatter context

- **WHEN** the Mustache parser returns parser state
- **THEN** frontmatter context is exposed by the content result rather than by
  parser state.

#### Scenario: Partial frontmatter is skipped during partial parsing

- **WHEN** a resolved partial file starts with frontmatter followed by Mustache
  content
- **THEN** the parsed partial AST contains nodes only for the Mustache content
  after the partial frontmatter block.

#### Scenario: Partial frontmatter is not merged into parent context

- **WHEN** a resolved partial file contains frontmatter
- **THEN** the parent template content result exposes only the parent
  frontmatter context.

#### Scenario: Partial diagnostics use original partial source locations

- **WHEN** parsing reports a diagnostic for content after a partial
  frontmatter block
- **THEN** the diagnostic line and column are calculated relative to the
  original partial file, not relative to a stripped partial body string.

## ADDED Requirements

### Requirement: Partial Source Preparation

The system SHALL provide a reusable source preparation boundary for primary
templates and partial templates.

#### Scenario: Primary template source is prepared

- **WHEN** the system processes a primary template
- **THEN** it extracts frontmatter, determines the body byte offset, determines
  the body starting line number, and passes those values to parsing.

#### Scenario: Partial template source is prepared

- **WHEN** the system reads a resolved partial file
- **THEN** it determines the partial body byte offset and body starting line
  number before parsing the partial body.

#### Scenario: Partial source preparation ignores context merge

- **WHEN** partial source preparation extracts partial frontmatter
- **THEN** it does not merge the partial frontmatter context with the parent
  template frontmatter context.
