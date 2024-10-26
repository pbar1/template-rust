//! OpenTelemetry support for logs, metrics, and traces.

use anyhow::Result;
use bon::builder;
use bon::Builder;
use opentelemetry::trace::Tracer;
use opentelemetry::trace::TracerProvider as _;
use opentelemetry_sdk::trace::TracerProvider;
use tracing::error;
use tracing::span;
use tracing_opentelemetry::MetricsLayer;
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Layer;
use tracing_subscriber::Registry;

use super::BoxLayer;

#[derive(Debug, Builder)]
pub struct OtelConfig {}

impl OtelConfig {
    pub fn layer(self) -> BoxLayer {
        // let otlp_exporter = opentelemetry_otlp::new_exporter().tonic();

        let provider = TracerProvider::builder()
            .with_simple_exporter(opentelemetry_stdout::SpanExporter::default())
            .build();
        let tracer = provider.tracer("readme_example");

        // Create a tracing layer with the configured tracer
        let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

        Box::new(telemetry)
    }
}
