use std::path::PathBuf;

use crate::parser::{Diagnostic, SourceSpan};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum CheckEventLevel {
    Error,
    #[default]
    Warning,
    Info,
    Debug,
    Trace,
}

impl CheckEventLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Error => "error",
            Self::Warning => "warning",
            Self::Info => "info",
            Self::Debug => "debug",
            Self::Trace => "trace",
        }
    }

    pub fn visible_at(&self, threshold: Self) -> bool {
        *self <= threshold
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CheckEvent {
    RunStarted {
        input_count: usize,
    },
    InputStarted {
        source_name: String,
    },
    PartialStarted(PartialEvent),
    PartialFinished(PartialEvent),
    PartialSkipped(PartialEvent),
    Progress(ProgressEvent),
    Trace(ProgressEvent),
    Diagnostic(DiagnosticEvent),
    InputFinished {
        source_name: String,
        has_error: bool,
    },
    RunFinished {
        has_error: bool,
    },
}

impl CheckEvent {
    pub fn run_started(input_count: usize) -> Self {
        Self::RunStarted { input_count }
    }

    pub fn run_finished(has_error: bool) -> Self {
        Self::RunFinished { has_error }
    }

    pub fn progress(level: CheckEventLevel, event: ProgressEvent) -> Self {
        if level == CheckEventLevel::Trace {
            Self::Trace(event.with_level(level))
        } else {
            Self::Progress(event.with_level(level))
        }
    }

    pub fn diagnostic(diagnostic: Diagnostic) -> Self {
        Self::Diagnostic(DiagnosticEvent::new(diagnostic))
    }

    pub fn level(&self) -> CheckEventLevel {
        match self {
            Self::RunStarted { .. }
            | Self::InputStarted { .. }
            | Self::InputFinished { .. }
            | Self::RunFinished { .. } => CheckEventLevel::Info,
            Self::PartialStarted(event)
            | Self::PartialFinished(event)
            | Self::PartialSkipped(event) => event.level,
            Self::Progress(event) | Self::Trace(event) => event.level,
            Self::Diagnostic(event) => event.level,
        }
    }

    pub fn kind(&self) -> &'static str {
        match self {
            Self::RunStarted { .. } => "run-started",
            Self::InputStarted { .. } => "input-started",
            Self::PartialStarted(_) => "partial-started",
            Self::PartialFinished(_) => "partial-finished",
            Self::PartialSkipped(_) => "partial-skipped",
            Self::Progress(_) => "progress",
            Self::Trace(_) => "trace",
            Self::Diagnostic(_) => "diagnostic",
            Self::InputFinished { .. } => "input-finished",
            Self::RunFinished { .. } => "run-finished",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagnosticEvent {
    pub level: CheckEventLevel,
    pub diagnostic: Diagnostic,
}

impl DiagnosticEvent {
    pub fn new(diagnostic: Diagnostic) -> Self {
        let level = match diagnostic.severity {
            crate::parser::DiagnosticSeverity::Error => CheckEventLevel::Error,
            crate::parser::DiagnosticSeverity::Warning => CheckEventLevel::Warning,
            crate::parser::DiagnosticSeverity::Info => CheckEventLevel::Info,
            crate::parser::DiagnosticSeverity::Debug => CheckEventLevel::Debug,
        };
        Self { level, diagnostic }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartialEvent {
    pub level: CheckEventLevel,
    pub name: String,
    pub path: Option<PathBuf>,
    pub referred_from_source: Option<String>,
    pub referred_from_span: Option<SourceSpan>,
    pub recursive: bool,
}

impl PartialEvent {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            level: CheckEventLevel::Debug,
            name: name.into(),
            path: None,
            referred_from_source: None,
            referred_from_span: None,
            recursive: false,
        }
    }

    pub fn with_path(mut self, path: impl Into<PathBuf>) -> Self {
        self.path = Some(path.into());
        self
    }

    pub fn with_referrer(mut self, source: impl Into<String>, span: SourceSpan) -> Self {
        self.referred_from_source = Some(source.into());
        self.referred_from_span = Some(span);
        self
    }

    pub fn recursive(mut self, recursive: bool) -> Self {
        self.recursive = recursive;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProgressEvent {
    pub level: CheckEventLevel,
    pub message: String,
}

impl ProgressEvent {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            level: CheckEventLevel::Info,
            message: message.into(),
        }
    }

    pub fn with_level(mut self, level: CheckEventLevel) -> Self {
        self.level = level;
        self
    }
}

pub trait CheckEventListener {
    fn on_event(&mut self, event: &CheckEvent) -> Result<(), String>;
}

#[derive(Default)]
pub struct CheckEventBus {
    listeners: Vec<Box<dyn CheckEventListener>>,
    failure: Option<String>,
}

impl CheckEventBus {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_listener(&mut self, listener: impl CheckEventListener + 'static) {
        self.listeners.push(Box::new(listener));
    }

    pub fn emit(&mut self, event: CheckEvent) -> Result<(), String> {
        let mut event_failure = None;
        for listener in &mut self.listeners {
            if let Err(error) = listener.on_event(&event) {
                if event_failure.is_none() {
                    event_failure = Some(error.clone());
                }
                if self.failure.is_none() {
                    self.failure = Some(error);
                }
            }
        }
        event_failure.map_or(Ok(()), Err)
    }

    pub fn failure(&self) -> Option<&str> {
        self.failure.as_deref()
    }
}
