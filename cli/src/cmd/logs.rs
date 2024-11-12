use std::hint::black_box;

use anyhow::Result;
use clap::Args;
use tracing::info;
use tracing::instrument;

use super::Run;

/// Log lines demo
#[derive(Debug, Args)]
pub struct LogsArgs {}

impl Run for LogsArgs {
    async fn run(&self) -> Result<()> {
        let foo = "foo";
        let bar = "bar";

        tracing::error!(%foo, ?bar, "message was error");
        tracing::warn!(%foo, ?bar, "message was warn");
        tracing::info!(%foo, ?bar, "message was info");
        tracing::debug!(%foo, ?bar, "message was debug");
        tracing::trace!(%foo, ?bar, "message was trace");

        example();

        Ok(())
    }
}

#[instrument]
fn example() {
    info!("inside span");
    black_box(());
}
