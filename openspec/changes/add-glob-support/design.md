## Context

`check` and `parse` both call the shared `read_template_inputs` helper in
`src/commands/mod.rs`. That helper currently treats every non-stdin operand as
an exact filesystem path, so a quoted argument like `**/*.mustache` fails unless
the caller's shell expands it before `smoothe` starts.

This makes command behavior depend on how the process was invoked. Interactive
shell users may get expanded operands, while quoted commands and direct
software invocations receive an unexpanded pattern.

## Goals / Non-Goals

**Goals:**

- Expand glob patterns passed as template input operands for both `check` and
  `parse`.
- Preserve existing behavior for ordinary file operands and `-` stdin.
- Make match ordering deterministic.
- Fail clearly when a glob pattern matches no files.

**Non-Goals:**

- Do not expand globs for option values such as `--config`, `--schema`, or
  `--lambdas`.
- Do not change config discovery or path resolution.
- Do not require callers to invoke a shell.
- Do not change how already-expanded shell operands are processed.

## Decisions

1. Implement expansion in the shared template input reader.

   `read_template_inputs` is already used by both commands, so adding expansion
   there gives `check` and `parse` identical behavior without duplicating CLI
   code.

   Alternative considered: expand in each command before calling the reader.
   That would work, but it increases the risk of `check` and `parse` drifting.

2. Detect glob operands before filesystem reads.

   Non-stdin operands containing glob metacharacters should be expanded before
   attempting `fs::read_to_string`. Operands without glob metacharacters remain
   literal paths, preserving existing file handling.

   Alternative considered: pass every operand through a glob engine. That could
   mis-handle literal filenames containing glob characters and would make error
   messages less direct for normal missing files.

3. Sort matched paths deterministically.

   Glob matches should be sorted before reading so output order is stable across
   platforms and filesystems.

   Alternative considered: rely on the glob library or filesystem traversal
   order. That is simpler, but it weakens repeatability.

4. Treat unmatched globs as input errors.

   If a caller provides a glob pattern that matches no files, the command should
   fail like a missing literal file. That prevents CI or scripts from passing
   accidentally when no templates were checked.

   Alternative considered: ignore unmatched globs. That is less noisy, but it
   hides misspelled paths and wrong working directories.

## Risks / Trade-offs

- [Risk] Adding glob support may introduce a new dependency.
  -> Mitigation: use a small, established crate and keep expansion isolated in
  the command input layer.
- [Risk] Very broad patterns may match many files.
  -> Mitigation: only expand explicit operands and keep processing streaming at
  the current per-input granularity.
- [Risk] Literal filenames with glob metacharacters could be ambiguous.
  -> Mitigation: only treat operands as globs when they contain recognized glob
  metacharacters; document that such literal filenames may need escaping if the
  chosen glob library supports it.
