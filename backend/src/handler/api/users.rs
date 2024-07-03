use crate::database;

/// Get all users
pub async fn users() -> axum::Json<Vec<String>> {
    let users = database::utils::get_all_usernames().await;

    axum::Json(users)
}
