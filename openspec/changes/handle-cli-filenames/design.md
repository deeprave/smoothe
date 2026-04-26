## Context

`smoothe check` currently accepts no template inputs, and `smoothe parse`
currently reads only stdin and prints directly to stdout/stderr. Both commands
need a common CLI input convention so users can pass one or more template files
or use `-` for stdin. The parse command also needs an optional output file for
capturing diagnostics and AST output together.

## Goals / Non-Goals

**Goals:**

- Give `check` and `parse` concise, consistent command help descriptions.
- Let both commands accept one or more input operands.
- Treat `-` as stdin for either command.
- Keep stdin support compatible with shell pipelines.
- Add `parse --out <path>` to write diagnostics and AST output to one file.
- Make parse AST output more compact while remaining developer-readable.

**Non-Goals:**

- Add glob expansion beyond what the shell already provides.
- Add recursive directory traversal.
- Add parser configuration, schema, partial, or lambda CLI options.
- Stabilize parse AST output as a machine-readable API.
- Change the parser API.

## Decisions

### Use Positional Input Operands

Both commands should accept positional input operands, where each operand is
either a file path or `-` for stdin. This follows common CLI conventions and
keeps command usage direct.

Alternative considered: add explicit `--file` options. That would be more
verbose and less idiomatic for tools that operate on files.

### Preserve Stdin Through `-`

The `-` operand should read the complete stdin stream. This allows existing
pipeline workflows to continue while making file arguments the default path for
normal usage.

Alternative considered: keep implicit stdin when no inputs are provided. That
can be convenient, but it makes accidental blocking reads more likely and is
less explicit once file operands exist.

### Share Input Reading Behavior

The input-reading logic should be shared between `check` and `parse` so file
path handling, stdin handling, and read errors behave consistently.

Alternative considered: implement input reading separately in each command.
That would duplicate behavior and increase the chance that the two commands
drift.

### Route Parse Output Through One Writer

`parse --out <path>` should write both diagnostics and AST output to the
specified file. Without `--out`, diagnostics should continue to go to stderr and
AST output to stdout.

Alternative considered: split diagnostics and AST output into separate output
options. That adds complexity before there is a concrete need for separate
streams.

### Compact AST Debug Output

Parse AST output should move away from fully pretty-printed Rust debug output
toward a compact developer-readable representation. The output should keep each
node or object on a concise line where practical.

Alternative considered: emit JSON. That would be compact and structured, but it
would create an implied stable output format before the AST model is stable.

## Risks / Trade-offs

- [Risk] Multiple `-` operands would imply reading stdin multiple times. ->
  Mitigation: define stdin as a stream read when encountered; users should pass
  `-` once.
- [Risk] Compact AST output may still change as AST nodes evolve. ->
  Mitigation: keep the output documented as developer-readable inspection
  output, not a stable machine-readable contract.
- [Risk] `parse --out` combines diagnostics and AST output in one file. ->
  Mitigation: this matches the immediate need for a single inspection artifact;
  split routing can be added later if needed.
