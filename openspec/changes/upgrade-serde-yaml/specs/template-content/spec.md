## ADDED Requirements

### Requirement: Maintained YAML Frontmatter Parser
The system SHALL parse supported YAML frontmatter through a maintained
Serde-compatible YAML parser dependency.

#### Scenario: Supported YAML frontmatter remains compatible
- **WHEN** a template starts with YAML frontmatter containing ordinary metadata
  fields and an `includes` sequence of string paths
- **THEN** the content result exposes the same frontmatter context shape used by
  content processing and partial include derivation.

#### Scenario: YAML parse failures remain warnings
- **WHEN** the system detects a YAML frontmatter block but the YAML content is
  invalid
- **THEN** the content result includes a warning diagnostic for the frontmatter
  parse failure and still parses the template body.

#### Scenario: Frontmatter state shape is unchanged
- **WHEN** YAML frontmatter is parsed successfully
- **THEN** the parsed context remains exposed as `serde_json::Value` through
  the content result.
