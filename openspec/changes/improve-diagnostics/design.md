## Context

Diagnostics currently carry severity, issue kind, source name, location, span,
and a message string. That is enough to print basic failures, but the next
engine capabilities need more explanatory output. Full partial support adds
multi-file template graphs. The context schema model can identify expected
fields, optionality, enum values, scalar traversal, and known fields. The
lambda model can identify expected usage, actual usage, shape metadata, and
side-effect declarations.

This change should improve diagnostic quality without adding new validation
rules. The same underlying diagnostic data should support human-readable CLI
output and structured JSON output.

## Goals / Non-Goals

**Goals:**

- Report what was expected, what was found, and where the expectation came
  from.
- Preserve accurate file, line, column, and span information for main templates
  and partials.
- Add structured diagnostic detail data that can be rendered in text and JSON.
- Include schema context such as known fields, optionality, enum values, and
  path traversal context.
- Include lambda context such as expected usage, actual usage, argument shape,
  return shape, and side-effect metadata where relevant.
- Include partial context such as reference name, mapped path, resolved path,
  and referring source location where relevant.
- Provide near-hit suggestions for variable paths, schema fields, lambda names,
  and partial names when candidate sets are available.
- Preserve stable issue identifiers and severity decisions from the validation
  rules.

**Non-Goals:**

- Do not introduce new validation rules.
- Do not make warnings fail unless a dependent validation rule already defines
  an error.
- Do not require exact wording stability for human-readable messages.
- Do not implement editor protocol integration.
- Do not require full source excerpts or colored output in this change.

## Decisions

1. Add structured diagnostic details alongside the message string.

   Extend diagnostics with optional detail fields such as expected, found,
   source context, notes, suggestions, and related locations. The existing
   message remains the primary text summary, while the structured fields allow
   JSON output and future tools to consume diagnostics without parsing prose.

   Alternative considered: only improve message strings. That is quick but
   forces downstream tools to scrape human text and makes consistency harder.

2. Preserve stable issue identifiers.

   Issue kind identifiers should remain stable and continue using explicit
   string representations. Improved diagnostics can add detail fields without
   changing existing issue names unless a dependent change explicitly adds a new
   issue kind.

   Alternative considered: encode finer distinctions by renaming issue kinds.
   That would make the API brittle and break existing JSON consumers.

3. Render text output from structured details.

   CLI diagnostic formatting should build the human message from the summary
   plus details. For example, a missing schema path can include the missing
   path, current object scope, known fields, and near-hit suggestions. The
   diagnostic should still be readable as one or a small number of lines.

   Alternative considered: duplicate text-specific formatting at every
   diagnostic construction site. That would make diagnostics inconsistent.

4. Use bounded near-hit suggestions.

   Suggestions should be generated only when a candidate set is available, such
   as known object fields, known lambda names, or known partial names. Use a
   deterministic string-distance algorithm and return a small bounded list.

   Alternative considered: always try to suggest from every known name in the
   program. That can be noisy and expensive, and it may suggest unrelated names.

5. Treat source metadata as part of diagnostic correctness.

   Diagnostics from partial parsing and semantic validation must refer to the
   source unit that caused the issue, not merely the root template. Diagnostic
   details may also include related locations such as the referring partial tag
   and the resolved partial file.

   Alternative considered: keep root-template locations for all diagnostics.
   That is simpler but makes partial-heavy templates difficult to debug.

6. Keep validation ownership separate from diagnostic enrichment.

   Schema, lambda, and partial validators should decide whether a diagnostic is
   emitted and its severity. Diagnostic enrichment should attach context and
   suggestions to that diagnostic without creating extra validation outcomes.

   Alternative considered: add diagnostics in a post-processing pass. That pass
   would lack local validator context such as current schema scope and expected
   usage.

7. Include improved diagnostics in JSON output.

   JSON diagnostics should continue to include issue, source, line, column,
   span, and message. Add optional structured fields for expected, found, notes,
   suggestions, and related locations so machine consumers can use the richer
   data.

   Alternative considered: keep JSON diagnostics unchanged. That would make
   CLI output better but limit the usefulness of the tool in CI and editors.

## Risks / Trade-offs

- Message text tests can become brittle. Mitigation: assert stable issue kinds
  and structured fields, and keep text assertions focused on important phrases.
- Suggestions can be noisy. Mitigation: generate suggestions only from local
  candidate sets and cap the number of near hits.
- Adding diagnostic detail fields can touch many call sites. Mitigation: keep
  helper constructors and default empty details so simple parser diagnostics
  remain easy to create.
- Multi-file source reporting can be subtle. Mitigation: rely on source
  metadata from the partial graph and add tests for main-template and partial
  diagnostics.
- JSON shape changes can affect consumers. Mitigation: add optional fields
  while preserving existing required diagnostic fields.

## Migration Plan

- Add optional structured detail fields to the diagnostic model.
- Add helper constructors/builders for common diagnostics.
- Add a small near-hit suggestion utility for local candidate sets.
- Update schema diagnostics to include expected/found/source context, known
  fields, enum values, optionality, and suggestions.
- Update lambda diagnostics to include expected usage, actual usage, shape
  context, side-effect metadata where relevant, and suggestions.
- Update partial diagnostics to include reference names, candidate partial
  names, mapped/resolved paths, and related source locations where available.
- Update text formatting to render structured details consistently.
- Update JSON diagnostic projection to include optional structured detail
  fields.
- Add tests for source locations, structured details, suggestions, and stable
  issue identifiers.

## Open Questions

- Should related locations be included in text output immediately, or only in
  JSON for now?
- What maximum number of suggestions should be displayed by default?
- Should diagnostic detail keys be stable API now, or explicitly marked as
  evolving until the machine-readable `check` output is added?
