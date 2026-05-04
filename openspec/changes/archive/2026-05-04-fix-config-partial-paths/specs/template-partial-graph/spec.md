## ADDED Requirements

### Requirement: Partial Mapping Filename Normalization
The system SHALL normalize configured and frontmatter-derived partial paths by
prefixing the resolved filename basename with `_` when the basename is not
already underscore-prefixed.

#### Scenario: Config partial filename gains underscore
- **WHEN** the loaded configuration file declares `[check.partials] header = "partials/header.mustache"`
- **THEN** the system attempts to read the mapped partial from
  `partials/_header.mustache` relative to the configuration file's directory.

#### Scenario: Config partial filename keeps existing underscore
- **WHEN** the loaded configuration file declares `[check.partials] header = "partials/_header.mustache"`
- **THEN** the system attempts to read the mapped partial from
  `partials/_header.mustache` without adding another underscore.

#### Scenario: Frontmatter partial filename gains underscore
- **WHEN** `pages/index.mustache` declares frontmatter `includes` containing
  `partials/header.mustache`
- **THEN** the system attempts to read the mapped partial from
  `pages/partials/_header.mustache`.

#### Scenario: Frontmatter partial filename keeps existing underscore
- **WHEN** `pages/index.mustache` declares frontmatter `includes` containing
  `partials/_header.mustache`
- **THEN** the system attempts to read the mapped partial from
  `pages/partials/_header.mustache` without adding another underscore.

### Requirement: Partial Origin Directories
The system SHALL use the partial declaration origin to choose the base
directory for relative partial paths.

#### Scenario: Config partials use config directory
- **WHEN** a relative partial path is declared in the configuration file
- **THEN** the system resolves that path relative to the directory containing
  the configuration file.

#### Scenario: Template partials use origin template directory
- **WHEN** a relative partial path is declared in template frontmatter
- **THEN** the system resolves that path relative to the directory containing
  the template that declared the frontmatter.
