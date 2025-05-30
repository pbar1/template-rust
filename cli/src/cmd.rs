//! CLI command structure

mod logs;
mod metrics;
mod wait;

use anyhow::Result;
use clap::Parser;
use enum_dispatch::enum_dispatch;

/// Template for a Rust CLI
#[derive(Debug, Parser)]
#[clap(version, about)]
struct Cli {
    #[clap(subcommand)]
    subcommand: Subcommand,

    #[clap(flatten)]
    tracing: crate::args::TracingArgs,
}

/// Subcommands must implement [`Run`] to be executed at runtime
#[enum_dispatch]
pub trait Run {
    async fn run(&self) -> Result<()>;
}

#[enum_dispatch(Run)]
#[derive(Debug, clap::Subcommand)]
enum Subcommand {
    Logs(logs::LogsArgs),
    Metrics(metrics::MetricsArgs),
    Wait(wait::WaitArgs),
}

/// Entrypoint into the CLI, to be called by [`crate::main`].
pub async fn run() -> Result<()> {
    let cli = Cli::parse();

    let _guard = cli.tracing.init()?;

    cli.subcommand.run().await?;

    Ok(())
}
