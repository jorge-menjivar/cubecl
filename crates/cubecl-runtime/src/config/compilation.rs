use super::logger::{LogLevel, LoggerConfig};

/// Configuration for compilation settings in `CubeCL`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CompilationConfig {
    /// Logger configuration for compilation logs, using binary log levels.
    #[serde(default)]
    pub logger: LoggerConfig<CompilationLogLevel>,
    /// Whether compiled kernels are cached in the active environment.
    ///
    /// Enabled by default: compiling the kernels of a program takes seconds to tens of seconds
    /// every time a process starts, and a cached artifact is only reused by the very build that
    /// produced it, for the very device it was compiled for. Disable it to compile everything
    /// again on every run, for instance when working on a compiler.
    #[serde(default = "default_cache")]
    #[cfg(std_io)]
    pub cache: bool,
    /// Controls whether kernel launches enforce bounds checks.
    #[serde(default)]
    pub check_mode: BoundsCheckMode,
}

impl Default for CompilationConfig {
    fn default() -> Self {
        Self {
            logger: LoggerConfig::default(),
            #[cfg(std_io)]
            cache: default_cache(),
            check_mode: BoundsCheckMode::default(),
        }
    }
}

#[cfg(std_io)]
fn default_cache() -> bool {
    true
}

/// Bounds checks options.
#[derive(Default, Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
pub enum BoundsCheckMode {
    #[serde(rename = "enforce")]
    /// Always enforce bounds checks on every kernel launch.
    Enforce,
    #[serde(rename = "validate")]
    /// Always enforce bounds checks on every kernel launch, and validate unchecked kernels for OOB.
    Validate,
    /// Enforce bounds checking on standard launches, but skip checks on
    /// explicitly unchecked launches for better performance.
    #[default]
    #[serde(rename = "auto")]
    Auto,
}

/// Log levels for compilation in `CubeCL`.
#[derive(Default, Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
pub enum CompilationLogLevel {
    /// Compilation logging is disabled.
    #[default]
    #[serde(rename = "disabled")]
    Disabled,

    /// Basic compilation information is logged such as when kernels are compiled.
    #[serde(rename = "basic")]
    Basic,

    /// Full compilation details are logged including source code.
    #[serde(rename = "full")]
    Full,
}

impl LogLevel for CompilationLogLevel {}

#[cfg(all(test, std_io))]
mod tests {
    use super::*;

    #[test]
    fn cache_is_enabled_by_default() {
        assert!(CompilationConfig::default().cache);
    }

    #[test]
    fn cache_omitted_in_toml_stays_enabled() {
        // A `[compilation]` section written for another setting must not turn the cache off.
        let config: CompilationConfig = toml::from_str("check_mode = \"validate\"").unwrap();
        assert!(config.cache);
    }

    #[test]
    fn cache_can_be_disabled_in_toml() {
        let config: CompilationConfig = toml::from_str("cache = false").unwrap();
        assert!(!config.cache);
    }
}
