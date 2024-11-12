//! `tokio-console` support.

use bon::Builder;
use tracing_subscriber::Layer;

use super::BoxLayer;

/// Config for a `tokio-console` layer. Will respect the `TOKIO_CONSOLE_*`
/// environment variables.
///
/// **WARNING:** Default retention period is **1h**. This can result in very
/// high memory usage under high load.
#[derive(Debug, Builder)]
pub struct ConsoleConfig {}

impl ConsoleConfig {
    pub fn layer(self) -> BoxLayer {
        console_subscriber::Builder::default()
            .with_default_env()
            .spawn()
            .boxed()
    }
}
