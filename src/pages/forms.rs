//! Forms and validation demo.
//!
//! Educational note: text inputs in Iced do not own your application data.
//! They render whatever string you pass in, so the draft lives in app state and
//! the widgets simply emit changes.

use iced::widget::{button, column, container, row, text, text_input};
use iced::{Element, Length};

use crate::app::App;
use crate::message::Message;
use crate::widgets;

fn error_lines(lines: Vec<String>) -> Element<'static, Message> {
    if lines.is_empty() {
        text("Validation status: ready to submit.").into()
    } else {
        lines
            .into_iter()
            .fold(column![], |column, message| {
                column.push(text(format!("• {message}")))
            })
            .spacing(4)
            .into()
    }
}

pub fn view(app: &App) -> Element<'_, Message> {
    let errors = app.form_draft.validate();
    let can_submit = errors.is_valid();

    // This page is a production-leaning recommendation: keep parsing and
    // validation pure, then let `view` merely describe the current state.
    let editor = column![
        text_input("Display name", &app.form_draft.name)
            .on_input(Message::FormNameChanged)
            .padding(10),
        text_input("Email", &app.form_draft.email)
            .on_input(Message::FormEmailChanged)
            .padding(10),
        text_input("What do you want to build next?", &app.form_draft.goal)
            .on_input(Message::FormGoalChanged)
            .padding(10),
        error_lines(errors.messages()),
        row![
            button("Load example form").on_press(Message::FormLoadExample),
            button("Submit draft").on_press_maybe(can_submit.then_some(Message::FormSubmitted)),
        ]
        .spacing(12),
    ]
    .spacing(12);

    let preview = column![
        text(format!("Name preview: {}", app.form_draft.name.trim())),
        text(format!("Email preview: {}", app.form_draft.email.trim())),
        text(format!("Summary: {}", app.form_submission_summary)),
        text(format!(
            "Interesting item to notice: validation lives in src/forms.rs, so these widgets stay declarative even as rules grow."
        )),
    ]
    .spacing(10);

    let content = column![
        widgets::section_title("Forms / Validation"),
        widgets::note("Controlled inputs plus pure validation are easier to test than UI-driven form behavior."),
        row![
            widgets::section_card(
                "Controlled inputs",
                "The draft is owned by App state, not by the widget tree. That makes resets, previews, and tests deterministic.",
                editor,
            ),
            widgets::section_card(
                "Preview and teaching notes",
                "Use a side-by-side preview to show how the same form state can feed multiple views without cloning entire widgets.",
                preview,
            )
        ]
        .spacing(16),
    ]
    .spacing(16);

    container(content).width(Length::Fill).into()
}
