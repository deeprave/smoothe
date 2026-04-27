## 1. CLI Surface

- [x] 1.1 Add `parse` to the CLI command enum.
- [x] 1.2 Add command dispatch for the `parse` subcommand.
- [x] 1.3 Add a `parse` command module.

## 2. Parse Command Behavior

- [x] 2.1 Read the complete template source from stdin.
- [x] 2.2 Parse stdin content using the existing parser API.
- [x] 2.3 Print the parsed AST to stdout in developer-readable debug format.
- [x] 2.4 Print parser diagnostics, including warnings, when present.
- [x] 2.5 Return success when no error diagnostics are present.
- [x] 2.6 Return failure when one or more error diagnostics are present.

## 3. Verification

- [x] 3.1 Add CLI tests for valid stdin parsing and AST output.
- [x] 3.2 Add CLI tests for invalid stdin diagnostics.
- [x] 3.3 Add CLI tests for parse command exit status behavior.
- [x] 3.4 Add CLI tests for warning display when warning-producing stdin-only
  parser behavior is available.
- [x] 3.5 Run `cargo fmt --check`.
- [x] 3.6 Run `cargo clippy --all-targets --all-features -- -D warnings`.
- [x] 3.7 Run `cargo nextest run`.
