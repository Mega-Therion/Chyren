//! Module description here
#![warn(missing_docs)]

pub mod types;
pub mod service;

pub use types::*;
pub use service::*;

mod types {
    // Placeholder types
    #[derive(Clone, Debug)]
    pub struct Placeholder;
}

mod service {
    use crate::types::*;

    /// Placeholder service
    pub struct Service;

    impl Service {
        /// Initialize service
        pub fn new() -> Self {
            Service
        }
    }
}
