## Context

The parser work is intentionally split into stages. This first stage creates the
core parser foundation: source-aware tokenization, an AST, delimiter tracking,
structural validation, and diagnostics. Later changes will add parser inputs
such as partial file loading and JSON Schema validation, then advanced Mustache
features such as lambdas, inheritance, and dynamic names.

## Goals / Non-Goals

**Goals:**

- Provide a parser module independent of CLI commands.
- Accept source filename/path metadata, source text, and feedback handlers.
- Produce an AST plus parser state.
- Preserve source spans for meaningful nodes and diagnostics.
- Support core Mustache nodes: text, variables, unescaped variables, comments,
  sections, inverted sections, and partial references.
- Track active delimiter pairs and apply set-delimiter tags.
- Validate section structure and malformed tags.
- Return safe partial results while prioritizing diagnostics.

**Non-Goals:**

- Resolve or parse partial files.
- Validate references against JSON Schema.
- Parse frontmatter.
- Recognize lambdas, inheritance blocks, or dynamic names beyond preserving
  unsupported syntax as diagnostics.
- Render templates.

## Decisions

### Build Tokenization and AST Together

The first implementation should tokenize source while preserving spans, then
construct an AST with section nesting validation. This keeps diagnostics close to
the source positions that produced them.

Alternative considered: start with a string-scanning checker only. That would be
smaller, but it would not establish the AST foundation needed by later stages.

### Track Delimiter State in the Tokenizer

Set-delimiter tags should update tokenizer state so subsequent tags are
recognized with the active delimiters. The delimiter change itself can be
represented in parser state or the AST for future rendering.

Alternative considered: defer delimiter support to advanced features. Delimiter
state changes the meaning of subsequent tags, so it belongs in the core parser.

### Keep Diagnostics Structured and Decoupled

Diagnostics should be structured events containing severity, issue type,
filename, line, column, span when available, and message. Parser code should emit
through caller-provided feedback handlers and retain diagnostics in parser state.

Alternative considered: emit plain strings or use a logging framework directly.
That would couple parser output to one consumer and make tests less precise.

### Return Best-Effort Results

The parser should return safe AST fragments and state even when recoverable
errors occur. Serious structural errors should be reflected in diagnostics and
state, not by panicking.

Alternative considered: fail fast with no AST. That would be simpler but less
useful for editor/checking workflows that need multiple diagnostics.

## Risks / Trade-offs

- [Risk] Core AST choices can constrain later advanced features. -> Mitigation:
  include extensible node/state enums and keep unsupported syntax diagnosable
  rather than erased.
- [Risk] Recovery can produce confusing AST fragments. -> Mitigation: mark
  incomplete/recovered state clearly and make diagnostics authoritative.
- [Risk] Delimiter-aware tokenization is more complex than fixed delimiter
  parsing. -> Mitigation: include delimiter behavior in first-stage tests so the
  foundation is correct before later features build on it.
