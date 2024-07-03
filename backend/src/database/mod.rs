use std::sync::Arc;

/// Database connection
pub static DATABASE: once_cell::sync::Lazy<Arc<tokio::sync::Mutex<Database>>> =
    once_cell::sync::Lazy::new(|| Arc::new(tokio::sync::Mutex::new(Database::new())));

mod prelude;
pub use prelude::*;

/// Database model structs
pub mod models;
/// Database schema definition
pub mod schema;

/// Database utilities, e.g. queries
pub mod utils;
