//! The frontend application.

use leptos::mount_to_body;

/// The main application component.
mod app;

/// The entry point for the frontend application.
///
/// Run it via `trunk serve` or `trunk watch`.
fn main() {
    mount_to_body(app::Main);
}
