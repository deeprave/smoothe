## Why

`smoothe` needs a stable command-line entry point before checker functionality can
be exposed to users or exercised in integration workflows. Establishing the CLI
shape now gives later parser, checker, and renderer work a consistent dispatch
surface to build on.

## What Changes

- Add a basic CLI parser using `clap`.
- Support standard global CLI flags for Rust CLI applications:
  - `--color` and `--colour`
  - `--version`
  - `--help`
- Add a command dispatcher that routes parsed commands to command handlers.
- Introduce an initial `check` command with its own argument surface reserved
  for future definition.
- Route `check` execution to a `check` function that initially exists as a stub.

## Capabilities

### New Capabilities

- `cli`: Command-line parsing, global options, command dispatch, and the initial
  `check` command entry point.

### Modified Capabilities

None.

## Impact

- Affected code: binary entry point, CLI parsing module, command dispatch module,
  and initial check command handler.
- Dependencies: add `clap` for argument parsing.
- APIs: establishes the user-facing CLI surface for global flags and command
  dispatch, without defining the final `check` command arguments yet.
