## Context

The `parse` command already has a JSON output mode, but the `check` command
prints diagnostics directly through a single text formatter. As semantic
validation grows, `check` needs output that can be consumed by CI systems,
editor tooling, and scripts. It also needs an event boundary so compiler-style,
JSON, IDE integrations, progress reporters, and future consumers can observe a
check run without changing validation logic.

Diagnostics already carry severity, source name, line, column, span, and
message, and the `improve-diagnostics` change adds richer structured detail
data. This change should emit diagnostics and lifecycle state as structured
check events, then project those events through selectable listeners while
preserving existing check exit behavior.

## Goals / Non-Goals

**Goals:**

- Add selectable output formats for the `check` command.
- Provide a compiler-style diagnostic output format suitable for common tool
  integrations.
- Provide an explicit JSON output format for structured consumers.
- Introduce a structured check event stream for diagnostics and run/input
  lifecycle events.
- Encapsulate output behavior behind event listeners.
- Allow multiple listeners to observe the same check run.
- Distinguish error, warning, info, debug, and trace event levels in every
  format that renders those event types.
- Add CLI and configuration controls for output format and diagnostic
  verbosity.
- Preserve accurate file, line, and column reporting across main templates and
  resolved partials.
- Keep validation, event emission, display filtering, and exit-status behavior
  independent from output formatting.

**Non-Goals:**

- Do not add new semantic validation rules.
- Do not change parse command JSON behavior except where shared output
  projection code can be reused safely.
- Do not implement every possible diagnostic format in this change.
- Do not require colorized output.
- Do not make filtered-out errors stop affecting exit status.
- Do not require JSON document output to stream incrementally; JSON may buffer
  internally to emit a valid document.

## Decisions

1. Introduce a check event stream and listener abstraction.

   The check command should emit structured events as work progresses, including
   run started, input started, diagnostic, input finished, and run finished
   events. Listeners receive events and may stream, buffer, count, display, or
   ignore them. Output formats are implemented as listeners, not as validation
   logic.

   Alternative considered: keep calling `eprintln!` at each diagnostic site.
   That prevents JSON output, makes additional formats expensive to add, and
   prevents attaching multiple consumers to the same check run.

2. Use compiler-style text as the default machine-friendly format.

   Default check output should use a stable compiler-style line shape such as
   `file:line:column: severity: issue: message`. This is familiar to tools and
   users, preserves source locations, remains readable in terminals, and can be
   streamed as diagnostic events arrive.

   Alternative considered: make JSON the default. JSON is better for machines
   but worse for humans and would be a larger behavior change for existing
   users.

3. Add JSON as an explicit output option.

   `check --json` should emit a single valid JSON document. JSON should consume
   the same check events as other listeners, buffer internally as needed, group
   diagnostics by severity, and include source, line, column, span, issue,
   message, lifecycle result state, and any structured diagnostic details
   provided by the diagnostics model.

   Alternative considered: emit one JSON object per line. That is useful for
   streaming and should remain a future listener option, but it is less
   consistent with the existing parse JSON document.

4. Model output selection as an enum, not boolean flags internally.

   CLI may expose convenience default flags such as `--json` and `--no-json`,
   but resolved check options should use an output-format enum. Explicit
   `--format` selections override those default flags. That keeps room for
   future formats such as SARIF, GitHub annotations, or NDJSON.

   Alternative considered: use only `--json: bool`. That repeats the parse
   command pattern but makes adding a third format awkward.

5. Add event verbosity filtering separately from exit behavior.

   A verbosity option should control which events are displayed by output
   listeners. It must not change whether the check command exits with failure:
   all error diagnostics still count even if filtered out of display. Event
   levels should include error, warning, info, debug, and trace so diagnostics,
   progress, partial traversal, and detailed trace information can share one
   filtering model.

   Alternative considered: make filtering remove diagnostics before exit-code
   calculation. That would be surprising and dangerous in CI.

6. Allow configuration defaults with CLI override.

   `[check]` configuration should be able to set default output format and
   verbosity. CLI options override configuration values, matching the existing
   schema and lambda path precedence model.

   Alternative considered: support only CLI options. That makes repeated CI and
   project usage noisier.

7. Keep stdout/stderr behavior format-specific.

   Compiler-style diagnostics should continue to go to stderr. JSON output
   should go to stdout as a single document so it can be redirected or piped
   cleanly, while fatal command setup errors can still use stderr.

   Alternative considered: send all diagnostics to stderr even in JSON mode.
   That would make machine consumption awkward.

8. Track command result state independently of listeners.

   The check runner should track whether any error diagnostic was emitted and
   use that state for exit status. This state can be maintained while events
   stream and does not require buffering all diagnostics.

   Alternative considered: derive exit status from listener output. That would
   couple command correctness to display filtering and listener behavior.

9. Include partial lifecycle, progress, and trace events.

   The event stream should include more than diagnostics. Run/input lifecycle
   events provide coarse progress, partial started/finished/skipped events
   expose template graph traversal, and trace events provide detailed internal
   progress for tools that need it. Output listeners may ignore event types
   they do not display.

   Alternative considered: emit only diagnostic events. That is simpler but
   loses much of the value of an event-based API for IDEs and interactive
   integrations.

## Risks / Trade-offs

- JSON schema for check output can become a public contract. Mitigation:
  document required fields and keep new detail fields optional.
- Filtering diagnostics can hide important warnings. Mitigation: default to
  showing warnings and errors, and ensure errors still affect exit status.
- Event refactoring can accidentally alter validation flow. Mitigation: keep
  event emission thin, track run state separately, and test exit behavior
  separately from output.
- Multi-input JSON output needs a stable shape. Mitigation: follow parse JSON
  precedent with an `inputs` list and per-input diagnostic grouping.
- Compiler-style output may not satisfy every tool. Mitigation: design the
  event/listener boundary so additional listeners can be added later.
- Listener failures introduce error-handling choices. Mitigation: output
  listener I/O failures should be reported as command setup/runtime failures,
  while validation diagnostics remain normal check events.

## Migration Plan

- Add check output format and verbosity option types.
- Add CLI options for selecting JSON output and verbosity.
- Add `[check]` configuration values for default output format and diagnostic
  verbosity, with CLI override.
- Define check event types for run lifecycle, input lifecycle, diagnostics, and
  final results.
- Add partial lifecycle, progress, and trace event types.
- Define an event listener interface and fan-out support for multiple listeners.
- Refactor check command diagnostic emission to publish check events instead of
  printing directly from validation flow.
- Implement compiler-style check output as the default listener.
- Implement JSON check output as an explicit listener that may buffer internally
  to emit one valid document.
- Apply verbosity filtering only in output listeners.
- Preserve check exit status calculation from runner state derived from all
  emitted diagnostics.
- Add tests for output format selection, JSON shape, compiler-style output,
  event emission, listener fan-out, verbosity filtering, config defaults, CLI
  override, and source locations across partials.

## Open Questions

- Should the CLI expose both `--json` and `--format json`, or only the more
  general `--format` option? Current direction: expose both. Treat `--json`
  and `--no-json` as convenience flags for changing the configured default,
  while `--format` remains the explicit output-format selector.
- Should the verbosity option be named `--level` or `--verbosity`? Current
  direction: prefer `--verbosity` because the event stream includes progress
  and trace events, not only diagnostics.
- Should JSON output include all diagnostics plus a separate filtered view, or
  only diagnostics that pass the display filter?
- Should this change include an NDJSON event listener now, or leave that as the
  first future consumer of the event stream?
- Should an early-termination or fail-fast policy be part of this change, or
  remain a follow-on once event streaming exists?
