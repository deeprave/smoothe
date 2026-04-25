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
- Provide short aliases `-h` for help, `-V` for version, and `-c` for color.
- Use `clap::ColorChoice` with `Auto` as the default, including `NOCOLOR`
  environment support and `--no-color` as a negating flag.
- Dispatch the parsed `check` command to a dedicated `check` function.
- Require `smoothe check` explicitly; do not make `check` the default command.
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

### Model color with `clap::ColorChoice`

Expose a single parsed color setting while accepting both `--color` and
`--colour` on the command line. The implementation should use one canonical
field name internally, configure the alternate spelling as an alias, and use
`clap::ColorChoice` with `Auto` as the default. The parser should also support
`-c` as the short form, `--no-color` as a negating flag, and the `NOCOLOR`
environment variable for color-disabled behavior.

Alternative considered: define two separate fields and reconcile them after
parsing. That adds conflict behavior and validation code with no meaningful
benefit for this initial surface.

Alternative considered: define a project-specific color enum. `ColorChoice`
already expresses the standard clap behavior this CLI needs, so a custom enum
would add translation code without clarifying the public interface.

### Keep dispatch explicit

After parsing, the entry point should call a dispatcher that matches on the
subcommand enum and invokes the relevant command handler. For this change,
`check` should call a `check` function that returns success without performing
real checking. There should be no default command; invoking the checker requires
the explicit `smoothe check` subcommand.

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

- [Risk] `--color`, `--colour`, `-c`, `--no-color`, and `NOCOLOR` can create
  precedence ambiguity. -> Mitigation: keep one canonical parsed color field and
  rely on `clap::ColorChoice` conventions, with explicit parser tests for the
  supported invocation forms.
- [Risk] The stubbed `check` command may look complete even though it performs no
  validation. -> Mitigation: keep the handler behavior minimal and cover only
  parser/dispatch expectations in this change.
- [Risk] Module boundaries could be overbuilt for one command. -> Mitigation:
  keep modules small and limited to parser types, dispatch, and command handler
  stubs.
