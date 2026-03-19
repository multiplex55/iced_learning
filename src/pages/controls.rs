//! Control widgets page.
//!
//! These controls are intentionally simple and favor clarity over production
//! validation, error messaging, or accessibility polish.

use iced::widget::{
    button, checkbox, column, container, pick_list, progress_bar, radio, row, slider, text,
    text_input, toggler,
};
use iced::{Alignment, Element, Length};

use crate::app::App;
use crate::message::{ControlChoice, Message};
use crate::state::SharedState;
use crate::widgets;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ControlInteractivity {
    pub master_toggle_enabled: bool,
    pub learner_name_enabled: bool,
    pub counter_buttons_enabled: bool,
    pub checkbox_enabled: bool,
    pub slider_enabled: bool,
    pub choice_inputs_enabled: bool,
    pub advance_progress_enabled: bool,
    pub reset_enabled: bool,
}

impl ControlInteractivity {
    pub fn status_copy(self) -> &'static str {
        if self.dependent_controls_enabled() {
            "Related controls are live. Toggle the switch to pause the form without resetting values."
        } else {
            "Controls paused: dependent inputs are disabled until you re-enable related controls."
        }
    }

    pub fn dependent_controls_enabled(self) -> bool {
        self.learner_name_enabled
            && self.counter_buttons_enabled
            && self.checkbox_enabled
            && self.slider_enabled
            && self.choice_inputs_enabled
            && self.advance_progress_enabled
    }
}

pub fn interactivity(shared: &SharedState) -> ControlInteractivity {
    let controls_enabled = shared.controls_enabled;

    ControlInteractivity {
        master_toggle_enabled: true,
        learner_name_enabled: controls_enabled,
        counter_buttons_enabled: controls_enabled,
        checkbox_enabled: controls_enabled,
        slider_enabled: controls_enabled,
        choice_inputs_enabled: controls_enabled,
        advance_progress_enabled: controls_enabled,
        reset_enabled: true,
    }
}

pub fn view(app: &App) -> Element<'_, Message> {
    let selected = Some(app.shared.selected_control);
    let control_state = interactivity(&app.shared);

    let first_body: Element<'_, Message> = column![
        text(control_state.status_copy()),
        text_input("Learner name", &app.shared.learner_name)
            .on_input_maybe(
                control_state
                    .learner_name_enabled
                    .then_some(Message::SharedTextChanged),
            )
            .padding(10),
        row![
            button("-1").on_press_maybe(
                control_state
                    .counter_buttons_enabled
                    .then_some(Message::CounterDecremented),
            ),
            text(format!("Counter: {}", app.shared.shared_counter)),
            button("+1").on_press_maybe(
                control_state
                    .counter_buttons_enabled
                    .then_some(Message::CounterIncremented),
            ),
        ]
        .spacing(12)
        .align_y(Alignment::Center),
        toggler(app.shared.controls_enabled)
            .label("Enable related controls")
            .on_toggle_maybe(
                control_state
                    .master_toggle_enabled
                    .then_some(Message::ControlsToggled),
            ),
        checkbox("Checkbox state demo", app.shared.controls_checked).on_toggle_maybe(
            control_state
                .checkbox_enabled
                .then_some(Message::ControlsCheckboxChanged),
        ),
    ]
    .spacing(12)
    .into();

    let mut radio_column = column![text("Preferred widget family")].spacing(8);
    if control_state.choice_inputs_enabled {
        for choice in ControlChoice::ALL {
            radio_column = radio_column.push(radio(
                choice.label(),
                choice,
                selected,
                Message::ControlsChoiceSelected,
            ));
        }
    } else {
        radio_column = radio_column.push(text(format!(
            "Radio choices paused. Current selection: {}",
            app.shared.selected_control.label()
        )));
    }

    let slider_block: Element<'_, Message> = if control_state.slider_enabled {
        slider(
            0..=100,
            app.shared.slider_value,
            Message::ControlsSliderChanged,
        )
        .into()
    } else {
        progress_bar(0.0..=100.0, f32::from(app.shared.slider_value)).into()
    };

    let pick_list_block: Element<'_, Message> = if control_state.choice_inputs_enabled {
        pick_list(
            ControlChoice::ALL,
            selected,
            Message::ControlsChoiceSelected,
        )
        .placeholder("Choose a widget family")
        .into()
    } else {
        text(format!(
            "Pick list paused. Current selection: {}",
            app.shared.selected_control.label()
        ))
        .into()
    };

    let second_body: Element<'_, Message> = column![
        slider_block,
        text(format!("Slider value: {}", app.shared.slider_value)),
        radio_column,
        pick_list_block,
        progress_bar(0.0..=100.0, f32::from(app.shared.progress_value)),
        button("Advance progress demo").on_press_maybe(
            control_state
                .advance_progress_enabled
                .then_some(Message::ProgressStepped),
        ),
    ]
    .spacing(12)
    .into();

    let content = column![
        widgets::section_title("Controls"),
        widgets::note("This page pairs each widget with the state field it updates so the update loop stays easy to follow."),
        widgets::section_card(
            "Buttons, text input, and toggles",
            "Interact with shared state and watch every page reflect the changes.",
            first_body,
        ),
        widgets::section_card(
            "Sliders, radios, pick lists, and progress",
            "These widgets demonstrate value selection and richer state mapping.",
            second_body,
        ),
        container(button("Reset sandbox state").on_press_maybe(
            control_state.reset_enabled.then_some(Message::ResetSandbox),
        )),
    ]
    .spacing(16);

    container(content).width(Length::Fill).into()
}

#[cfg(test)]
mod tests {
    use super::interactivity;
    use crate::state::SharedState;

    #[test]
    fn interactivity_keeps_master_toggle_and_reset_available_when_paused() {
        let mut shared = SharedState::default();
        shared.controls_enabled = false;

        let flags = interactivity(&shared);

        assert!(flags.master_toggle_enabled);
        assert!(flags.reset_enabled);
        assert!(!flags.learner_name_enabled);
        assert!(!flags.counter_buttons_enabled);
        assert!(!flags.checkbox_enabled);
        assert!(!flags.slider_enabled);
        assert!(!flags.choice_inputs_enabled);
        assert!(!flags.advance_progress_enabled);
        assert!(!flags.dependent_controls_enabled());
        assert!(flags.status_copy().contains("Controls paused"));
    }

    #[test]
    fn interactivity_re_enables_all_dependent_controls_together() {
        let shared = SharedState::default();

        let flags = interactivity(&shared);

        assert!(flags.master_toggle_enabled);
        assert!(flags.reset_enabled);
        assert!(flags.dependent_controls_enabled());
        assert!(flags.status_copy().contains("Related controls are live"));
    }

    #[test]
    fn master_toggle_label_and_disabled_behavior_stay_aligned() {
        let mut shared = SharedState::default();
        shared.controls_enabled = false;

        let flags = interactivity(&shared);
        let label = "Enable related controls";

        assert!(label.contains("related controls"));
        assert!(flags.master_toggle_enabled);
        assert!(!flags.dependent_controls_enabled());
    }
}
