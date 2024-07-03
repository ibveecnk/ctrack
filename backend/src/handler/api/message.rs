use axum::Json;
use shared::dto::Message;

/// Handler for the message API.
pub async fn message() -> axum::response::Json<Message> {
    Json(Message {
        text: "Hello, world!".to_string(),
    })
}
