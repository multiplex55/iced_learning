//! Top-level application state and orchestration.
//!
//! Iced applications are usually modeled around two core functions:
//! - `update`, which handles a `Message` and mutates state.
//! - `view`, which renders widgets from the current immutable state.

use iced::time;
use iced::widget::{button, column, container, row, text};
use iced::{window, Alignment, Element, Length, Subscription, Task, Theme};
use iced_aw::{TabLabel, Tabs};

use crate::forms::FormDraft;
use crate::message::{MenuAction, Message};
use crate::pages::{self, data_flow::DataFlowMessage, windows, Page};
use crate::state::reducer::{apply_shared_state_action, SharedStateAction};
use crate::state::SharedState;
use crate::widgets;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AsyncDemoState {
    pub is_loading: bool,
    pub completed_steps: u8,
    pub total_steps: u8,
    pub status: String,
    pub last_result: String,
}

impl AsyncDemoState {
    pub fn progress_ratio(&self) -> f32 {
        if self.total_steps == 0 {
            0.0
        } else {
            f32::from(self.completed_steps) / f32::from(self.total_steps)
        }
    }
}

impl Default for AsyncDemoState {
    fn default() -> Self {
        Self {
            is_loading: false,
            completed_steps: 0,
            total_steps: 3,
            status: "No async work running yet. Start a simulated fetch.".into(),
            last_result: "No result yet".into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct App {
    pub active_page: Page,
    pub shared: SharedState,
    pub windows: windows::WindowDemoState,
    pub form_draft: FormDraft,
    pub form_submission_summary: String,
    pub async_demo: AsyncDemoState,
}

impl Default for App {
    fn default() -> Self {
        Self {
            active_page: Page::Dashboard,
            shared: SharedState::default(),
            windows: windows::WindowDemoState::default(),
            form_draft: FormDraft::example(),
            form_submission_summary: "Submit the form to capture a stable summary.".into(),
            async_demo: AsyncDemoState::default(),
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
        self.shared.theme_choice.to_iced_theme()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([
            time::every(std::time::Duration::from_secs(1)).map(|_| Message::Tick),
            window::open_events().map(Message::WindowOpened),
            window::close_events().map(Message::WindowClosed),
        ])
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
            Message::ThemeSelected(choice) => {
                self.apply_shared_state_action(SharedStateAction::SetThemeChoice {
                    value: choice,
                    source: Page::Advanced,
                });
            }
            Message::DataFlow(message) => self.apply_data_flow_message(message),
            Message::Tick => self.shared.ticks += 1,
            Message::FormNameChanged(value) => self.form_draft.name = value,
            Message::FormEmailChanged(value) => self.form_draft.email = value,
            Message::FormGoalChanged(value) => self.form_draft.goal = value,
            Message::FormLoadExample => self.form_draft = FormDraft::example(),
            Message::FormSubmitted => {
                self.form_submission_summary = self.form_draft.submission_summary();
                self.shared.last_event = format!(
                    "Forms page submitted a valid draft for {}.",
                    self.form_draft.name.trim()
                );
            }
            Message::AsyncStarted => {
                self.async_demo.is_loading = true;
                self.async_demo.status = "Simulating async work with Task::perform...".into();
                return Task::perform(simulate_async_lesson(), Message::AsyncFinished);
            }
            Message::AsyncFinished(result) => {
                self.async_demo.is_loading = false;
                self.async_demo.completed_steps = self.async_demo.total_steps;
                self.async_demo.status =
                    "Async lesson finished. Review the result text below.".into();
                self.async_demo.last_result = result;
            }
            Message::AsyncReset => self.async_demo = AsyncDemoState::default(),
            Message::WindowOpenRequested(kind) => {
                return windows::open_window_task(&mut self.windows, kind)
            }
            Message::WindowOpened(id) => self.windows.mark_opened(id),
            Message::WindowSelected(id) => self.windows.select(id),
            Message::WindowFocusRequested(id) => {
                self.windows.mark_focused(id);
                return window::gain_focus(id);
            }
            Message::WindowCloseSelected => {
                if let Some(id) = self.windows.close_selected() {
                    return window::close(id);
                }
            }
            Message::WindowClosed(id) => {
                self.windows.mark_closed(id);
            }
            Message::WindowIncrementSelectedToolbox => self.windows.increment_selected_toolbox(),
            Message::ResetSandbox => *self = Self::default(),
        }

        self.shared.status_line = self.status_text();
        Task::none()
    }

    pub fn apply_shared_state_action(&mut self, action: SharedStateAction) {
        apply_shared_state_action(&mut self.shared, action);
    }

    pub fn apply_data_flow_message(&mut self, message: DataFlowMessage) {
        let event = message.event_label();
        let action = match message {
            DataFlowMessage::ProfileNameEdited(value) => SharedStateAction::SetLearnerName {
                value,
                source: Page::DataFlow,
            },
            DataFlowMessage::DashboardStatusEdited(value) => {
                SharedStateAction::SetDashboardStatus {
                    value,
                    source: Page::DataFlow,
                }
            }
            DataFlowMessage::SidebarTipsToggled(value) => {
                SharedStateAction::SetSidebarTipsVisible {
                    value,
                    source: Page::DataFlow,
                }
            }
            DataFlowMessage::ResetSharedState => SharedStateAction::ResetDataFlowDemo {
                source: Page::DataFlow,
            },
        };

        self.apply_shared_state_action(action);
        self.shared.last_event = format!("{event} → {}", self.shared.last_event);
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
            MenuAction::OpenInspectorWindow => {
                self.active_page = Page::Windows;
                self.windows.status = "Use the Windows page button to spawn the inspector so the demo can register its handle and lifecycle cleanly.".into();
            }
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
            "Active page: {} • Last menu action: {} • Shared summary: {} • Theme: {} • Open windows: {} • Tick demo: {}",
            self.active_page.label(),
            action,
            self.shared.dashboard_summary(),
            self.shared.theme_choice.label(),
            self.windows.open_count(),
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
            "Footer/status bar: {} Last shared event: {} Theme {}.",
            self.shared.visibility_summary(),
            self.shared.last_event,
            self.shared.theme_choice.label(),
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
            Page::Forms => pages::forms::view(self),
            Page::AsyncTasks => pages::async_tasks::view(self),
            Page::Windows => pages::windows::view(self),
            Page::Advanced => pages::advanced::view(self),
        }
    }
}

async fn simulate_async_lesson() -> String {
    std::thread::sleep(std::time::Duration::from_millis(40));
    "Fetched lesson outline: keep async state local, keep view declarative, and return messages when work completes.".into()
}

#[cfg(test)]
mod tests {
    use super::App;
    use crate::message::{MenuAction, Message};
    use crate::pages::{data_flow::DataFlowMessage, windows::WindowKind, Page};
    use crate::state::reducer::SharedStateAction;

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

    #[test]
    fn shared_state_reducer_updates_all_data_flow_paths() {
        let mut app = App::default();

        app.apply_shared_state_action(SharedStateAction::SetLearnerName {
            value: "Morgan".into(),
            source: Page::DataFlow,
        });
        assert_eq!(app.shared.learner_name, "Morgan");
        assert_eq!(app.shared.profile_last_changed_by, Page::DataFlow);

        app.apply_shared_state_action(SharedStateAction::SetDashboardStatus {
            value: "Preview synchronized".into(),
            source: Page::DataFlow,
        });
        assert_eq!(app.shared.dashboard_status, "Preview synchronized");
        assert_eq!(app.shared.status_last_changed_by, Page::DataFlow);

        app.apply_shared_state_action(SharedStateAction::SetSidebarTipsVisible {
            value: false,
            source: Page::DataFlow,
        });
        assert!(!app.shared.show_sidebar_tips);
        assert_eq!(app.shared.preference_last_changed_by, Page::DataFlow);
    }

    #[test]
    fn data_flow_message_updates_expected_shared_fields() {
        let mut app = App::default();

        let _ = app.update(Message::DataFlow(DataFlowMessage::DashboardStatusEdited(
            "Tracing message forwarding".into(),
        )));

        assert_eq!(app.shared.dashboard_status, "Tracing message forwarding");
        assert_eq!(app.shared.status_last_changed_by, Page::DataFlow);
        assert!(app
            .shared
            .last_event
            .contains("DataFlow updated the dashboard status"));
    }

    #[test]
    fn window_messages_update_registry_counts_and_identifiers() {
        let mut app = App::default();
        let id = iced::window::Id::unique();

        app.windows.register_open_request(id, WindowKind::Toolbox);
        let _ = app.update(Message::WindowOpened(id));
        let _ = app.update(Message::WindowSelected(id));
        let _ = app.update(Message::WindowIncrementSelectedToolbox);
        let _ = app.update(Message::WindowClosed(id));

        assert_eq!(app.windows.open_count(), 0);
        assert_eq!(app.windows.selected, None);
        assert!(app.windows.status.contains("Registry cleanup"));
    }

    #[test]
    fn reset_data_flow_action_restores_defaults_cleanly() {
        let mut app = App::default();
        app.shared.learner_name = "Changed".into();
        app.shared.dashboard_status = "Changed status".into();
        app.shared.show_sidebar_tips = false;
        app.shared.theme_choice = crate::theme::ThemeChoice::HighContrast;

        app.apply_shared_state_action(SharedStateAction::ResetDataFlowDemo {
            source: Page::DataFlow,
        });

        let defaults = App::default();
        assert_eq!(app.shared.learner_name, defaults.shared.learner_name);
        assert_eq!(
            app.shared.dashboard_status,
            defaults.shared.dashboard_status
        );
        assert_eq!(
            app.shared.show_sidebar_tips,
            defaults.shared.show_sidebar_tips
        );
        assert_eq!(app.shared.theme_choice, defaults.shared.theme_choice);
        assert_eq!(app.shared.profile_last_changed_by, Page::DataFlow);
        assert_eq!(app.shared.status_last_changed_by, Page::DataFlow);
        assert_eq!(app.shared.preference_last_changed_by, Page::DataFlow);
    }

    #[test]
    fn forms_submission_regression_updates_summary_without_validation_logic_in_view() {
        let mut app = App::default();
        app.form_draft = crate::forms::FormDraft::example();

        let _ = app.update(Message::FormSubmitted);

        assert!(app.form_submission_summary.contains("plans to"));
        assert!(app.shared.last_event.contains("Forms page submitted"));
    }

    #[test]
    fn async_reset_restores_idle_state() {
        let mut app = App::default();
        app.async_demo.is_loading = true;
        app.async_demo.completed_steps = 3;
        app.async_demo.last_result = "done".into();

        let _ = app.update(Message::AsyncReset);

        assert!(!app.async_demo.is_loading);
        assert_eq!(app.async_demo.completed_steps, 0);
        assert_eq!(app.async_demo.last_result, "No result yet");
    }
}
