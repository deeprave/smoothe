use std::ops::Range;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ast {
    pub nodes: Vec<Node>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node {
    Text {
        text: String,
        span: Range<usize>,
    },
    EscapedVariable {
        name: String,
        span: Range<usize>,
    },
    UnescapedVariable {
        name: String,
        span: Range<usize>,
    },
    Comment {
        text: String,
        span: Range<usize>,
    },
    Section {
        name: String,
        span: Range<usize>,
        children: Vec<Node>,
    },
    InvertedSection {
        name: String,
        span: Range<usize>,
        children: Vec<Node>,
    },
    Partial {
        name: String,
        span: Range<usize>,
    },
    DelimiterChange {
        open: String,
        close: String,
        span: Range<usize>,
    },
}

impl Node {
    pub fn text(text: impl Into<String>, span: Range<usize>) -> Self {
        Self::Text {
            text: text.into(),
            span,
        }
    }

    pub fn escaped_variable(name: impl Into<String>, span: Range<usize>) -> Self {
        Self::EscapedVariable {
            name: name.into(),
            span,
        }
    }

    pub fn unescaped_variable(name: impl Into<String>, span: Range<usize>) -> Self {
        Self::UnescapedVariable {
            name: name.into(),
            span,
        }
    }

    pub fn comment(text: impl Into<String>, span: Range<usize>) -> Self {
        Self::Comment {
            text: text.into(),
            span,
        }
    }

    pub fn section(name: impl Into<String>, span: Range<usize>, children: Vec<Node>) -> Self {
        Self::Section {
            name: name.into(),
            span,
            children,
        }
    }

    pub fn inverted_section(
        name: impl Into<String>,
        span: Range<usize>,
        children: Vec<Node>,
    ) -> Self {
        Self::InvertedSection {
            name: name.into(),
            span,
            children,
        }
    }

    pub fn partial(name: impl Into<String>, span: Range<usize>) -> Self {
        Self::Partial {
            name: name.into(),
            span,
        }
    }

    pub fn delimiter_change(
        open: impl Into<String>,
        close: impl Into<String>,
        span: Range<usize>,
    ) -> Self {
        Self::DelimiterChange {
            open: open.into(),
            close: close.into(),
            span,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Delimiters {
    pub open: String,
    pub close: String,
}

impl Delimiters {
    pub fn new(open: impl Into<String>, close: impl Into<String>) -> Self {
        Self {
            open: open.into(),
            close: close.into(),
        }
    }

    pub fn is_default(&self) -> bool {
        self.open == "{{" && self.close == "}}"
    }
}

impl Default for Delimiters {
    fn default() -> Self {
        Self::new("{{", "}}")
    }
}
