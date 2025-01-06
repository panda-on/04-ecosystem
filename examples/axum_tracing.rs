use std::time::{Duration, Instant};

use anyhow::Result;
use axum::{routing::get, Router};
use tokio::{net::TcpListener, time::sleep};
use tracing::{debug, info, level_filters::LevelFilter, warn};
use tracing_subscriber::{fmt::format::FmtSpan, layer::SubscriberExt as _, util::SubscriberInitExt as _, Layer};

#[tokio::main]
async fn main() -> Result<()> {
    let file_appender = tracing_appender::rolling::hourly("/tmp/logs", "prefix.log");
    let (non_block, _gurad) = tracing_appender::non_blocking(file_appender);

    let file = tracing_subscriber::fmt::Layer::new()
        .with_writer(non_block)
        .pretty()
        .with_filter(LevelFilter::INFO);

    let console = tracing_subscriber::fmt::Layer::new()
        .with_span_events(FmtSpan::CLOSE)
        .pretty()
        .with_filter(LevelFilter::DEBUG);

    tracing_subscriber::registry().with(file).with(console).init();

    tracing::info!("Hello tracing!");
    tracing::debug!("tracing debug");
    tracing::info!("tracing warn");

    let addr = "0.0.0.0:8080";
    let app = Router::new().route("/", get(index_handler));
    let listner = TcpListener::bind(addr).await?;
    info!("Starting server on {}", addr);
    axum::serve(listner,app.into_make_service()).await?;
    Ok(())
}

async fn index_handler() -> &'static str {
    debug!("index handler started");
    sleep(Duration::from_millis(10)).await;
    let ret = long_task().await;
    info!(http.status = 200, "index handler completed");
    ret
}

async fn long_task() -> &'static str {
    let start = Instant::now();
    sleep(Duration::from_millis(112)).await;
    let elapsed = start.elapsed().as_millis();
    warn!(app.task_duration = elapsed, "task takes too long");
    "long task"
}
