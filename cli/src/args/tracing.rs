use anyhow::Result;
use camino::Utf8PathBuf;
use clap::Args;
use pbar_telemetry::lines::LinesFormat;
use pbar_telemetry::TracingConfig;

const HEADING: &str = "Tracing Options";

#[derive(Debug, Args)]
pub struct TracingArgs {
    // TODO: Reloadable on SIGUSR1
    /// Filter directive for log lines.
    #[clap(short, long, default_value = "info", env = "RUST_LOG", help_heading = HEADING, global = true)]
    pub log_level: String,

    // TODO: Anstream handling
    /// Output file for log lines.
    #[clap(long, default_value = "/dev/fd/2", help_heading = HEADING, global = true)]
    pub log_file: Utf8PathBuf,

    // TODO: Variants in help
    /// Output format for log lines.
    #[clap(long, default_value = "glog", help_heading = HEADING, global = true)]
    pub log_format: LinesFormat,
}

impl TracingArgs {
    pub fn init(self) -> Result<()> {
        TracingConfig::builder()
            .log_level(self.log_level)
            .log_file(self.log_file)
            .log_format(self.log_format)
            .build()
            .init()
    }
}
