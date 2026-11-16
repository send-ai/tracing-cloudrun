# tracing-cloudrun

Logging and request tracing for Rust services on Google Cloud Run.

On Cloud Run, logs go out as structured JSON that Cloud Logging understands, and each log line is tied to its request in Cloud Trace. Locally, logs print in a readable format instead.

## Usage

Add the crate as a git dependency:

```toml
tracing-cloudrun = { git = "https://github.com/send-ai/tracing-cloudrun.git", tag = "v0.4.0" }
```

Set up logging once at startup. The second argument turns structured logging on, so release builds log JSON and debug builds log plain text:

```rust
tracing_cloudrun::logging::init("app=info", !cfg!(debug_assertions)).await;
```

Add the trace layer to the router, so each incoming request continues the trace from its `traceparent` header:

```rust
let app = Router::new().layer(tracing_cloudrun::routing::traceparent_span_layer());
```

Pass the trace on to the services you call by adding its headers to outgoing requests:

```rust
let response = client
    .get(url)
    .headers(tracing_cloudrun::http::tracing_headers())
    .send()
    .await?;
```

For a Pub/Sub push message, `tracing_cloudrun::routing::inject_context(traceparent)` continues the trace from the message's `traceparent` attribute.
