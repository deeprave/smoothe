## ADDED Requirements

### Requirement: Check Output Format Selection

The system SHALL support selectable check output listeners for the `check`
command.

#### Scenario: Default check output uses compiler-style format

- **WHEN** the user runs `smoothe check` without selecting an output format
- **THEN** the check command emits diagnostics in compiler-style text format.

#### Scenario: JSON check output is selectable

- **WHEN** the user runs `smoothe check --json <input>`
- **THEN** the check command emits a single valid JSON document for check
  results.

#### Scenario: Format selection is listener-independent

- **WHEN** check events are produced
- **THEN** validation logic publishes structured events to selected listeners
  rather than formatting diagnostics at validation sites.

#### Scenario: Future formats can be added

- **WHEN** a new check output format is implemented later
- **THEN** it can follow the existing listener pattern without changing
  semantic validation logic.

### Requirement: Check Event Stream

The system SHALL emit structured check events as check execution progresses.

#### Scenario: Run lifecycle events are emitted

- **WHEN** a check run starts and finishes
- **THEN** the event stream includes run-started and run-finished events.

#### Scenario: Input lifecycle events are emitted

- **WHEN** the check command processes an input
- **THEN** the event stream includes input-started and input-finished events for
  that input.

#### Scenario: Diagnostic events are emitted incrementally

- **WHEN** schema loading, lambda loading, content processing, parser output, or
  semantic validation produces a diagnostic
- **THEN** the check command emits a diagnostic event without requiring the
  entire run to finish first.

#### Scenario: Partial lifecycle events are emitted

- **WHEN** check enters, finishes, or skips a resolved partial during template
  graph traversal
- **THEN** the event stream includes partial lifecycle events with the partial
  name, resolved path when known, and referring source context when known.

#### Scenario: Progress events are emitted

- **WHEN** check reaches meaningful progress points that are not diagnostics
- **THEN** the event stream can include informational progress events.

#### Scenario: Trace events are emitted for detailed progress

- **WHEN** detailed internal progress is useful for debugging or integrations
- **THEN** the event stream can include trace-level events that listeners may
  ignore unless trace verbosity is selected.

#### Scenario: Multiple listeners can observe one run

- **WHEN** more than one listener is attached to the check event stream
- **THEN** every listener receives the same check events in the same order.

### Requirement: Check Event Levels

The system SHALL assign every check event a display level.

#### Scenario: Diagnostic event level follows diagnostic severity

- **WHEN** a diagnostic event is emitted
- **THEN** its event level corresponds to the diagnostic severity.

#### Scenario: Lifecycle event levels are informational or lower

- **WHEN** run or input lifecycle events are emitted
- **THEN** they use an informational, debug, or trace level appropriate to their
  detail.

#### Scenario: Partial traversal event levels support detailed progress

- **WHEN** partial lifecycle events are emitted
- **THEN** they use debug or trace level unless promoted by a listener-specific
  output policy.

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

#### Scenario: Compiler-style output streams diagnostics

- **WHEN** diagnostic events are emitted during check execution
- **THEN** the compiler-style listener writes matching diagnostics as the events
  arrive.

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
- **AND** trace-level output is represented as `events` records rather than a
  diagnostic group, because diagnostics do not have a trace severity.

#### Scenario: JSON check output includes run-level diagnostics

- **WHEN** schema loading, lambda loading, or other run setup emits a diagnostic
  that is not tied to a specific template input
- **THEN** JSON output includes that diagnostic in top-level `errors`,
  `warnings`, `info`, or `debug` lists.

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

#### Scenario: JSON output may buffer events internally

- **WHEN** JSON check output consumes check events
- **THEN** it may buffer events as needed to emit one valid JSON document
  without changing validation behavior.

#### Scenario: JSON check output includes run-level events

- **WHEN** progress, trace, or lifecycle events pass the selected verbosity filter
  outside an active template input
- **THEN** JSON output includes those events in a top-level `events` list.

### Requirement: Check Event Verbosity Filtering

The system SHALL support listener-level filtering of displayed check events by
verbosity level.

#### Scenario: Default verbosity shows warnings and errors

- **WHEN** the user runs `smoothe check` without a verbosity option
- **THEN** output listeners display error and warning events.

#### Scenario: Info verbosity includes progress events

- **WHEN** the user selects verbosity `info`
- **THEN** output listeners display errors, warnings, and informational events.

#### Scenario: Debug verbosity includes detailed lifecycle events

- **WHEN** the user selects verbosity `debug`
- **THEN** output listeners display errors, warnings, info, and debug events.

#### Scenario: Trace verbosity includes trace events

- **WHEN** the user selects verbosity `trace`
- **THEN** output listeners display errors, warnings, info, debug, and trace
  events.

#### Scenario: Error verbosity hides warnings

- **WHEN** the user selects verbosity `error`
- **THEN** output listeners display error events and hide lower-severity events.

#### Scenario: Filtering does not change exit status

- **WHEN** diagnostic display is filtered by verbosity
- **THEN** the check command still computes exit status from all emitted
  diagnostics, not only displayed events.

#### Scenario: Filtering does not suppress events globally

- **WHEN** an event does not pass one listener's verbosity filter
- **THEN** that filtering does not prevent other listeners from receiving the
  event.

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
