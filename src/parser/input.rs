use std::path::PathBuf;

pub struct ParserInput<'a> {
    pub source: SourceMetadata,
    pub source_text: &'a str,
    pub feedback: FeedbackHandlers<'a>,
    pub partials: Vec<PartialMapping>,
    pub lambdas: Vec<LambdaSpec>,
    pub context_schema: Option<serde_json::Value>,
    pub frontmatter: FrontmatterOptions,
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
            frontmatter: FrontmatterOptions::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceMetadata {
    pub name: String,
    pub root: Option<PathBuf>,
}

impl SourceMetadata {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            root: None,
        }
    }

    pub fn with_root(mut self, root: impl Into<PathBuf>) -> Self {
        self.root = Some(root.into());
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LambdaSpec {
    pub name: String,
}

impl LambdaSpec {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FrontmatterOptions {
    pub enabled: bool,
}

impl FrontmatterOptions {
    pub fn disabled() -> Self {
        Self { enabled: false }
    }
}

impl Default for FrontmatterOptions {
    fn default() -> Self {
        Self { enabled: true }
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
