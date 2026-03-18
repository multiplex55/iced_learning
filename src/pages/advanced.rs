//! Advanced demos page.
//!
//! This page points learners at feature-gated topics like markdown, SVG, menus,
//! tabs, and custom widgets.

use iced::widget::{column, container, text};
use iced::{Element, Length};

use crate::app::App;
use crate::message::Message;
use crate::widgets;

pub fn view(_app: &App) -> Element<'_, Message> {
    let content = column![
        widgets::section_title("Advanced demos"),
        text("Enabled features in Cargo.toml include markdown, SVG, tabs, menus, and badges."),
        text("What to explore next: add a markdown renderer, SVG icon gallery, or iced_aw menu demo."),
        text("Production note: examples stay intentionally small so the update/view loop remains visible."),
    ]
    .spacing(12);

    container(content).width(Length::Fill).into()
}
