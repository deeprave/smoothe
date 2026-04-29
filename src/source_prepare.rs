use crate::parser::{Diagnostic, DiagnosticSeverity, IssueKind, SourceLocation, SourceSpan};

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
pub struct PreparedSource {
    pub frontmatter: FrontmatterState,
    pub body_offset: usize,
    pub body_start_line: usize,
    pub diagnostics: Vec<Diagnostic>,
}

pub fn prepare_source(source: &str, source_name: &str) -> PreparedSource {
    let Some(frontmatter) = frontmatter_bounds(source) else {
        return PreparedSource {
            frontmatter: FrontmatterState::default(),
            body_offset: 0,
            body_start_line: 1,
            diagnostics: Vec::new(),
        };
    };
    let body_start_line = line_for_offset(source, frontmatter.body_offset);
    let body = &source[frontmatter.body_start..frontmatter.body_end];
    if body.trim().is_empty() {
        return PreparedSource {
            frontmatter: FrontmatterState::default(),
            body_offset: frontmatter.body_offset,
            body_start_line,
            diagnostics: Vec::new(),
        };
    }

    match parse_frontmatter_value(body) {
        Ok((format, context)) => PreparedSource {
            frontmatter: FrontmatterState {
                format: Some(format),
                context,
            },
            body_offset: frontmatter.body_offset,
            body_start_line,
            diagnostics: Vec::new(),
        },
        Err(message) => PreparedSource {
            frontmatter: FrontmatterState::default(),
            body_offset: frontmatter.body_offset,
            body_start_line,
            diagnostics: vec![Diagnostic::new(
                DiagnosticSeverity::Warning,
                IssueKind::FrontmatterParseError,
                source_name,
                SourceLocation::for_offset(source, 0),
                SourceSpan::new(0, frontmatter.body_offset),
                message,
            )],
        },
    }
}

struct FrontmatterBounds {
    body_start: usize,
    body_end: usize,
    body_offset: usize,
}

fn frontmatter_bounds(source: &str) -> Option<FrontmatterBounds> {
    let first_line = next_line(source, 0)?;
    if !is_frontmatter_delimiter(first_line.text) {
        return None;
    }

    let mut cursor = first_line.end;
    while cursor < source.len() {
        let line = next_line(source, cursor)?;
        if is_frontmatter_delimiter(line.text) {
            return Some(FrontmatterBounds {
                body_start: first_line.end,
                body_end: line.start,
                body_offset: line.end,
            });
        }
        cursor = line.end;
    }

    None
}

struct SourceLine<'a> {
    text: &'a str,
    start: usize,
    end: usize,
}

fn next_line(source: &str, start: usize) -> Option<SourceLine<'_>> {
    if start > source.len() {
        return None;
    }
    let remaining = &source[start..];
    if remaining.is_empty() {
        return Some(SourceLine {
            text: "",
            start,
            end: start,
        });
    }

    let line_end = remaining
        .find('\n')
        .map_or(source.len(), |relative| start + relative + 1);
    Some(SourceLine {
        text: &source[start..line_end],
        start,
        end: line_end,
    })
}

fn is_frontmatter_delimiter(line: &str) -> bool {
    line.trim_end_matches(['\r', '\n']).trim_end() == "---"
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

fn line_for_offset(source: &str, offset: usize) -> usize {
    source[..offset]
        .chars()
        .filter(|character| *character == '\n')
        .count()
        + 1
}
