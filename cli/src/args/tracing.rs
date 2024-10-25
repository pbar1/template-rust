use anyhow::bail;
use anyhow::Result;
use camino::Utf8PathBuf;
use clap::Args;
use pbar_telemetry::LogFormat;
use pbar_telemetry::TracingConfig;

const HEADING: &str = "Tracing Options";

#[derive(Debug, Args)]
pub struct TracingArgs {
    /// Output file for log lines.
    #[clap(short, long, default_value = "info", env = "RUST_LOG", help_heading = HEADING, global = true)]
    pub log_level: String,

    /// Filter directive for log lines.
    #[clap(long, default_value = "/dev/fd/2", help_heading = HEADING, global = true)]
    pub log_file: Utf8PathBuf,

    // TODO: Enum values
    /// Output format for log lines.
    #[clap(long, default_value = "glog", help_heading = HEADING, global = true)]
    pub log_format: String,
}

impl TracingArgs {
    pub fn init_tracing(self) -> Result<()> {
        let log_format = match self.log_format.as_str() {
            "glog" => LogFormat::Glog,
            "json" => LogFormat::Json,
            unknown => bail!("unsupported log format: {unknown}"),
        };

        TracingConfig::builder()
            .log_level(self.log_level)
            .log_file(self.log_file)
            .log_format(log_format)
            .build()
            .init()
    }
}
