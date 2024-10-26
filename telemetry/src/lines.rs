use bon::builder;
use bon::Builder;
use strum::EnumString;
use tracing_glog::Glog;
use tracing_glog::GlogFields;
use tracing_glog::LocalTime;
use tracing_subscriber::fmt::writer::BoxMakeWriter;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::Layer;
use tracing_subscriber::Registry;

type BoxLayer = Box<dyn Layer<Registry> + Send + Sync>;

#[derive(Debug, Clone, Copy, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum LinesFormat {
    Glog,
    Json,
}

#[derive(Debug, Builder)]
pub struct LinesConfig {
    writer: BoxMakeWriter,
    filter: EnvFilter,
    #[builder(default = LinesFormat::Glog)]
    format: LinesFormat,
}

impl LinesConfig {
    pub fn layer(self) -> BoxLayer {
        match self.format {
            LinesFormat::Glog => self.glog_layer(),
            LinesFormat::Json => self.json_layer(),
        }
    }

    fn glog_layer(self) -> BoxLayer {
        tracing_subscriber::fmt::layer()
            .event_format(Glog::default().with_timer(LocalTime::default()))
            .fmt_fields(GlogFields::default())
            .with_writer(self.writer)
            .with_filter(self.filter)
            .boxed()
    }

    fn json_layer(self) -> BoxLayer {
        tracing_subscriber::fmt::layer()
            .json()
            .with_writer(self.writer)
            .with_filter(self.filter)
            .boxed()
    }
}
