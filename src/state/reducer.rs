//! Pure shared-state reducers.
//!
//! Keeping these transitions outside widget-building code makes regression
//! testing straightforward as the learning sandbox grows.

use crate::message::ControlChoice;
use crate::pages::Page;
use crate::state::SharedState;
use crate::theme::ThemeChoice;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SharedStateAction {
    SetLearnerName { value: String, source: Page },
    SetDashboardStatus { value: String, source: Page },
    SetSidebarTipsVisible { value: bool, source: Page },
    SetThemeChoice { value: ThemeChoice, source: Page },
    ResetDataFlowDemo { source: Page },
}

pub fn apply_shared_state_action(state: &mut SharedState, action: SharedStateAction) {
    match action {
        SharedStateAction::SetLearnerName { value, source } => {
            state.learner_name = value;
            state.profile_last_changed_by = source;
            state.last_event = format!(
                "{} updated the shared learner profile to {:?}.",
                source.label(),
                state.learner_name
            );
        }
        SharedStateAction::SetDashboardStatus { value, source } => {
            state.dashboard_status = value;
            state.status_last_changed_by = source;
            state.last_event = format!(
                "{} updated the dashboard status to {:?}.",
                source.label(),
                state.dashboard_status
            );
        }
        SharedStateAction::SetSidebarTipsVisible { value, source } => {
            state.show_sidebar_tips = value;
            state.preference_last_changed_by = source;
            state.last_event = format!(
                "{} changed sidebar tips visibility to {}.",
                source.label(),
                value
            );
        }
        SharedStateAction::SetThemeChoice { value, source } => {
            state.theme_choice = value;
            state.preference_last_changed_by = source;
            state.last_event = format!(
                "{} switched the sandbox theme to {}.",
                source.label(),
                value.label()
            );
        }
        SharedStateAction::ResetDataFlowDemo { source } => {
            let defaults = SharedState::default();
            state.learner_name = defaults.learner_name;
            state.dashboard_status = defaults.dashboard_status;
            state.show_sidebar_tips = defaults.show_sidebar_tips;
            state.theme_choice = defaults.theme_choice;
            state.profile_last_changed_by = source;
            state.status_last_changed_by = source;
            state.preference_last_changed_by = source;
            state.last_event = format!(
                "{} reset the shared data-flow fields back to their defaults.",
                source.label()
            );
        }
    }
}

pub fn cycle_preferred_control(current: ControlChoice) -> ControlChoice {
    let all = ControlChoice::ALL;
    let index = all
        .iter()
        .position(|choice| *choice == current)
        .unwrap_or(0);
    all[(index + 1) % all.len()]
}

#[cfg(test)]
mod tests {
    use super::{apply_shared_state_action, cycle_preferred_control, SharedStateAction};
    use crate::message::ControlChoice;
    use crate::pages::Page;
    use crate::state::SharedState;
    use crate::theme::ThemeChoice;

    #[test]
    fn reducer_updates_expected_fields() {
        let mut state = SharedState::default();
        apply_shared_state_action(
            &mut state,
            SharedStateAction::SetThemeChoice {
                value: ThemeChoice::Light,
                source: Page::Advanced,
            },
        );

        assert_eq!(state.theme_choice, ThemeChoice::Light);
        assert_eq!(state.preference_last_changed_by, Page::Advanced);
        assert!(state.last_event.contains("switched the sandbox theme"));
    }

    #[test]
    fn reset_restores_shared_defaults() {
        let mut state = SharedState::default();
        state.learner_name = "Changed".into();
        state.dashboard_status = "Changed".into();
        state.show_sidebar_tips = false;
        state.theme_choice = ThemeChoice::HighContrast;

        apply_shared_state_action(
            &mut state,
            SharedStateAction::ResetDataFlowDemo {
                source: Page::DataFlow,
            },
        );

        let defaults = SharedState::default();
        assert_eq!(state.learner_name, defaults.learner_name);
        assert_eq!(state.dashboard_status, defaults.dashboard_status);
        assert_eq!(state.show_sidebar_tips, defaults.show_sidebar_tips);
        assert_eq!(state.theme_choice, defaults.theme_choice);
    }

    #[test]
    fn control_cycle_wraps_cleanly() {
        assert_eq!(
            cycle_preferred_control(ControlChoice::PickList),
            ControlChoice::Button
        );
    }
}
