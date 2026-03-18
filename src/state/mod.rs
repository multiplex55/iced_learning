//! Shared state types.
//!
//! This module exists so future demos can move longer-lived state out of the
//! top-level `App` without introducing unnecessary abstraction too early.

/// Data shared across multiple pages to demonstrate cross-page communication.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SharedState {
    pub learner_name: String,
    pub shared_counter: i32,
    pub notes: Vec<String>,
}

impl Default for SharedState {
    fn default() -> Self {
        Self {
            learner_name: "Iced explorer".into(),
            shared_counter: 3,
            notes: vec![
                "Dashboard summarizes the whole sandbox".into(),
                "Controls mutate shared state".into(),
                "Data flow shows how updates ripple between pages".into(),
            ],
        }
    }
}
