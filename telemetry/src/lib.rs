#![warn(clippy::pedantic)]

mod lines;

use std::fs::OpenOptions;

use anyhow::Result;
use bon::builder;
use bon::Builder;
use camino::Utf8PathBuf;
use tracing_log::LogTracer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::EnvFilter;

/// Opinionated all-in-one config for [`tracing`].
#[derive(Debug, Clone, Builder)]
pub struct TracingConfig {
    /// Output file for log lines. Default is **stderr**.
    #[builder(default = Utf8PathBuf::from("/dev/fd/2"))]
    log_file: Utf8PathBuf,

    /// Filter directive for log lines. Default is **info**.
    #[builder(default = String::from("info"))]
    log_level: String,

    /// Output format for log lines. Default is **glog**.
    #[builder(default = LogFormat::Glog)]
    log_format: LogFormat,
}

/// Supported output formats for log lines.
#[derive(Debug, Clone, Copy)]
pub enum LogFormat {
    /// [Glog format](https://docs.rs/tracing-glog/latest/tracing_glog).
    Glog,
    /// [JSON format](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/fmt/format/struct.Json.html).
    Json,
}

impl Default for TracingConfig {
    fn default() -> Self {
        Self::builder().build()
    }
}

impl TracingConfig {
    /// Initializes a global subscriber and all configured layers.
    ///
    /// # Errors
    ///
    /// - On failure to setup `tracing_log`
    /// - On failure opening log lines file
    /// - On failure setting global default subscriber
    pub fn init(&self) -> Result<()> {
        LogTracer::init()?;

        let lines_filter = EnvFilter::builder().parse_lossy(&self.log_level);
        let lines_writer = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.log_file)?;
        let lines_layer = match self.log_format {
            LogFormat::Glog => lines::glog_layer(lines_filter, lines_writer),
            LogFormat::Json => lines::json_layer(lines_filter, lines_writer),
        };

        let subscriber = tracing_subscriber::registry().with(lines_layer);
        tracing::subscriber::set_global_default(subscriber)?;

        Ok(())
    }
}
