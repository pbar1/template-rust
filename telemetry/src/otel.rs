//! OpenTelemetry support for logs, metrics, and traces.

use anyhow::Result;
use bon::Builder;
use opentelemetry::KeyValue;
use opentelemetry_sdk::runtime::Tokio;
use opentelemetry_sdk::Resource;
use tracing_subscriber::Layer;

use super::BoxLayer;

#[derive(Debug, Builder)]
pub struct OtelConfig {
    service_name: String,
}

impl OtelConfig {
    pub fn layer(self) -> Result<BoxLayer> {
        // TODO: See if protocol can be selected by
        // OTEL_EXPORTER_OTLP_PROTOCOL=http/protobuf
        let exporter = opentelemetry_otlp::new_exporter().http();

        let meter = opentelemetry_otlp::new_pipeline()
            .metrics(Tokio)
            .with_exporter(exporter)
            .with_resource(Resource::new(vec![KeyValue::new(
                "service.name",
                self.service_name,
            )]))
            .build()?;

        let layer = tracing_opentelemetry::MetricsLayer::new(meter).boxed();

        Ok(layer)
    }
}
