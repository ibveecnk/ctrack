/// User module
pub mod users {
    use diesel::{Identifiable, Insertable, Queryable};

    /// User struct
    #[derive(Debug, Queryable, Clone, Identifiable)]
    #[diesel(table_name = crate::database::schema::users)]
    pub struct User {
        /// User ID
        pub id: i32,
        /// Username
        pub username: String,
        /// Email
        pub email: String,
        /// Password
        pub password: String,
        /// Unique ID
        pub unique_id: String,
    }

    impl User {
        /// Verify password
        pub fn verify(&self, password: &str) -> bool {
            bcrypt::verify(password, &self.password).map_or(false, |v| v)
        }
    }

    /// New user struct
    #[derive(Debug, Insertable, Clone)]
    #[diesel(table_name = crate::database::schema::users)]
    pub struct NewUser {
        /// Username
        pub username: String,
        /// Email
        pub email: String,
        /// Password
        pub password: String,
        /// Unique ID
        pub unique_id: String,
    }

    impl NewUser {
        /// Create a new user
        pub fn new(username: String, email: String, password: &str) -> Self {
            let hashed_password: String =
                bcrypt::hash(password, bcrypt::DEFAULT_COST).expect("crypt error");
            let uuid = uuid::Uuid::new_v4().to_string();

            Self {
                username,
                email,
                password: hashed_password,
                unique_id: uuid,
            }
        }
    }
}
