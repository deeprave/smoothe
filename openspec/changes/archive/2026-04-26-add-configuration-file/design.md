## Context

`smoothe` currently parses global CLI options directly and dispatches the
explicit `check` command to a stub handler. Configuration support needs to fit
into that startup path without reading configuration multiple times or making
command handlers responsible for discovery.

The configuration file is TOML. Default discovery is optional, while an explicit
`--config` / `-C` path is mandatory and must fail the invocation if unreadable.
Resolved values must follow the user-facing precedence order: built-in defaults,
configuration file, environment variables, then CLI overrides.

## Goals / Non-Goals

**Goals:**

- Load at most one configuration file during startup.
- Discover `smoothe.toml` in the current directory before falling back to the XDG
  config home path, using `~/.config` when `$XDG_CONFIG_HOME` is unset.
- Add `--config` / `-C <path>` as a global CLI option that overrides discovery.
- Treat missing discovered config files as non-errors.
- Treat unreadable explicit config paths and malformed loaded TOML as fatal
  startup errors.
- Parse global configuration from `[options]`, initially `color`.
- Parse command-specific configuration from top-level command tables, initially
  `[check]`.
- Resolve effective options in the order built-in defaults, configuration file,
  environment variables, CLI overrides.
- Pass resolved global options and resolved check-specific options into command
  dispatch.

**Non-Goals:**

- Define final check-specific configuration fields.
- Add validation for checker inputs, schemas, partials, or diagnostics.
- Add configuration writes, migration, includes, profiles, or layered config
  files.
- Add project-root searching beyond the specified current-directory and XDG
  config-home locations.

## Decisions

### Add a dedicated configuration module

Configuration discovery, TOML parsing, and typed config structs should live in a
separate module from CLI parsing. Startup should parse CLI arguments first to
obtain the optional explicit config path, load configuration once, then resolve
effective options before dispatch.

Alternative considered: read configuration inside command handlers. That would
make it harder to guarantee single-read behavior and would spread precedence
logic across commands.

### Represent raw CLI inputs separately from effective options

The CLI parser should retain optional raw override fields, such as optional
color and optional config path. A separate resolved-options model should combine
built-in defaults, loaded config, environment values, and CLI overrides before
calling command handlers.

Alternative considered: mutate the parsed CLI struct in place after loading
configuration. That is less explicit about which values came from the CLI and
which are effective runtime settings.

### Use typed TOML deserialization

The TOML file should deserialize into typed structs for `[options]` and `[check]`
rather than using ad hoc table lookups throughout startup. Unknown future
command fields can be added to those structs as requirements are introduced.

Alternative considered: keep the parsed TOML as a dynamic value map. That would
avoid early structs, but it pushes validation and defaulting into less focused
call sites.

### Treat explicit and discovered config failures differently

Default discovery should attempt the current-directory path first and then the
XDG path. Missing files at these locations are acceptable and simply mean no
configuration file was loaded. When `--config` / `-C` is provided, the exact path
must be read and any read or parse failure should return a fatal startup error.

Alternative considered: make discovered malformed config non-fatal. That would
hide user mistakes in an existing config file and make configuration behavior
hard to diagnose.

### Keep command configuration typed even while empty

The resolved `check` options should be passed into the `check` handler even if
there are no check-specific fields yet. This preserves the command boundary for
future flags and avoids changing the dispatch shape when `[check]` gains
meaningful options.

Alternative considered: only pass global options until check settings exist.
That is simpler now but creates avoidable churn as soon as check defaults are
introduced.

### Keep template inputs explicit

Configuration discovery and option resolution must not change the command input
model. `check` and `parse` continue to require one or more template input
operands; tests and examples should use a file operand or `-` for stdin.

## Risks / Trade-offs

- [Risk] A config-home app subdirectory can produce redundant names such as
  `$XDG_CONFIG_HOME/smoothe/smoothe.toml`. -> Mitigation: use the simpler direct
  config-home path `$XDG_CONFIG_HOME/smoothe.toml`.
- [Risk] Boolean config values for color differ from CLI color strings. ->
  Mitigation: normalize TOML `true`, `false`, `"always"`, `"never"`, and
  `"auto"` into the same effective color model used by CLI values.
- [Risk] Config loading can become a hidden dependency of tests and commands. ->
  Mitigation: make discovery explicit in startup, keep loading single-use, and
  test explicit paths plus no-config behavior.
- [Risk] Adding typed structures before check options exist could become
  overbuilt. -> Mitigation: keep check options empty and only model the boundary
  required by the proposal.
