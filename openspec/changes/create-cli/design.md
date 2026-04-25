## Context

`smoothe` is starting as a Rust command-line tool for Mustache-compatible
template checking and later rendering. The first CLI change needs to establish
the application entry point, global options, and command dispatch without
prematurely defining the full checker argument model.

The initial user-facing command surface is small: standard help/version/color
flags and a `check` command. The `check` command should route to a stub handler
so future checker work can fill in behavior behind a stable dispatch boundary.

## Goals / Non-Goals

**Goals:**

- Use `clap` as the parser for global options and subcommands.
- Provide global `--help`, `--version`, and color control flags.
- Accept both `--color` and `--colour` spellings for color behavior.
- Dispatch the parsed `check` command to a dedicated `check` function.
- Keep the `check` command argument surface intentionally minimal until its
  requirements are specified.
- Keep CLI parsing, command dispatch, and check command handling separated enough
  for later commands to be added without rewriting the entry point.

**Non-Goals:**

- Implement Mustache parsing or semantic checking.
- Define the final `check` command flags, file inputs, config discovery, or
  diagnostic output format.
- Add render-mode commands.
- Introduce shell completions, config files, or logging/tracing behavior.

## Decisions

### Use `clap` derive APIs for the initial parser

The CLI should define typed parser structs/enums with `clap` derive macros,
including a top-level parser type and a subcommand enum containing `Check`.
This keeps parsing declarative, gives help/version output for free, and creates
a typed boundary between raw CLI input and application dispatch.

Alternative considered: build commands manually with `clap::Command`. The manual
builder API is flexible, but the derive API is clearer for the small initial
surface and makes command handler inputs explicit.

### Model color as a global option with US and UK aliases

Expose a single parsed color setting while accepting both `--color` and
`--colour` on the command line. The implementation should use one canonical
field name internally and configure the alternate spelling as an alias, so the
rest of the code does not care which spelling the user typed.

Alternative considered: define two separate fields and reconcile them after
parsing. That adds conflict behavior and validation code with no meaningful
benefit for this initial surface.

### Keep dispatch explicit

After parsing, the entry point should call a dispatcher that matches on the
subcommand enum and invokes the relevant command handler. For this change,
`check` should call a `check` function that returns success without performing
real checking.

Alternative considered: put command behavior directly in `main`. That is
adequate for one command but would mix parsing, dispatch, and behavior at the
point where the project is expected to grow additional modes.

### Keep the `check` handler stub narrow

The `check` command should have a dedicated argument type even if it contains no
fields yet, and the handler should accept that type. This preserves a clear place
to add future arguments while avoiding speculative options.

Alternative considered: dispatch `check` without an argument type until options
exist. That would be simpler today but creates avoidable churn as soon as the
first check-specific argument is added.

## Risks / Trade-offs

- [Risk] The color option semantics may be under-specified at implementation
  time. -> Mitigation: choose `clap`'s conventional color value handling and
  defer output styling behavior until diagnostics exist.
- [Risk] The stubbed `check` command may look complete even though it performs no
  validation. -> Mitigation: keep the handler behavior minimal and cover only
  parser/dispatch expectations in this change.
- [Risk] Module boundaries could be overbuilt for one command. -> Mitigation:
  keep modules small and limited to parser types, dispatch, and command handler
  stubs.
