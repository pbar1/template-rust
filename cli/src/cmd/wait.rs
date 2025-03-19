use anyhow::Context;
use anyhow::Result;
use camino::Utf8PathBuf;
use clap::Args;
use humantime::Duration;
use tracing::error;
use tracing::info;

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

        let killfile_path =
            camino::absolute_utf8("killfile").context("unable to get killfile absolute path")?;

        let event_config = event_pbar::EventConfig::builder()
            .killfile_path(killfile_path)
            .build();

        let (rx, _guards) = event_config.listen()?;

        while let res = rx.recv() {
            match res {
                Ok(event) => info!(?event, "received event"),
                // Err(error) => error!(?error, "receive error"),
                Err(error) => {}
            }
            tokio::task::yield_now().await;
        }

        Ok(())
    }
}
