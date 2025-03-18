use anyhow::Result;
use clap::Args;
use humantime::Duration;

use super::Run;

/// Wait for a period of time.
#[derive(Debug, Args)]
pub struct WaitArgs {
    /// Time to wait for.
    #[clap(short, long, default_value = "3600s")]
    duration: Duration,
}

impl Run for WaitArgs {
    async fn run(&self) -> Result<()> {
        tracing::info!("waiting forever - send a signal to exit");

        tokio::time::sleep(self.duration.into()).await;

        Ok(())
    }
}
