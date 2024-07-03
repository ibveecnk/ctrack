/// Fallback handler for 404 Not Found.
mod not_found;
pub use not_found::not_found;

/// API handlers.
pub mod api;
