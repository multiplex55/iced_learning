//! Shared state types.
//!
//! This module exists so future demos can move longer-lived state out of the
//! top-level `App` without introducing unnecessary abstraction too early.

use crate::message::{ControlChoice, MenuAction};

/// Data shared across multiple pages to demonstrate cross-page communication.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SharedState {
    pub learner_name: String,
    pub shared_counter: i32,
    pub notes: Vec<String>,
    pub last_menu_action: Option<MenuAction>,
    pub status_line: String,
    pub show_sidebar_tips: bool,
    pub controls_enabled: bool,
    pub controls_checked: bool,
    pub slider_value: u8,
    pub selected_control: ControlChoice,
    pub progress_value: u8,
    pub dark_mode_demo: bool,
    pub ticks: u64,
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
            last_menu_action: None,
            status_line: "Ready: use the navigation tabs to inspect each learning page.".into(),
            show_sidebar_tips: true,
            controls_enabled: true,
            controls_checked: true,
            slider_value: 42,
            selected_control: ControlChoice::Button,
            progress_value: 30,
            dark_mode_demo: true,
            ticks: 0,
        }
    }
}
