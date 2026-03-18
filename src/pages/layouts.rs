//! Layout examples.

use iced::widget::{column, container, horizontal_rule, row, scrollable, text};
use iced::{Alignment, Element, Length};

use crate::app::App;
use crate::message::Message;
use crate::widgets;

pub fn view(app: &App) -> Element<'_, Message> {
    let left_body: Element<'_, Message> = column![
        text("Left section uses FillPortion(1) like a sidebar."),
        text(format!(
            "Shared counter echoed here: {}",
            app.shared.shared_counter
        )),
    ]
    .spacing(8)
    .into();

    let right_body: Element<'_, Message> = column![
        text("Centered teaching callout inside a padded container."),
        text("Try resizing the window to see how Fill and FillPortion cooperate."),
    ]
    .spacing(8)
    .into();

    let responsive_row = row![
        widgets::section_card(
            "Rows and columns",
            "Rows place children horizontally; columns stack them vertically. Spacing and padding make the layout readable.",
            left_body,
        ),
        widgets::section_card(
            "Containers and alignment",
            "Containers add padding, boundaries, and room for later styling without changing the underlying child widget.",
            right_body,
        )
    ]
    .spacing(16)
    .align_y(Alignment::Start);

    let mut scroll_notes = column![].spacing(8);
    for (index, note) in app.shared.notes.iter().enumerate() {
        scroll_notes = scroll_notes
            .push(container(text(format!("Scrollable note {}: {}", index + 1, note))).padding(8));
    }

    let scroll_body: Element<'_, Message> = scrollable(
        column![
            text("Responsive section list:"),
            text("• Rows distribute horizontal space."),
            text("• Columns make readable stacks."),
            text("• Containers isolate styling and padding."),
            text("• Scrollables clip and reveal extra content."),
            scroll_notes,
        ]
        .spacing(8),
    )
    .height(180)
    .into();

    let content = column![
        widgets::section_title("Layouts"),
        widgets::note("This page focuses on how layout widgets compose, not on heavy application logic."),
        responsive_row,
        horizontal_rule(1),
        widgets::section_card(
            "Scrollable area",
            "Scrollable lets long content remain accessible without forcing the whole window to grow.",
            scroll_body,
        ),
    ]
    .spacing(16);

    container(content).width(Length::Fill).into()
}
