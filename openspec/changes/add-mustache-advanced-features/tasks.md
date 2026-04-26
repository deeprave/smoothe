## 1. Advanced AST Model

- [x] 1.1 Add AST/state representation for lambda-recognized variables and
  sections.
- [x] 1.2 Add AST/state representation for inheritance parent references.
- [x] 1.3 Add AST/state representation for inheritance block definitions.
- [x] 1.4 Add AST/state representation for dynamic names.

## 2. Advanced Parsing and Diagnostics

- [x] 2.1 Parse supported lambda-related structures without executing lambdas.
- [x] 2.2 Parse supported inheritance syntax.
- [x] 2.3 Parse supported dynamic-name syntax.
- [x] 2.4 Emit diagnostics for malformed inheritance syntax.
- [x] 2.5 Emit diagnostics for malformed dynamic-name syntax.

## 3. Fixture Coverage

- [x] 3.1 Add supported upstream lambda fixture coverage.
- [x] 3.2 Add supported upstream inheritance fixture coverage.
- [x] 3.3 Add supported upstream dynamic-name fixture coverage.
- [x] 3.4 Document unsupported upstream advanced cases in tests or fixtures.

## 4. Verification

- [x] 4.1 Run `cargo fmt --check`.
- [x] 4.2 Run `cargo clippy --all-targets --all-features -- -D warnings`.
- [x] 4.3 Run `cargo nextest run`.
