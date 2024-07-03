pub use users::*;

/// Users database utilities
mod users {
    use diesel::{query_dsl::methods::SelectDsl, RunQueryDsl};

    use crate::database::schema::users;

    /// Get all usernames from the database
    pub async fn get_all_usernames() -> Vec<String> {
        users::table
            .select(users::username)
            .get_results(&mut *crate::database::DATABASE.lock().await.connection())
            .expect("Failed to load users")
    }

    /// Add a new user to the database
    pub async fn add_user(username: &str, email: &str, password: &str) {
        let new_user = crate::database::models::users::NewUser::new(
            username.to_string(),
            email.to_string(),
            password,
        );
        diesel::insert_into(users::table)
            .values(&new_user)
            .execute(&mut *crate::database::DATABASE.lock().await.connection())
            .expect("Failed to add user");
    }
}
