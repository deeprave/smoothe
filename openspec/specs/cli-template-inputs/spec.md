# cli-template-inputs Specification

## Purpose
Define how commands accept template input operands from files and stdin, and how
parse output can be redirected.

## Requirements
### Requirement: Command Help Descriptions
The system SHALL present concise and consistent help descriptions for template
CLI commands.

#### Scenario: Help lists check command purpose
- **WHEN** a user runs `smoothe --help`
- **THEN** the command list describes `check` as checking template syntax and
  correctness.

#### Scenario: Help lists parse command purpose
- **WHEN** a user runs `smoothe --help`
- **THEN** the command list describes `parse` as parsing templates and printing
  AST output.

### Requirement: Template Input Operands
The system SHALL allow `check` and `parse` to accept one or more template input
operands.

#### Scenario: Check accepts one file
- **WHEN** a user runs `smoothe check template.mustache`
- **THEN** the command accepts `template.mustache` as an input template.

#### Scenario: Check accepts multiple files
- **WHEN** a user runs `smoothe check one.mustache two.mustache`
- **THEN** the command accepts both files as input templates.

#### Scenario: Parse accepts one file
- **WHEN** a user runs `smoothe parse template.mustache`
- **THEN** the command accepts `template.mustache` as an input template.

#### Scenario: Parse accepts multiple files
- **WHEN** a user runs `smoothe parse one.mustache two.mustache`
- **THEN** the command accepts both files as input templates.

### Requirement: Stdin Input Operand
The system SHALL treat `-` as stdin for template input operands.

#### Scenario: Check reads stdin operand
- **WHEN** a user pipes template content into `smoothe check -`
- **THEN** the command reads the template content from stdin.

#### Scenario: Parse reads stdin operand
- **WHEN** a user pipes template content into `smoothe parse -`
- **THEN** the command reads the template content from stdin.

#### Scenario: Stdin can be mixed with file operands
- **WHEN** a user provides file operands and `-` in the same command
- **THEN** the command processes each operand in the order provided.

### Requirement: Parse Output File
The system SHALL allow `parse` output to be written to a caller-provided file.

#### Scenario: Parse writes output file
- **WHEN** a user runs `smoothe parse --out parse.txt template.mustache`
- **THEN** diagnostics and AST output are written to `parse.txt`.

#### Scenario: Parse output file suppresses standard parse output
- **WHEN** a user provides `parse --out <path>`
- **THEN** diagnostics and AST output that would normally go to stdout or stderr
  are written to the output file instead.

### Requirement: Compact AST Output
The system SHALL print parse AST output in a compact developer-readable format.

#### Scenario: Parse prints compact AST to stdout
- **WHEN** a user runs `smoothe parse template.mustache` without `--out`
- **THEN** the AST output is printed to stdout using a compact format.

#### Scenario: Compact AST output remains inspectable
- **WHEN** the compact AST output is printed
- **THEN** it preserves node kinds, names, text, spans, and child structure.

### Requirement: Input Read Errors
The system SHALL report input read failures and exit unsuccessfully.

#### Scenario: Missing file reports error
- **WHEN** a user runs `smoothe parse missing.mustache`
- **THEN** the command reports that the input file could not be read and exits
  unsuccessfully.

#### Scenario: Check missing file reports error
- **WHEN** a user runs `smoothe check missing.mustache`
- **THEN** the command reports that the input file could not be read and exits
  unsuccessfully.
