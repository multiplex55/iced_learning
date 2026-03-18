//! Reusable view helpers.
//!
//! These helpers are intentionally lightweight. They reduce repetition without
//! hiding Iced concepts behind a heavy component system.

use iced::widget::{container, text};
use iced::{Element, Length};

use crate::message::Message;

/// Builds a simple boxed section title for page content.
pub fn section_title<'a>(label: &'a str) -> Element<'a, Message> {
    container(text(label).size(24))
        .width(Length::Fill)
        .padding([4, 0])
        .into()
}
