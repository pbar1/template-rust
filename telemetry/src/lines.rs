use std::fs::File;
use std::io::IsTerminal;

use tracing_glog::Glog;
use tracing_glog::GlogFields;
use tracing_glog::LocalTime;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::Layer;
use tracing_subscriber::Registry;

// FIXME: Take `MakeWriter` instead of `File` for arbitrary writers
/// Log lines layer in Glog format with local time. Enables color if the
/// destination is a terminal.
pub fn glog_layer(filter: EnvFilter, file: File) -> Box<dyn Layer<Registry> + Send + Sync> {
    let color = file.is_terminal();

    tracing_subscriber::fmt::layer()
        .event_format(Glog::default().with_timer(LocalTime::default()))
        .fmt_fields(GlogFields::default())
        .with_ansi(color)
        .with_writer(file)
        .with_filter(filter)
        .boxed()
}

/// Log lines layer in JSON format.
pub fn json_layer(filter: EnvFilter, file: File) -> Box<dyn Layer<Registry> + Send + Sync> {
    tracing_subscriber::fmt::layer()
        .json()
        .with_writer(file)
        .with_filter(filter)
        .boxed()
}
