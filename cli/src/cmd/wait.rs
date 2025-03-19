use anyhow::Context;
use anyhow::Result;
use camino::Utf8PathBuf;
use clap::Args;
use event_pbar::Event;
use humantime::Duration;
use tracing::info;

use super::Run;

/// Wait for a period of time.
#[derive(Debug, Args)]
pub struct WaitArgs {
    /// Time to wait for.
    #[clap(short, long, default_value = "1h")]
    duration: Duration,

    /// Killfile to watch.
    #[clap(long, default_value = "killfile")]
    killfile: Utf8PathBuf,
}

impl Run for WaitArgs {
    async fn run(&self) -> Result<()> {
        info!(duration = %self.duration, "waiting, will timeout after duration");

        let killfile_path = camino::absolute_utf8(&self.killfile)
            .context("unable to get killfile absolute path")?;
        info!(killfile = %&killfile_path, "registered watch on killfile");

        let event_config = event_pbar::EventConfig::builder()
            .killfile_path(killfile_path)
            .build();

        let duration: tokio::time::Duration = self.duration.into();
        let deadline = tokio::time::Instant::now() + duration;

        let (mut rx, _guards) = event_config.listen_tokio()?;

        loop {
            let _ = tokio::select! {
                Some(event) = rx.recv() => {
                    info!(?event, "received event");
                    match event {
                        Event::Terminate => {
                            info!("termination event received, shutting down");
                            break;
                        },
                        _other => {}
                    };
                }
                _ = tokio::time::sleep_until(deadline) => {
                    info!(duration = %self.duration, "timed out");
                    break;
                }
            };
        }

        Ok(())
    }
}
