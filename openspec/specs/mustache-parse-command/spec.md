# mustache-parse-command Specification

## Purpose
Define the `parse` command behavior for reading template input, printing AST
output, reporting diagnostics, and returning exit status.

## Requirements
### Requirement: Parse Command Input
The system SHALL provide a production `parse` CLI command that reads the
complete template source from stdin.

#### Scenario: Parse command reads stdin
- **WHEN** a user pipes template content into `smoothe parse`
- **THEN** the command reads the complete stdin stream as the parser input.

#### Scenario: Parse command accepts empty stdin
- **WHEN** a user runs `smoothe parse` with empty stdin
- **THEN** the command parses an empty template input without requiring a file
  argument.

### Requirement: Parse Command AST Output
The system SHALL print the parsed AST in a developer-readable debug format.

#### Scenario: Valid template prints AST
- **WHEN** a user pipes a valid Mustache template into `smoothe parse`
- **THEN** the command prints the parsed AST to stdout.

#### Scenario: AST output is debug formatted
- **WHEN** the command prints AST output
- **THEN** the output uses debug formatting suitable for developer inspection.

### Requirement: Parse Command Diagnostics
The system SHALL print parser diagnostics produced while parsing stdin content.

#### Scenario: Invalid template prints diagnostics
- **WHEN** a user pipes a template with parser diagnostics into `smoothe parse`
- **THEN** the command prints those diagnostics.

#### Scenario: Diagnostics include source location
- **WHEN** the command prints a parser diagnostic
- **THEN** the diagnostic output includes severity, issue type, line, column,
  and message.

#### Scenario: Warning diagnostics are displayed
- **WHEN** parsing emits a warning diagnostic
- **THEN** the command prints the warning diagnostic.

### Requirement: Parse Command Exit Status
The system SHALL return an exit status based on parser error diagnostics.

#### Scenario: No error diagnostics exits successfully
- **WHEN** parsing completes with no error diagnostics
- **THEN** the command exits successfully.

#### Scenario: Error diagnostics exits unsuccessfully
- **WHEN** parsing emits one or more error diagnostics
- **THEN** the command exits unsuccessfully.

#### Scenario: Warning diagnostics do not fail command
- **WHEN** parsing emits warning diagnostics but no error diagnostics
- **THEN** the command exits successfully.
