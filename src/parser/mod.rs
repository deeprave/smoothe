mod ast;
mod diagnostic;
mod input;
mod source;

use std::{collections::HashSet, fs, ops::Range, path::PathBuf};

pub use ast::{Ast, Delimiters, Node};
pub use diagnostic::{Diagnostic, DiagnosticSeverity, IssueKind, ParseEvent};
pub use input::{
    FeedbackHandlers, FrontmatterOptions, LambdaSpec, ParserInput, PartialMapping, SourceMetadata,
};
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
    pub partials: Vec<ParsedPartial>,
    pub nested_partials: Vec<PartialReference>,
    pub lambda_references: Vec<LambdaReference>,
    pub frontmatter: FrontmatterState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedPartial {
    pub name: String,
    pub path: PathBuf,
    pub ast: Ast,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartialReference {
    pub name: String,
    pub source_name: String,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LambdaReference {
    pub name: String,
    pub source_name: String,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrontmatterFormat {
    Yaml,
    Json,
    Toml,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrontmatterState {
    pub format: Option<FrontmatterFormat>,
    pub context: serde_json::Value,
}

struct Parser<'a> {
    source_name: String,
    source_root: Option<PathBuf>,
    source: &'a str,
    feedback: FeedbackHandlers<'a>,
    partials: Vec<PartialMapping>,
    lambdas: Vec<LambdaSpec>,
    context_schema: Option<serde_json::Value>,
    frontmatter: FrontmatterOptions,
    expand_partials: bool,
    state: ParserState,
}

impl<'a> Parser<'a> {
    fn new(input: ParserInput<'a>) -> Self {
        Self::new_with_partial_expansion(input, true)
    }

    fn new_with_partial_expansion(input: ParserInput<'a>, expand_partials: bool) -> Self {
        Self {
            source_name: input.source.name.clone(),
            source_root: input.source.root,
            source: input.source_text,
            feedback: input.feedback,
            partials: input.partials,
            lambdas: input.lambdas,
            context_schema: input.context_schema,
            frontmatter: input.frontmatter,
            expand_partials,
            state: ParserState {
                diagnostics: Vec::new(),
                delimiters: Delimiters::default(),
                recovered: false,
                partials: Vec::new(),
                nested_partials: Vec::new(),
                lambda_references: Vec::new(),
                frontmatter: FrontmatterState {
                    format: None,
                    context: serde_json::Value::Object(serde_json::Map::new()),
                },
            },
        }
    }

    fn parse(mut self) -> ParseResult {
        let content_start = self.parse_frontmatter();
        let nodes = self.parse_nodes(None, content_start).nodes;
        self.classify_lambdas(&nodes);
        self.check_schema_references(&nodes);
        if self.expand_partials {
            self.expand_partial_references(&nodes);
        } else {
            self.record_nested_partials(&nodes);
        }

        ParseResult {
            ast: Ast { nodes },
            state: self.state,
        }
    }

    fn parse_nodes(
        &mut self,
        expected_closing: Option<OpenSection>,
        start_cursor: usize,
    ) -> ParseNodesResult {
        let mut nodes = Vec::new();
        let mut cursor = expected_closing
            .as_ref()
            .map_or(start_cursor, |section| section.content_start);

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
                    let parsed = self.parse_nodes(Some(section.clone()), section.content_start);
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

    fn parse_frontmatter(&mut self) -> usize {
        if !self.frontmatter.enabled || !self.source.starts_with("---\n") {
            return 0;
        }

        let body_start = 4;
        let Some(relative_end) = self.source[body_start..].find("\n---\n") else {
            return 0;
        };
        let body_end = body_start + relative_end;
        let content_start = body_end + "\n---\n".len();
        let body = &self.source[body_start..body_end];
        if body.trim().is_empty() {
            return content_start;
        }

        match parse_frontmatter_value(body) {
            Ok((format, context)) => {
                self.state.frontmatter = FrontmatterState {
                    format: Some(format),
                    context,
                };
            }
            Err(message) => {
                self.emit(
                    DiagnosticSeverity::Warning,
                    IssueKind::FrontmatterParseError,
                    0..content_start,
                    message,
                );
            }
        }

        content_start
    }

    fn expand_partial_references(&mut self, nodes: &[Node]) {
        if self.partials.is_empty() {
            return;
        }

        for partial in collect_partial_nodes(nodes) {
            let Some(mapping) = self
                .partials
                .iter()
                .find(|mapping| mapping.name == partial.name)
                .cloned()
            else {
                self.emit(
                    DiagnosticSeverity::Warning,
                    IssueKind::UnresolvedPartial,
                    partial.span.start..partial.span.end,
                    format!("unresolved partial `{}`", partial.name),
                );
                continue;
            };

            let path = self.resolve_partial_path(&mapping.path);
            match fs::read_to_string(&path) {
                Ok(source) => {
                    let mut input =
                        ParserInput::new(SourceMetadata::new(path.to_string_lossy()), &source);
                    input.partials = self.partials.clone();
                    input.lambdas = self.lambdas.clone();
                    input.context_schema = self.context_schema.clone();
                    input.frontmatter = self.frontmatter;
                    let parsed = Parser::new_with_partial_expansion(input, false).parse();
                    self.state.diagnostics.extend(parsed.state.diagnostics);
                    self.state
                        .nested_partials
                        .extend(parsed.state.nested_partials);
                    self.state
                        .lambda_references
                        .extend(parsed.state.lambda_references);
                    self.state.partials.push(ParsedPartial {
                        name: mapping.name,
                        path,
                        ast: parsed.ast,
                    });
                }
                Err(error) => {
                    self.emit(
                        DiagnosticSeverity::Warning,
                        IssueKind::UnresolvedPartial,
                        partial.span.start..partial.span.end,
                        format!("unresolved partial `{}`: {error}", partial.name),
                    );
                }
            }
        }
    }

    fn record_nested_partials(&mut self, nodes: &[Node]) {
        self.state
            .nested_partials
            .extend(
                collect_partial_nodes(nodes)
                    .into_iter()
                    .map(|partial| PartialReference {
                        name: partial.name,
                        source_name: self.source_name.clone(),
                        span: SourceSpan::new(partial.span.start, partial.span.end),
                    }),
            );
    }

    fn classify_lambdas(&mut self, nodes: &[Node]) {
        if self.lambdas.is_empty() {
            return;
        }

        let lambda_names = self
            .lambdas
            .iter()
            .map(|lambda| lambda.name.clone())
            .collect::<HashSet<_>>();
        self.state
            .lambda_references
            .extend(
                collect_reference_nodes(nodes)
                    .into_iter()
                    .filter_map(|reference| {
                        lambda_names
                            .contains(&reference.name)
                            .then(|| LambdaReference {
                                name: reference.name,
                                source_name: self.source_name.clone(),
                                span: SourceSpan::new(reference.span.start, reference.span.end),
                            })
                    }),
            );
    }

    fn check_schema_references(&mut self, nodes: &[Node]) {
        let Some(schema) = self.context_schema.clone() else {
            return;
        };

        let lambda_names = self
            .lambdas
            .iter()
            .map(|lambda| lambda.name.clone())
            .collect::<HashSet<_>>();

        for reference in collect_reference_nodes(nodes) {
            if lambda_names.contains(&reference.name) {
                continue;
            }
            if !schema_defines_path(&schema, &reference.name) {
                self.emit(
                    DiagnosticSeverity::Warning,
                    IssueKind::MissingSchemaPath,
                    reference.span.start..reference.span.end,
                    format!("missing schema path `{}`", reference.name),
                );
            }
        }
    }

    fn resolve_partial_path(&self, path: &PathBuf) -> PathBuf {
        if path.is_absolute() {
            return path.clone();
        }

        self.source_root
            .as_ref()
            .map_or_else(|| path.clone(), |root| root.join(path))
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

#[derive(Clone)]
struct NamedSpan {
    name: String,
    span: Range<usize>,
}

fn collect_partial_nodes(nodes: &[Node]) -> Vec<NamedSpan> {
    let mut partials = Vec::new();
    collect_partial_nodes_into(nodes, &mut partials);
    partials
}

fn collect_partial_nodes_into(nodes: &[Node], partials: &mut Vec<NamedSpan>) {
    for node in nodes {
        match node {
            Node::Partial { name, span } => partials.push(NamedSpan {
                name: name.clone(),
                span: span.clone(),
            }),
            Node::Section { children, .. } | Node::InvertedSection { children, .. } => {
                collect_partial_nodes_into(children, partials);
            }
            Node::Text { .. }
            | Node::EscapedVariable { .. }
            | Node::UnescapedVariable { .. }
            | Node::Comment { .. }
            | Node::DelimiterChange { .. } => {}
        }
    }
}

fn collect_reference_nodes(nodes: &[Node]) -> Vec<NamedSpan> {
    let mut references = Vec::new();
    collect_reference_nodes_into(nodes, &mut references);
    references
}

fn collect_reference_nodes_into(nodes: &[Node], references: &mut Vec<NamedSpan>) {
    for node in nodes {
        match node {
            Node::EscapedVariable { name, span } | Node::UnescapedVariable { name, span } => {
                references.push(NamedSpan {
                    name: name.clone(),
                    span: span.clone(),
                });
            }
            Node::Section {
                name,
                span,
                children,
            }
            | Node::InvertedSection {
                name,
                span,
                children,
            } => {
                references.push(NamedSpan {
                    name: name.clone(),
                    span: span.clone(),
                });
                collect_reference_nodes_into(children, references);
            }
            Node::Text { .. }
            | Node::Comment { .. }
            | Node::Partial { .. }
            | Node::DelimiterChange { .. } => {}
        }
    }
}

fn parse_frontmatter_value(body: &str) -> Result<(FrontmatterFormat, serde_json::Value), String> {
    let trimmed = body.trim_start();
    if trimmed.starts_with('{') {
        return serde_json::from_str(body)
            .map(|value| (FrontmatterFormat::Json, value))
            .map_err(|error| format!("failed to parse JSON frontmatter: {error}"));
    }

    if looks_like_toml(body) {
        let value: toml::Value = toml::from_str(body)
            .map_err(|error| format!("failed to parse TOML frontmatter: {error}"))?;
        return serde_json::to_value(value)
            .map(|value| (FrontmatterFormat::Toml, value))
            .map_err(|error| format!("failed to convert TOML frontmatter: {error}"));
    }

    let value: serde_yaml::Value = serde_yaml::from_str(body)
        .map_err(|error| format!("failed to parse YAML frontmatter: {error}"))?;
    serde_json::to_value(value)
        .map(|value| (FrontmatterFormat::Yaml, value))
        .map_err(|error| format!("failed to convert YAML frontmatter: {error}"))
}

fn looks_like_toml(body: &str) -> bool {
    body.lines().any(|line| {
        let trimmed = line.trim();
        !trimmed.is_empty()
            && !trimmed.starts_with('#')
            && (trimmed.starts_with('[') || trimmed.contains('='))
    })
}

fn schema_defines_path(schema: &serde_json::Value, path: &str) -> bool {
    let mut current = schema;
    for component in path.split('.') {
        let Some(properties) = current
            .get("properties")
            .and_then(serde_json::Value::as_object)
        else {
            return false;
        };
        let Some(next) = properties.get(component) else {
            return false;
        };
        current = next;
    }
    true
}
