#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    Info,
    Debug,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IssueKind {
    MalformedTag,
    MismatchedClosingTag,
    UnclosedSection,
    UnmatchedClosingTag,
    UnresolvedPartial,
    MissingSchemaPath,
    FrontmatterParseError,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub severity: DiagnosticSeverity,
    pub issue: IssueKind,
    pub source_name: String,
    pub location: crate::parser::SourceLocation,
    pub span: crate::parser::SourceSpan,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseEvent {
    pub diagnostic: Diagnostic,
}
