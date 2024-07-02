//! Shared code between services

/// DTOs for shared data between services
pub mod dto {
    use serde::{Deserialize, Serialize};

    /// A message
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Message {
        /// The text of the message
        pub text: String,
    }
}
