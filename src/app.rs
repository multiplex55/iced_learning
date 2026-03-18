//! Top-level application state and orchestration.
//!
//! Iced applications are usually modeled around two core functions:
//! - `update`, which handles a `Message` and mutates state.
//! - `view`, which renders widgets from the current immutable state.
//!
//! This module keeps those ideas explicit so learners can inspect the full flow
//! before introducing asynchronous tasks, services, or deeper abstractions.

use iced::widget::{button, column, container, row, text};
use iced::{Element, Length, Task, Theme};
use iced_aw::{TabLabel, Tabs};

use crate::message::Message;
use crate::pages;
use crate::state::SharedState;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Page {
    Dashboard,
    Layouts,
    Controls,
    DataFlow,
    Windows,
    Advanced,
}

impl Page {
    pub const ALL: [Self; 6] = [
        Self::Dashboard,
        Self::Layouts,
        Self::Controls,
        Self::DataFlow,
        Self::Windows,
        Self::Advanced,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::Dashboard => "Dashboard",
            Self::Layouts => "Layouts",
            Self::Controls => "Controls",
            Self::DataFlow => "Data flow",
            Self::Windows => "Windows",
            Self::Advanced => "Advanced",
        }
    }

    fn tab_label(self) -> TabLabel {
        TabLabel::Text(self.label().into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DemoWindowState {
    pub is_open: bool,
    pub title: String,
}

impl Default for DemoWindowState {
    fn default() -> Self {
        Self {
            is_open: false,
            title: "Inspector".into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct App {
    pub active_page: Page,
    pub shared: SharedState,
    pub window: DemoWindowState,
    pub menu_open: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            active_page: Page::Dashboard,
            shared: SharedState::default(),
            window: DemoWindowState::default(),
            menu_open: false,
        }
    }
}

impl App {
    pub fn run() -> iced::Result {
        iced::application(Self::title, Self::update, Self::view)
            .theme(|_| Theme::TokyoNight)
            .centered()
            .run_with(Self::boot)
    }

    pub fn boot() -> (Self, Task<Message>) {
        (Self::default(), Task::none())
    }

    pub fn title(&self) -> String {
        format!("Iced Learning Sandbox — {}", self.active_page.label())
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Navigate(page) => self.active_page = page,
            Message::SharedTextChanged(value) => self.shared.learner_name = value,
            Message::CounterIncremented => self.shared.shared_counter += 1,
            Message::CounterDecremented => self.shared.shared_counter -= 1,
            Message::ToggleMenu => self.menu_open = !self.menu_open,
            Message::ToggleChildWindow => self.window.is_open = !self.window.is_open,
            Message::ResetSandbox => *self = Self::default(),
        }

        Task::none()
    }

    pub fn navigation_items() -> Vec<(Page, &'static str)> {
        Page::ALL
            .into_iter()
            .map(|page| (page, page.label()))
            .collect()
    }

    pub fn view(&self) -> Element<'_, Message> {
        // The navigation is intentionally implemented with iced_aw tabs so the
        // codebase demonstrates an ecosystem widget early on.
        let tabs = Page::ALL.into_iter().fold(
            Tabs::new(Message::Navigate).set_active_tab(&self.active_page),
            |tabs, page| tabs.push(page, page.tab_label(), self.page_content(page)),
        );

        let header = row![
            text("Iced learning sandbox").size(28),
            button(if self.menu_open {
                "Hide menu state"
            } else {
                "Show menu state"
            })
            .on_press(Message::ToggleMenu),
        ]
        .spacing(16);

        let content = column![
            header,
            text("Update mutates state; view renders the latest snapshot."),
            tabs,
        ]
        .spacing(16)
        .padding(20);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn page_content(&self, page: Page) -> Element<'_, Message> {
        match page {
            Page::Dashboard => pages::dashboard::view(self),
            Page::Layouts => pages::layouts::view(self),
            Page::Controls => pages::controls::view(self),
            Page::DataFlow => pages::data_flow::view(self),
            Page::Windows => pages::windows::view(self),
            Page::Advanced => pages::advanced::view(self),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::{App, Page};

    #[test]
    fn default_app_starts_on_dashboard() {
        let app = App::default();
        assert_eq!(app.active_page, Page::Dashboard);
    }

    #[test]
    fn default_shared_state_is_consistent() {
        let app = App::default();
        assert_eq!(app.shared.learner_name, "Iced explorer");
        assert_eq!(app.shared.shared_counter, 3);
        assert_eq!(app.shared.notes.len(), 3);
        assert!(!app.window.is_open);
        assert!(!app.menu_open);
    }

    #[test]
    fn navigation_metadata_is_complete_and_unique() {
        let items = App::navigation_items();
        assert_eq!(items.len(), Page::ALL.len());

        let unique_labels: HashSet<_> = items.iter().map(|(_, label)| *label).collect();
        let unique_pages: HashSet<_> = items.iter().map(|(page, _)| *page).collect();

        assert_eq!(unique_labels.len(), items.len());
        assert_eq!(unique_pages.len(), items.len());
    }

    #[test]
    fn boot_produces_stable_defaults() {
        let (booted, _task) = App::boot();
        assert_eq!(booted, App::default());
    }
}
