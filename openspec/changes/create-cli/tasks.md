## 1. CLI Dependency and Parser Types

- [x] 1.1 Add `clap` with derive support to the Rust package dependencies.
- [x] 1.2 Define a top-level CLI parser type with package version metadata.
- [x] 1.3 Define the global color option using `clap::ColorChoice`, defaulting
  to `Auto`.
- [x] 1.4 Add canonical `--color`, alias `--colour`, and short `-c` support for
  the color option.
- [x] 1.5 Add `--no-color` and `NOCOLOR` support for disabling color.
- [x] 1.6 Confirm `--help`/`-h` and `--version`/`-V` are available through
  `clap`.
- [x] 1.7 Define a subcommand enum containing the initial `check` command.
- [x] 1.8 Define a dedicated check command argument type, even if it has no
  command-specific fields yet.

## 2. Command Dispatch and Handler Stub

- [x] 2.1 Add an explicit dispatcher that matches parsed subcommands.
- [x] 2.2 Add a `check` handler function that accepts the check command argument
  type.
- [x] 2.3 Route the `check` subcommand through the dispatcher to the `check`
  handler.
- [x] 2.4 Make the initial `check` handler return success without performing
  Mustache parsing or semantic validation.
- [x] 2.5 Ensure there is no default command and the check handler only runs for
  explicit `smoothe check`.

## 3. Entry Point Integration

- [x] 3.1 Update the binary entry point to parse CLI arguments through the
  top-level parser.
- [x] 3.2 Update the binary entry point to execute the dispatcher and propagate
  success or failure as the process result.

## 4. Verification

- [x] 4.1 Add or update tests that verify `smoothe --help` exits successfully.
- [x] 4.2 Add or update tests that verify `smoothe -h` exits successfully.
- [x] 4.3 Add or update tests that verify `smoothe --version` exits
  successfully.
- [x] 4.4 Add or update tests that verify `smoothe -V` exits successfully.
- [x] 4.5 Add or update tests that verify `smoothe --color <value> check`
  parses and dispatches successfully.
- [x] 4.6 Add or update tests that verify `smoothe --colour <value> check`
  parses and dispatches successfully using the same internal color setting.
- [x] 4.7 Add or update tests that verify `smoothe -c <value> check` parses and
  dispatches successfully using the same internal color setting.
- [x] 4.8 Add or update tests that verify `smoothe --no-color check` parses and
  dispatches successfully with color disabled.
- [x] 4.9 Add or update tests that verify `NOCOLOR` disables color for
  `smoothe check`.
- [x] 4.10 Add or update tests that verify `smoothe check` invokes the stubbed
  check handler and exits successfully.
- [x] 4.11 Add or update tests that verify `smoothe` without a subcommand does
  not invoke the check handler.
