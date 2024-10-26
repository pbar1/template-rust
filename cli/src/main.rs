#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

mod args;
mod cmd;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    cmd::run().await
}
