## Context

The `parse` command already has a JSON output mode, but the `check` command
prints diagnostics directly through a single text formatter. As semantic
validation grows, `check` needs output that can be consumed by CI systems,
editor tooling, and scripts. It also needs a formatter boundary so compiler
style, JSON, and future formats can be selected without changing validation
logic.

Diagnostics already carry severity, source name, line, column, span, and
message, and the `improve-diagnostics` change adds richer structured detail
data. This change should project those diagnostics through selectable output
formats and severity filtering while preserving existing check exit behavior.

## Goals / Non-Goals

**Goals:**

- Add selectable output formats for the `check` command.
- Provide a compiler-style diagnostic output format suitable for common tool
  integrations.
- Provide an explicit JSON output format for structured consumers.
- Encapsulate check diagnostic formatting behind a common formatter pattern.
- Distinguish error, warning, info, and debug diagnostics in every format.
- Add CLI and configuration controls for output format and diagnostic
  verbosity.
- Preserve accurate file, line, and column reporting across main templates and
  resolved partials.
- Keep validation and exit-status behavior independent from output formatting.

**Non-Goals:**

- Do not add new semantic validation rules.
- Do not change parse command JSON behavior except where shared formatter code
  can be reused safely.
- Do not implement every possible diagnostic format in this change.
- Do not require colorized output.
- Do not make filtered-out errors stop affecting exit status.

## Decisions

1. Introduce a check diagnostic formatter abstraction.

   The check command should collect diagnostics from schema loading, lambda
   loading, content processing, parser output, and semantic validation, then
   pass them to a selected formatter. A formatter receives structured diagnostic
   data and emits output, but it does not decide validation behavior.

   Alternative considered: keep calling `eprintln!` at each diagnostic site.
   That prevents JSON output and makes additional formats expensive to add.

2. Use compiler-style text as the default machine-friendly format.

   Default check output should use a stable compiler-style line shape such as
   `file:line:column: severity: issue: message`. This is familiar to tools and
   users, preserves source locations, and remains readable in terminals.

   Alternative considered: make JSON the default. JSON is better for machines
   but worse for humans and would be a larger behavior change for existing
   users.

3. Add JSON as an explicit output option.

   `check --json` should emit a single valid JSON document. JSON should group
   diagnostics by severity and include source, line, column, span, issue,
   message, and any structured diagnostic details provided by the diagnostics
   model.

   Alternative considered: emit one JSON object per line. That is useful for
   streaming but less consistent with the existing parse JSON document.

4. Model output selection as an enum, not boolean flags internally.

   CLI may expose convenience flags such as `--json`, but resolved check
   options should use an output-format enum. That keeps room for future formats
   such as SARIF, GitHub annotations, or NDJSON.

   Alternative considered: use only `--json: bool`. That repeats the parse
   command pattern but makes adding a third format awkward.

5. Add severity filtering separately from exit behavior.

   A log-level or minimum-severity option should control which diagnostics are
   displayed. It must not change whether the check command exits with failure:
   all error diagnostics still count even if filtered out of display.

   Alternative considered: make filtering remove diagnostics before exit-code
   calculation. That would be surprising and dangerous in CI.

6. Allow configuration defaults with CLI override.

   `[check]` configuration should be able to set default output format and
   diagnostic level. CLI options override configuration values, matching the
   existing schema and lambda path precedence model.

   Alternative considered: support only CLI options. That makes repeated CI and
   project usage noisier.

7. Keep stdout/stderr behavior format-specific.

   Compiler-style diagnostics should continue to go to stderr. JSON output
   should go to stdout as a single document so it can be redirected or piped
   cleanly, while fatal command setup errors can still use stderr.

   Alternative considered: send all diagnostics to stderr even in JSON mode.
   That would make machine consumption awkward.

## Risks / Trade-offs

- JSON schema for check output can become a public contract. Mitigation:
  document required fields and keep new detail fields optional.
- Filtering diagnostics can hide important warnings. Mitigation: default to
  showing warnings and errors, and ensure errors still affect exit status.
- Formatter refactoring can accidentally alter validation flow. Mitigation:
  collect diagnostics first and test exit behavior separately from output.
- Multi-input JSON output needs a stable shape. Mitigation: follow parse JSON
  precedent with an `inputs` list and per-input diagnostic grouping.
- Compiler-style output may not satisfy every tool. Mitigation: design the
  formatter boundary so additional formats can be added later.

## Migration Plan

- Add check output format and diagnostic level option types.
- Add CLI options for selecting JSON output and diagnostic level.
- Add `[check]` configuration values for default output format and diagnostic
  level, with CLI override.
- Refactor check command diagnostic emission to collect diagnostics before
  formatting.
- Implement compiler-style check formatter as the default.
- Implement JSON check formatter as an explicit option.
- Apply severity filtering only in the formatter layer.
- Preserve check exit status calculation from the unfiltered diagnostic set.
- Add tests for output format selection, JSON shape, compiler-style output,
  severity filtering, config defaults, CLI override, and source locations across
  partials.

## Open Questions

- Should the CLI expose both `--json` and `--format json`, or only the more
  general `--format` option?
- What should the severity option be named: `--level`, `--log-level`, or
  `--diagnostic-level`?
- Should JSON output include all diagnostics plus a separate filtered view, or
  only diagnostics that pass the display filter?
