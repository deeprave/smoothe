## ADDED Requirements

### Requirement: Check Output Format Selection

The system SHALL support selectable diagnostic output formats for the `check`
command.

#### Scenario: Default check output uses compiler-style format

- **WHEN** the user runs `smoothe check` without selecting an output format
- **THEN** the check command emits diagnostics in compiler-style text format.

#### Scenario: JSON check output is selectable

- **WHEN** the user runs `smoothe check --json <input>`
- **THEN** the check command emits a single valid JSON document for check
  results.

#### Scenario: Format selection is formatter-independent

- **WHEN** check diagnostics are produced
- **THEN** validation logic passes structured diagnostics to the selected
  formatter rather than formatting diagnostics at validation sites.

#### Scenario: Future formats can be added

- **WHEN** a new check output format is implemented later
- **THEN** it can follow the existing formatter pattern without changing
  semantic validation logic.

### Requirement: Compiler-Style Check Diagnostics

The system SHALL provide compiler-style diagnostic output for the `check`
command.

#### Scenario: Compiler-style diagnostic includes source location

- **WHEN** check emits a diagnostic in compiler-style output
- **THEN** the diagnostic line includes source file, line, column, severity,
  issue kind, and message.

#### Scenario: Compiler-style diagnostics distinguish severities

- **WHEN** check emits errors, warnings, info, or debug diagnostics in
  compiler-style output
- **THEN** each diagnostic includes its severity label.

#### Scenario: Compiler-style output uses stderr

- **WHEN** check emits compiler-style diagnostics
- **THEN** diagnostics are written to stderr.

### Requirement: JSON Check Diagnostics

The system SHALL provide JSON diagnostic output for the `check` command.

#### Scenario: JSON check output is parseable

- **WHEN** the check command runs in JSON output mode
- **THEN** stdout contains one valid JSON document.

#### Scenario: JSON check output contains input results

- **WHEN** the check command runs in JSON output mode for one or more inputs
- **THEN** the JSON document contains an `inputs` list with one result object
  per checked input.

#### Scenario: JSON check diagnostics are grouped by severity

- **WHEN** the check command runs in JSON output mode
- **THEN** diagnostics are grouped into `errors`, `warnings`, `info`, and
  `debug` lists.

#### Scenario: JSON diagnostic includes core fields

- **WHEN** a diagnostic appears in JSON check output
- **THEN** the diagnostic object includes issue kind, source name, line, column,
  span, severity, and message.

#### Scenario: JSON diagnostic includes structured details

- **WHEN** a diagnostic has structured diagnostic details
- **THEN** the JSON diagnostic object includes those details in optional
  structured fields.

#### Scenario: JSON check output uses stdout

- **WHEN** check emits JSON output
- **THEN** the JSON document is written to stdout.

### Requirement: Check Diagnostic Severity Filtering

The system SHALL support filtering displayed check diagnostics by minimum
diagnostic severity.

#### Scenario: Default diagnostic level shows warnings and errors

- **WHEN** the user runs `smoothe check` without a diagnostic level option
- **THEN** the check command displays errors and warnings.

#### Scenario: Info level includes info diagnostics

- **WHEN** the user selects diagnostic level `info`
- **THEN** the check command displays errors, warnings, and info diagnostics.

#### Scenario: Debug level includes debug diagnostics

- **WHEN** the user selects diagnostic level `debug`
- **THEN** the check command displays errors, warnings, info, and debug
  diagnostics.

#### Scenario: Error level hides warnings

- **WHEN** the user selects diagnostic level `error`
- **THEN** the check command displays error diagnostics and hides lower-severity
  diagnostics.

#### Scenario: Filtering does not change exit status

- **WHEN** an error diagnostic is produced but hidden by diagnostic filtering
- **THEN** the check command still exits unsuccessfully.

### Requirement: Check Output Source Accuracy

The system SHALL preserve source-accurate reporting in every check output
format.

#### Scenario: Main template diagnostic keeps main source

- **WHEN** a check diagnostic originates in a main template
- **THEN** compiler-style and JSON output report the main template source name,
  line, and column.

#### Scenario: Partial diagnostic keeps partial source

- **WHEN** a check diagnostic originates in a resolved partial
- **THEN** compiler-style and JSON output report the partial source name, line,
  and column.

### Requirement: Check Exit Status Is Format Independent

The system SHALL preserve check exit behavior independently of selected output
format.

#### Scenario: Errors fail in compiler-style output

- **WHEN** check produces one or more error diagnostics in compiler-style output
- **THEN** the command exits unsuccessfully.

#### Scenario: Errors fail in JSON output

- **WHEN** check produces one or more error diagnostics in JSON output
- **THEN** the command exits unsuccessfully.

#### Scenario: Warnings do not fail in any output format

- **WHEN** check produces warnings but no error diagnostics
- **THEN** the command exits successfully in every output format.
