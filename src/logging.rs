use opentelemetry;
use opentelemetry::trace::TracerProvider;
use opentelemetry_sdk::propagation::TraceContextPropagator;
use tracing_stackdriver::CloudTraceConfiguration;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::prelude::*;

#[derive(Debug, Clone)]
pub struct TraceparentMakeSpan;

pub async fn init(env_filter: &str, structured: bool) {
    let project_id = google_cloud_metadata::project_id().await;
    init_logging(env_filter, structured, project_id)
}

// private

fn init_tracer() -> opentelemetry_sdk::trace::Tracer {
    opentelemetry::global::set_text_map_propagator(TraceContextPropagator::new());

    let provider = opentelemetry_sdk::trace::SdkTracerProvider::builder()
        // to see human-readable opentelemetry spans:
        // uncomment with_simple_exporter
        // and you might need to 'otel::tracing=trace,otel=debug' to filter (see below)
        //
        //.with_simple_exporter(opentelemetry_stdout::SpanExporter::default())
        .build();

    opentelemetry::global::set_tracer_provider(provider.clone());
    provider.tracer("app")
}

fn init_logging(env_filter: &str, structured: bool, project_id: String) {
    if !structured {
        return init_dev_logging(env_filter);
    }

    let filter = EnvFilter::try_new(env_filter).expect("valid logging filter");

    let otel_tracer = init_tracer();
    let otel_layer = tracing_opentelemetry::layer().with_tracer(otel_tracer);

    let stackdriver = tracing_stackdriver::layer()
        .with_source_location(false)
        .with_cloud_trace(CloudTraceConfiguration { project_id });

    tracing_subscriber::registry()
        .with(filter)
        .with(otel_layer)
        .with(stackdriver)
        .init();
}

fn init_dev_logging(env_filter: &str) {
    // make sure http request is logged in development
    let env_filter = env_filter.to_owned() + ",tower_http=trace";
    tracing_subscriber::fmt()
        .with_span_events(FmtSpan::CLOSE)
        .with_env_filter(env_filter)
        .pretty()
        .init();
}
