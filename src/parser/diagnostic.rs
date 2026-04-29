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
    PartialSkipped,
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
            Self::PartialSkipped => "PartialSkipped",
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
    pub details: DiagnosticDetails,
}

impl Diagnostic {
    pub fn new(
        severity: DiagnosticSeverity,
        issue: IssueKind,
        source_name: impl Into<String>,
        location: crate::parser::SourceLocation,
        span: crate::parser::SourceSpan,
        message: impl Into<String>,
    ) -> Self {
        Self {
            severity,
            issue,
            source_name: source_name.into(),
            location,
            span,
            message: message.into(),
            details: DiagnosticDetails::default(),
        }
    }

    pub fn with_expected(mut self, expected: impl Into<String>) -> Self {
        self.details.expected = Some(expected.into());
        self
    }

    pub fn with_found(mut self, found: impl Into<String>) -> Self {
        self.details.found = Some(found.into());
        self
    }

    pub fn with_expectation_source(mut self, source: impl Into<String>) -> Self {
        self.details.expectation_source = Some(source.into());
        self
    }

    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.details.notes.push(note.into());
        self
    }

    pub fn with_suggestion(mut self, suggestion: DiagnosticSuggestion) -> Self {
        self.details.suggestions.push(suggestion);
        self
    }

    pub fn with_suggestions(mut self, suggestions: Vec<DiagnosticSuggestion>) -> Self {
        self.details.suggestions.extend(suggestions);
        self
    }

    pub fn with_related_location(mut self, location: RelatedLocation) -> Self {
        self.details.related_locations.push(location);
        self
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DiagnosticDetails {
    pub expected: Option<String>,
    pub found: Option<String>,
    pub expectation_source: Option<String>,
    pub notes: Vec<String>,
    pub suggestions: Vec<DiagnosticSuggestion>,
    pub related_locations: Vec<RelatedLocation>,
}

impl DiagnosticDetails {
    pub fn is_empty(&self) -> bool {
        self.expected.is_none()
            && self.found.is_none()
            && self.expectation_source.is_none()
            && self.notes.is_empty()
            && self.suggestions.is_empty()
            && self.related_locations.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagnosticSuggestion {
    pub kind: DiagnosticSuggestionKind,
    pub value: String,
}

impl DiagnosticSuggestion {
    pub fn new(kind: DiagnosticSuggestionKind, value: impl Into<String>) -> Self {
        Self {
            kind,
            value: value.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticSuggestionKind {
    SchemaField,
    SchemaValue,
    LambdaName,
    PartialName,
}

impl DiagnosticSuggestionKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SchemaField => "schema_field",
            Self::SchemaValue => "schema_value",
            Self::LambdaName => "lambda_name",
            Self::PartialName => "partial_name",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelatedLocation {
    pub source_name: String,
    pub location: crate::parser::SourceLocation,
    pub span: crate::parser::SourceSpan,
    pub message: String,
}

impl RelatedLocation {
    pub fn new(
        source_name: impl Into<String>,
        location: crate::parser::SourceLocation,
        span: crate::parser::SourceSpan,
        message: impl Into<String>,
    ) -> Self {
        Self {
            source_name: source_name.into(),
            location,
            span,
            message: message.into(),
        }
    }
}

pub fn near_hit_suggestions(
    target: &str,
    candidates: &[String],
    kind: DiagnosticSuggestionKind,
    limit: usize,
) -> Vec<DiagnosticSuggestion> {
    if limit == 0 || candidates.is_empty() {
        return Vec::new();
    }

    let mut ranked = candidates
        .iter()
        .filter(|candidate| !candidate.is_empty())
        .filter_map(|candidate| {
            let distance = levenshtein(target, candidate);
            let threshold = suggestion_threshold(target.len(), candidate.len());
            (distance <= threshold).then_some((distance, candidate))
        })
        .collect::<Vec<_>>();

    ranked.sort_by(|(left_distance, left), (right_distance, right)| {
        left_distance
            .cmp(right_distance)
            .then_with(|| left.cmp(right))
    });
    ranked.dedup_by(|(_, left), (_, right)| left == right);
    ranked
        .into_iter()
        .take(limit)
        .map(|(_, candidate)| DiagnosticSuggestion::new(kind, candidate.clone()))
        .collect()
}

fn suggestion_threshold(target_len: usize, candidate_len: usize) -> usize {
    let max_len = target_len.max(candidate_len);
    match max_len {
        0..=4 => 2,
        5..=8 => 3,
        _ => 4,
    }
}

fn levenshtein(left: &str, right: &str) -> usize {
    let right_chars = right.chars().collect::<Vec<_>>();
    let mut previous = (0..=right_chars.len()).collect::<Vec<_>>();
    let mut current = vec![0; right_chars.len() + 1];

    for (left_index, left_char) in left.chars().enumerate() {
        current[0] = left_index + 1;
        for (right_index, right_char) in right_chars.iter().enumerate() {
            let insertion = current[right_index] + 1;
            let deletion = previous[right_index + 1] + 1;
            let substitution = previous[right_index] + usize::from(left_char != *right_char);
            current[right_index + 1] = insertion.min(deletion).min(substitution);
        }
        std::mem::swap(&mut previous, &mut current);
    }

    previous[right_chars.len()]
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseEvent {
    pub diagnostic: Diagnostic,
}
