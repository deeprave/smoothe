use crate::context_schema::ContextShape;

#[derive(Debug, Clone, PartialEq)]
pub struct LambdaSpec {
    pub name: String,
    pub usage: LambdaUsage,
    pub argument: Option<ContextShape>,
    pub returns: Option<ContextShape>,
    pub side_effects: LambdaSideEffects,
}

impl LambdaSpec {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            usage: LambdaUsage::Both,
            argument: None,
            returns: None,
            side_effects: LambdaSideEffects::Unknown,
        }
    }

    pub fn with_usage(mut self, usage: LambdaUsage) -> Self {
        self.usage = usage;
        self
    }

    pub fn with_argument(mut self, argument: ContextShape) -> Self {
        self.argument = Some(argument);
        self
    }

    pub fn with_returns(mut self, returns: ContextShape) -> Self {
        self.returns = Some(returns);
        self
    }

    pub fn with_side_effects(mut self, side_effects: LambdaSideEffects) -> Self {
        self.side_effects = side_effects;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LambdaUsage {
    Variable,
    Section,
    Both,
}

impl LambdaUsage {
    pub fn allows_variable(self) -> bool {
        matches!(self, Self::Variable | Self::Both)
    }

    pub fn allows_section(self) -> bool {
        matches!(self, Self::Section | Self::Both)
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Variable => "variable",
            Self::Section => "section",
            Self::Both => "both",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LambdaSideEffects {
    None,
    Declared,
    Unknown,
}

impl LambdaSideEffects {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Declared => "declared",
            Self::Unknown => "unknown",
        }
    }
}
