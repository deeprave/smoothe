## 1. Dependencies and Config Types

- [x] 1.1 Add TOML deserialization support to the Rust package dependencies.
- [x] 1.2 Add any required path/home-directory support for resolving
  `~/.config`.
- [x] 1.3 Add a dedicated configuration module.
- [x] 1.4 Define typed configuration structs for `[options]` and `[check]`.
- [x] 1.5 Define a typed color configuration value that accepts `true`,
  `false`, `"always"`, `"never"`, and `"auto"`.

## 2. Config Discovery and Loading

- [x] 2.1 Implement default discovery for `./smoothe.toml`.
- [x] 2.2 Implement fallback discovery for `$XDG_CONFIG_HOME/smoothe.toml`.
- [x] 2.3 Implement fallback discovery for `~/.config/smoothe.toml` when
  `$XDG_CONFIG_HOME` is unset.
- [x] 2.4 Ensure missing discovered config files are treated as no-config
  success.
- [x] 2.5 Implement explicit config loading for `--config` and `-C` paths.
- [x] 2.6 Ensure unreadable explicit config paths are fatal.
- [x] 2.7 Ensure malformed loaded TOML is fatal.
- [x] 2.8 Ensure startup reads at most one configuration file.

## 3. Effective Option Resolution

- [x] 3.1 Define resolved global options with built-in defaults.
- [x] 3.2 Define resolved check options, even if no check-specific fields exist
  yet.
- [x] 3.3 Apply configuration file values over built-in defaults.
- [x] 3.4 Apply environment values over configuration values.
- [x] 3.5 Apply CLI values over environment values.
- [x] 3.6 Normalize CLI and TOML color values into one effective color model.

## 4. CLI and Dispatch Integration

- [x] 4.1 Add a global `--config <path>` CLI option.
- [x] 4.2 Add the `-C <path>` short alias for the config option.
- [x] 4.3 Update startup to parse CLI values, load configuration once, resolve
  effective options, and dispatch.
- [x] 4.4 Update command dispatch to pass resolved global options and resolved
  check options.
- [x] 4.5 Update the `check` handler signature to accept resolved options.

## 5. Verification

- [x] 5.1 Add tests that verify default startup succeeds when no config file
  exists.
- [x] 5.2 Add tests that verify `./smoothe.toml` is loaded when present.
- [x] 5.3 Add tests that verify `$XDG_CONFIG_HOME/smoothe.toml` is loaded when no
  current-directory config exists.
- [x] 5.4 Add tests that verify `~/.config/smoothe.toml` is loaded when
  `$XDG_CONFIG_HOME` is unset.
- [x] 5.5 Add tests that verify `--config <path>` loads the explicit config
  path.
- [x] 5.6 Add tests that verify `-C <path>` loads the explicit config path.
- [x] 5.7 Add tests that verify missing explicit config paths fail startup.
- [x] 5.8 Add tests that verify malformed explicit config paths fail startup.
- [x] 5.9 Add tests that verify config color values `true`, `false`,
  `"always"`, `"never"`, and `"auto"` parse successfully.
- [x] 5.10 Add tests that verify precedence from defaults to config,
  environment, and CLI overrides.
- [x] 5.11 Add tests that verify discovered config reads only one file.
- [x] 5.12 Add tests that verify explicit config bypasses discovered config
  paths.
- [x] 5.13 Run `cargo fmt --check`.
- [x] 5.14 Run `cargo clippy --all-targets --all-features -- -D warnings`.
- [x] 5.15 Run `cargo nextest run`.
