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
    OptionalSchemaPath,
    InvalidSchemaTraversal,
    UnexpectedSchemaType,
    SchemaInputError,
    LambdaInputError,
    InvalidLambdaUsage,
    LambdaTypeMismatch,
    FrontmatterParseError,
    UnsupportedIncludes,
    MalformedInheritance,
    MalformedDynamicName,
}

impl IssueKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::MalformedTag => "MalformedTag",
            Self::MismatchedClosingTag => "MismatchedClosingTag",
            Self::UnclosedSection => "UnclosedSection",
            Self::UnmatchedClosingTag => "UnmatchedClosingTag",
            Self::UnresolvedPartial => "UnresolvedPartial",
            Self::MissingSchemaPath => "MissingSchemaPath",
            Self::OptionalSchemaPath => "OptionalSchemaPath",
            Self::InvalidSchemaTraversal => "InvalidSchemaTraversal",
            Self::UnexpectedSchemaType => "UnexpectedSchemaType",
            Self::SchemaInputError => "SchemaInputError",
            Self::LambdaInputError => "LambdaInputError",
            Self::InvalidLambdaUsage => "InvalidLambdaUsage",
            Self::LambdaTypeMismatch => "LambdaTypeMismatch",
            Self::FrontmatterParseError => "FrontmatterParseError",
            Self::UnsupportedIncludes => "UnsupportedIncludes",
            Self::MalformedInheritance => "MalformedInheritance",
            Self::MalformedDynamicName => "MalformedDynamicName",
        }
    }
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
