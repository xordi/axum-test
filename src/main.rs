use std::error::Error;

use axum::routing::get;
use axum::Router;
use opentelemetry::sdk::trace::{self, Sampler};
use opentelemetry::sdk::Resource;
use opentelemetry::KeyValue;
use opentelemetry_otlp::{Protocol, WithExportConfig};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::{info, instrument};
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, EnvFilter};

#[instrument]
async fn home_handler() -> String {
    info!("Hey from the home handler");
    "ola k ase".to_string()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    configure_tracing("axum-test")?;

    let app = Router::new()
        .route("/", get(home_handler))
        .route_layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

fn configure_tracing(service_name: impl Into<String>) -> Result<(), Box<dyn Error>> {
    // Create a new OpenTelemetry trace pipeline that prints to stdout
    let otlp_exporter = opentelemetry_otlp::new_exporter()
        .tonic()
        .with_protocol(Protocol::Grpc)
        .with_endpoint("http://localhost:4317");

    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(otlp_exporter)
        .with_trace_config(
            trace::config()
                .with_resource(Resource::new(vec![KeyValue::new(
                    "service.name",
                    service_name.into(),
                )]))
                .with_sampler(Sampler::AlwaysOn),
        )
        .install_batch(opentelemetry::runtime::TokioCurrentThread)?;

    // Create a tracing layer with the configured tracer
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
    let fmt_layer = fmt::layer();

    // Create the subscriber
    tracing_subscriber::registry()
        .with(telemetry)
        .with(fmt_layer)
        .with(EnvFilter::from_default_env())
        .init();

    Ok(())
}
