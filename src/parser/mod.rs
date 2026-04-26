mod ast;
mod diagnostic;
mod input;
mod source;

use std::ops::Range;

pub use ast::{Ast, Delimiters, Node};
pub use diagnostic::{Diagnostic, DiagnosticSeverity, IssueKind, ParseEvent};
pub use input::{FeedbackHandlers, ParserInput, SourceMetadata};
pub use source::{SourceLocation, SourceSpan};

pub fn parse(input: ParserInput<'_>) -> ParseResult {
    Parser::new(input).parse()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseResult {
    pub ast: Ast,
    pub state: ParserState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParserState {
    pub diagnostics: Vec<Diagnostic>,
    pub delimiters: Delimiters,
    pub recovered: bool,
}

struct Parser<'a> {
    source_name: String,
    source: &'a str,
    feedback: FeedbackHandlers<'a>,
    state: ParserState,
}

impl<'a> Parser<'a> {
    fn new(input: ParserInput<'a>) -> Self {
        Self {
            source_name: input.source.name,
            source: input.source_text,
            feedback: input.feedback,
            state: ParserState {
                diagnostics: Vec::new(),
                delimiters: Delimiters::default(),
                recovered: false,
            },
        }
    }

    fn parse(mut self) -> ParseResult {
        let nodes = self.parse_nodes(None).nodes;

        ParseResult {
            ast: Ast { nodes },
            state: self.state,
        }
    }

    fn parse_nodes(&mut self, expected_closing: Option<OpenSection>) -> ParseNodesResult {
        let mut nodes = Vec::new();
        let mut cursor = expected_closing
            .as_ref()
            .map_or(0, |section| section.content_start);

        while cursor < self.source.len() {
            let Some(open_offset) = self.find_from(cursor, &self.state.delimiters.open) else {
                self.push_text(&mut nodes, cursor, self.source.len());
                break;
            };

            self.push_text(&mut nodes, cursor, open_offset);

            let tag = match self.parse_tag(open_offset) {
                Some(tag) => tag,
                None => {
                    self.emit(
                        DiagnosticSeverity::Error,
                        IssueKind::MalformedTag,
                        open_offset..self.source.len(),
                        "malformed Mustache tag",
                    );
                    break;
                }
            };

            cursor = tag.span.end;

            match tag.kind {
                TagKind::EscapedVariable(name) => {
                    nodes.push(Node::escaped_variable(name, tag.span))
                }
                TagKind::UnescapedVariable(name) => {
                    nodes.push(Node::unescaped_variable(name, tag.span));
                }
                TagKind::Comment(text) => nodes.push(Node::comment(text, tag.span)),
                TagKind::Partial(name) => nodes.push(Node::partial(name, tag.span)),
                TagKind::DelimiterChange(open, close) => {
                    self.state.delimiters = Delimiters::new(open.clone(), close.clone());
                    nodes.push(Node::delimiter_change(open, close, tag.span));
                }
                TagKind::Section { name, inverted } => {
                    let section = OpenSection {
                        name,
                        span_start: tag.span.start,
                        content_start: tag.span.end,
                        inverted,
                    };
                    let parsed = self.parse_nodes(Some(section.clone()));
                    cursor = parsed.cursor;
                    let span = section.span_start..cursor;
                    if section.inverted {
                        nodes.push(Node::inverted_section(section.name, span, parsed.nodes));
                    } else {
                        nodes.push(Node::section(section.name, span, parsed.nodes));
                    }
                }
                TagKind::Closing(name) => {
                    if let Some(section) = expected_closing {
                        if section.name == name {
                            return ParseNodesResult {
                                nodes,
                                cursor: tag.span.end,
                            };
                        }

                        self.emit(
                            DiagnosticSeverity::Error,
                            IssueKind::MismatchedClosingTag,
                            tag.span.clone(),
                            format!(
                                "closing tag `{name}` does not match open section `{}`",
                                section.name
                            ),
                        );
                        return ParseNodesResult {
                            nodes,
                            cursor: tag.span.end,
                        };
                    }

                    self.emit(
                        DiagnosticSeverity::Error,
                        IssueKind::UnmatchedClosingTag,
                        tag.span,
                        format!("closing tag `{name}` has no open section"),
                    );
                }
            }
        }

        if let Some(section) = expected_closing {
            self.emit(
                DiagnosticSeverity::Error,
                IssueKind::UnclosedSection,
                section.span_start..section.content_start,
                format!("section `{}` is not closed", section.name),
            );
            ParseNodesResult {
                nodes,
                cursor: self.source.len(),
            }
        } else {
            ParseNodesResult { nodes, cursor }
        }
    }

    fn parse_tag(&self, open_offset: usize) -> Option<Tag> {
        let delimiters = &self.state.delimiters;
        if delimiters.is_default()
            && self.source[open_offset..].starts_with("{{{")
            && let Some(relative_close) = self.source[open_offset + 3..].find("}}}")
        {
            let close_start = open_offset + 3 + relative_close;
            let name = self.source[open_offset + 3..close_start].trim().to_owned();
            return Some(Tag {
                kind: TagKind::UnescapedVariable(name),
                span: open_offset..close_start + 3,
            });
        }

        let content_start = open_offset + delimiters.open.len();
        let close_offset = self.find_from(content_start, &delimiters.close)?;
        let span = open_offset..close_offset + delimiters.close.len();
        let raw = self.source[content_start..close_offset].trim();
        if raw.is_empty() {
            return Some(Tag {
                kind: TagKind::EscapedVariable(String::new()),
                span,
            });
        }

        let kind = match raw.as_bytes()[0] {
            b'#' => TagKind::Section {
                name: raw[1..].trim().to_owned(),
                inverted: false,
            },
            b'^' => TagKind::Section {
                name: raw[1..].trim().to_owned(),
                inverted: true,
            },
            b'/' => TagKind::Closing(raw[1..].trim().to_owned()),
            b'!' => TagKind::Comment(raw[1..].trim().to_owned()),
            b'>' => TagKind::Partial(raw[1..].trim().to_owned()),
            b'&' => TagKind::UnescapedVariable(raw[1..].trim().to_owned()),
            b'=' if raw.ends_with('=') => {
                let body = raw[1..raw.len() - 1].trim();
                let mut parts = body.split_whitespace();
                let open = parts.next()?.to_owned();
                let close = parts.next()?.to_owned();
                if parts.next().is_some() {
                    return None;
                }
                TagKind::DelimiterChange(open, close)
            }
            _ => TagKind::EscapedVariable(raw.to_owned()),
        };

        Some(Tag { kind, span })
    }

    fn push_text(&self, nodes: &mut Vec<Node>, start: usize, end: usize) {
        if start < end {
            nodes.push(Node::text(&self.source[start..end], start..end));
        }
    }

    fn find_from(&self, start: usize, needle: &str) -> Option<usize> {
        self.source[start..]
            .find(needle)
            .map(|offset| start + offset)
    }

    fn emit(
        &mut self,
        severity: DiagnosticSeverity,
        issue: IssueKind,
        span: Range<usize>,
        message: impl Into<String>,
    ) {
        self.state.recovered = true;
        let diagnostic = Diagnostic {
            severity,
            issue,
            source_name: self.source_name.clone(),
            location: SourceLocation::for_offset(self.source, span.start),
            span: SourceSpan::new(span.start, span.end),
            message: message.into(),
        };
        let event = ParseEvent {
            diagnostic: diagnostic.clone(),
        };

        match diagnostic.severity {
            DiagnosticSeverity::Error => {
                if let Some(handler) = &self.feedback.on_error {
                    handler(&event);
                }
            }
            DiagnosticSeverity::Warning => {
                if let Some(handler) = &self.feedback.on_warning {
                    handler(&event);
                }
            }
            DiagnosticSeverity::Info => {
                if let Some(handler) = &self.feedback.on_info {
                    handler(&event);
                }
            }
            DiagnosticSeverity::Debug => {
                if let Some(handler) = &self.feedback.on_debug {
                    handler(&event);
                }
            }
        }

        self.state.diagnostics.push(diagnostic);
    }
}

#[derive(Debug, Clone)]
struct OpenSection {
    name: String,
    span_start: usize,
    content_start: usize,
    inverted: bool,
}

struct ParseNodesResult {
    nodes: Vec<Node>,
    cursor: usize,
}

struct Tag {
    kind: TagKind,
    span: Range<usize>,
}

enum TagKind {
    EscapedVariable(String),
    UnescapedVariable(String),
    Comment(String),
    Partial(String),
    DelimiterChange(String, String),
    Section { name: String, inverted: bool },
    Closing(String),
}
