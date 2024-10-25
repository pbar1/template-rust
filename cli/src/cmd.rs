//! CLI command structure

mod mercury;

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
    Mercury(mercury::MercuryArgs),
}

/// Entrypoint into the CLI, to be called by [`crate::main`].
pub async fn run() -> Result<()> {
    let cli = Cli::parse();

    cli.tracing.init()?;

    cli.subcommand.run().await
}
