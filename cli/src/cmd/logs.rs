use anyhow::Result;
use clap::Args;

use super::Run;

/// Log lines demo
#[derive(Debug, Args)]
pub struct LogsArgs {}

impl Run for LogsArgs {
    async fn run(&self) -> Result<()> {
        tracing::error!("error");
        tracing::warn!("warn");
        tracing::info!("info");
        tracing::debug!("debug");
        tracing::trace!("trace");

        Ok(())
    }
}
