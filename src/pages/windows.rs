//! Dedicated multi-window teaching demo.
//!
//! This module keeps the bulk of the window-management logic close to the page
//! that explains it. The actual Iced `window::open` / `window::close` tasks are
//! launched from `App::update`, but the registry and helper functions here stay
//! easy to unit test.
//!
//! ## Important API note for learners
//! In this repository we use the high-level `iced::application(...)` builder.
//! With Iced 0.13.x that builder renders the same `view` for every window and
//! widget messages do not automatically include the originating `window::Id`.
//! Therefore this demo can:
//! - open, focus, and close real native windows,
//! - track a real registry of `window::Id` values,
//! - react to close/open lifecycle events,
//! but it cannot give each child window a fully distinct widget tree without
//! switching the whole app to a lower-level multi-window program API.
//!
//! We keep the code educational by modeling the child-window state explicitly
//! anyway, and we add comments showing where a production app would route
//! per-window messages.

use iced::widget::{button, column, container, row, scrollable, text};
use iced::{window, Element, Length, Task};

use crate::app::App;
use crate::message::Message;
use crate::widgets;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WindowKind {
    Inspector,
    Notes,
    Preview,
    Toolbox,
}

impl WindowKind {
    pub const ALL: [Self; 4] = [Self::Inspector, Self::Notes, Self::Preview, Self::Toolbox];

    pub fn label(self) -> &'static str {
        match self {
            Self::Inspector => "Inspector",
            Self::Notes => "Notes / Help",
            Self::Preview => "Live Preview",
            Self::Toolbox => "Counter Toolbox",
        }
    }

    pub fn default_title(self) -> &'static str {
        match self {
            Self::Inspector => "Inspector Window",
            Self::Notes => "Notes & Help Window",
            Self::Preview => "Live Preview Window",
            Self::Toolbox => "Counter Toolbox Window",
        }
    }

    pub fn teaching_purpose(self) -> &'static str {
        match self {
            Self::Inspector => "Reflects the shared app state and window registry.",
            Self::Notes => "Carries guidance or study notes that can stay visible while you work.",
            Self::Preview => "Represents a child window created from root-state configuration.",
            Self::Toolbox => {
                "Owns independent local counter state while still reading global values."
            }
        }
    }

    pub fn allows_duplicates(self) -> bool {
        matches!(self, Self::Preview | Self::Toolbox)
    }
}

impl std::fmt::Display for WindowKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.label())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChildWindowState {
    pub local_counter: i32,
    pub scratchpad: String,
}

impl ChildWindowState {
    pub fn new(kind: WindowKind) -> Self {
        let scratchpad = match kind {
            WindowKind::Inspector => "Watching root-state changes.".into(),
            WindowKind::Notes => "Pin quick guidance here.".into(),
            WindowKind::Preview => "Preview reflects the root setting below.".into(),
            WindowKind::Toolbox => "Toolbox counter is local to this child record.".into(),
        };

        Self {
            local_counter: if matches!(kind, WindowKind::Toolbox) {
                1
            } else {
                0
            },
            scratchpad,
        }
    }

    pub fn reset(&mut self, kind: WindowKind) {
        *self = Self::new(kind);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WindowRecord {
    pub id: window::Id,
    pub kind: WindowKind,
    pub title: String,
    pub purpose: String,
    pub is_open: bool,
    pub local: ChildWindowState,
}

impl WindowRecord {
    pub fn new(id: window::Id, kind: WindowKind) -> Self {
        Self {
            id,
            kind,
            title: kind.default_title().into(),
            purpose: kind.teaching_purpose().into(),
            is_open: false,
            local: ChildWindowState::new(kind),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WindowDemoState {
    pub records: Vec<WindowRecord>,
    pub selected: Option<window::Id>,
    pub focused: Option<window::Id>,
    pub shared_label: String,
    pub next_preview_number: u32,
    pub status: String,
}

impl Default for WindowDemoState {
    fn default() -> Self {
        Self {
            records: Vec::new(),
            selected: None,
            focused: None,
            shared_label: "Preview the shared study plan".into(),
            next_preview_number: 1,
            status: "No child windows open yet. Use the buttons below to create them.".into(),
        }
    }
}

impl WindowDemoState {
    pub fn open_count(&self) -> usize {
        self.records.iter().filter(|record| record.is_open).count()
    }

    pub fn open_count_for(&self, kind: WindowKind) -> usize {
        self.records
            .iter()
            .filter(|record| record.is_open && record.kind == kind)
            .count()
    }

    pub fn can_open_kind(&self, kind: WindowKind) -> bool {
        kind.allows_duplicates() || !self.records.iter().any(|record| record.kind == kind)
    }

    pub fn selected_record(&self) -> Option<&WindowRecord> {
        let selected = self.selected?;
        self.records.iter().find(|record| record.id == selected)
    }

    pub fn selected_record_mut(&mut self) -> Option<&mut WindowRecord> {
        let selected = self.selected?;
        self.records.iter_mut().find(|record| record.id == selected)
    }

    pub fn register_open_request(&mut self, id: window::Id, kind: WindowKind) {
        if !self.can_open_kind(kind) {
            self.status = format!(
                "Skipped opening {} because this demo avoids duplicate singleton windows.",
                kind.label()
            );
            self.selected = self
                .records
                .iter()
                .find(|record| record.kind == kind)
                .map(|record| record.id);
            return;
        }

        let mut record = WindowRecord::new(id, kind);

        if matches!(kind, WindowKind::Preview | WindowKind::Toolbox) {
            let sequence = self.next_preview_number;
            self.next_preview_number += 1;
            record.title = format!("{} #{sequence}", kind.default_title());
            record.local.scratchpad = format!(
                "Created from root state while shared label was {:?}.",
                self.shared_label
            );
        }

        self.records.push(record);
        self.selected = Some(id);
        self.focused = Some(id);
        self.status = format!(
            "Opening {} and registering its handle immediately.",
            kind.label()
        );
    }

    pub fn mark_opened(&mut self, id: window::Id) {
        if let Some(record) = self.records.iter_mut().find(|record| record.id == id) {
            record.is_open = true;
            self.selected = Some(id);
            self.focused = Some(id);
            self.status = format!("{} is now open and tracked in app state.", record.title);
        }
    }

    pub fn select(&mut self, id: window::Id) {
        if self.records.iter().any(|record| record.id == id) {
            self.selected = Some(id);
            self.status = format!("Selected window handle {:?} for focus/close actions.", id);
        }
    }

    pub fn mark_focused(&mut self, id: window::Id) {
        if self
            .records
            .iter()
            .any(|record| record.id == id && record.is_open)
        {
            self.focused = Some(id);
            self.selected = Some(id);
            self.status = format!("Requested focus for window {:?}.", id);
        }
    }

    pub fn close_selected(&mut self) -> Option<window::Id> {
        let selected = self.selected?;
        if self
            .records
            .iter()
            .any(|record| record.id == selected && record.is_open)
        {
            self.status = format!("Closing selected window {:?}.", selected);
            Some(selected)
        } else {
            self.status = "The selected handle is already closed or stale.".into();
            None
        }
    }

    pub fn mark_closed(&mut self, id: window::Id) -> bool {
        if let Some(index) = self.records.iter().position(|record| record.id == id) {
            self.records.remove(index);
            if self.selected == Some(id) {
                self.selected = self
                    .records
                    .iter()
                    .find(|record| record.is_open)
                    .map(|record| record.id);
            }
            if self.focused == Some(id) {
                self.focused = self.selected;
            }
            self.status = format!(
                "Window {:?} closed. Registry cleanup removed stale handles from app state.",
                id
            );
            true
        } else {
            self.status = format!(
                "Received a close event for {:?}, but the registry was already clean.",
                id
            );
            false
        }
    }

    pub fn increment_selected_toolbox(&mut self) {
        if let Some(record) = self.selected_record_mut() {
            if matches!(record.kind, WindowKind::Toolbox) {
                record.local.local_counter += 1;
                self.status = format!(
                    "Updated toolbox-local counter for {:?} without touching shared global state.",
                    record.id
                );
                return;
            }
        }

        self.status = "Select a toolbox window to demonstrate local child state.".into();
    }
}

pub fn open_window_task(state: &mut WindowDemoState, kind: WindowKind) -> Task<Message> {
    if !state.can_open_kind(kind) {
        state.status = format!(
            "Skipped opening {} because this demo avoids duplicate singleton windows.",
            kind.label()
        );
        state.selected = state
            .records
            .iter()
            .find(|record| record.kind == kind)
            .map(|record| record.id);
        return Task::none();
    }

    let settings = window::Settings {
        size: match kind {
            WindowKind::Inspector => iced::Size::new(560.0, 420.0),
            WindowKind::Notes => iced::Size::new(620.0, 460.0),
            WindowKind::Preview => iced::Size::new(720.0, 520.0),
            WindowKind::Toolbox => iced::Size::new(420.0, 360.0),
        },
        exit_on_close_request: true,
        ..window::Settings::default()
    };

    let (id, task) = window::open(settings);
    state.register_open_request(id, kind);
    task.map(Message::WindowOpened)
}

pub fn view(app: &App) -> Element<'_, Message> {
    let window_state = &app.windows;
    let selected_record = window_state.selected_record();

    let actions = widgets::section_card(
        "Window controls",
        "These buttons perform real `iced::window` tasks while the registry below explains the state transitions.",
        column![
            text(format!(
                "Shared root label visible in every window record: {}",
                window_state.shared_label
            )),
            row![
                button("Open inspector")
                    .on_press(Message::WindowOpenRequested(WindowKind::Inspector)),
                button("Open notes/help")
                    .on_press(Message::WindowOpenRequested(WindowKind::Notes)),
            ]
            .spacing(10),
            row![
                button("Open live preview")
                    .on_press(Message::WindowOpenRequested(WindowKind::Preview)),
                button("Open toolbox")
                    .on_press(Message::WindowOpenRequested(WindowKind::Toolbox)),
            ]
            .spacing(10),
            row![
                button("Focus selected")
                    .on_press_maybe(window_state.selected.map(Message::WindowFocusRequested)),
                button("Close selected")
                    .on_press(Message::WindowCloseSelected),
                button("Bump selected toolbox counter")
                    .on_press(Message::WindowIncrementSelectedToolbox),
            ]
            .spacing(10),
            text("Lifecycle note: singleton windows like Inspector and Notes are deduplicated intentionally; Preview and Toolbox windows may be opened multiple times."),
        ]
        .spacing(12),
    );

    let summary = widgets::section_card(
        "Registry summary",
        "Model IDs, purpose, and local-vs-shared state explicitly so close events can clean up stale handles.",
        column![
            text(format!("Open windows: {}", window_state.open_count())),
            text(
                WindowKind::ALL
                    .into_iter()
                    .map(|kind| format!("{}: {}", kind.label(), window_state.open_count_for(kind)))
                    .collect::<Vec<_>>()
                    .join(" • "),
            ),
            text(format!(
                "Selected handle: {}",
                selected_record
                    .map(|record| format!("{:?} ({})", record.id, record.kind.label()))
                    .unwrap_or_else(|| "None".into())
            )),
            text(format!(
                "Focused handle: {}",
                window_state
                    .focused
                    .map(|id| format!("{:?}", id))
                    .unwrap_or_else(|| "None".into())
            )),
            text(format!("Status: {}", window_state.status)),
        ]
        .spacing(8),
    );

    let registry = window_state
        .records
        .iter()
        .fold(column![].spacing(10), |column, record| {
            let shared_counter = app.shared.shared_counter;
            column.push(
                button(
                    column![
                        text(format!("{} — {:?}", record.title, record.id)),
                        text(format!(
                            "Kind: {} • Open: {}",
                            record.kind.label(),
                            record.is_open
                        )),
                        text(format!("Purpose: {}", record.purpose)),
                        text(format!(
                            "Local counter: {} • Shared counter visible here: {}",
                            record.local.local_counter, shared_counter
                        )),
                        text(format!("Notes: {}", record.local.scratchpad)),
                    ]
                    .spacing(4),
                )
                .width(Length::Fill)
                .on_press(Message::WindowSelected(record.id)),
            )
        });

    let teaching = widgets::section_card(
        "Teaching scenario",
        "These records correspond to common child-window roles used in desktop apps.",
        column![
            text("• Inspector window: watches shared state and registry data."),
            text("• Notes/help window: keeps learning instructions visible while the main window stays interactive."),
            text("• Live preview window: created from root state, including the shared label shown above."),
            text("• Counter/toolbox window: demonstrates local child state that changes independently of global state."),
            text("Constraint note: because this app uses the high-level `iced::application` builder, widget messages do not include the originating window ID. The registry is real, but fully distinct per-window widget trees would require the lower-level multi-window program API."),
            text("Lifecycle note: always remove records on `window::close_events()` so stale handles cannot be focused after the OS closes a child window."),
            text("Coordination note: root messages update shared state, while child-record helpers mutate only per-window local fields like the toolbox counter."),
        ]
        .spacing(8),
    );

    let content = column![
        widgets::section_title("Windows"),
        widgets::note("This page is now a dedicated multi-window demo: it opens real Iced child windows and keeps a registry of their IDs, titles, purposes, and local payload/state."),
        actions,
        row![summary, teaching].spacing(16),
        widgets::section_card(
            "Open window registry",
            "Click any record to select it for focus or close actions.",
            scrollable(registry).height(220),
        ),
    ]
    .spacing(16);

    container(content).width(Length::Fill).into()
}

#[cfg(test)]
mod tests {
    use super::{ChildWindowState, WindowDemoState, WindowKind, WindowRecord};
    use iced::window;

    #[test]
    fn helper_maps_window_kinds_to_stable_titles_and_metadata() {
        assert_eq!(WindowKind::Inspector.default_title(), "Inspector Window");
        assert!(WindowKind::Notes.teaching_purpose().contains("guidance"));
        assert!(WindowKind::Preview.allows_duplicates());
        assert!(!WindowKind::Inspector.allows_duplicates());
    }

    #[test]
    fn registry_prevents_invalid_duplicate_singletons() {
        let mut state = WindowDemoState::default();
        let first_id = window::Id::unique();
        let duplicate_id = window::Id::unique();

        assert!(state.can_open_kind(WindowKind::Inspector));
        state.register_open_request(first_id, WindowKind::Inspector);
        assert!(!state.can_open_kind(WindowKind::Inspector));

        state.register_open_request(duplicate_id, WindowKind::Inspector);

        assert_eq!(state.records.len(), 1);
        assert_eq!(state.selected, Some(first_id));
    }

    #[test]
    fn opening_and_closing_updates_counts_and_selection() {
        let mut state = WindowDemoState::default();
        let id = window::Id::unique();
        state.register_open_request(id, WindowKind::Preview);
        state.mark_opened(id);

        assert_eq!(state.open_count(), 1);
        assert_eq!(state.open_count_for(WindowKind::Preview), 1);
        assert_eq!(state.selected, Some(id));

        assert!(state.mark_closed(id));
        assert_eq!(state.open_count(), 0);
        assert!(state.records.is_empty());
        assert_eq!(state.selected, None);
    }

    #[test]
    fn closing_unknown_window_keeps_registry_clean() {
        let mut state = WindowDemoState::default();
        let removed = state.mark_closed(window::Id::unique());

        assert!(!removed);
        assert!(state.records.is_empty());
        assert!(state.status.contains("already clean"));
    }

    #[test]
    fn child_window_state_constructor_and_reset_behave_as_expected() {
        let mut local = ChildWindowState::new(WindowKind::Toolbox);
        assert_eq!(local.local_counter, 1);
        local.local_counter = 99;
        local.reset(WindowKind::Toolbox);
        assert_eq!(local.local_counter, 1);
        assert!(local.scratchpad.contains("Toolbox counter"));
    }

    #[test]
    fn selecting_and_incrementing_toolbox_only_changes_local_state() {
        let mut state = WindowDemoState::default();
        let id = window::Id::unique();
        state.register_open_request(id, WindowKind::Toolbox);
        state.mark_opened(id);
        state.select(id);
        state.increment_selected_toolbox();

        let record = state.selected_record().expect("selected toolbox");
        assert_eq!(record.local.local_counter, 2);
    }

    #[test]
    fn window_record_constructor_uses_kind_metadata() {
        let id = window::Id::unique();
        let record = WindowRecord::new(id, WindowKind::Notes);

        assert_eq!(record.id, id);
        assert_eq!(record.title, "Notes & Help Window");
        assert!(record.purpose.contains("study notes"));
        assert!(!record.is_open);
    }
}
