//! Dashboard page.
//!
//! Start here if you are new to Iced: it shows the current application state
//! without introducing many interactive widgets at once.

use iced::widget::{column, container, text};
use iced::{Element, Length};

use crate::app::App;
use crate::message::Message;
use crate::theme;
use crate::widgets;

pub fn view(app: &App) -> Element<'_, Message> {
    let content = column![
        widgets::section_title("Dashboard"),
        text("This landing page summarizes the sandbox and points at the next demos."),
        text(format!("Accent color helper: {}", theme::accent_hex())),
        text(format!("Learner profile: {}", app.shared.learner_name)),
        text(format!("Shared counter: {}", app.shared.shared_counter)),
        text(format!(
            "Open child window example active: {}",
            app.window.is_open
        )),
    ]
    .spacing(12);

    container(content).width(Length::Fill).into()
}
