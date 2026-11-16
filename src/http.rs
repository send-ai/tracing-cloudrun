use http::header::HeaderMap;
use opentelemetry_http::HeaderInjector;

// Returns headers which could be used for tracing context propagation
pub fn tracing_headers() -> HeaderMap {
    use tracing_opentelemetry::OpenTelemetrySpanExt;

    let span = tracing::Span::current();
    let ctx = span.context();
    let mut headers = HeaderMap::new();
    opentelemetry::global::get_text_map_propagator(|propagator| {
        propagator.inject_context(&ctx, &mut HeaderInjector(&mut headers))
    });
    headers
}
