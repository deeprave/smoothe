# configuration Specification

## Purpose
Define configuration file discovery, explicit configuration loading, supported
configuration shape, and option precedence.
## Requirements
### Requirement: Configuration File Discovery

The system SHALL look for a TOML configuration file named `smoothe.toml` during
startup when no explicit config path is provided.

#### Scenario: Current directory config is loaded

- **WHEN** the user runs `smoothe check template.mustache` from a directory
  containing `smoothe.toml`
- **THEN** the system reads that file as the configuration file.

#### Scenario: XDG config home config is loaded

- **WHEN** the current directory does not contain `smoothe.toml` and
  `$XDG_CONFIG_HOME/smoothe.toml` exists
- **THEN** the system reads `$XDG_CONFIG_HOME/smoothe.toml` as the configuration
  file.

#### Scenario: Default config home is used

- **WHEN** the current directory does not contain `smoothe.toml`,
  `$XDG_CONFIG_HOME` is unset, and `~/.config/smoothe.toml` exists
- **THEN** the system reads `~/.config/smoothe.toml` as the configuration file.

#### Scenario: Missing discovered config is allowed

- **WHEN** no configuration file exists in the current directory or config home
- **THEN** startup continues without a configuration file.

### Requirement: Explicit Configuration Path

The system SHALL support an explicit configuration path that bypasses default
configuration discovery.

#### Scenario: Explicit config path is loaded

- **WHEN** the user runs `smoothe --config custom.toml check template.mustache`
- **THEN** the system reads `custom.toml` as the configuration file.

#### Scenario: Explicit config path is loaded with short alias

- **WHEN** the user runs `smoothe -C custom.toml check template.mustache`
- **THEN** the system reads `custom.toml` as the configuration file.

#### Scenario: Missing explicit config is fatal

- **WHEN** the user specifies `--config missing.toml` and that file cannot be
  read
- **THEN** startup fails with an error.

#### Scenario: Malformed explicit config is fatal

- **WHEN** the user specifies `--config invalid.toml` and that file contains
  invalid TOML
- **THEN** startup fails with an error.

### Requirement: Configuration Shape

The system SHALL parse top-level TOML tables for global and command-specific
configuration.

#### Scenario: Global options table is parsed

- **WHEN** the configuration file contains an `[options]` table
- **THEN** the system parses global option defaults from that table.

#### Scenario: Color option accepts true

- **WHEN** the configuration file contains `[options]` with `color = true`
- **THEN** the effective color setting from configuration is always enabled.

#### Scenario: Color option accepts false

- **WHEN** the configuration file contains `[options]` with `color = false`
- **THEN** the effective color setting from configuration is never enabled.

#### Scenario: Color option accepts always

- **WHEN** the configuration file contains `[options]` with `color = "always"`
- **THEN** the effective color setting from configuration is always enabled.

#### Scenario: Color option accepts never

- **WHEN** the configuration file contains `[options]` with `color = "never"`
- **THEN** the effective color setting from configuration is never enabled.

#### Scenario: Color option accepts auto

- **WHEN** the configuration file contains `[options]` with `color = "auto"`
- **THEN** the effective color setting from configuration is automatic.

#### Scenario: Check command table is parsed

- **WHEN** the configuration file contains a `[check]` table
- **THEN** the system parses check-specific defaults from that table.

### Requirement: Option Precedence

The system SHALL resolve effective options in the order built-in defaults,
configuration file, environment variables, and CLI overrides.

#### Scenario: Configuration overrides built-in defaults

- **WHEN** the configuration file sets `[options]` `color = false`
- **THEN** the effective color setting is disabled instead of the built-in
  automatic default.

#### Scenario: Environment overrides configuration

- **WHEN** the configuration file sets `[options]` `color = true` and `NOCOLOR`
  is set in the environment
- **THEN** the effective color setting is disabled.

#### Scenario: CLI overrides environment

- **WHEN** the configuration file sets `[options]` `color = false`, `NOCOLOR` is
  set, and the user runs `smoothe --color always check template.mustache`
- **THEN** the effective color setting is enabled.

### Requirement: Single Configuration Read

The system SHALL read at most one configuration file during a startup.

#### Scenario: Current directory config prevents fallback read

- **WHEN** both `./smoothe.toml` and `$XDG_CONFIG_HOME/smoothe.toml` exist
- **THEN** the system reads `./smoothe.toml` and does not read the config-home
  file.

#### Scenario: Explicit config prevents discovery reads

- **WHEN** the user provides `--config custom.toml`
- **THEN** the system reads only `custom.toml` and does not read discovered
  config paths.

### Requirement: Check Output Configuration

The system SHALL allow check output format defaults to be configured in the
`[check]` configuration section.

#### Scenario: Config selects JSON output

- **WHEN** `[check] output = "json"` is configured
- **THEN** the check command uses JSON output unless overridden by the CLI.

#### Scenario: Config selects compiler output

- **WHEN** `[check] output = "compiler"` is configured
- **THEN** the check command uses compiler-style output unless overridden by
  the CLI.

#### Scenario: CLI output overrides config output

- **WHEN** `[check] output = "json"` is configured and the user runs
  `smoothe check --no-json template.mustache`
- **THEN** the check command uses compiler-style output.

### Requirement: Check Verbosity Configuration

The system SHALL allow check event display verbosity defaults to be configured
in the `[check]` configuration section.

#### Scenario: Config selects verbosity

- **WHEN** `[check] verbosity = "info"` is configured
- **THEN** the check command displays events at info verbosity and above unless
  overridden by the CLI.

#### Scenario: CLI verbosity overrides config

- **WHEN** `[check] verbosity = "debug"` is configured and the user runs
  `smoothe check --verbosity error template.mustache`
- **THEN** the check command displays only error events.

#### Scenario: Invalid configured verbosity fails config load

- **WHEN** `[check] verbosity` contains an unsupported value
- **THEN** configuration loading reports a configuration error.

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
