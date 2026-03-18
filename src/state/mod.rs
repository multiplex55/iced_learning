//! Shared state types.
//!
//! This module exists so future demos can move longer-lived state out of the
//! top-level `App` without introducing unnecessary abstraction too early.

use crate::message::{ControlChoice, MenuAction};
use crate::pages::Page;

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
    pub dashboard_status: String,
    pub last_event: String,
    pub profile_last_changed_by: Page,
    pub status_last_changed_by: Page,
    pub preference_last_changed_by: Page,
}

impl SharedState {
    pub fn profile_preview_name(&self) -> &str {
        if self.learner_name.trim().is_empty() {
            "Anonymous learner"
        } else {
            self.learner_name.trim()
        }
    }

    pub fn visibility_summary(&self) -> &'static str {
        if self.show_sidebar_tips {
            "Sidebar tips are visible across the shell."
        } else {
            "Sidebar tips are hidden across the shell."
        }
    }

    pub fn dashboard_summary(&self) -> String {
        format!(
            "{} • Status: {} • Preferred widget family: {}",
            self.profile_preview_name(),
            self.dashboard_status,
            self.selected_control.label()
        )
    }
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
            dashboard_status: "Ready to trace data flow through the app.".into(),
            last_event: "App booted with default shared state.".into(),
            profile_last_changed_by: Page::Dashboard,
            status_last_changed_by: Page::Dashboard,
            preference_last_changed_by: Page::Advanced,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SharedState;

    #[test]
    fn derived_helpers_return_expected_visible_text() {
        let mut state = SharedState::default();
        state.learner_name = "  ".into();
        state.show_sidebar_tips = false;
        state.dashboard_status = "Reviewing reducer output".into();

        assert_eq!(state.profile_preview_name(), "Anonymous learner");
        assert_eq!(
            state.visibility_summary(),
            "Sidebar tips are hidden across the shell."
        );
        assert_eq!(
            state.dashboard_summary(),
            "Anonymous learner • Status: Reviewing reducer output • Preferred widget family: Buttons"
        );
    }
}
