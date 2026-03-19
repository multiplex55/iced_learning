//! About page for the sandbox.

use iced::widget::{column, container, text};
use iced::{Element, Length};

use crate::app::App;
use crate::message::Message;
use crate::widgets;

pub fn view(app: &App) -> Element<'_, Message> {
    let content = column![
        widgets::section_title("About Sandbox"),
        widgets::note("This page is intentionally distinct from the dashboard so Help actions describe exactly what happens."),
        widgets::section_card(
            "Project details",
            "Use this page when learners need quick context about the repository and why certain actions stay in-app.",
            column![
                text(format!("Package: {}", env!("CARGO_PKG_NAME"))),
                text(format!("Version: {}", env!("CARGO_PKG_VERSION"))),
                text(format!("Description: {}", env!("CARGO_PKG_DESCRIPTION"))),
                text("Purpose: a compact Iced learning sandbox for menu routing, page state, shared state, and multi-window demos."),
                text(format!("Latest menu action: {}", app.shared.last_menu_action.map(|a| a.label()).unwrap_or("None"))),
                text(format!("Teaching notes captured: {}", app.shared.notes.len())),
            ]
            .spacing(8),
        ),
    ]
    .spacing(16);

    container(content).width(Length::Fill).into()
}
