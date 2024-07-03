//! The backend server.

use axum::routing::get;
use color_eyre::eyre::eyre;

/// The handler module contains the request handlers for the backend server.
mod handler;

/// This is the main entry point for the backend server.
#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    let api_subrouter = axum::Router::new().route("/message", get(handler::api::message));

    // Create a new Axum service.
    let app = axum::Router::new()
        .nest("/api", api_subrouter)
        .fallback(handler::not_found);

    // Start the server.
    let addr = "0.0.0.0:8000".parse::<std::net::SocketAddr>()?;

    let tcp_listener = tokio::net::TcpListener::bind(&addr)
        .await
        .map_err(|e| eyre!("Failed to bind to address {addr}: {e}"))?;

    tokio::spawn(async move {
        axum::serve(tcp_listener, app.into_make_service())
            .await
            .expect("server failed");
    });

    #[allow(clippy::print_stdout)]
    {
        println!("Listening on http://{addr}");
    }

    loop {
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
    }
}
