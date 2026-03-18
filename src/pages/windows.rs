//! Window and modal notes.
//!
//! The current implementation keeps window state local to the app model so it
//! stays easy to understand before introducing actual multi-window tasks.

use iced::widget::{button, column, container, text};
use iced::{Element, Length};

use crate::app::App;
use crate::message::Message;
use crate::widgets;

pub fn view(app: &App) -> Element<'_, Message> {
    let content = column![
        widgets::section_title("Windows"),
        text("Tracks child-window intent without overengineering the first pass."),
        text(format!("Child window open: {}", app.window.is_open)),
        text(format!("Child window title: {}", app.window.title)),
        button(if app.window.is_open {
            "Close child window demo"
        } else {
            "Open child window demo"
        })
        .on_press(Message::ToggleChildWindow),
        text("What to explore next: wire this state into iced::window::open and close tasks."),
    ]
    .spacing(12);

    container(content).width(Length::Fill).into()
}
