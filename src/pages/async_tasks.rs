//! Async task demo.
//!
//! This page uses a tiny simulated background job to show where `Task` fits in
//! the Iced architecture. The delay is educational rather than production-useful.

use iced::widget::{button, column, container, progress_bar, row, text};
use iced::{Element, Length};

use crate::app::App;
use crate::message::Message;
use crate::widgets;

pub fn view(app: &App) -> Element<'_, Message> {
    let status = if app.async_demo.is_loading {
        format!(
            "Loading lesson {} of {}...",
            app.async_demo.completed_steps + 1,
            app.async_demo.total_steps
        )
    } else {
        app.async_demo.status.clone()
    };

    // Educational note: the progress bar is derived from task state. In Iced,
    // you usually do not mutate widgets directly; you mutate the state they read.
    let task_controls = column![
        text(status),
        progress_bar(0.0..=1.0, app.async_demo.progress_ratio()),
        row![
            button("Start simulated fetch")
                .on_press_maybe((!app.async_demo.is_loading).then_some(Message::AsyncStarted)),
            button("Reset async demo").on_press(Message::AsyncReset),
        ]
        .spacing(12),
        text(format!("Completed result: {}", app.async_demo.last_result)),
        text("Interesting item to notice: the async reducer only tracks booleans, counts, and strings. The widgets are thin consumers of that state."),
    ]
    .spacing(12);

    let content = column![
        widgets::section_title("Async / Tasks"),
        widgets::note("Use this page after reading App::update: it shows how a task starts in response to a message and finishes by emitting another message."),
        widgets::section_card(
            "Task lifecycle",
            "Start a background action, show loading feedback, and keep the result formatting separate from the widget tree.",
            task_controls,
        ),
    ]
    .spacing(16);

    container(content).width(Length::Fill).into()
}
