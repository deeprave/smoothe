## 1. CLI Dependency and Parser Types

- [ ] 1.1 Add `clap` with derive support to the Rust package dependencies.
- [ ] 1.2 Define a top-level CLI parser type with package version metadata.
- [ ] 1.3 Define the global color option with canonical `--color` support and
  `--colour` as an alias.
- [ ] 1.4 Define a subcommand enum containing the initial `check` command.
- [ ] 1.5 Define a dedicated check command argument type, even if it has no
  command-specific fields yet.

## 2. Command Dispatch and Handler Stub

- [ ] 2.1 Add an explicit dispatcher that matches parsed subcommands.
- [ ] 2.2 Add a `check` handler function that accepts the check command argument
  type.
- [ ] 2.3 Route the `check` subcommand through the dispatcher to the `check`
  handler.
- [ ] 2.4 Make the initial `check` handler return success without performing
  Mustache parsing or semantic validation.

## 3. Entry Point Integration

- [ ] 3.1 Update the binary entry point to parse CLI arguments through the
  top-level parser.
- [ ] 3.2 Update the binary entry point to execute the dispatcher and propagate
  success or failure as the process result.

## 4. Verification

- [ ] 4.1 Add or update tests that verify `smoothe --help` exits successfully.
- [ ] 4.2 Add or update tests that verify `smoothe --version` exits successfully.
- [ ] 4.3 Add or update tests that verify `smoothe --color <value> check`
  parses and dispatches successfully.
- [ ] 4.4 Add or update tests that verify `smoothe --colour <value> check`
  parses and dispatches successfully using the same internal color setting.
- [ ] 4.5 Add or update tests that verify `smoothe check` invokes the stubbed
  check handler and exits successfully.
