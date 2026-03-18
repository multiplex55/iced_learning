//! Advanced demos.

use iced::widget::{column, container, text, toggler};
use iced::{Background, Border, Color, Element, Length, Theme};

use crate::app::App;
use crate::message::Message;
use crate::widgets;

pub fn view(app: &App) -> Element<'_, Message> {
    let themed_box: Element<'_, Message> = container(text(
        "Custom styled container: inspect this closure for a lightweight styling example.",
    ))
    .padding(16)
    .width(Length::Fill)
    .style(|theme: &Theme| {
        let palette = theme.extended_palette();

        iced::widget::container::Style {
            background: Some(Background::Color(palette.primary.strong.color)),
            text_color: Some(Color::WHITE),
            border: Border {
                color: palette.primary.base.color,
                width: 1.0,
                radius: 14.0.into(),
            },
            ..Default::default()
        }
    })
    .into();

    let body: Element<'_, Message> = column![
        toggler(app.shared.dark_mode_demo)
            .label("Use dark theme for the whole sandbox")
            .on_toggle(Message::AdvancedThemeToggled),
        themed_box,
        text(format!("Subscription ticks observed: {}", app.shared.ticks)),
    ]
    .spacing(12)
    .into();

    let content = column![
        widgets::section_title("Advanced"),
        widgets::note("This page demonstrates theme switching, custom styles, and a simple time subscription."),
        widgets::section_card(
            "Theming and styling",
            "Core iced styling often uses closures over Theme; this demo keeps the style function local so learners can read it in one file.",
            body,
        ),
    ]
    .spacing(16);

    container(content).width(Length::Fill).into()
}
