//! omega-dream: Dream-to-Waking feedback loop
#![warn(missing_docs)]

pub mod types {
    //! Types for dream feedback
    use serde::{Deserialize, Serialize};

    /// Placeholder for dream types
    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct Placeholder;
}

pub mod service {
    //! Dream feedback service
    use crate::types::*;

    /// Dream feedback service
    pub struct Service;

    impl Service {
        /// Create a new dream service
        pub fn new() -> Self {
            Service
        }
    }
}
