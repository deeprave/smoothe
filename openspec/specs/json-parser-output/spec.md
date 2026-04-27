# json-parser-output Specification

## Purpose
Define JSON output mode for the parse command, including AST projection and
diagnostic grouping.

## Requirements
### Requirement: Parse Command JSON Flag
The system SHALL provide a JSON output mode for the `parse` command using both
the `--json` long flag and the `-j` short flag.

#### Scenario: Long JSON flag selects JSON output
- **WHEN** a user runs `smoothe parse --json <input>`
- **THEN** the parse command emits JSON output instead of compact tree output.

#### Scenario: Short JSON flag selects JSON output
- **WHEN** a user runs `smoothe parse -j <input>`
- **THEN** the parse command emits JSON output instead of compact tree output.

#### Scenario: Default output remains compact tree
- **WHEN** a user runs `smoothe parse <input>` without `--json` or `-j`
- **THEN** the parse command emits the existing compact tree output.

### Requirement: JSON Parse Result Document
The system SHALL emit a single valid JSON document for JSON parse output.

#### Scenario: JSON output is parseable
- **WHEN** the parse command runs in JSON mode
- **THEN** stdout or the configured output file contains one valid JSON
  document.

#### Scenario: JSON output contains input results
- **WHEN** the parse command runs in JSON mode for one or more inputs
- **THEN** the JSON document contains an `inputs` list with one result object
  per parsed input.

#### Scenario: Input result includes AST
- **WHEN** an input is parsed in JSON mode
- **THEN** that input result contains the input name and parsed AST nodes.

### Requirement: JSON AST Node Projection
The system SHALL represent AST nodes in JSON using explicit node kinds and
structured fields.

#### Scenario: Scalar node is represented
- **WHEN** the AST contains a scalar node such as text, variable, comment,
  partial, dynamic partial, or delimiter change
- **THEN** the JSON node includes a `kind`, source span, and the fields specific
  to that node kind.

#### Scenario: Container node is represented
- **WHEN** the AST contains a section, inverted section, lambda section, parent,
  or block node
- **THEN** the JSON node includes a `kind`, source span, node-specific name
  fields, and a `children` list.

#### Scenario: Empty AST is represented
- **WHEN** the parsed AST contains no nodes
- **THEN** the JSON result represents the AST as an empty node list.

### Requirement: JSON Diagnostics Projection
The system SHALL include parser diagnostics in JSON mode as structured lists
grouped by severity.

#### Scenario: Error diagnostics are grouped
- **WHEN** parsing produces one or more error diagnostics in JSON mode
- **THEN** the input result includes those diagnostics in an `errors` list.

#### Scenario: Warning diagnostics are grouped
- **WHEN** parsing produces one or more warning diagnostics in JSON mode
- **THEN** the input result includes those diagnostics in a `warnings` list.

#### Scenario: Diagnostics include location and message
- **WHEN** a diagnostic appears in the `errors` or `warnings` list
- **THEN** the diagnostic object includes the issue kind, source name, line,
  column, span, and message.

#### Scenario: No diagnostics uses empty lists
- **WHEN** parsing produces no error or warning diagnostics in JSON mode
- **THEN** the input result includes empty `errors` and `warnings` lists.

### Requirement: JSON Mode Exit Status
The system SHALL preserve existing parse command exit-status behavior in JSON
mode.

#### Scenario: Error diagnostic causes failure
- **WHEN** parsing in JSON mode produces at least one error diagnostic
- **THEN** the command exits with a non-zero status.

#### Scenario: Warnings do not cause failure
- **WHEN** parsing in JSON mode produces warnings but no error diagnostics
- **THEN** the command exits successfully.
