//! Telemetry: tracing spans, metrics, and structured events for client calls.
//!
//! This is the transport-neutral instrumentation the client crates wrap their
//! RPCs with (Option B: telemetry lives in `canton-core`). Every instrumented
//! call opens a `canton.rpc` [`tracing`] span, emits request/error counters via
//! the [`metrics`] facade, and logs a structured success/error event.
//!
//! **Exporting.** Following the standard Rust telemetry model, this crate only
//! *emits* — the application chooses the exporters by installing a
//! `tracing_subscriber` (for logs/spans) and a [`metrics`] recorder. Metrics
//! carry `method` + `transport` labels, so any recorder (Prometheus, an OTLP
//! bridge, …) gets the per-endpoint request/error breakdown for free (success
//! = requests − errors). To ship spans to an OTLP collector (Jaeger/Tempo),
//! enable the `otel` feature: `otel::otlp_tracer` builds the span exporter, and
//! trace context is injected into every outgoing gRPC/JSON request
//! automatically (see the `otel` module's `inject_trace_context`).

use std::future::Future;

use tracing::Instrument;

use crate::Result;

/// Counter: total client requests, labelled by `method` and `transport`.
pub const METRIC_REQUESTS: &str = "canton_client_requests_total";
/// Counter: client errors, labelled by `method`, `transport`, and `retriable`.
pub const METRIC_ERRORS: &str = "canton_client_errors_total";

/// `transport` label / span-field value for the gRPC lane.
pub const TRANSPORT_GRPC: &str = "grpc";
/// `transport` label / span-field value for the JSON lane.
pub const TRANSPORT_JSON: &str = "json";

/// Instrument a client RPC future: open a `canton.rpc` span, count the request
/// (and any error), and log a structured outcome event.
///
/// `method` and `transport` become both span fields and metric labels. The
/// future is polled inside the span, so any spans/events the RPC itself emits
/// nest correctly and inherit the trace context.
pub async fn instrument<T, F>(method: &'static str, transport: &'static str, fut: F) -> Result<T>
where
    F: Future<Output = Result<T>>,
{
    metrics::counter!(METRIC_REQUESTS, "method" => method, "transport" => transport).increment(1);

    let span = tracing::info_span!("canton.rpc", method = method, transport = transport);
    async move {
        let result = fut.await;
        match &result {
            Ok(_) => tracing::debug!(method, transport, "rpc completed"),
            Err(error) => {
                let retriable = error.is_retriable();
                metrics::counter!(
                    METRIC_ERRORS,
                    "method" => method,
                    "transport" => transport,
                    "retriable" => retriable.to_string(),
                )
                .increment(1);
                tracing::warn!(method, transport, retriable, error = %error, "rpc failed");
            }
        }
        result
    }
    .instrument(span)
    .await
}

/// OpenTelemetry export helpers (enable the `otel` feature).
///
/// The SDK emits `tracing` spans unconditionally; this module bridges them to
/// an OTLP collector and propagates W3C trace context into outgoing requests.
#[cfg(feature = "otel")]
pub mod otel {
    use opentelemetry::propagation::TextMapPropagator as _;
    use opentelemetry::trace::TracerProvider as _;
    use opentelemetry_otlp::WithExportConfig as _;
    use opentelemetry_sdk::propagation::TraceContextPropagator;

    /// Build an OTLP-exporting tracer named `service_name`, batch-sending spans
    /// to the gRPC OTLP `endpoint` (e.g. `http://localhost:4317`). Compose the
    /// returned tracer into a `tracing` subscriber with
    /// `tracing_opentelemetry::layer().with_tracer(tracer)`.
    ///
    /// # Errors
    /// Returns a [`opentelemetry::trace::TraceError`] if the exporter cannot be
    /// built (e.g. an invalid endpoint).
    pub fn otlp_tracer(
        service_name: &'static str,
        endpoint: impl Into<String>,
    ) -> Result<opentelemetry_sdk::trace::Tracer, opentelemetry::trace::TraceError> {
        let exporter = opentelemetry_otlp::SpanExporter::builder()
            .with_tonic()
            .with_endpoint(endpoint.into())
            .build()?;
        let provider = opentelemetry_sdk::trace::TracerProvider::builder()
            .with_batch_exporter(exporter, opentelemetry_sdk::runtime::Tokio)
            .build();
        Ok(provider.tracer(service_name))
    }

    /// The W3C trace-context headers (`traceparent` / `tracestate`) for the
    /// current span, or empty when no valid OpenTelemetry context is active
    /// (i.e. no `tracing_opentelemetry` layer installed, or an unsampled span).
    fn trace_context_carrier() -> std::collections::HashMap<String, String> {
        use opentelemetry::trace::TraceContextExt as _;
        use tracing_opentelemetry::OpenTelemetrySpanExt as _;

        let context = tracing::Span::current().context();
        let mut carrier = std::collections::HashMap::new();
        if context.span().span_context().is_valid() {
            TraceContextPropagator::new().inject_context(&context, &mut carrier);
        }
        carrier
    }

    /// Inject the current span's W3C trace context into an outgoing HTTP header
    /// map (the JSON transport), so the participant can correlate the request.
    pub fn inject_trace_context(headers: &mut http::HeaderMap) {
        for (key, value) in trace_context_carrier() {
            if let (Ok(name), Ok(val)) = (
                http::header::HeaderName::try_from(key),
                http::HeaderValue::from_str(&value),
            ) {
                headers.insert(name, val);
            }
        }
    }

    /// Inject the current span's W3C trace context into outgoing gRPC request
    /// metadata, so the participant can correlate the request.
    pub fn inject_trace_context_metadata(metadata: &mut tonic::metadata::MetadataMap) {
        for (key, value) in trace_context_carrier() {
            if let (Ok(name), Ok(val)) = (
                tonic::metadata::MetadataKey::from_bytes(key.as_bytes()),
                tonic::metadata::MetadataValue::try_from(value),
            ) {
                metadata.insert(name, val);
            }
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use crate::Error;
    use std::sync::{Arc, Mutex};
    use tracing::subscriber::set_default;
    use tracing_subscriber::Layer;
    use tracing_subscriber::layer::{Context, SubscriberExt};
    use tracing_subscriber::registry::LookupSpan;

    /// A tiny tracing layer that records the names of spans it sees created.
    #[derive(Clone, Default)]
    struct SpanCapture(Arc<Mutex<Vec<String>>>);

    impl<S> Layer<S> for SpanCapture
    where
        S: tracing::Subscriber + for<'a> LookupSpan<'a>,
    {
        fn on_new_span(
            &self,
            attrs: &tracing::span::Attributes<'_>,
            _id: &tracing::span::Id,
            _ctx: Context<'_, S>,
        ) {
            self.0
                .lock()
                .unwrap()
                .push(attrs.metadata().name().to_string());
        }
    }

    #[tokio::test]
    async fn instrument_emits_span_and_metrics() {
        // Global metrics recorder (installed once for this test binary).
        let recorder = metrics_util::debugging::DebuggingRecorder::new();
        let snapshotter = recorder.snapshotter();
        recorder.install().expect("install metrics recorder");

        // Capture tracing spans on this (current-thread) test runtime.
        let captured = SpanCapture::default();
        let subscriber = tracing_subscriber::registry().with(captured.clone());
        let _guard = set_default(subscriber);

        // One success, one (non-retriable) failure.
        let ok: Result<u8> = instrument("version", TRANSPORT_GRPC, async { Ok(1) }).await;
        assert_eq!(ok.unwrap(), 1);
        let err: Result<u8> = instrument("ledger_end", TRANSPORT_GRPC, async {
            Err(Error::InvalidRequest("boom".into()))
        })
        .await;
        assert!(err.is_err());

        // A `canton.rpc` span was opened for each call.
        let spans = captured.0.lock().unwrap();
        assert!(
            spans.iter().filter(|n| *n == "canton.rpc").count() >= 2,
            "expected canton.rpc spans, saw {spans:?}"
        );

        // Metrics: 2 requests, 1 error.
        let snapshot = snapshotter.snapshot().into_vec();
        let counter_total = |name: &str| -> u64 {
            snapshot
                .iter()
                .filter(|(key, _, _, _)| key.key().name() == name)
                .filter_map(|(_, _, _, value)| match value {
                    metrics_util::debugging::DebugValue::Counter(c) => Some(*c),
                    _ => None,
                })
                .sum()
        };
        assert_eq!(counter_total(METRIC_REQUESTS), 2, "two requests counted");
        assert_eq!(counter_total(METRIC_ERRORS), 1, "one error counted");
    }

    /// With no active OTel span context, injection is a no-op (nothing to
    /// propagate) and must never panic.
    #[cfg(feature = "otel")]
    #[test]
    fn inject_trace_context_is_a_noop_without_a_context() {
        let mut headers = http::HeaderMap::new();
        super::otel::inject_trace_context(&mut headers);
        assert!(
            headers.is_empty(),
            "no trace context should be injected outside a span, saw {headers:?}"
        );
    }

    /// Under an installed OTel tracer, an active span's W3C trace context is
    /// injected into both HTTP headers (JSON) and gRPC metadata.
    #[cfg(feature = "otel")]
    #[test]
    fn trace_context_is_injected_under_a_tracer() {
        use opentelemetry::trace::TracerProvider as _;

        let provider = opentelemetry_sdk::trace::TracerProvider::builder().build();
        let otel_layer = tracing_opentelemetry::layer().with_tracer(provider.tracer("test"));
        let subscriber = tracing_subscriber::registry().with(otel_layer);
        let _guard = set_default(subscriber);

        let span = tracing::info_span!("test.rpc");
        let _entered = span.enter();

        let mut headers = http::HeaderMap::new();
        super::otel::inject_trace_context(&mut headers);
        assert!(
            headers.contains_key("traceparent"),
            "expected a W3C traceparent header, saw {headers:?}"
        );

        let mut metadata = tonic::metadata::MetadataMap::new();
        super::otel::inject_trace_context_metadata(&mut metadata);
        assert!(
            metadata.get("traceparent").is_some(),
            "expected traceparent in gRPC metadata"
        );
    }
}
