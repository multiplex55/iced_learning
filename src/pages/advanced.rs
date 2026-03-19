//! Advanced demos.

use iced::widget::{column, container, pick_list, text};
use iced::{Background, Border, Color, Element, Length, Theme};

use crate::app::App;
use crate::message::Message;
use crate::theme::ThemeChoice;
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
        pick_list(ThemeChoice::ALL, Some(app.shared.theme_choice), Message::ThemeSelected)
            .placeholder("Choose a sandbox theme"),
        text(format!("Current theme note: {}", app.shared.theme_choice.teaching_note())),
        themed_box,
        text(format!("Subscription ticks observed: {}", app.shared.ticks)),
        text("Interesting item to notice: core iced styling is closure-based, while the card helper uses iced_aw to avoid repeating container scaffolding in every page."),
        text("Version note: the high-level application builder and Task API are Iced 0.13-era patterns; older guides may still refer to Command.")
    ]
    .spacing(12)
    .into();

    let content = column![
        widgets::section_title("Styling / Theming"),
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
