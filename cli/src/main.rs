use anyhow::Result;
use pbar_telemetry::TracingConfig;

fn main() -> Result<()> {
    TracingConfig::builder().build().init()?;

    tracing::info!(foo = "foo", "this");
    tracing::error!(bar = "bar", "that");
    tracing::debug!(baz = "baz", "thence");

    Ok(())
}
