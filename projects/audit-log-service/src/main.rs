
use axum::{routing::get, Router};
use std::{net::SocketAddr};
use tracing_subscriber::{EnvFilter, fmt::SubscriberBuilder};

#[tokio::main]
async fn main() {
    // tracing setup
    SubscriberBuilder::default()
        .with_env_filter(EnvFilter::from_default_env())
        .with_target(false)
        .json()
        .init();

    let app = Router::new().route("/health", get(|| async { "ok" }));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!(%addr, "starting server");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    use tokio::signal::unix::{signal, SignalKind};
    let mut sigint = signal(SignalKind::interrupt()).expect("signal");
    let mut sigterm = signal(SignalKind::terminate()).expect("signal");
    tokio::select! {
        _ = sigint.recv() => {}
        _ = sigterm.recv() => {}
    }
    tracing::info!("shutdown signal received");
}
