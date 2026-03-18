//! Top-level application state and orchestration.
//!
//! Iced applications are usually modeled around two core functions:
//! - `update`, which handles a `Message` and mutates state.
//! - `view`, which renders widgets from the current immutable state.

use iced::time;
use iced::widget::{button, column, container, row, text};
use iced::{Alignment, Element, Length, Subscription, Task, Theme};
use iced_aw::{TabLabel, Tabs};

use crate::message::{MenuAction, Message};
use crate::pages::{self, Page};
use crate::state::SharedState;
use crate::widgets;

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
}

impl Default for App {
    fn default() -> Self {
        Self {
            active_page: Page::Dashboard,
            shared: SharedState::default(),
            window: DemoWindowState::default(),
        }
    }
}

impl App {
    pub fn run() -> iced::Result {
        iced::application(Self::title, Self::update, Self::view)
            .subscription(Self::subscription)
            .theme(Self::theme)
            .centered()
            .run_with(Self::boot)
    }

    pub fn boot() -> (Self, Task<Message>) {
        (Self::default(), Task::none())
    }

    pub fn title(&self) -> String {
        format!("Iced Learning Sandbox — {}", self.active_page.label())
    }

    pub fn theme(&self) -> Theme {
        if self.shared.dark_mode_demo {
            Theme::TokyoNight
        } else {
            Theme::Light
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        time::every(std::time::Duration::from_secs(1)).map(|_| Message::Tick)
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Navigate(page) => self.active_page = page,
            Message::MenuSelected(action) => self.apply_menu_action(action),
            Message::SharedTextChanged(value) => self.shared.learner_name = value,
            Message::CounterIncremented => self.shared.shared_counter += 1,
            Message::CounterDecremented => self.shared.shared_counter -= 1,
            Message::ControlsToggled(value) => self.shared.controls_enabled = value,
            Message::ControlsCheckboxChanged(value) => self.shared.controls_checked = value,
            Message::ControlsSliderChanged(value) => self.shared.slider_value = value,
            Message::ControlsChoiceSelected(choice) => self.shared.selected_control = choice,
            Message::ProgressStepped => {
                self.shared.progress_value = (self.shared.progress_value + 10).min(100)
            }
            Message::AdvancedThemeToggled(is_dark) => self.shared.dark_mode_demo = is_dark,
            Message::Tick => self.shared.ticks += 1,
            Message::ToggleChildWindow => self.window.is_open = !self.window.is_open,
            Message::ResetSandbox => *self = Self::default(),
        }

        self.shared.status_line = self.status_text();
        Task::none()
    }

    pub fn apply_menu_action(&mut self, action: MenuAction) {
        self.shared.last_menu_action = Some(action);

        match action {
            MenuAction::NewSandbox => *self = Self::default(),
            MenuAction::OpenRecipe => {
                self.active_page = Page::Layouts;
                self.shared
                    .notes
                    .push("Opened the layout recipe from the dashboard menu.".into());
            }
            MenuAction::SaveSnapshot => {
                self.shared.notes.push(format!(
                    "Saved snapshot for {} with counter {}.",
                    self.shared.learner_name, self.shared.shared_counter
                ));
            }
            MenuAction::ExportCode => self.active_page = Page::DataFlow,
            MenuAction::ToggleSidebarTips => {
                self.shared.show_sidebar_tips = !self.shared.show_sidebar_tips
            }
            MenuAction::FocusControlsPage => self.active_page = Page::Controls,
            MenuAction::OpenInspectorWindow => self.window.is_open = true,
            MenuAction::ArrangeStudyLayout => self.active_page = Page::Windows,
            MenuAction::ViewDocs => self.active_page = Page::Advanced,
            MenuAction::AboutSandbox => self.active_page = Page::Dashboard,
        }
    }

    fn status_text(&self) -> String {
        let action = self
            .shared
            .last_menu_action
            .map(MenuAction::label)
            .unwrap_or("No menu action yet");

        format!(
            "Active page: {} • Last menu action: {} • Tick demo: {}",
            self.active_page.label(),
            action,
            self.shared.ticks
        )
    }

    pub fn view(&self) -> Element<'_, Message> {
        let tabs = Page::ALL.into_iter().fold(
            Tabs::new(Message::Navigate).set_active_tab(&self.active_page),
            |tabs, page| {
                tabs.push(
                    page,
                    TabLabel::Text(page.label().into()),
                    self.page_content(page),
                )
            },
        );

        let header = column![
            row![
                text("Iced learning sandbox").size(30),
                button("Reset all demos").on_press(Message::ResetSandbox),
            ]
            .spacing(16)
            .align_y(Alignment::Center),
            text(format!(
                "A single shell hosts focused learning pages. Current lesson: {}",
                self.active_page.lesson()
            )),
            widgets::status_banner(self.shared.status_line.clone()),
        ]
        .spacing(12);

        let footer = container(text(format!(
            "Footer/status bar: sidebar tips {}, child window {}, current theme {}.",
            if self.shared.show_sidebar_tips {
                "enabled"
            } else {
                "hidden"
            },
            if self.window.is_open {
                "open"
            } else {
                "closed"
            },
            if self.shared.dark_mode_demo {
                "dark"
            } else {
                "light"
            },
        )))
        .padding([10, 12]);

        let content = column![header, tabs, footer].spacing(16).padding(20);

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
    use super::App;
    use crate::message::{MenuAction, Message};
    use crate::pages::Page;

    #[test]
    fn navigation_messages_activate_expected_pages() {
        for page in Page::ALL {
            let mut app = App::default();
            let _ = app.update(Message::Navigate(page));
            assert_eq!(app.active_page, page);
        }
    }

    #[test]
    fn menu_actions_update_expected_state() {
        let scenarios = [
            (MenuAction::OpenRecipe, Page::Layouts),
            (MenuAction::FocusControlsPage, Page::Controls),
            (MenuAction::ArrangeStudyLayout, Page::Windows),
            (MenuAction::ViewDocs, Page::Advanced),
        ];

        for (action, expected_page) in scenarios {
            let mut app = App::default();
            app.apply_menu_action(action);
            assert_eq!(app.active_page, expected_page);
            assert_eq!(app.shared.last_menu_action, Some(action));
        }
    }

    #[test]
    fn toggle_sidebar_menu_action_flips_visible_state() {
        let mut app = App::default();
        let start = app.shared.show_sidebar_tips;

        app.apply_menu_action(MenuAction::ToggleSidebarTips);

        assert_eq!(app.shared.show_sidebar_tips, !start);
        assert_eq!(
            app.shared.last_menu_action,
            Some(MenuAction::ToggleSidebarTips)
        );
    }
}
