## 1. Glob Expansion

- [ ] 1.1 Add or select a glob expansion mechanism for template input operands.
- [ ] 1.2 Detect glob metacharacters in non-stdin template operands before file reads.
- [ ] 1.3 Expand matching glob operands into filesystem paths.
- [ ] 1.4 Sort matched paths deterministically before reading inputs.
- [ ] 1.5 Preserve literal handling for ordinary file operands.
- [ ] 1.6 Preserve `-` as stdin without glob expansion.
- [ ] 1.7 Return an input error when a glob operand matches no files.

## 2. Command Integration

- [ ] 2.1 Wire glob expansion through the shared template input reader used by `check` and `parse`.
- [ ] 2.2 Confirm shell-expanded operands still process in the provided order.
- [ ] 2.3 Confirm mixed operands preserve command intent across literals, globs, and stdin.

## 3. Test Coverage

- [ ] 3.1 Add CLI test coverage for `check '**/*.mustache'`.
- [ ] 3.2 Add CLI test coverage for `parse '**/*.mustache'`.
- [ ] 3.3 Add CLI test coverage for direct glob operands without shell expansion.
- [ ] 3.4 Add CLI test coverage for unmatched glob failure.
- [ ] 3.5 Add CLI test coverage that `-` remains stdin.
- [ ] 3.6 Add behavioral fixture coverage for quoted glob input.

## 4. Documentation

- [ ] 4.1 Update README usage examples to mention direct glob operand support.
- [ ] 4.2 Update configuration or behavior documentation if it describes template input path handling.

## 5. Validation

- [ ] 5.1 Run `cargo fmt --check`.
- [ ] 5.2 Run `cargo clippy --all-targets --all-features -- -D warnings`.
- [ ] 5.3 Run `cargo nextest run`.
- [ ] 5.4 Run `cargo behave`.
- [ ] 5.5 Run `openspec validate add-glob-support --strict`.
