//! Window and modal notes.

use iced::widget::{button, column, container, text};
use iced::{Element, Length};

use crate::app::App;
use crate::message::Message;
use crate::widgets;

pub fn view(app: &App) -> Element<'_, Message> {
    let content = column![
        widgets::section_title("Windows"),
        widgets::note("This page keeps the first window demo stateful but simple: it models intent before introducing real multi-window tasks."),
        widgets::section_card(
            "Window state",
            "The button below flips a boolean in App. Learners can later replace that with iced window open/close tasks.",
            column![
                text(format!("Child window open: {}", app.window.is_open)),
                text(format!("Child window title: {}", app.window.title)),
                button(if app.window.is_open {
                    "Close child window demo"
                } else {
                    "Open child window demo"
                })
                .on_press(Message::ToggleChildWindow),
                text("Teaching note: pair this with dashboard menu actions to see shared shell state in motion."),
            ]
            .spacing(12),
        ),
    ]
    .spacing(16);

    container(content).width(Length::Fill).into()
}
