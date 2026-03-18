//! Layout examples.
//!
//! What to explore next: replace these nested columns and rows with responsive
//! containers, spacing experiments, or custom widgets.

use iced::widget::{column, container, row, text};
use iced::{Element, Length};

use crate::app::App;
use crate::message::Message;
use crate::widgets;

pub fn view(app: &App) -> Element<'_, Message> {
    let content = column![
        widgets::section_title("Layouts"),
        text("Shows how rows, columns, spacing, and container widths compose."),
        row![
            container(text("Sidebar-sized column"))
                .width(Length::FillPortion(1))
                .padding(12),
            container(text(format!(
                "Main content reacts to shared counter {}",
                app.shared.shared_counter
            )))
            .width(Length::FillPortion(2))
            .padding(12),
        ]
        .spacing(12),
    ]
    .spacing(12);

    container(content).width(Length::Fill).into()
}
