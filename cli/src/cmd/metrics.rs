use anyhow::Result;
use clap::Args;

use super::Run;

/// Metrics demo
#[derive(Debug, Args)]
pub struct MetricsArgs {}

impl Run for MetricsArgs {
    async fn run(&self) -> Result<()> {
        tracing::info!(monotonic_counter.foo = 1);
        tracing::info!(monotonic_counter.bar = 1.1);

        tracing::info!(counter.baz = 1);
        tracing::info!(counter.baz = -1);
        tracing::info!(counter.xyz = 1.1);

        tracing::info!(histogram.qux = 1);
        tracing::info!(histogram.abc = -1);
        tracing::info!(histogram.def = 1.1);

        Ok(())
    }
}
