## MODIFIED Requirements

### Requirement: Frontmatter Is Handled Outside Parser

The system SHALL extract and parse frontmatter before invoking the Mustache
parser, and the parser SHALL parse only the template body using caller-provided
source position metadata.

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

#### Scenario: Content diagnostic preserves source context

- **WHEN** content processing emits a diagnostic for frontmatter or includes
  handling
- **THEN** the diagnostic identifies the source file and can include structured
  context about the content metadata that caused it.
