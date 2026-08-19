//! Telemetry: tracing spans, metrics, and structured events for client calls.
//!
//! This is the transport-neutral instrumentation the client crates wrap their
//! RPCs with (Option B: telemetry lives in `canton-core`). Every instrumented
//! call opens a `canton.rpc` [`tracing`] span, emits request/error counters via
//! the [`metrics`] facade, and logs a structured success/error event.
//!
//! **Exporting.** Following the standard Rust telemetry model, this crate
//! *emits* and the application chooses the exporters: a `tracing_subscriber`
//! for logs and spans, a [`metrics`] recorder for counters. Metrics carry
//! `method` + `transport` labels, so any recorder (Prometheus, an OTLP bridge,
//! …) gets the per-endpoint request/error breakdown for free (success =
//! requests − errors).
//!
//! The `otel` feature supplies a supported OpenTelemetry path for both halves
//! rather than leaving it as an exercise: `otel::otlp_tracer` builds the span
//! exporter, `otel::otlp_metrics` builds the metrics pipeline **and** installs
//! the recorder that bridges this crate's counters onto it, and trace context
//! is injected into every outgoing request — gRPC metadata, JSON headers, and
//! the WebSocket upgrade — automatically.
//!
//! **Streams.** Use [`instrument`] for a call that returns a value and
//! [`instrument_stream`] for one that returns a stream. The difference is not
//! cosmetic: a subscription's outcome is not known when it opens, so
//! instrumenting only the opening records every long-lived stream as a success
//! and never revisits it.

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
        // Recorded on the event rather than left to the subscriber: correlating
        // a log line with its trace otherwise needs subscriber plumbing the
        // application has to write, and the SDK is the thing that knows.
        let trace_id = current_trace_id().unwrap_or_default();
        match &result {
            Ok(_) => tracing::debug!(method, transport, trace_id, "rpc completed"),
            Err(error) => {
                let retriable = error.is_retriable();
                metrics::counter!(
                    METRIC_ERRORS,
                    "method" => method,
                    "transport" => transport,
                    "retriable" => retriable.to_string(),
                )
                .increment(1);
                tracing::warn!(
                    method,
                    transport,
                    retriable,
                    trace_id,
                    error = %error,
                    "rpc failed",
                );
            }
        }
        result
    }
    .instrument(span)
    .await
}

/// Instrument a client *stream*: the same span, counters and events as
/// [`instrument`], but for the life of the stream rather than the moment it
/// opens.
///
/// Opening a stream and consuming it are different events, and only the first
/// one is a future. A subscription that opens cleanly and fails ten minutes
/// later — the participant restarting, the connection dropping — was recorded
/// as a success and never corrected, so the error counter said nothing about
/// the failure mode long-lived clients actually hit.
///
/// Each item is polled inside the span, so whatever the transport logs while
/// producing it is attributed correctly. Errors increment
/// [`METRIC_ERRORS`] as they arrive; the stream ending is logged with the
/// number of items it delivered.
pub fn instrument_stream<T, S>(
    method: &'static str,
    transport: &'static str,
    stream: S,
) -> impl futures_core::Stream<Item = Result<T>> + Send
where
    S: futures_core::Stream<Item = Result<T>> + Send,
    T: Send,
{
    use tokio_stream::StreamExt as _;

    let span = tracing::info_span!("canton.stream", method = method, transport = transport);
    async_stream::stream! {
        tokio::pin!(stream);
        let mut items = 0u64;
        loop {
            // The span wraps the poll rather than being held across it: a
            // guard kept over an await attributes whatever else the task runs
            // to this stream.
            let next = stream.next().instrument(span.clone()).await;
            match next {
                Some(Ok(item)) => {
                    items += 1;
                    yield Ok(item);
                }
                Some(Err(error)) => {
                    let retriable = error.is_retriable();
                    metrics::counter!(
                        METRIC_ERRORS,
                        "method" => method,
                        "transport" => transport,
                        "retriable" => retriable.to_string(),
                    )
                    .increment(1);
                    span.in_scope(|| {
                        tracing::warn!(
                            method,
                            transport,
                            retriable,
                            items,
                            trace_id = current_trace_id().unwrap_or_default(),
                            error = %error,
                            "stream failed",
                        );
                    });
                    yield Err(error);
                }
                None => {
                    span.in_scope(|| {
                        tracing::debug!(
                            method,
                            transport,
                            items,
                            trace_id = current_trace_id().unwrap_or_default(),
                            "stream ended",
                        );
                    });
                    return;
                }
            }
        }
    }
}

/// The current span's trace id as 32 hex characters, when an OpenTelemetry
/// context is active.
///
/// Structured events carry this so a log line can be joined to the trace it
/// belongs to. Without the `otel` feature — or with no subscriber bridging to
/// OpenTelemetry — there is no trace to name, and this is `None`.
#[must_use]
pub fn current_trace_id() -> Option<String> {
    #[cfg(feature = "otel")]
    {
        otel::current_trace_id()
    }
    #[cfg(not(feature = "otel"))]
    {
        None
    }
}

/// OpenTelemetry export helpers (enable the `otel` feature).
///
/// The SDK emits `tracing` spans unconditionally; this module bridges them to
/// an OTLP collector and propagates W3C trace context into outgoing requests.
#[cfg(feature = "otel")]
pub mod otel {
    use std::collections::HashMap;
    use std::sync::Arc;
    use std::sync::Mutex;
    use std::sync::atomic::{AtomicU64, Ordering};

    use opentelemetry::metrics::MeterProvider as _;
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
        Ok(otlp_tracer_provider(service_name, endpoint)?.tracer(service_name))
    }

    /// [`otlp_tracer`]'s provider, for an application that needs to flush or
    /// shut it down.
    ///
    /// Spans are exported in batches, so a process that exits without calling
    /// `force_flush()` or `shutdown()` loses whatever the last batch held —
    /// which tends to be the spans around whatever made it exit. The tracer
    /// alone does not expose its provider, so this returns it.
    ///
    /// # Errors
    /// Returns a [`opentelemetry::trace::TraceError`] if the exporter cannot be
    /// built (e.g. an invalid endpoint).
    pub fn otlp_tracer_provider(
        service_name: &'static str,
        endpoint: impl Into<String>,
    ) -> Result<opentelemetry_sdk::trace::TracerProvider, opentelemetry::trace::TraceError> {
        let exporter = opentelemetry_otlp::SpanExporter::builder()
            .with_tonic()
            .with_endpoint(endpoint.into())
            .build()?;
        Ok(opentelemetry_sdk::trace::TracerProvider::builder()
            .with_batch_exporter(exporter, opentelemetry_sdk::runtime::Tokio)
            .with_resource(opentelemetry_sdk::Resource::new(vec![
                opentelemetry::KeyValue::new("service.name", service_name),
            ]))
            .build())
    }

    /// Build an OTLP metrics pipeline and install it as the process's
    /// [`metrics`] recorder, so the counters this SDK emits reach a collector.
    ///
    /// The SDK emits through the [`metrics`] facade, which does nothing until
    /// an application installs a recorder — and the one supported path to
    /// OpenTelemetry was left to the reader. This is that path: one call, after
    /// which `canton_client_requests_total` and `canton_client_errors_total`
    /// arrive at `endpoint` with their `method`, `transport` and `retriable`
    /// labels intact.
    ///
    /// Keep the returned provider alive for the life of the process and call
    /// `shutdown()` before exit — metrics are exported periodically, so
    /// dropping it early loses the last interval.
    ///
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let meters = canton_core::telemetry::otel::otlp_metrics(
    ///     "my-service",
    ///     "http://localhost:4317",
    /// )?;
    /// // … run the application …
    /// meters.shutdown()?;
    /// # Ok(()) }
    /// ```
    ///
    /// # Errors
    /// Returns an error if the exporter cannot be built (an invalid endpoint,
    /// say) or if a `metrics` recorder is already installed.
    pub fn otlp_metrics(
        service_name: &'static str,
        endpoint: impl Into<String>,
    ) -> Result<opentelemetry_sdk::metrics::SdkMeterProvider, Box<dyn std::error::Error>> {
        let exporter = opentelemetry_otlp::MetricExporter::builder()
            .with_tonic()
            .with_endpoint(endpoint.into())
            .build()?;
        let reader = opentelemetry_sdk::metrics::PeriodicReader::builder(
            exporter,
            opentelemetry_sdk::runtime::Tokio,
        )
        .build();
        let provider = opentelemetry_sdk::metrics::SdkMeterProvider::builder()
            .with_reader(reader)
            .with_resource(opentelemetry_sdk::Resource::new(vec![
                opentelemetry::KeyValue::new("service.name", service_name),
            ]))
            .build();
        metrics::set_global_recorder(OtelRecorder::new(provider.meter(service_name)))
            .map_err(|e| -> Box<dyn std::error::Error> { Box::new(e) })?;
        Ok(provider)
    }

    /// Bridges the [`metrics`] facade onto an OpenTelemetry meter.
    ///
    /// The two models differ in one way that matters: a `metrics` key carries
    /// its labels, while an OpenTelemetry instrument is created once per name
    /// and takes attributes at record time. So instruments are cached by name
    /// and the key's labels become attributes.
    struct OtelRecorder {
        meter: opentelemetry::metrics::Meter,
        counters: Mutex<HashMap<String, opentelemetry::metrics::Counter<u64>>>,
        gauges: Mutex<HashMap<String, opentelemetry::metrics::Gauge<f64>>>,
        histograms: Mutex<HashMap<String, opentelemetry::metrics::Histogram<f64>>>,
    }

    impl OtelRecorder {
        fn new(meter: opentelemetry::metrics::Meter) -> Self {
            Self {
                meter,
                counters: Mutex::new(HashMap::new()),
                gauges: Mutex::new(HashMap::new()),
                histograms: Mutex::new(HashMap::new()),
            }
        }
    }

    /// A key's labels, as OpenTelemetry attributes.
    fn attributes(key: &metrics::Key) -> Vec<opentelemetry::KeyValue> {
        key.labels()
            .map(|label| {
                opentelemetry::KeyValue::new(label.key().to_string(), label.value().to_string())
            })
            .collect()
    }

    struct BridgedCounter {
        counter: opentelemetry::metrics::Counter<u64>,
        attributes: Vec<opentelemetry::KeyValue>,
        /// The last value seen from `absolute`, so a cumulative report can be
        /// turned into the delta an OpenTelemetry counter takes.
        last_absolute: AtomicU64,
    }

    impl metrics::CounterFn for BridgedCounter {
        fn increment(&self, value: u64) {
            self.counter.add(value, &self.attributes);
        }

        fn absolute(&self, value: u64) {
            let previous = self.last_absolute.swap(value, Ordering::SeqCst);
            self.counter
                .add(value.saturating_sub(previous), &self.attributes);
        }
    }

    struct BridgedGauge {
        gauge: opentelemetry::metrics::Gauge<f64>,
        attributes: Vec<opentelemetry::KeyValue>,
        /// OpenTelemetry's synchronous gauge only takes absolute values, so
        /// relative moves are tracked here.
        value: Mutex<f64>,
    }

    impl BridgedGauge {
        fn apply(&self, change: impl FnOnce(f64) -> f64) {
            let mut current = self
                .value
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            *current = change(*current);
            self.gauge.record(*current, &self.attributes);
        }
    }

    impl metrics::GaugeFn for BridgedGauge {
        fn increment(&self, value: f64) {
            self.apply(|current| current + value);
        }

        fn decrement(&self, value: f64) {
            self.apply(|current| current - value);
        }

        fn set(&self, value: f64) {
            self.apply(|_| value);
        }
    }

    struct BridgedHistogram {
        histogram: opentelemetry::metrics::Histogram<f64>,
        attributes: Vec<opentelemetry::KeyValue>,
    }

    impl metrics::HistogramFn for BridgedHistogram {
        fn record(&self, value: f64) {
            self.histogram.record(value, &self.attributes);
        }
    }

    impl metrics::Recorder for OtelRecorder {
        fn describe_counter(
            &self,
            _key: metrics::KeyName,
            _unit: Option<metrics::Unit>,
            _description: metrics::SharedString,
        ) {
        }
        fn describe_gauge(
            &self,
            _key: metrics::KeyName,
            _unit: Option<metrics::Unit>,
            _description: metrics::SharedString,
        ) {
        }
        fn describe_histogram(
            &self,
            _key: metrics::KeyName,
            _unit: Option<metrics::Unit>,
            _description: metrics::SharedString,
        ) {
        }

        fn register_counter(
            &self,
            key: &metrics::Key,
            _metadata: &metrics::Metadata<'_>,
        ) -> metrics::Counter {
            let name = key.name().to_string();
            let counter = {
                let mut counters = self
                    .counters
                    .lock()
                    .unwrap_or_else(std::sync::PoisonError::into_inner);
                counters
                    .entry(name.clone())
                    .or_insert_with(|| self.meter.u64_counter(name).build())
                    .clone()
            };
            metrics::Counter::from_arc(Arc::new(BridgedCounter {
                counter,
                attributes: attributes(key),
                last_absolute: AtomicU64::new(0),
            }))
        }

        fn register_gauge(
            &self,
            key: &metrics::Key,
            _metadata: &metrics::Metadata<'_>,
        ) -> metrics::Gauge {
            let name = key.name().to_string();
            let gauge = {
                let mut gauges = self
                    .gauges
                    .lock()
                    .unwrap_or_else(std::sync::PoisonError::into_inner);
                gauges
                    .entry(name.clone())
                    .or_insert_with(|| self.meter.f64_gauge(name).build())
                    .clone()
            };
            metrics::Gauge::from_arc(Arc::new(BridgedGauge {
                gauge,
                attributes: attributes(key),
                value: Mutex::new(0.0),
            }))
        }

        fn register_histogram(
            &self,
            key: &metrics::Key,
            _metadata: &metrics::Metadata<'_>,
        ) -> metrics::Histogram {
            let name = key.name().to_string();
            let histogram = {
                let mut histograms = self
                    .histograms
                    .lock()
                    .unwrap_or_else(std::sync::PoisonError::into_inner);
                histograms
                    .entry(name.clone())
                    .or_insert_with(|| self.meter.f64_histogram(name).build())
                    .clone()
            };
            metrics::Histogram::from_arc(Arc::new(BridgedHistogram {
                histogram,
                attributes: attributes(key),
            }))
        }
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

    /// The current span's trace id as 32 hex characters, when a valid
    /// OpenTelemetry context is active.
    pub(super) fn current_trace_id() -> Option<String> {
        use opentelemetry::trace::TraceContextExt as _;
        use tracing_opentelemetry::OpenTelemetrySpanExt as _;

        let context = tracing::Span::current().context();
        let span_context = context.span().span_context().clone();
        span_context.is_valid().then(|| {
            format!(
                "{:032x}",
                u128::from_be_bytes(span_context.trace_id().to_bytes())
            )
        })
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
    use tokio_stream::StreamExt as _;
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
        {
            let spans = captured.0.lock().unwrap();
            assert!(
                spans.iter().filter(|n| *n == "canton.rpc").count() >= 2,
                "expected canton.rpc spans, saw {spans:?}"
            );
        }

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

        // A stream that opens cleanly and fails later: the failure arrives
        // after the call that opened it has already returned, which is why
        // wrapping only that call recorded this as a success and stopped
        // watching.
        let source = tokio_stream::iter(vec![
            Ok(1u8),
            Err(Error::Connection("the participant went away".into())),
        ]);
        let stream = instrument_stream("updates", TRANSPORT_GRPC, source);
        tokio::pin!(stream);
        let mut outcomes = Vec::new();
        while let Some(item) = stream.next().await {
            outcomes.push(item.is_ok());
        }
        assert_eq!(outcomes, vec![true, false], "both items reach the caller");

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
        assert_eq!(
            counter_total(METRIC_ERRORS),
            2,
            "the stream's mid-life failure is counted too"
        );
        let spans = captured.0.lock().unwrap();
        assert!(
            spans.iter().any(|name| name == "canton.stream"),
            "expected a canton.stream span, saw {spans:?}"
        );
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
