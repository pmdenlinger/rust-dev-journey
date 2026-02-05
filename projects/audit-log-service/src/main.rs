use axum::{routing::get, Router};
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Build your application with routes
    let app = Router::new().route("/", get(root));

    // Bind address
    let addr: SocketAddr = "0.0.0.0:3000".parse()?;

    // Bind a Tokio TcpListener (Axum 0.7 / Hyper 1.0 pattern)
    let listener = TcpListener::bind(addr).await?;

    // Serve the app with graceful shutdown
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn root() -> &'static str {
    "Hello from audit-log-service"
}

// Cross-platform graceful shutdown (Ctrl+C; also listens for SIGTERM on Unix)
async fn shutdown_signal() {
    use tokio::signal;

    // Ctrl+C
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        use tokio::signal::unix::{signal, SignalKind};
        let mut term = signal(SignalKind::terminate()).expect("failed to install SIGTERM handler");
        term.recv().await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}