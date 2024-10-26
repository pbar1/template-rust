//! OpenTelemetry support for logs, metrics, and traces.

use anyhow::Result;
use bon::Builder;
use opentelemetry_sdk::metrics::reader::DefaultTemporalitySelector;
use opentelemetry_sdk::metrics::PeriodicReader;
use opentelemetry_sdk::metrics::SdkMeterProvider;
use opentelemetry_sdk::runtime::Tokio;
use tracing_subscriber::Layer;

use super::BoxLayer;

#[derive(Debug, Builder)]
pub struct OtelConfig {}

impl OtelConfig {
    pub fn layer(self) -> Result<BoxLayer> {
        let temporality = Box::new(DefaultTemporalitySelector::new());

        let exporter = opentelemetry_otlp::new_exporter()
            .tonic()
            .build_metrics_exporter(temporality)?;

        let reader = PeriodicReader::builder(exporter, Tokio).build();

        let meter = SdkMeterProvider::builder().with_reader(reader).build();

        let layer = tracing_opentelemetry::MetricsLayer::new(meter).boxed();

        Ok(layer)
    }
}
