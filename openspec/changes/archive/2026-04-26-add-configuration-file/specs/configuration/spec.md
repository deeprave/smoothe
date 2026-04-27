## ADDED Requirements

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
