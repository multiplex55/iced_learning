//! Data flow page.
//!
//! This page teaches the central Iced idea: the `view` reads immutable state,
//! widgets emit messages, and `update` mutates state before the next render.

use iced::widget::{column, container, text};
use iced::{Element, Length};

use crate::app::App;
use crate::message::Message;
use crate::widgets;

pub fn view(app: &App) -> Element<'_, Message> {
    let mut notes = column![].spacing(8);
    for (index, note) in app.shared.notes.iter().enumerate() {
        notes = notes.push(text(format!("{}. {}", index + 1, note)));
    }

    let projection: Element<'_, Message> = column![
        text(format!("Active page: {}", app.active_page.label())),
        text(format!(
            "Selected control family: {}",
            app.shared.selected_control.label()
        )),
        text(format!(
            "Last menu action: {}",
            app.shared
                .last_menu_action
                .map(|action| action.label())
                .unwrap_or("none")
        )),
        text(format!("Tick subscription count: {}", app.shared.ticks)),
        notes,
    ]
    .spacing(10)
    .into();

    let content = column![
        widgets::section_title("DataFlow"),
        widgets::note("Read this page alongside App::update: every line below is derived from the current immutable state snapshot."),
        widgets::section_card(
            "Shared state projection",
            "One App owns the state and each page reads the bits it needs. That keeps event flow explicit for learners.",
            projection,
        ),
    ]
    .spacing(16);

    container(content).width(Length::Fill).into()
}
