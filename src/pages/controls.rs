//! Control widgets page.
//!
//! These controls are intentionally simple and favor clarity over production
//! validation, error messaging, or accessibility polish.

use iced::widget::{button, column, container, row, text, text_input};
use iced::{Element, Length};

use crate::app::App;
use crate::message::Message;
use crate::widgets;

pub fn view(app: &App) -> Element<'_, Message> {
    let content = column![
        widgets::section_title("Controls"),
        text("Interact with shared state and watch every page reflect the changes."),
        text_input("Learner name", &app.shared.learner_name)
            .on_input(Message::SharedTextChanged)
            .padding(10),
        row![
            button("-1").on_press(Message::CounterDecremented),
            text(format!("Counter: {}", app.shared.shared_counter)),
            button("+1").on_press(Message::CounterIncremented),
        ]
        .spacing(12),
        button("Reset sandbox state").on_press(Message::ResetSandbox),
    ]
    .spacing(12);

    container(content).width(Length::Fill).into()
}
