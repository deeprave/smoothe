## Context

The project already has Rust integration tests and module-oriented tests under
`tests/`. Those tests are valuable for focused development, but they are not a
complete behavioral conformance suite for the installed command-line utility.
As `smoothe` gains semantic checks, partial graph parsing, structured schema
and lambda models, richer diagnostics, and machine-readable output, the project
needs a black-box fixture suite that treats `smoothe` like an external program.

The fixture suite should exercise behavior through command-line invocation,
configuration files, templates, partials, schemas, lambdas, stdout, stderr, and
exit status. It should not import production Rust modules or rely on parser
internals. The first implementation should try `trycmd`, because it is designed
for many CLI fixture tests and is built on `snapbox`.

## Goals / Non-Goals

**Goals:**

- Add black-box CLI behavioral tests for `smoothe`.
- Start with `trycmd` as the runner.
- Keep fixtures input/output driven and independent from implementation
  internals.
- Define a repeatable fixture layout for command inputs, working directories,
  config files, templates, partials, schemas, lambdas, expected stdout,
  expected stderr, and expected exit status.
- Support incremental fixture growth from simple CLI checks to full end-to-end
  semantic validation.
- Normalize output where required, especially filesystem paths and line endings.
- Compare JSON output structurally where the runner supports it or where a thin
  harness layer can provide it.
- Allow intentional expected-output updates.

**Non-Goals:**

- Do not replace focused Rust unit or integration tests.
- Do not expose parser or checker internals to fixture cases.
- Do not require every existing test to be migrated into behavioral fixtures.
- Do not build a large custom runner before evaluating `trycmd`.
- Do not make fixture snapshots obscure the reason for failures.

## Decisions

1. Use `trycmd` as the first runner.

   `trycmd` is purpose-built for running many CLI cases and comparing command
   behavior. It also gives the project a low-cost way to start writing
   black-box fixtures. If `trycmd` cannot handle required normalization or
   structured JSON comparison, move to `snapbox` directly or add a thin custom
   harness around the fixture data.

   Alternative considered: build a custom runner immediately. That gives full
   control but delays feedback on whether existing Rust CLI tooling already
   covers the need.

2. Keep behavioral fixtures separate from current implementation tests.

   Use a dedicated fixture hierarchy, for example
   `tests/behavior/fixtures/**`, with a small Rust test entry point that
   invokes the runner. This keeps black-box conformance data separate from
   focused parser/content/config tests.

   Alternative considered: place cases in the existing `tests/fixtures`
   directory. That risks mixing internal fixtures with CLI behavioral fixtures.

3. Model each case as command plus filesystem inputs plus expected outputs.

   Each fixture should define the command invocation, working directory,
   optional environment, expected exit status, expected stdout, and expected
   stderr. Case directories may include `smoothe.toml`, templates, partials,
   schemas, lambda definitions, and expected output files.

   Alternative considered: store only command transcript files. That is
   compact, but it can make larger multi-file scenarios harder to read and
   maintain.

4. Normalize paths and line endings before comparison.

   Behavioral output should be stable across local machines and CI. Replace
   workspace-specific absolute paths with stable placeholders where needed and
   normalize line endings. Keep normalization explicit and minimal so it does
   not hide real output differences.

   Alternative considered: require exact raw output. That is simpler but brittle
   for path-heavy diagnostics.

5. Prefer structural JSON comparison for JSON output.

   JSON diagnostics should be compared as JSON values where supported, not raw
   text. This avoids false failures from object key order or whitespace. If
   `trycmd` cannot do this directly, evaluate a small wrapper or `snapbox`
   helper before writing a full custom runner.

   Alternative considered: snapshot JSON as raw text. That works but makes
   formatting details part of the behavior contract.

6. Allow intentional expected-output updates.

   The suite should support a deliberate workflow for updating snapshots or
   expected output files when behavior intentionally changes. This must be
   explicit so accidental output drift still fails tests.

   Alternative considered: regenerate expected output automatically. That would
   weaken the suite’s ability to catch regressions.

7. Build fixture coverage incrementally.

   Start with simple command behavior and diagnostic cases, then add more
   complex scenarios as the engine capabilities land: partials, schema checks,
   lambda checks, improved diagnostics, and machine-readable output.

   Alternative considered: wait until all engine features are complete before
   adding fixtures. That would lose regression coverage while the CLI behavior
   is actively changing.

## Risks / Trade-offs

- `trycmd` may not support the desired fixture layout or JSON structural
  comparison directly. Mitigation: start with `trycmd`, document gaps, and move
  selectively to `snapbox` or a thin custom runner if needed.
- Snapshot tests can become noisy when message text changes. Mitigation: use
  behavioral fixtures for stable user-facing output and keep lower-level tests
  for precise diagnostic data.
- Path normalization can hide real bugs. Mitigation: normalize only known
  machine-specific prefixes and preserve relative paths and line/column data.
- Large fixture trees can become hard to maintain. Mitigation: use clear naming,
  small case directories, and maintenance inventory expectations.
- Behavioral tests can be slower than unit tests. Mitigation: keep the suite
  focused and rely on `cargo nextest run` to manage integration test execution.

## Migration Plan

- Add `trycmd` as a dev dependency if it fits the first fixture cases.
- Add a behavioral test entry point under `tests/`.
- Add a dedicated behavioral fixture directory.
- Create a minimal passing case that runs `smoothe` as a black-box command.
- Add a failing diagnostic case covering stdout, stderr, and exit status.
- Add normalization for paths and line endings where needed.
- Add JSON-output fixture comparison for a simple case.
- Add progressively richer fixtures for config, templates, schemas, lambdas,
  partials, diagnostics, and machine-readable output as those features land.
- Document how to add a new fixture and intentionally update expected output.

## Open Questions

- Should behavioral fixtures live under `tests/behavior/fixtures` or another
  top-level directory such as `behavior/`?
- Can `trycmd` satisfy structural JSON comparison, or should JSON cases use a
  `snapbox` helper from the start?
- What command should developers use to intentionally refresh expected output?
