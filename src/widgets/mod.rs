//! Reusable view helpers.
//!
//! These helpers are intentionally lightweight. They reduce repetition without
//! hiding Iced concepts behind a heavy component system.

use iced::widget::{column, container, text};
use iced::{Background, Border, Color, Element, Length, Theme};
use iced_aw::Card;

use crate::message::Message;

/// Builds a simple boxed section title for page content.
pub fn section_title<'a>(label: &'a str) -> Element<'a, Message> {
    container(text(label).size(24))
        .width(Length::Fill)
        .padding([4, 0])
        .into()
}

pub fn note<'a>(label: &'a str) -> Element<'a, Message> {
    text(label).size(14).into()
}

pub fn section_card<'a>(
    title: &'a str,
    note_text: &'a str,
    body: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    // iced_aw::Card is an ecosystem widget; unlike core `iced`, it gives us a
    // ready-made title/body container that is nice for teaching grouped demos.
    Card::new(
        text(title).size(20),
        column![text(note_text).size(14), body.into()].spacing(12),
    )
    .foot(text(
        "Code-reading hint: inspect the page module view/update pair for this section.",
    ))
    .style(|_theme: &Theme, _status| iced_aw::style::card::Style {
        background: Background::Color(Color::from_rgb8(17, 20, 32)),
        border_radius: 12.0,
        border_width: 1.0,
        border_color: Color::from_rgb8(124, 77, 255),
        head_background: Background::Color(Color::from_rgb8(39, 45, 71)),
        head_text_color: Color::WHITE,
        body_background: Background::Color(Color::from_rgb8(24, 28, 43)),
        body_text_color: Color::WHITE,
        foot_background: Background::Color(Color::from_rgb8(30, 34, 52)),
        foot_text_color: Color::from_rgb8(210, 214, 230),
        close_color: Color::WHITE,
    })
    .padding(16.into())
    .width(Length::Fill)
    .into()
}

pub fn status_banner<'a>(label: String) -> Element<'a, Message> {
    container(text(label).size(14))
        .padding(10)
        .width(Length::Fill)
        .style(|_theme: &Theme| iced::widget::container::Style {
            background: Some(Background::Color(Color::from_rgb8(30, 53, 96))),
            text_color: Some(Color::WHITE),
            border: Border {
                color: Color::from_rgb8(124, 77, 255),
                width: 1.0,
                radius: 10.0.into(),
            },
            ..Default::default()
        })
        .into()
}
