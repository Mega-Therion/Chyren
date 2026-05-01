pub mod engine;
pub mod handover;
pub mod models;

pub use engine::{MetacognitiveEngine, MetacogStatus, ReflectionResponse};
pub use handover::{
    DefaultHandover, HandoverSignature, VerifiedHumanHandover, HUMAN_ATTRIBUTION_REQUIRED,
};
pub use models::{SystemSnapshot, IntrospectionReport, MetaQuery};
