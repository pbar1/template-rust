#![doc = include_str!("../README.md")]
#![warn(clippy::pedantic)]

#[cfg(feature = "console")]
pub mod console;
#[cfg(feature = "lines")]
pub mod lines;
#[cfg(feature = "otel")]
pub mod otel;

use std::fs::OpenOptions;

use anyhow::Result;
use bon::builder;
use bon::Builder;
use camino::Utf8PathBuf;
use tracing_log::LogTracer;
use tracing_subscriber::fmt::writer::BoxMakeWriter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::Layer;
use tracing_subscriber::Registry;

/// Type-erased `Layer` for ease of construction.
pub type BoxLayer = Box<dyn Layer<Registry> + Send + Sync>;

/// All-in-one config for `tracing` layers.
#[derive(Debug, Clone, Builder)]
pub struct TracingConfig {
    #[cfg(feature = "lines")]
    /// Output file for log lines. Default is **stderr**.
    #[builder(default = Utf8PathBuf::from("/dev/fd/2"))]
    log_file: Utf8PathBuf,

    #[cfg(feature = "lines")]
    /// Filter directive for log lines. Default is **info**.
    #[builder(default = String::from("info"))]
    log_level: String,

    #[cfg(feature = "lines")]
    /// Output format for log lines. Default is **glog**.
    #[builder(default = lines::LinesFormat::Glog)]
    log_format: lines::LinesFormat,

    #[cfg(feature = "otel")]
    /// Service name for OpenTelemetry. Default is **template-rust**.
    #[builder(default = String::from("template-rust"))]
    service_name: String,
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
    /// - (If enabled) On failure opening log lines file
    /// - (If enabled) On failure to setup OpenTelemetry layer
    /// - On failure setting global default subscriber
    pub fn init(&self) -> Result<()> {
        // TODO: Is this necessary or does tracing-subscriber already do this?
        LogTracer::init()?;

        // `Registry::with` can take a Vec, easing dynamic construction
        let mut layers = Vec::new();

        #[cfg(feature = "lines")]
        {
            let filter = EnvFilter::builder().parse_lossy(&self.log_level);
            let writer = OpenOptions::new()
                .create(true)
                .append(true)
                .open(&self.log_file)?;
            let layer = lines::LinesConfig::builder()
                .writer(BoxMakeWriter::new(writer))
                .filter(filter)
                .format(self.log_format)
                .build()
                .layer();
            layers.push(layer);
        }

        #[cfg(feature = "otel")]
        {
            let layer = otel::OtelConfig::builder()
                .service_name(self.service_name.to_owned())
                .build()
                .layer()?;
            layers.push(layer);
        }

        #[cfg(feature = "console")]
        {
            let layer = console::ConsoleConfig::builder().build().layer();
            layers.push(layer);
        }

        let subscriber = tracing_subscriber::registry().with(layers);
        tracing::subscriber::set_global_default(subscriber)?;

        Ok(())
    }
}
