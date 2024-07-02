//! The backend server.

use axum::{routing::get, Json};
use shared::dto::Message;

/// This is the main entry point for the backend server.
#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    // Create a new Axum service.
    let app = axum::Router::new().route("/api/message", get(message));

    // Start the server.
    let addr = "0.0.0.0:8000".parse::<std::net::SocketAddr>()?;

    let tcp_listener = tokio::net::TcpListener::bind(&addr)
        .await
        .map_err(|e| color_eyre::Report::msg(format!("Failed to bind to address {addr}: {e}")))?;

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

/// This is a simple message handler.
async fn message() -> axum::response::Json<Message> {
    Json(Message {
        text: "Hello, world!".to_string(),
    })
}
