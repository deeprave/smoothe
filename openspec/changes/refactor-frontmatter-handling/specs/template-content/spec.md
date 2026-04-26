## ADDED Requirements

### Requirement: Template Content Result
The system SHALL expose a template content result that contains the raw input
data, parsed frontmatter context, template body byte offset, template body
starting line number, parsed AST, and parser state including diagnostics.

#### Scenario: Content result preserves raw data and parsed output
- **WHEN** the system processes a template file containing frontmatter and a
  Mustache body
- **THEN** the returned content result contains the full raw data, frontmatter
  context, body offset, body starting line number, AST, and parser state.

#### Scenario: Template without frontmatter starts at beginning
- **WHEN** the system processes a template with no frontmatter block
- **THEN** the returned content result has a body offset of `0` and a body
  starting line number of `1`.

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

### Requirement: Frontmatter Context Extraction
The system SHALL parse supported frontmatter formats into content frontmatter
context while excluding the frontmatter block from the template body.

#### Scenario: YAML frontmatter is extracted
- **WHEN** a template starts with YAML frontmatter
- **THEN** the content result exposes the parsed YAML value as frontmatter
  context.

#### Scenario: JSON frontmatter is extracted
- **WHEN** a template starts with frontmatter that is clearly JSON
- **THEN** the content result exposes the parsed JSON value as frontmatter
  context.

#### Scenario: TOML frontmatter is extracted
- **WHEN** a template starts with frontmatter that is clearly TOML
- **THEN** the content result exposes the parsed TOML value as frontmatter
  context.

#### Scenario: Invalid frontmatter reports diagnostic
- **WHEN** the system detects a frontmatter block but cannot parse its content
- **THEN** the content result includes a warning diagnostic for the frontmatter
  parse failure.

### Requirement: Includes-Derived Partial Mappings
The system SHALL derive parser partial mappings from a frontmatter `includes`
list.

#### Scenario: Include path derives partial key
- **WHEN** frontmatter contains `includes` with the path
  `../_partials/path.mustache`
- **THEN** the derived partial mapping uses `path` as the partial key.

#### Scenario: Include path derives underscore-prefixed filesystem path
- **WHEN** frontmatter contains `includes` with the path
  `../_partials/path.mustache`
- **THEN** the derived partial mapping path passed to the parser is
  `../_partials/_path.mustache`.

#### Scenario: Hyphenated include basename is preserved
- **WHEN** frontmatter contains `includes` with the path
  `../_partials/another-path.mustache`
- **THEN** the derived partial mapping uses `another-path` as the partial key
  and `../_partials/_another-path.mustache` as the parser path.

#### Scenario: Existing underscore prefix is not duplicated
- **WHEN** frontmatter contains `includes` with a final filename that is already
  underscore-prefixed
- **THEN** the derived parser path keeps a single leading underscore on that
  filename.

### Requirement: Partial Mapping Precedence
The system SHALL merge frontmatter-derived partial mappings with caller-provided
partial mappings using deterministic precedence.

#### Scenario: Explicit caller mapping wins
- **WHEN** a caller provides a partial mapping with the same key as a mapping
  derived from frontmatter `includes`
- **THEN** the parser receives the caller-provided mapping for that key.

#### Scenario: Frontmatter mapping is used when no explicit mapping exists
- **WHEN** frontmatter provides an `includes` entry whose key is absent from the
  caller-provided partial mappings
- **THEN** the parser receives the frontmatter-derived mapping for that key.

### Requirement: Unsupported Includes Values Report Diagnostics
The system SHALL report warnings for unsupported `includes` values without
preventing template body parsing.

#### Scenario: Includes is not a list
- **WHEN** frontmatter contains an `includes` key whose value is not a list
- **THEN** the content result includes a warning diagnostic and still parses the
  template body.

#### Scenario: Includes entry is not a string path
- **WHEN** frontmatter contains an `includes` list entry that is not a string
- **THEN** the content result includes a warning diagnostic and still processes
  the remaining valid include entries.
