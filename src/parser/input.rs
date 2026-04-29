use std::path::PathBuf;

pub use crate::lambda::LambdaSpec;

pub struct ParserInput<'a> {
    pub source: SourceMetadata,
    pub source_text: &'a str,
    pub feedback: FeedbackHandlers<'a>,
    pub partials: Vec<PartialMapping>,
    pub lambdas: Vec<LambdaSpec>,
    pub context_schema: Option<serde_json::Value>,
}

impl<'a> ParserInput<'a> {
    pub fn new(source: SourceMetadata, source_text: &'a str) -> Self {
        Self {
            source,
            source_text,
            feedback: FeedbackHandlers::default(),
            partials: Vec::new(),
            lambdas: Vec::new(),
            context_schema: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceMetadata {
    pub name: String,
    pub root: Option<PathBuf>,
    pub body_offset: usize,
    pub body_start_line: usize,
}

impl SourceMetadata {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            root: None,
            body_offset: 0,
            body_start_line: 1,
        }
    }

    pub fn with_root(mut self, root: impl Into<PathBuf>) -> Self {
        self.root = Some(root.into());
        self
    }

    pub fn with_body_start(mut self, offset: usize, line: usize) -> Self {
        self.body_offset = offset;
        self.body_start_line = line;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartialMapping {
    pub name: String,
    pub path: PathBuf,
}

impl PartialMapping {
    pub fn new(name: impl Into<String>, path: impl Into<PathBuf>) -> Self {
        Self {
            name: name.into(),
            path: path.into(),
        }
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
