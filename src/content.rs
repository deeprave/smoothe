use std::path::PathBuf;

use crate::parser::{
    Ast, Diagnostic, DiagnosticSeverity, IssueKind, LambdaSpec, ParseResult, ParserInput,
    ParserState, PartialMapping, SourceLocation, SourceMetadata, SourceSpan, parse,
};
use crate::source_prepare::prepare_source;

pub use crate::source_prepare::{FrontmatterFormat, FrontmatterState};

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
    let prepared = prepare_source(input.raw_data, &input.source.name);
    let mut diagnostics = prepared.diagnostics;
    let mut partials = derive_partial_mappings(&prepared.frontmatter, &input.source.name);
    diagnostics.extend(partials.diagnostics);
    merge_explicit_partials(&mut partials.mappings, input.partials);

    let mut parser_input = ParserInput::new(
        input
            .source
            .with_body_start(prepared.body_offset, prepared.body_start_line),
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
        frontmatter: prepared.frontmatter,
        body_offset: prepared.body_offset,
        body_start_line: prepared.body_start_line,
        ast,
        state,
    }
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
    Diagnostic::new(
        DiagnosticSeverity::Warning,
        IssueKind::UnsupportedIncludes,
        source_name,
        SourceLocation { line: 1, column: 1 },
        SourceSpan::new(0, 0),
        "unsupported frontmatter `includes` value",
    )
}
