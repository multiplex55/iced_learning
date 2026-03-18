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
    let notes = app
        .shared
        .notes
        .iter()
        .enumerate()
        .fold(column![], |column, (index, note)| {
            column.push(text(format!("{}. {}", index + 1, note)))
        });

    let content = column![
        widgets::section_title("Data flow"),
        text("Shared state lives in App, then gets projected into each page view."),
        text(format!("Active page: {}", app.active_page.label())),
        text(format!("Menu open: {}", app.menu_open)),
        notes.spacing(8),
    ]
    .spacing(12);

    container(content).width(Length::Fill).into()
}
