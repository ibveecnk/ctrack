/// 404 Not Found handler
pub async fn not_found() -> axum::response::Html<&'static str> {
    axum::response::Html("<h1>404 Not Found</h1>")
}
