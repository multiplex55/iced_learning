//! Data flow page.
//!
//! This page teaches the central Iced idea: the `view` reads immutable state,
//! widgets emit messages, and `update` mutates state before the next render.

use iced::widget::{button, checkbox, column, container, row, text, text_input};
use iced::{Element, Length};

use crate::app::App;
use crate::message::Message;
use crate::pages::Page;
use crate::widgets;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataFlowMessage {
    ProfileNameEdited(String),
    DashboardStatusEdited(String),
    SidebarTipsToggled(bool),
    ResetSharedState,
}

impl DataFlowMessage {
    pub fn event_label(&self) -> String {
        match self {
            Self::ProfileNameEdited(value) => {
                format!("DataFlow page emitted ProfileNameEdited({value:?})")
            }
            Self::DashboardStatusEdited(value) => {
                format!("DataFlow page emitted DashboardStatusEdited({value:?})")
            }
            Self::SidebarTipsToggled(value) => {
                format!("DataFlow page emitted SidebarTipsToggled({value})")
            }
            Self::ResetSharedState => "DataFlow page emitted ResetSharedState".into(),
        }
    }
}

pub fn view(app: &App) -> Element<'_, Message> {
    let event_pipeline: Element<'_, Message> = column![
        text("1. A widget on this page emits DataFlowMessage."),
        text("2. Message::from(DataFlowMessage) forwards it into the root Message enum."),
        text("3. App::update applies the shared-state reducer as the canonical mutation point."),
        text("4. Dashboard, footer, and this preview re-read the same shared snapshot."),
    ]
    .spacing(8)
    .into();

    // Unidirectional data flow note: these widgets never mutate local copies.
    // They only emit page-scoped messages, and the root app decides how shared
    // state changes before every page renders again.
    let controls: Element<'_, Message> = column![
        text_input("Profile name", &app.shared.learner_name)
            .on_input(|value| DataFlowMessage::ProfileNameEdited(value).into())
            .padding(10),
        text_input("Dashboard status message", &app.shared.dashboard_status)
            .on_input(|value| DataFlowMessage::DashboardStatusEdited(value).into())
            .padding(10),
        checkbox(
            "Show sidebar teaching tips in the shell",
            app.shared.show_sidebar_tips
        )
        .on_toggle(|value| DataFlowMessage::SidebarTipsToggled(value).into()),
        button("Reset shared state demo").on_press(DataFlowMessage::ResetSharedState.into()),
    ]
    .spacing(12)
    .into();

    let preview: Element<'_, Message> = column![
        text(format!(
            "Profile preview: {}",
            app.shared.profile_preview_name()
        )),
        text(format!(
            "Dashboard summary: {}",
            app.shared.dashboard_summary()
        )),
        text(format!(
            "Shell preference summary: {}",
            app.shared.visibility_summary()
        )),
        text(format!(
            "Status last changed by: {}",
            app.shared.status_last_changed_by.label()
        )),
    ]
    .spacing(10)
    .into();

    let state_summary: Element<'_, Message> = column![
        text(format!("Last event: {}", app.shared.last_event)),
        text(format!(
            "Current shared state: name={:?}, status={:?}, sidebar_tips={}, dark_mode={}",
            app.shared.learner_name,
            app.shared.dashboard_status,
            app.shared.show_sidebar_tips,
            app.shared.theme_choice.is_dark(),
        )),
        text(format!(
            "Which page changed the profile name? {}",
            app.shared.profile_last_changed_by.label()
        )),
        text(format!(
            "Which page changed the status message? {}",
            app.shared.status_last_changed_by.label()
        )),
        text(format!(
            "Which page changed the shell preference? {}",
            app.shared.preference_last_changed_by.label()
        )),
    ]
    .spacing(8)
    .into();

    let content = column![
        widgets::section_title("DataFlow"),
        widgets::note("Read this page alongside App::update and the shared-state reducer: the goal is to make each transition traceable."),
        widgets::section_card(
            "Trace the event pipeline",
            "Follow the same message from widget emission to root-state mutation to cross-page rendering.",
            event_pipeline,
        ),
        row![
            widgets::section_card(
                "Emit page-level messages",
                "Each input emits a DataFlowMessage. That page-level enum is forwarded into the root Message enum before the app mutates shared state.",
                controls,
            ),
            widgets::section_card(
                "Consume shared and derived state",
                "The preview panel reads direct shared state and also derived text helpers built from that state.",
                preview,
            ),
        ]
        .spacing(16),
        widgets::section_card(
            "Instrumentation",
            "These labels make the reducer's effects visible for learners while keeping mutation logic centralized.",
            state_summary,
        ),
        widgets::section_card(
            "Where else can you see this state?",
            "Open Dashboard or watch the footer while editing the fields above. Those pages read the same shared state without owning conflicting copies.",
            column![
                text(format!("Dashboard page label: {} (id: {})", Page::Dashboard.label(), Page::Dashboard.id())),
                text(format!("App shell status line: {}", app.shared.status_line)),
            ]
            .spacing(8),
        ),
    ]
    .spacing(16);

    container(content).width(Length::Fill).into()
}
