mod ast;
mod diagnostic;
mod input;
mod source;

use std::{collections::HashSet, fs, ops::Range, path::PathBuf};

pub use ast::{Ast, Delimiters, Node, TemplateName};
pub use diagnostic::{Diagnostic, DiagnosticSeverity, IssueKind, ParseEvent};
pub use input::{FeedbackHandlers, LambdaSpec, ParserInput, PartialMapping, SourceMetadata};
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
    pub parent_references: Vec<ParentReference>,
    pub block_definitions: Vec<BlockDefinition>,
    pub dynamic_names: Vec<DynamicName>,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParentReference {
    pub name: TemplateName,
    pub source_name: String,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockDefinition {
    pub name: String,
    pub source_name: String,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DynamicName {
    pub name: TemplateName,
    pub source_name: String,
    pub span: SourceSpan,
}

struct Parser<'a> {
    source_name: String,
    source_root: Option<PathBuf>,
    body_offset: usize,
    body_start_line: usize,
    source: &'a str,
    feedback: FeedbackHandlers<'a>,
    partials: Vec<PartialMapping>,
    lambdas: Vec<LambdaSpec>,
    context_schema: Option<serde_json::Value>,
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
            body_offset: input.source.body_offset,
            body_start_line: input.source.body_start_line,
            source: input.source_text,
            feedback: input.feedback,
            partials: input.partials,
            lambdas: input.lambdas,
            context_schema: input.context_schema,
            expand_partials,
            state: ParserState {
                diagnostics: Vec::new(),
                delimiters: Delimiters::default(),
                recovered: false,
                partials: Vec::new(),
                nested_partials: Vec::new(),
                lambda_references: Vec::new(),
                parent_references: Vec::new(),
                block_definitions: Vec::new(),
                dynamic_names: Vec::new(),
            },
        }
    }

    fn parse(mut self) -> ParseResult {
        let nodes = self.parse_nodes(None, self.body_offset).nodes;
        self.classify_lambdas(&nodes);
        self.index_advanced_nodes(&nodes);
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
        expected_closing: Option<OpenContainer>,
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
                    if self.is_lambda(&name) {
                        nodes.push(Node::lambda_variable(name, tag.span));
                    } else {
                        nodes.push(Node::escaped_variable(name, tag.span));
                    }
                }
                TagKind::UnescapedVariable(name) => {
                    nodes.push(Node::unescaped_variable(name, tag.span));
                }
                TagKind::Comment(text) => nodes.push(Node::comment(text, tag.span)),
                TagKind::Partial(name) => nodes.push(Node::partial(name, tag.span)),
                TagKind::DynamicPartial(expression) => {
                    nodes.push(Node::dynamic_partial(expression, tag.span));
                }
                TagKind::DelimiterChange(open, close) => {
                    self.state.delimiters = Delimiters::new(open.clone(), close.clone());
                    nodes.push(Node::delimiter_change(open, close, tag.span));
                }
                TagKind::Section { name, inverted } => {
                    let kind = if inverted {
                        ContainerKind::InvertedSection
                    } else if self.is_lambda(&name) {
                        ContainerKind::LambdaSection
                    } else {
                        ContainerKind::Section
                    };
                    let container = OpenContainer {
                        name,
                        span_start: tag.span.start,
                        content_start: tag.span.end,
                        kind,
                    };
                    let parsed = self.parse_nodes(Some(container.clone()), container.content_start);
                    cursor = parsed.cursor;
                    let span = container.span_start..cursor;
                    match container.kind {
                        ContainerKind::Section => {
                            nodes.push(Node::section(container.name, span, parsed.nodes));
                        }
                        ContainerKind::InvertedSection => {
                            nodes.push(Node::inverted_section(container.name, span, parsed.nodes));
                        }
                        ContainerKind::LambdaSection => {
                            nodes.push(Node::lambda_section(container.name, span, parsed.nodes));
                        }
                        ContainerKind::Parent | ContainerKind::Block => unreachable!(),
                    }
                }
                TagKind::Parent(name) => {
                    let container = OpenContainer {
                        name: name.value().to_owned(),
                        span_start: tag.span.start,
                        content_start: tag.span.end,
                        kind: ContainerKind::Parent,
                    };
                    let parsed = self.parse_nodes(Some(container.clone()), container.content_start);
                    cursor = parsed.cursor;
                    nodes.push(Node::parent(
                        name,
                        container.span_start..cursor,
                        parsed.nodes,
                    ));
                }
                TagKind::Block(name) => {
                    let container = OpenContainer {
                        name,
                        span_start: tag.span.start,
                        content_start: tag.span.end,
                        kind: ContainerKind::Block,
                    };
                    let parsed = self.parse_nodes(Some(container.clone()), container.content_start);
                    cursor = parsed.cursor;
                    nodes.push(Node::block(
                        container.name,
                        container.span_start..cursor,
                        parsed.nodes,
                    ));
                }
                TagKind::MalformedInheritance => {
                    self.emit(
                        DiagnosticSeverity::Error,
                        IssueKind::MalformedInheritance,
                        tag.span,
                        "malformed inheritance tag",
                    );
                }
                TagKind::MalformedDynamicName => {
                    self.emit(
                        DiagnosticSeverity::Error,
                        IssueKind::MalformedDynamicName,
                        tag.span,
                        "malformed dynamic name",
                    );
                }
                TagKind::Closing(name) => {
                    if let Some(container) = expected_closing {
                        if container.name == name {
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
                                "closing tag `{name}` does not match open tag `{}`",
                                container.name
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

        if let Some(container) = expected_closing {
            self.emit(
                DiagnosticSeverity::Error,
                IssueKind::UnclosedSection,
                container.span_start..container.content_start,
                format!("tag `{}` is not closed", container.name),
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
            b'>' => self.parse_partial_tag(raw)?,
            b'<' => self.parse_parent_tag(raw)?,
            b'$' => {
                let name = raw[1..].trim();
                if name.is_empty() {
                    return None;
                }
                TagKind::Block(name.to_owned())
            }
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

    fn parse_partial_tag(&self, raw: &str) -> Option<TagKind> {
        let name = raw[1..].trim();
        if let Some(expression) = dynamic_expression(name) {
            if expression.is_empty() {
                return Some(TagKind::MalformedDynamicName);
            }
            return Some(TagKind::DynamicPartial(expression.to_owned()));
        }
        Some(TagKind::Partial(name.to_owned()))
    }

    fn parse_parent_tag(&self, raw: &str) -> Option<TagKind> {
        let name = raw[1..].trim();
        if name.is_empty() {
            return Some(TagKind::MalformedInheritance);
        }

        if let Some(expression) = dynamic_expression(name) {
            if expression.is_empty() {
                return Some(TagKind::MalformedDynamicName);
            }
            return Some(TagKind::Parent(TemplateName::Dynamic(
                expression.to_owned(),
            )));
        }

        Some(TagKind::Parent(TemplateName::Static(name.to_owned())))
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

    fn index_advanced_nodes(&mut self, nodes: &[Node]) {
        for node in nodes {
            match node {
                Node::Parent {
                    name,
                    span,
                    children,
                } => {
                    self.state.parent_references.push(ParentReference {
                        name: name.clone(),
                        source_name: self.source_name.clone(),
                        span: SourceSpan::new(span.start, span.end),
                    });
                    if matches!(name, TemplateName::Dynamic(_)) {
                        self.state.dynamic_names.push(DynamicName {
                            name: name.clone(),
                            source_name: self.source_name.clone(),
                            span: SourceSpan::new(span.start, span.end),
                        });
                    }
                    self.index_advanced_nodes(children);
                }
                Node::Block {
                    name,
                    span,
                    children,
                } => {
                    self.state.block_definitions.push(BlockDefinition {
                        name: name.clone(),
                        source_name: self.source_name.clone(),
                        span: SourceSpan::new(span.start, span.end),
                    });
                    self.index_advanced_nodes(children);
                }
                Node::DynamicPartial { expression, span } => {
                    self.state.dynamic_names.push(DynamicName {
                        name: TemplateName::Dynamic(expression.clone()),
                        source_name: self.source_name.clone(),
                        span: SourceSpan::new(span.start, span.end),
                    });
                }
                Node::Section { children, .. }
                | Node::InvertedSection { children, .. }
                | Node::LambdaSection { children, .. } => {
                    self.index_advanced_nodes(children);
                }
                Node::Text { .. }
                | Node::EscapedVariable { .. }
                | Node::LambdaVariable { .. }
                | Node::UnescapedVariable { .. }
                | Node::Comment { .. }
                | Node::Partial { .. }
                | Node::DelimiterChange { .. } => {}
            }
        }
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

    fn is_lambda(&self, name: &str) -> bool {
        self.lambdas.iter().any(|lambda| lambda.name == name)
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
            location: self.location_for_offset(span.start),
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

    fn location_for_offset(&self, offset: usize) -> SourceLocation {
        if offset < self.body_offset {
            return SourceLocation::for_offset(self.source, offset);
        }

        let relative = offset - self.body_offset;
        let mut location = SourceLocation::for_offset(&self.source[self.body_offset..], relative);
        location.line += self.body_start_line.saturating_sub(1);
        location
    }
}

#[derive(Debug, Clone)]
struct OpenContainer {
    name: String,
    span_start: usize,
    content_start: usize,
    kind: ContainerKind,
}

#[derive(Debug, Clone, Copy)]
enum ContainerKind {
    Section,
    InvertedSection,
    LambdaSection,
    Parent,
    Block,
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
    DynamicPartial(String),
    Parent(TemplateName),
    Block(String),
    DelimiterChange(String, String),
    Section { name: String, inverted: bool },
    Closing(String),
    MalformedInheritance,
    MalformedDynamicName,
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
            Node::Section { children, .. }
            | Node::InvertedSection { children, .. }
            | Node::LambdaSection { children, .. }
            | Node::Parent { children, .. }
            | Node::Block { children, .. } => {
                collect_partial_nodes_into(children, partials);
            }
            Node::Text { .. }
            | Node::EscapedVariable { .. }
            | Node::LambdaVariable { .. }
            | Node::UnescapedVariable { .. }
            | Node::Comment { .. }
            | Node::DynamicPartial { .. }
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
            Node::EscapedVariable { name, span }
            | Node::LambdaVariable { name, span }
            | Node::UnescapedVariable { name, span } => references.push(NamedSpan {
                name: name.clone(),
                span: span.clone(),
            }),
            Node::Section {
                name,
                span,
                children,
            }
            | Node::InvertedSection {
                name,
                span,
                children,
            }
            | Node::LambdaSection {
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
            Node::Parent { children, .. } | Node::Block { children, .. } => {
                collect_reference_nodes_into(children, references);
            }
            Node::Text { .. }
            | Node::Comment { .. }
            | Node::Partial { .. }
            | Node::DynamicPartial { .. }
            | Node::DelimiterChange { .. } => {}
        }
    }
}

fn dynamic_expression(name: &str) -> Option<&str> {
    name.strip_prefix('*').map(str::trim)
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
