## ADDED Requirements

### Requirement: Config Partial Path Resolution
The system SHALL resolve relative partial paths declared in the configuration
file against the directory containing the loaded configuration file.

#### Scenario: Explicit config partial path is config-relative
- **WHEN** the user runs `smoothe --config config/smoothe.toml check pages/index.mustache`
- **AND** `config/smoothe.toml` declares `[check.partials] header = "partials/header.mustache"`
- **THEN** the system resolves the `header` partial path relative to `config/`.

#### Scenario: Discovered config partial path is config-relative
- **WHEN** the system discovers a `smoothe.toml` configuration file outside the
  process current directory
- **AND** that file declares a relative partial path under `[check.partials]`
- **THEN** the system resolves that partial path relative to the discovered
  configuration file's directory.

#### Scenario: Absolute config partial path remains absolute
- **WHEN** the loaded configuration file declares an absolute partial path under
  `[check.partials]`
- **THEN** the system uses that absolute path without joining it to the
  configuration file's directory.

#### Scenario: Config partial path is not template-relative
- **WHEN** the user checks `pages/index.mustache` from a different process
  current directory
- **AND** the loaded configuration file declares `[check.partials] header = "partials/header.mustache"`
- **THEN** the resolved config partial path does not change based on the
  checked template's directory or the process current directory.
