## 1. CLI Arguments and Help

- [x] 1.1 Update `check` command help description.
- [x] 1.2 Update `parse` command help description.
- [x] 1.3 Add positional input operands to `check`.
- [x] 1.4 Add positional input operands to `parse`.
- [x] 1.5 Add `parse --out <path>`.

## 2. Shared Input Handling

- [x] 2.1 Add shared input-reading support for file paths and `-`.
- [x] 2.2 Preserve operand processing order.
- [x] 2.3 Report file read failures with unsuccessful exit status.
- [x] 2.4 Support stdin input for both `check -` and `parse -`.

## 3. Command Behavior

- [x] 3.1 Update `check` to process one or more input operands.
- [x] 3.2 Update `parse` to process one or more input operands.
- [x] 3.3 Write parse diagnostics and AST output to `--out` when provided.
- [x] 3.4 Suppress normal stdout/stderr parse output when `--out` is provided.
- [x] 3.5 Compact parse AST output while preserving key node details.

## 4. Verification

- [x] 4.1 Add CLI tests for updated command help descriptions.
- [x] 4.2 Add CLI tests for `check` file operands and `check -`.
- [x] 4.3 Add CLI tests for `parse` file operands and `parse -`.
- [x] 4.4 Add CLI tests for multiple input operands preserving order.
- [x] 4.5 Add CLI tests for missing file errors.
- [x] 4.6 Add CLI tests for `parse --out`.
- [x] 4.7 Add CLI tests for compact AST output.
- [x] 4.8 Run `cargo fmt --check`.
- [x] 4.9 Run `cargo clippy --all-targets --all-features -- -D warnings`.
- [x] 4.10 Run `cargo nextest run`.
