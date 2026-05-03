use std::{
    collections::BTreeMap,
    env, fs, io,
    path::{Path, PathBuf},
};

use clap::{ColorChoice, ValueEnum};
use serde::Deserialize;

use crate::parser::PartialMapping;

const CONFIG_FILE_NAME: &str = "smoothe.toml";

#[derive(Debug, Default, Deserialize)]
pub struct Configuration {
    #[serde(default)]
    options: GlobalConfig,
    #[serde(default)]
    check: CheckConfig,
    // Set only after loading from a config file; when present it must be
    // absolute so config-relative paths never depend on later CWD changes.
    #[serde(skip)]
    source_dir: Option<PathBuf>,
}

#[derive(Debug, Default, Deserialize)]
struct GlobalConfig {
    color: Option<ConfigColor>,
}

#[derive(Debug, Default, Deserialize)]
pub struct CheckConfig {
    schema: Option<String>,
    lambdas: Option<String>,
    output: Option<CheckOutputFormat>,
    verbosity: Option<CheckVerbosity>,
    #[serde(default)]
    partials: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResolvedGlobalOptions {
    pub color: ColorChoice,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedCheckOptions {
    pub schema: SemanticInput,
    pub lambdas: SemanticInput,
    pub partials: Vec<PartialMapping>,
    pub output: CheckOutputFormat,
    pub verbosity: CheckVerbosity,
}

impl Default for ResolvedCheckOptions {
    fn default() -> Self {
        Self {
            schema: SemanticInput::Disabled,
            lambdas: SemanticInput::Disabled,
            partials: Vec::new(),
            output: CheckOutputFormat::Compiler,
            verbosity: CheckVerbosity::Warning,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, ValueEnum)]
#[serde(rename_all = "kebab-case")]
#[value(rename_all = "kebab-case")]
pub enum CheckOutputFormat {
    Compiler,
    Json,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, ValueEnum)]
#[serde(rename_all = "kebab-case")]
#[value(rename_all = "kebab-case")]
pub enum CheckVerbosity {
    Error,
    Warning,
    Info,
    Debug,
    Trace,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SemanticInput {
    Disabled,
    Path(PathBuf),
}

#[derive(Debug)]
pub struct CliGlobalOptions {
    pub color: Option<ColorChoice>,
    pub no_color: bool,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct EnvironmentOptions {
    pub nocolor: bool,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum ConfigColor {
    Boolean(bool),
    String(ConfigColorString),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum ConfigColorString {
    Always,
    Auto,
    Never,
}

impl ConfigColor {
    fn color_choice(&self) -> ColorChoice {
        match self {
            Self::Boolean(true) | Self::String(ConfigColorString::Always) => ColorChoice::Always,
            Self::Boolean(false) | Self::String(ConfigColorString::Never) => ColorChoice::Never,
            Self::String(ConfigColorString::Auto) => ColorChoice::Auto,
        }
    }
}

impl Configuration {
    fn resolve_global_options(
        &self,
        cli: &CliGlobalOptions,
        nocolor: bool,
    ) -> ResolvedGlobalOptions {
        let mut color = ColorChoice::Auto;

        if let Some(config_color) = &self.options.color {
            color = config_color.color_choice();
        }

        if nocolor {
            color = ColorChoice::Never;
        }

        if cli.no_color {
            color = ColorChoice::Never;
        } else if let Some(cli_color) = cli.color {
            color = cli_color;
        }

        ResolvedGlobalOptions { color }
    }

    fn resolve_check_options(&self) -> Result<ResolvedCheckOptions, ConfigError> {
        Ok(ResolvedCheckOptions {
            schema: self.resolve_semantic_input(self.check.schema.as_deref()),
            lambdas: self.resolve_semantic_input(self.check.lambdas.as_deref()),
            partials: self.resolve_partial_mappings()?,
            output: self.check.output.unwrap_or(CheckOutputFormat::Compiler),
            verbosity: self.check.verbosity.unwrap_or(CheckVerbosity::Warning),
        })
    }

    fn resolve_semantic_input(&self, value: Option<&str>) -> SemanticInput {
        let Some(value) = value else {
            return SemanticInput::Disabled;
        };

        if value.eq_ignore_ascii_case("none") {
            return SemanticInput::Disabled;
        }

        let path = PathBuf::from(value);
        if path.is_absolute() {
            return SemanticInput::Path(path);
        }

        match &self.source_dir {
            Some(source_dir) => SemanticInput::Path(source_dir.join(path)),
            None => SemanticInput::Path(path),
        }
    }

    fn resolve_partial_mappings(&self) -> Result<Vec<PartialMapping>, ConfigError> {
        self.check
            .partials
            .iter()
            .map(|(name, path)| self.resolve_partial_mapping(name.clone(), path))
            .collect()
    }

    fn resolve_partial_mapping(
        &self,
        name: impl Into<String>,
        path: impl AsRef<Path>,
    ) -> Result<PartialMapping, ConfigError> {
        self.resolve_partial_path(path.as_ref())
            .map(|path| PartialMapping::from_partial_path(name, path))
    }

    fn resolve_partial_path(&self, path: &Path) -> Result<PathBuf, ConfigError> {
        if path.is_absolute() {
            return Ok(path.to_owned());
        }

        let Some(source_dir) = &self.source_dir else {
            return Ok(path.to_owned());
        };

        // `source_dir` is normalized by `config_source_dir` when the
        // configuration is loaded. Keep this guard as a defensive invariant
        // check so partials never silently fall back to the process CWD.
        debug_assert!(
            source_dir.is_absolute(),
            "configuration source directory should be normalized before partial resolution"
        );
        if !source_dir.is_absolute() {
            return Err(ConfigError::relative_source_dir(source_dir));
        }

        // Partial paths are resolved relative to the config file directory,
        // but this is not a sandbox boundary: explicit `..` components remain
        // part of the caller-provided path.
        Ok(source_dir.join(path))
    }
}

#[derive(Debug)]
pub struct ResolvedOptions {
    pub global: ResolvedGlobalOptions,
    pub check: ResolvedCheckOptions,
}

pub fn load(explicit_path: Option<&Path>) -> Result<Configuration, ConfigError> {
    if let Some(path) = explicit_path {
        return read_explicit_config(path);
    }

    for path in discovery_paths() {
        match read_config(&path) {
            Ok(Some(config)) => return Ok(config),
            Ok(None) => {}
            Err(error) => return Err(error),
        }
    }

    Ok(Configuration::default())
}

pub fn resolve(
    configuration: &Configuration,
    cli: &CliGlobalOptions,
) -> Result<ResolvedOptions, ConfigError> {
    resolve_with_environment(
        configuration,
        cli,
        EnvironmentOptions {
            nocolor: env::var_os("NOCOLOR").is_some(),
        },
    )
}

pub fn resolve_with_environment(
    configuration: &Configuration,
    cli: &CliGlobalOptions,
    environment: EnvironmentOptions,
) -> Result<ResolvedOptions, ConfigError> {
    Ok(ResolvedOptions {
        global: configuration.resolve_global_options(cli, environment.nocolor),
        check: configuration.resolve_check_options()?,
    })
}

fn read_config(path: &Path) -> Result<Option<Configuration>, ConfigError> {
    let source = match fs::read_to_string(path) {
        Ok(source) => source,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(ConfigError::read(path, error)),
    };

    parse_config_source(path, &source).map(Some)
}

fn read_explicit_config(path: &Path) -> Result<Configuration, ConfigError> {
    let source = fs::read_to_string(path).map_err(|error| ConfigError::read(path, error))?;

    parse_config_source(path, &source)
}

fn parse_config_source(path: &Path, source: &str) -> Result<Configuration, ConfigError> {
    let mut configuration: Configuration =
        toml::from_str(source).map_err(|error| ConfigError::parse(path, error))?;
    configuration.source_dir = config_source_dir(path)?;
    Ok(configuration)
}

fn config_source_dir(path: &Path) -> Result<Option<PathBuf>, ConfigError> {
    let Some(source_dir) = path.parent() else {
        return Ok(None);
    };

    // Absolute config paths already identify their source directory without
    // consulting the process working directory.
    if source_dir.is_absolute() {
        return Ok(Some(source_dir.to_owned()));
    }

    // Relative config paths are interpreted from the process working directory,
    // matching how the path was supplied during discovery or explicit loading.
    let current_dir = env::current_dir().map_err(ConfigError::current_dir)?;
    Ok(Some(current_dir.join(source_dir)))
}

fn discovery_paths() -> Vec<PathBuf> {
    let mut paths = vec![PathBuf::from(CONFIG_FILE_NAME)];

    if let Some(config_home) = env::var_os("XDG_CONFIG_HOME").filter(|value| !value.is_empty()) {
        paths.push(PathBuf::from(config_home).join(CONFIG_FILE_NAME));
    } else if let Some(home) = env::var_os("HOME").filter(|value| !value.is_empty()) {
        paths.push(PathBuf::from(home).join(".config").join(CONFIG_FILE_NAME));
    }

    paths
}

#[derive(Debug)]
pub struct ConfigError {
    kind: ConfigErrorKind,
}

#[derive(Debug)]
enum ConfigErrorKind {
    Read {
        path: PathBuf,
        source: io::Error,
    },
    Parse {
        path: PathBuf,
        source: toml::de::Error,
    },
    CurrentDir {
        source: io::Error,
    },
    // Internal invariant violation: loaded configurations should always store
    // an absolute source directory before option resolution.
    RelativeSourceDir {
        path: PathBuf,
    },
}

impl ConfigError {
    fn read(path: &Path, source: io::Error) -> Self {
        Self {
            kind: ConfigErrorKind::Read {
                path: path.to_owned(),
                source,
            },
        }
    }

    fn parse(path: &Path, source: toml::de::Error) -> Self {
        Self {
            kind: ConfigErrorKind::Parse {
                path: path.to_owned(),
                source,
            },
        }
    }

    fn current_dir(source: io::Error) -> Self {
        Self {
            kind: ConfigErrorKind::CurrentDir { source },
        }
    }

    fn relative_source_dir(path: &Path) -> Self {
        Self {
            kind: ConfigErrorKind::RelativeSourceDir {
                path: path.to_owned(),
            },
        }
    }
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ConfigErrorKind::Read { path, source } => {
                write!(
                    formatter,
                    "failed to read configuration {}: {}",
                    path.display(),
                    source
                )
            }
            ConfigErrorKind::Parse { path, source } => {
                write!(
                    formatter,
                    "failed to parse configuration {}: {}",
                    path.display(),
                    source
                )
            }
            ConfigErrorKind::CurrentDir { source } => {
                write!(
                    formatter,
                    "failed to determine current directory while resolving configuration source directory: {}",
                    source
                )
            }
            ConfigErrorKind::RelativeSourceDir { path } => {
                write!(
                    formatter,
                    "internal configuration error: source directory must be absolute for partial path resolution: {}; please file a bug",
                    path.display()
                )
            }
        }
    }
}
