use std::time::{Duration, Instant};

use anyhow::Result;
use axum::{routing::get, Router};
use opentelemetry::trace::TracerProvider as _;
use opentelemetry::KeyValue;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::trace::{RandomIdGenerator, Tracer, TracerProvider};
use opentelemetry_sdk::{runtime, Resource};
use tokio::join;
use tokio::{net::TcpListener, time::sleep};
use tracing::instrument;
use tracing::{debug, info, level_filters::LevelFilter, warn};
use tracing_subscriber::{
    fmt::format::FmtSpan, layer::SubscriberExt as _, util::SubscriberInitExt as _, Layer,
};

#[tokio::main]
async fn main() -> Result<()> {
    let file_appender = tracing_appender::rolling::hourly("/tmp/logs", "rust.ecosystem.usage.log");
    let (non_block, _gurad) = tracing_appender::non_blocking(file_appender);

    let file = tracing_subscriber::fmt::Layer::new()
        .with_writer(non_block)
        .pretty()
        .with_filter(LevelFilter::INFO);

    let console = tracing_subscriber::fmt::Layer::new()
        .with_span_events(FmtSpan::CLOSE)
        .pretty()
        .with_filter(LevelFilter::DEBUG);

    let tracer = init_tracer()?;

    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    tracing_subscriber::registry()
        .with(file)
        .with(console)
        .with(telemetry)
        .init();

    info!("Hello tracing!");
    debug!("tracing debug");
    info!("tracing warn");

    let addr = "0.0.0.0:8080";
    let app = Router::new().route("/", get(index_handler));
    let listener = TcpListener::bind(addr).await?;
    info!("Starting server on {}", addr);
    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}

// handle index request
async fn index_handler() -> &'static str {
    debug!("index handler started");
    sleep(Duration::from_millis(10)).await;
    let ret = long_task().await;
    info!(http.status = 200, "index handler completed");
    ret
}

fn init_tracer() -> anyhow::Result<Tracer> {
    // new api version, using builder pattern to construct a TracerProvider then build a Tracer
    Ok(TracerProvider::builder()
        .with_batch_exporter(
            opentelemetry_otlp::SpanExporter::builder()
                .with_tonic()
                .with_endpoint("http://localhost:4317")
                .build()?,
            runtime::Tokio,
        )
        .with_id_generator(RandomIdGenerator::default())
        .with_max_events_per_span(32)
        .with_max_attributes_per_span(64)
        .with_resource(Resource::new(vec![KeyValue::new(
            "service.name",
            "axum-tracing",
        )]))
        .build()
        .tracer("axum_tracing"))
}

// generate span for long_task to find the performance bottleneck
#[instrument]
async fn long_task() -> &'static str {
    let start = Instant::now();
    let sl = sleep(Duration::from_millis(12));

    let t1 = task1();
    let t2 = task2();
    let t3 = task3();
    join!(sl, t1, t2, t3);
    let elapsed = start.elapsed().as_millis();
    warn!(app.task_duration = elapsed, "task takes too long");
    "long task"
}

#[instrument]
async fn task1() {
    sleep(Duration::from_millis(10)).await;
}

#[instrument]
async fn task2() {
    sleep(Duration::from_millis(50)).await;
}

#[instrument]
async fn task3() {
    sleep(Duration::from_millis(30)).await;
}
