## Why

`smoothe` needs a persistent configuration layer so users can set common global
and command-specific defaults without repeating CLI flags on every invocation.
Adding configuration now establishes the option precedence model before the
`check` command grows its own argument surface.

## What Changes

- Add support for reading a TOML configuration file at startup.
- Discover `smoothe.toml` in the current directory, or in `$XDG_CONFIG_HOME`
  with `~/.config` as the default config home.
- Treat missing default-location configuration files as non-errors.
- Add `--config` / `-C <path>` to read configuration from an explicit path.
- Treat an unreadable explicitly specified config file as a fatal error.
- Apply option precedence in this order:
  - built-in defaults
  - configuration file
  - environment variables
  - CLI overrides
- Support global options under the top-level `[options]` table, initially:
  - `color = true | false | "always" | "never" | "auto"`
- Reserve top-level command tables, initially `[check]`, for command-specific
  defaults.
- Read the configuration file at most once and make the resulting configuration
  available to command execution.
- Pass check-related options to the `check` command after defaults and overrides
  have been applied.

## Capabilities

### New Capabilities

- `configuration`: TOML configuration discovery, loading, precedence, and typed
  availability to command execution.

### Modified Capabilities

- `cli`: Add `--config` / `-C <path>` and update global option handling so CLI
  values override configuration and environment-derived values.

## Impact

- Affected code: startup flow, CLI parser, configuration loading module, option
  merge logic, and command dispatch data passed to `check`.
- Dependencies: add TOML parsing support and likely home/config-directory path
  resolution support.
- APIs: extends the user-facing CLI with `--config` / `-C` and establishes the
  initial user-facing TOML configuration shape.
