## 1. Parser Module Foundation

- [x] 1.1 Add the parser module structure.
- [x] 1.2 Define parser input types for source metadata, source text, and
  feedback handlers.
- [x] 1.3 Define parser result and parser state types.
- [x] 1.4 Define structured diagnostic types with severity, issue type, source
  location, span, and message.

## 2. AST and Tokenization

- [x] 2.1 Define AST node types for text, escaped variables, unescaped
  variables, comments, sections, inverted sections, and partial references.
- [x] 2.2 Implement source position tracking for line, column, and byte spans.
- [x] 2.3 Implement tokenization using default Mustache delimiters.
- [x] 2.4 Implement delimiter-change tokenization support.
- [x] 2.5 Build the AST from core tokens.

## 3. Validation and Recovery

- [x] 3.1 Validate balanced section nesting.
- [x] 3.2 Emit diagnostics for unclosed sections.
- [x] 3.3 Emit diagnostics for mismatched closing tags.
- [x] 3.4 Emit diagnostics for malformed tags.
- [x] 3.5 Return safe partial AST/state for recoverable errors.
- [x] 3.6 Route diagnostics through caller-provided feedback handlers.

## 4. Verification

- [x] 4.1 Add parser tests under the separate `tests` hierarchy.
- [x] 4.2 Test parsing text, variables, unescaped variables, comments, sections,
  inverted sections, and partial references.
- [x] 4.3 Test delimiter changes.
- [x] 4.4 Test structural validation diagnostics.
- [x] 4.5 Test feedback handler diagnostic delivery.
- [x] 4.6 Run `cargo fmt --check`.
- [x] 4.7 Run `cargo clippy --all-targets --all-features -- -D warnings`.
- [x] 4.8 Run `cargo nextest run`.
