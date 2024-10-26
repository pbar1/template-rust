//! OpenTelemetry support for logs, metrics, and traces.

use anyhow::Result;
use bon::builder;
use bon::Builder;
use opentelemetry::KeyValue;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::runtime::Tokio;
use opentelemetry_sdk::Resource;
use tracing_subscriber::Layer;

use super::BoxLayer;

#[derive(Debug, Builder)]
pub struct OtelConfig {
    #[builder(default = "http://localhost:4317".into())]
    otlp_endpoint: String,

    service_name: String,
}

impl OtelConfig {
    pub fn layer(self) -> Result<BoxLayer> {
        let exporter = opentelemetry_otlp::new_exporter()
            .tonic()
            .with_endpoint(self.otlp_endpoint);

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
