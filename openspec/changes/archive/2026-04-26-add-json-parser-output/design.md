## Context

The `parse` command currently reads one or more template inputs, reports parser
diagnostics, and prints a compact tree representation of the AST. That tree is
useful for humans, but downstream tooling needs valid JSON that can be parsed
without depending on the tree text format or scraping diagnostic text.

The parser AST already has a structured model, while the command currently owns
the output projection. This change should keep the parser focused on parsing
and add JSON formatting at the command/output boundary.

## Goals / Non-Goals

**Goals:**

- Add `--json` and `-j` flags to the `parse` command.
- Emit valid JSON parse-result output when JSON mode is selected.
- Include diagnostics in JSON mode, grouped into `errors` and `warnings`
  arrays.
- Keep the existing compact tree output as the default.
- Preserve current exit-status behavior.
- Support multiple parse inputs in a deterministic JSON shape.

**Non-Goals:**

- Do not replace the compact tree output.
- Do not change parser semantics or AST construction.
- Do not add rendering, checking, or schema validation behavior.
- Do not define JSON serialization for info/debug diagnostics unless they are
  needed by current parse command output.

## Decisions

### Add a Parse Output Mode Flag

Add a boolean `json` field to `ParseArgs` with `#[arg(long, short = 'j')]`.
The command selects between the existing compact formatter and a new JSON
formatter after parsing succeeds or partially recovers.

Alternative considered: add a general `--format <tree|json>` option. That is
more extensible, but the requested interface is explicit and smaller. A future
format enum can still replace the boolean if more output formats appear.

### Use a Dedicated JSON Projection

Implement a command-level serializable projection for parse output instead of
requiring every parser type to derive `Serialize` immediately. The projection
should include:

- input name
- AST node list
- node kind
- node fields such as text, name, expression, delimiter values, and spans
- recursive children where applicable
- `errors` list
- `warnings` list
- diagnostic fields needed for tooling, including issue kind, source name,
  line, column, span, and message

This avoids committing the parser's internal Rust enum representation as the
external JSON contract while keeping output stable for callers.

Alternative considered: derive `serde::Serialize` on `Ast`, `Node`, and related
types and serialize them directly. That is faster to implement, but it exposes
Rust enum tagging choices as CLI output and makes later output cleanup a
breaking change.

### Shape Diagnostics Inside JSON Mode

In JSON mode, parser diagnostics should be represented inside the JSON document
rather than emitted as formatted diagnostic text. Error diagnostics go in an
`errors` array; warning diagnostics go in a `warnings` array. The command should
still return a non-zero exit status when any error diagnostic exists.

Diagnostics with severities outside error/warning should not block the initial
shape. They can be ignored by the JSON projection or added later under a
separate field if the parse command begins producing them.

Alternative considered: keep diagnostics on stderr in JSON mode. That would
make the AST JSON valid, but callers would still need to scrape human-readable
diagnostic text to understand parse failures.

### Shape JSON as a Single Result Object

JSON mode should emit a single valid JSON document. The top-level value should
contain an `inputs` array. Each input result should contain the input name, AST,
`errors`, and `warnings`. This keeps output valid for both single-input and
multi-input invocations while leaving space for future top-level metadata.

Alternative considered: emit newline-delimited JSON per input. That is useful
for streaming, but it is not a single JSON value and is less friendly for basic
callers expecting valid JSON from stdout.

## Risks / Trade-offs

- JSON output shape becomes a user-facing contract -> Keep the projection small,
  explicit, and covered by CLI tests.
- Diagnostic fields become part of the JSON contract -> Keep the diagnostic
  projection small and aligned with existing formatted diagnostic data.
- Projection drift from AST variants -> Add tests that cover representative node
  kinds and update the projection whenever AST variants are added.
- Pretty versus compact JSON ambiguity -> Use deterministic pretty JSON for
  readability unless implementation constraints favor compact output; tests
  should validate parseability and fields rather than whitespace.
