//! The whole observability path, end to end: emit → export → collector.
//!
//! Every other telemetry test in this crate asserts what the SDK *emits* — a
//! span was opened, a counter moved. That is one half. The half a reader cannot
//! take on trust is whether any of it leaves the process, and it is the half an
//! application depends on: `otel::otlp_tracer` and `otel::otlp_metrics` are the
//! SDK's answer to "how do I see this in my collector?", and nothing here had
//! ever received what they send.
//!
//! So this test *is* the collector. It stands up the two OTLP services on a
//! local port, points the SDK's own setup functions at them, makes one
//! successful call and one failing one, and asserts what arrives: a
//! `canton.rpc` span, and the request and error counters carrying the `method`
//! and `transport` attributes a dashboard is built from.
#![cfg(feature = "otel")]
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::{Arc, Mutex};
use std::time::Duration;

use canton_core::telemetry::{TRANSPORT_GRPC, instrument};
use canton_core::{Error, Result};
use opentelemetry::trace::TracerProvider as _;
use opentelemetry_proto::tonic::collector::metrics::v1::metrics_service_server::{
    MetricsService, MetricsServiceServer,
};
use opentelemetry_proto::tonic::collector::metrics::v1::{
    ExportMetricsServiceRequest, ExportMetricsServiceResponse,
};
use opentelemetry_proto::tonic::collector::trace::v1::trace_service_server::{
    TraceService, TraceServiceServer,
};
use opentelemetry_proto::tonic::collector::trace::v1::{
    ExportTraceServiceRequest, ExportTraceServiceResponse,
};
use tokio::net::TcpListener;
use tokio_stream::wrappers::TcpListenerStream;
use tracing_subscriber::layer::SubscriberExt as _;

/// One metric point as it arrived: its name and its string attributes.
type MetricPoint = (String, Vec<(String, String)>);

/// What the collector received.
#[derive(Clone, Default)]
struct Received {
    metrics: Arc<Mutex<Vec<MetricPoint>>>,
    spans: Arc<Mutex<Vec<String>>>,
}

#[otlp_tonic::async_trait]
impl MetricsService for Received {
    async fn export(
        &self,
        request: otlp_tonic::Request<ExportMetricsServiceRequest>,
    ) -> std::result::Result<otlp_tonic::Response<ExportMetricsServiceResponse>, otlp_tonic::Status>
    {
        use opentelemetry_proto::tonic::common::v1::any_value::Value;
        use opentelemetry_proto::tonic::metrics::v1::metric::Data;
        for resource in request.into_inner().resource_metrics {
            for scope in resource.scope_metrics {
                for metric in scope.metrics {
                    let points = match metric.data {
                        Some(Data::Sum(sum)) => sum.data_points,
                        _ => continue,
                    };
                    for point in points {
                        let attributes = point
                            .attributes
                            .iter()
                            .filter_map(|kv| match kv.value.as_ref()?.value.as_ref()? {
                                Value::StringValue(text) => Some((kv.key.clone(), text.clone())),
                                _ => None,
                            })
                            .collect();
                        self.metrics
                            .lock()
                            .unwrap()
                            .push((metric.name.clone(), attributes));
                    }
                }
            }
        }
        Ok(otlp_tonic::Response::new(
            ExportMetricsServiceResponse::default(),
        ))
    }
}

#[otlp_tonic::async_trait]
impl TraceService for Received {
    async fn export(
        &self,
        request: otlp_tonic::Request<ExportTraceServiceRequest>,
    ) -> std::result::Result<otlp_tonic::Response<ExportTraceServiceResponse>, otlp_tonic::Status>
    {
        for resource in request.into_inner().resource_spans {
            for scope in resource.scope_spans {
                for span in scope.spans {
                    self.spans.lock().unwrap().push(span.name);
                }
            }
        }
        Ok(otlp_tonic::Response::new(
            ExportTraceServiceResponse::default(),
        ))
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn spans_and_metrics_reach_an_otlp_collector() {
    let received = Received::default();
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let endpoint = format!("http://{}", listener.local_addr().unwrap());
    let incoming = TcpListenerStream::new(listener);
    let collector = received.clone();
    tokio::spawn(async move {
        otlp_tonic::transport::Server::builder()
            .add_service(MetricsServiceServer::new(collector.clone()))
            .add_service(TraceServiceServer::new(collector))
            .serve_with_incoming(incoming)
            .await
            .unwrap();
    });
    tokio::time::sleep(Duration::from_millis(200)).await;

    // The SDK's own setup, as its documentation describes it.
    let meters = canton_core::telemetry::otel::otlp_metrics("canton-otlp-test", &endpoint)
        .expect("the metrics pipeline builds and installs its recorder");
    let provider =
        canton_core::telemetry::otel::otlp_tracer_provider("canton-otlp-test", &endpoint)
            .expect("the span exporter builds");
    let tracer = provider.tracer("canton-otlp-test");
    let subscriber =
        tracing_subscriber::registry().with(tracing_opentelemetry::layer().with_tracer(tracer));
    let _guard = tracing::subscriber::set_default(subscriber);

    // One success and one failure, through the instrumentation every SDK call uses.
    let ok: Result<u8> = instrument("version", TRANSPORT_GRPC, async { Ok(1) }).await;
    assert_eq!(ok.unwrap(), 1);
    let failed: Result<u8> = instrument("ledger_end", TRANSPORT_GRPC, async {
        Err(Error::InvalidRequest("boom".into()))
    })
    .await;
    assert!(failed.is_err());

    // Flush both pipelines rather than waiting out their periodic export.
    provider.force_flush();
    meters.force_flush().expect("flush the metrics pipeline");
    tokio::time::sleep(Duration::from_millis(500)).await;

    let spans = received.spans.lock().unwrap().clone();
    assert!(
        spans.iter().any(|name| name == "canton.rpc"),
        "the collector should have received the rpc span, saw {spans:?}"
    );

    let metrics = received.metrics.lock().unwrap().clone();
    let named = |wanted: &str| -> Vec<Vec<(String, String)>> {
        metrics
            .iter()
            .filter(|(name, _)| name == wanted)
            .map(|(_, attributes)| attributes.clone())
            .collect()
    };
    let requests = named("canton_client_requests_total");
    assert!(
        !requests.is_empty(),
        "the collector should have received the request counter, saw {metrics:?}"
    );
    // The labels are the point: a dashboard breaks down by method and transport.
    assert!(
        requests.iter().any(|attributes| {
            attributes.contains(&("method".to_string(), "version".to_string()))
                && attributes.contains(&("transport".to_string(), "grpc".to_string()))
        }),
        "request-counter attributes should survive the bridge, saw {requests:?}"
    );
    let errors = named("canton_client_errors_total");
    assert!(
        errors.iter().any(|attributes| {
            attributes.contains(&("method".to_string(), "ledger_end".to_string()))
                && attributes.contains(&("retriable".to_string(), "false".to_string()))
        }),
        "error-counter attributes should survive the bridge, saw {errors:?}"
    );
}
