use http::Request;
use http::header::{HeaderMap, HeaderValue};
use opentelemetry;
use opentelemetry_http::HeaderExtractor;
use tower::{ServiceBuilder, layer::util::Stack};
use tower_http::classify::ServerErrorsAsFailures;
use tower_http::classify::SharedClassifier;
use tower_http::trace::TraceLayer;

const TRACEPARENT_HEADER: &str = "traceparent";
// for now we don't use tracestate
//const TRACESTATE_HEADER: &str = "tracestate";

#[derive(Debug, Clone)]
pub struct TraceparentMakeSpan;

// in your router you can use `.layer(crate::tracing::build_trace_layer())`
// it will be capturing the traceparent and will be visible in tracing-stackdriver
pub fn traceparent_span_layer() -> TraceServiceBuilder {
    ServiceBuilder::new().layer(TraceLayer::new_for_http().make_span_with(TraceparentMakeSpan))
}

// private

type TraceServiceBuilder = ServiceBuilder<
    Stack<
        TraceLayer<SharedClassifier<ServerErrorsAsFailures>, TraceparentMakeSpan>,
        tower::layer::util::Identity,
    >,
>;

// Utility function to extract the context from the incoming request headers
fn extract_context_from_headers(headers: &HeaderMap) -> opentelemetry::Context {
    opentelemetry::global::get_text_map_propagator(|propagator| {
        propagator.extract(&HeaderExtractor(headers))
    })
}

pub fn inject_context(traceparent: &str) {
    let value = match HeaderValue::from_str(traceparent) {
        Ok(value) => value,
        _ => {
            tracing::warn!("pubsub traceparent header not set");
            return;
        }
    };
    let span = tracing::Span::current();
    let mut headers = HeaderMap::new();
    headers.insert(TRACEPARENT_HEADER, value);
    set_span_parent(&span, &headers);
}

fn set_span_parent(span: &tracing::Span, headers: &HeaderMap) {
    use tracing_opentelemetry::OpenTelemetrySpanExt;
    if let Err(e) = span.set_parent(extract_context_from_headers(headers)) {
        tracing::warn!("tracing_opentelemetry error: {e}")
    }
}

impl<B> tower_http::trace::MakeSpan<B> for TraceparentMakeSpan {
    fn make_span(&mut self, request: &Request<B>) -> tracing::Span {
        let span = tracing::span!(tracing::Level::INFO, "request");
        set_span_parent(&span, request.headers());
        span
    }
}
