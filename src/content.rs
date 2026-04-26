use std::path::PathBuf;

use crate::parser::{
    Ast, Diagnostic, DiagnosticSeverity, IssueKind, LambdaSpec, ParseResult, ParserInput,
    ParserState, PartialMapping, SourceLocation, SourceMetadata, SourceSpan, parse,
};

pub struct ContentInput<'a> {
    pub source: SourceMetadata,
    pub raw_data: &'a str,
    pub partials: Vec<PartialMapping>,
    pub lambdas: Vec<LambdaSpec>,
    pub context_schema: Option<serde_json::Value>,
}

impl<'a> ContentInput<'a> {
    pub fn new(source: SourceMetadata, raw_data: &'a str) -> Self {
        Self {
            source,
            raw_data,
            partials: Vec::new(),
            lambdas: Vec::new(),
            context_schema: None,
        }
    }
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

impl Default for FrontmatterState {
    fn default() -> Self {
        Self {
            format: None,
            context: serde_json::Value::Object(serde_json::Map::new()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemplateContent {
    pub raw_data: String,
    pub frontmatter: FrontmatterState,
    pub body_offset: usize,
    pub body_start_line: usize,
    pub ast: Ast,
    pub state: ParserState,
}

pub fn process(input: ContentInput<'_>) -> TemplateContent {
    let extracted = extract_frontmatter(input.raw_data, &input.source.name);
    let mut diagnostics = extracted.diagnostics;
    let mut partials = derive_partial_mappings(&extracted.frontmatter, &input.source.name);
    diagnostics.extend(partials.diagnostics);
    merge_explicit_partials(&mut partials.mappings, input.partials);

    let mut parser_input = ParserInput::new(
        input
            .source
            .with_body_start(extracted.body_offset, extracted.body_start_line),
        input.raw_data,
    );
    parser_input.partials = partials.mappings;
    parser_input.lambdas = input.lambdas;
    parser_input.context_schema = input.context_schema;

    let ParseResult { ast, mut state } = parse(parser_input);
    if !diagnostics.is_empty() {
        state.recovered = true;
        diagnostics.extend(state.diagnostics);
        state.diagnostics = diagnostics;
    }

    TemplateContent {
        raw_data: input.raw_data.to_owned(),
        frontmatter: extracted.frontmatter,
        body_offset: extracted.body_offset,
        body_start_line: extracted.body_start_line,
        ast,
        state,
    }
}

struct ExtractedFrontmatter {
    frontmatter: FrontmatterState,
    body_offset: usize,
    body_start_line: usize,
    diagnostics: Vec<Diagnostic>,
}

fn extract_frontmatter(source: &str, source_name: &str) -> ExtractedFrontmatter {
    if !source.starts_with("---\n") {
        return ExtractedFrontmatter {
            frontmatter: FrontmatterState::default(),
            body_offset: 0,
            body_start_line: 1,
            diagnostics: Vec::new(),
        };
    }

    let body_start = 4;
    let Some(relative_end) = source[body_start..].find("\n---\n") else {
        return ExtractedFrontmatter {
            frontmatter: FrontmatterState::default(),
            body_offset: 0,
            body_start_line: 1,
            diagnostics: Vec::new(),
        };
    };
    let body_end = body_start + relative_end;
    let body_offset = body_end + "\n---\n".len();
    let body_start_line = line_for_offset(source, body_offset);
    let body = &source[body_start..body_end];
    if body.trim().is_empty() {
        return ExtractedFrontmatter {
            frontmatter: FrontmatterState::default(),
            body_offset,
            body_start_line,
            diagnostics: Vec::new(),
        };
    }

    match parse_frontmatter_value(body) {
        Ok((format, context)) => ExtractedFrontmatter {
            frontmatter: FrontmatterState {
                format: Some(format),
                context,
            },
            body_offset,
            body_start_line,
            diagnostics: Vec::new(),
        },
        Err(message) => ExtractedFrontmatter {
            frontmatter: FrontmatterState::default(),
            body_offset,
            body_start_line,
            diagnostics: vec![diagnostic(
                DiagnosticSeverity::Warning,
                IssueKind::FrontmatterParseError,
                source_name,
                source,
                0,
                body_offset,
                message,
            )],
        },
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

struct DerivedPartials {
    mappings: Vec<PartialMapping>,
    diagnostics: Vec<Diagnostic>,
}

fn derive_partial_mappings(frontmatter: &FrontmatterState, source_name: &str) -> DerivedPartials {
    let Some(includes) = frontmatter.context.get("includes") else {
        return DerivedPartials {
            mappings: Vec::new(),
            diagnostics: Vec::new(),
        };
    };

    let Some(entries) = includes.as_array() else {
        return DerivedPartials {
            mappings: Vec::new(),
            diagnostics: vec![unsupported_includes(source_name)],
        };
    };

    let mut mappings = Vec::new();
    let mut diagnostics = Vec::new();
    for entry in entries {
        let Some(path) = entry.as_str() else {
            diagnostics.push(unsupported_includes(source_name));
            continue;
        };
        if let Some(mapping) = include_mapping(path) {
            mappings.push(mapping);
        } else {
            diagnostics.push(unsupported_includes(source_name));
        }
    }

    DerivedPartials {
        mappings,
        diagnostics,
    }
}

fn include_mapping(path: &str) -> Option<PartialMapping> {
    let include_path = PathBuf::from(path);
    let stem = include_path.file_stem()?.to_str()?;
    let key = stem.strip_prefix('_').unwrap_or(stem).to_owned();
    let file_name = include_path.file_name()?.to_str()?;
    let parser_file_name = if file_name.starts_with('_') {
        file_name.to_owned()
    } else {
        format!("_{file_name}")
    };
    let mut parser_path = include_path;
    parser_path.set_file_name(parser_file_name);
    Some(PartialMapping::new(key, parser_path))
}

fn merge_explicit_partials(derived: &mut Vec<PartialMapping>, explicit: Vec<PartialMapping>) {
    for mapping in explicit {
        if let Some(existing) = derived
            .iter_mut()
            .find(|existing| existing.name == mapping.name)
        {
            *existing = mapping;
        } else {
            derived.push(mapping);
        }
    }
}

fn unsupported_includes(source_name: &str) -> Diagnostic {
    Diagnostic {
        severity: DiagnosticSeverity::Warning,
        issue: IssueKind::UnsupportedIncludes,
        source_name: source_name.to_owned(),
        location: SourceLocation { line: 1, column: 1 },
        span: SourceSpan::new(0, 0),
        message: "unsupported frontmatter `includes` value".to_owned(),
    }
}

fn diagnostic(
    severity: DiagnosticSeverity,
    issue: IssueKind,
    source_name: &str,
    source: &str,
    start: usize,
    end: usize,
    message: impl Into<String>,
) -> Diagnostic {
    Diagnostic {
        severity,
        issue,
        source_name: source_name.to_owned(),
        location: SourceLocation::for_offset(source, start),
        span: SourceSpan::new(start, end),
        message: message.into(),
    }
}

fn line_for_offset(source: &str, offset: usize) -> usize {
    source[..offset]
        .chars()
        .filter(|character| *character == '\n')
        .count()
        + 1
}
