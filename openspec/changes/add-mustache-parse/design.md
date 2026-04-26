## Context

The parser API already exists and returns an AST plus parser state containing
diagnostics. The current CLI has a `check` command but no direct way to exercise
the parser from a shell pipeline. This change adds a narrow production CLI
command for parser inspection without turning it into a full checker or
renderer.

## Goals / Non-Goals

**Goals:**

- Add a production `parse` subcommand to the existing CLI.
- Read all template input from stdin.
- Parse stdin content through the existing parser API.
- Print diagnostics, including warnings, when present.
- Print the parsed AST in a developer-readable debug format.
- Return a non-zero exit status when parser error diagnostics are present.
- Keep the command simple enough for parser development and debugging.

**Non-Goals:**

- Read files directly from command arguments.
- Add options for partial mappings, context schemas, frontmatter control, or
  other parser inputs.
- Provide JSON or other stable machine-readable output.
- Render templates.
- Replace the future `check` workflow.

## Decisions

### Add a First-Class CLI Subcommand

The utility should be exposed as `smoothe parse` rather than as an example
binary. The user-visible need is to run the installed tool against stdin, so a
subcommand is the most direct interface.

Alternative considered: add `examples/parse.rs`. That would be useful for local
development, but it would not provide a normal CLI workflow and would be less
discoverable once `smoothe` is installed.

### Read From Stdin Only

The first version should read the complete template source from stdin. This
keeps the command focused and avoids introducing file path options, globbing, or
partial-root behavior before the checker command needs them.

Alternative considered: accept a file path argument. That would be convenient,
but it overlaps with future checker input handling and is unnecessary for the
basic parser smoke-test workflow.

### Use Debug Formatting for AST Output

The AST should be printed with Rust debug formatting in this slice. That keeps
the output accurate to the current internal model while avoiding a premature
stable output format.

Alternative considered: output JSON. That would be easier for tooling to
consume, but it would make the AST output a compatibility contract before the
parser model has stabilized.

### Return Failure Only For Error Diagnostics

The command should return a non-zero exit status when the parser emits at least
one error diagnostic. Warning diagnostics should be printed but should not fail
the command.

Alternative considered: return failure for any diagnostic. That would be too
strict for parser warnings such as schema or unresolved partial warnings, which
do not necessarily prevent AST inspection.

### Test Warning Display Through Minimal Parser Inputs

The command should display warnings if the parser emits them. This slice should
test that path only through warning-producing behavior available from stdin-only
default parser input. It should not add flags or configuration solely to force a
warning case.

Alternative considered: add a small schema or partial option to trigger warnings
from the CLI. That would broaden the command beyond the intended minimal parser
smoke-test workflow.

## Risks / Trade-offs

- [Risk] Debug AST output may change as parser internals evolve. -> Mitigation:
  document this command as a developer inspection tool, not a stable output API.
- [Risk] Stdin-only input is less convenient than file arguments. -> Mitigation:
  users can pipe or redirect files, and richer file input can be designed later
  with checker behavior.
- [Risk] Diagnostics formatting may not match future CLI UX. -> Mitigation:
  keep formatting simple and local to the parse command for now.
- [Risk] Warning-only CLI coverage may be limited by stdin-only inputs. ->
  Mitigation: cover warning formatting if available from default parser input,
  and leave richer warning scenarios to future parser-input CLI work.
