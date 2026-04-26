pub struct ParserInput<'a> {
    pub source: SourceMetadata,
    pub source_text: &'a str,
    pub feedback: FeedbackHandlers<'a>,
}

impl<'a> ParserInput<'a> {
    pub fn new(source: SourceMetadata, source_text: &'a str) -> Self {
        Self {
            source,
            source_text,
            feedback: FeedbackHandlers::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceMetadata {
    pub name: String,
}

impl SourceMetadata {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

#[derive(Default)]
pub struct FeedbackHandlers<'a> {
    pub on_error: Option<FeedbackHandler<'a>>,
    pub on_warning: Option<FeedbackHandler<'a>>,
    pub on_info: Option<FeedbackHandler<'a>>,
    pub on_debug: Option<FeedbackHandler<'a>>,
}

pub type FeedbackHandler<'a> = Box<dyn Fn(&crate::parser::ParseEvent) + 'a>;
