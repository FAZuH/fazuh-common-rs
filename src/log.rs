use std::ops::Deref;

use derive_builder::Builder;
use tracing::Level;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

/// A guard that ensures background logging tasks complete on application shutdown.
///
/// When the `log-file` feature is enabled, file logging happens on a background thread.
/// Dropping this guard signals that thread to flush remaining logs and shut down.
/// You should hold this in your `main` function for the lifetime of your application.
pub struct LogGuard {
    #[cfg(feature = "log-file")]
    _guard: Option<tracing_appender::non_blocking::WorkerGuard>,
}

#[cfg(feature = "log-file")]
/// Configuration for daily rolling file logs.
#[derive(Builder, Clone)]
pub struct FileAppenderConfig {
    pub dir: std::path::PathBuf,
    pub prefix: String,
    pub suffix: String,
    #[builder(default = "7")]
    pub max_files: usize,
}

/// Core configuration for the tracing subscriber.
///
/// # Examples
///
/// Basic console logging (requires `tracing` feature):
/// ```no_run
/// use fazuh_common::log::{LogConfigBuilder, install_logging};
///
/// let config = LogConfigBuilder::default()
///     .verbosity(1) // INFO
///     .ansi(true)
///     .build()
///     .unwrap();
///
/// let _guard = install_logging(config);
/// tracing::info!("Hello, world!");
/// ```
///
/// File appending (requires `log-file` feature):
/// ```no_run
/// use std::path::PathBuf;
/// use fazuh_common::log::{LogConfigBuilder, FileAppenderConfigBuilder, install_logging};
///
/// # #[cfg(feature = "log-file")]
/// # fn main() {
/// let file_config = FileAppenderConfigBuilder::default()
///     .dir(PathBuf::from("/var/log/my-app"))
///     .prefix("my-app".to_string())
///     .suffix("log".to_string())
///     .build()
///     .unwrap();
///
/// let config = LogConfigBuilder::default()
///     .verbosity(2) // DEBUG
///     .ansi(true)
///     .file_appender(file_config)
///     .build()
///     .unwrap();
///
/// let _guard = install_logging(config);
/// # }
/// ```
#[derive(Builder, Clone)]
pub struct LogConfig {
    #[builder(setter(into))]
    verbosity: Verbosity,
    ansi: bool,
    #[cfg(feature = "log-file")]
    #[builder(default, setter(into, strip_option))]
    file_appender: Option<FileAppenderConfig>,
}

pub fn install_logging(conf: LogConfig) -> LogGuard {
    // Respect RUST_LOG env var; otherwise derive level from verbosity count.
    let filter = if let Ok(rust_log) = std::env::var("RUST_LOG") {
        tracing_subscriber::EnvFilter::new(rust_log)
    } else {
        tracing_subscriber::EnvFilter::new(conf.verbosity)
    };

    let stdout_layer = tracing_subscriber::fmt::layer().with_ansi(conf.ansi);

    #[cfg(feature = "log-file")]
    {
        let mut _guard = None;
        let file_layer = if let Some(file_conf) = conf.file_appender {
            let file_appender = tracing_appender::rolling::Builder::new()
                .rotation(tracing_appender::rolling::Rotation::DAILY)
                .filename_prefix(&file_conf.prefix)
                .filename_suffix(&file_conf.suffix)
                .max_log_files(file_conf.max_files)
                .build(&file_conf.dir)
                .expect("failed to initialize rolling file appender");

            let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
            _guard = Some(guard);

            Some(
                tracing_subscriber::fmt::layer()
                    .with_writer(non_blocking)
                    .with_ansi(false),
            )
        } else {
            None
        };

        tracing_subscriber::registry()
            .with(filter)
            .with(stdout_layer)
            .with(file_layer)
            .init();

        LogGuard { _guard }
    }

    #[cfg(not(feature = "log-file"))]
    {
        tracing_subscriber::registry()
            .with(filter)
            .with(stdout_layer)
            .init();

        LogGuard {}
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Verbosity {
    inner: Level,
}

impl Verbosity {
    pub fn new(level: impl Into<Level>) -> Self {
        Self {
            inner: level.into(),
        }
    }

    pub fn level(&self) -> Level {
        self.inner
    }
}

impl Deref for Verbosity {
    type Target = Level;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl AsRef<str> for Verbosity {
    fn as_ref(&self) -> &str {
        self.level().as_str()
    }
}

impl From<u8> for Verbosity {
    fn from(value: u8) -> Self {
        let inner = match value.into() {
            0 => Level::WARN,
            1 => Level::INFO,
            2 => Level::DEBUG,
            _ => Level::TRACE,
        };

        Self::new(inner)
    }
}

impl From<Level> for Verbosity {
    fn from(value: Level) -> Self {
        Self::new(value)
    }
}

#[cfg(feature = "log-cli")]
pub mod cli {
    use std::io::IsTerminal;

    use clap::ValueEnum;

    pub fn get_ansi(color: ColorMode) -> bool {
        let mut ret = match color {
            ColorMode::Always => true,
            ColorMode::Never => false,
            ColorMode::Auto => std::io::stderr().is_terminal(),
        };

        // NO_COLOR and CLICOLOR env var overrides (per spec).
        let no_color = std::env::var_os("NO_COLOR").is_some();
        let clicolor = std::env::var("CLICOLOR").ok();
        ret = match (no_color, clicolor.as_deref()) {
            (true, _) | (_, Some("0")) => false,
            _ => ret,
        };

        ret
    }

    /// Controls ANSI color output in logs and error formatting.
    #[derive(Clone, Debug, Default, ValueEnum)]
    pub enum ColorMode {
        #[default]
        Auto,
        Always,
        Never,
    }
}
