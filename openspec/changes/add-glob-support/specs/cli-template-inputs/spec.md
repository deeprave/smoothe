## ADDED Requirements

### Requirement: Glob Template Input Operands
The system SHALL expand glob patterns supplied as template input operands for
`check` and `parse`.

#### Scenario: Check expands quoted glob operand
- **WHEN** the user runs `smoothe check '**/*.mustache'`
- **THEN** the command expands the glob pattern and checks each matched template
  file.

#### Scenario: Parse expands quoted glob operand
- **WHEN** the user runs `smoothe parse '**/*.mustache'`
- **THEN** the command expands the glob pattern and parses each matched template
  file.

#### Scenario: Direct process invocation expands glob operand
- **WHEN** software invokes `smoothe` directly with a template input operand of
  `**/*.mustache`
- **THEN** the command expands that operand without relying on shell expansion.

#### Scenario: Shell-expanded operands remain valid
- **WHEN** the user runs `smoothe check one.mustache two.mustache`
- **THEN** the command accepts both operands as ordinary template file inputs.

#### Scenario: Glob matches are deterministic
- **WHEN** a glob input operand matches multiple template files
- **THEN** the command processes the matched files in deterministic sorted
  order.

#### Scenario: Unmatched glob reports input error
- **WHEN** the user runs `smoothe check 'missing/**/*.mustache'`
- **AND** the glob pattern matches no files
- **THEN** the command reports an input error and exits unsuccessfully.

#### Scenario: Stdin operand is preserved
- **WHEN** the user runs `smoothe check -`
- **THEN** the command reads standard input rather than attempting glob
  expansion.
