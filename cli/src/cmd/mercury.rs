use anyhow::Result;
use clap::Args;

use super::Run;

/// First planet
#[derive(Debug, Args)]
pub struct MercuryArgs {
    /// Name of the planet
    #[clap(long, default_value = "Mercury")]
    name: String,
}

impl Run for MercuryArgs {
    async fn run(&self) -> Result<()> {
        println!("The first planet is: {}", self.name);
        Ok(())
    }
}
