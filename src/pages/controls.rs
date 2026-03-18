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
use crate::widgets;

pub fn view(app: &App) -> Element<'_, Message> {
    let selected = Some(app.shared.selected_control);

    let first_body: Element<'_, Message> = column![
        text_input("Learner name", &app.shared.learner_name)
            .on_input(Message::SharedTextChanged)
            .padding(10),
        row![
            button("-1").on_press(Message::CounterDecremented),
            text(format!("Counter: {}", app.shared.shared_counter)),
            button("+1").on_press(Message::CounterIncremented),
        ]
        .spacing(12)
        .align_y(Alignment::Center),
        toggler(app.shared.controls_enabled)
            .label("Enable or disable related controls")
            .on_toggle(Message::ControlsToggled),
        checkbox("Checkbox state demo", app.shared.controls_checked)
            .on_toggle(Message::ControlsCheckboxChanged),
    ]
    .spacing(12)
    .into();

    let mut radio_column = column![].spacing(8);
    for choice in ControlChoice::ALL {
        radio_column = radio_column.push(radio(
            choice.label(),
            choice,
            selected,
            Message::ControlsChoiceSelected,
        ));
    }

    let second_body: Element<'_, Message> = column![
        slider(
            0..=100,
            app.shared.slider_value,
            Message::ControlsSliderChanged
        ),
        text(format!("Slider value: {}", app.shared.slider_value)),
        radio_column,
        pick_list(
            ControlChoice::ALL,
            selected,
            Message::ControlsChoiceSelected
        )
        .placeholder("Choose a widget family"),
        progress_bar(0.0..=100.0, f32::from(app.shared.progress_value)),
        button("Advance progress demo").on_press(Message::ProgressStepped),
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
        container(button("Reset sandbox state").on_press(Message::ResetSandbox)),
    ]
    .spacing(16);

    container(content).width(Length::Fill).into()
}
