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
        let task = match message {
            Message::Navigate(page) => {
                self.active_page = page;
                Task::none()
            }
            Message::MenuSelected(action) => self.apply_menu_action(action),
            Message::SharedTextChanged(value) => {
                self.shared.learner_name = value;
                Task::none()
            }
            Message::CounterIncremented => {
                self.shared.shared_counter += 1;
                Task::none()
            }
            Message::CounterDecremented => {
                self.shared.shared_counter -= 1;
                Task::none()
            }
            Message::ControlsToggled(value) => {
                self.shared.controls_enabled = value;
                Task::none()
            }
            Message::ControlsCheckboxChanged(value) => {
                self.shared.controls_checked = value;
                Task::none()
            }
            Message::ControlsSliderChanged(value) => {
                self.shared.slider_value = value;
                Task::none()
            }
            Message::ControlsChoiceSelected(choice) => {
                self.shared.selected_control = choice;
                Task::none()
            }
            Message::ProgressStepped => {
                self.shared.progress_value = (self.shared.progress_value + 10).min(100);
                Task::none()
            }
            Message::ThemeSelected(choice) => {
                self.apply_shared_state_action(SharedStateAction::SetThemeChoice {
                    value: choice,
                    source: Page::Advanced,
                });
                Task::none()
            }
            Message::DataFlow(message) => {
                self.apply_data_flow_message(message);
                Task::none()
            }
            Message::Tick => {
                self.shared.ticks += 1;
                Task::none()
            }
            Message::FormNameChanged(value) => {
                self.form_draft.name = value;
                Task::none()
            }
            Message::FormEmailChanged(value) => {
                self.form_draft.email = value;
                Task::none()
            }
            Message::FormGoalChanged(value) => {
                self.form_draft.goal = value;
                Task::none()
            }
            Message::FormLoadExample => {
                self.form_draft = FormDraft::example();
                Task::none()
            }
            Message::FormSubmitted => {
                self.form_submission_summary = self.form_draft.submission_summary();
                self.shared.last_event = format!(
                    "Forms page submitted a valid draft for {}.",
                    self.form_draft.name.trim()
                );
                Task::none()
            }
            Message::AsyncStarted => {
                self.async_demo.is_loading = true;
                self.async_demo.status = "Simulating async work with Task::perform...".into();
                Task::perform(simulate_async_lesson(), Message::AsyncFinished)
            }
            Message::AsyncFinished(result) => {
                self.async_demo.is_loading = false;
                self.async_demo.completed_steps = self.async_demo.total_steps;
                self.async_demo.status =
                    "Async lesson finished. Review the result text below.".into();
                self.async_demo.last_result = result;
                Task::none()
            }
            Message::AsyncReset => {
                self.async_demo = AsyncDemoState::default();
                Task::none()
            }
            Message::WindowOpenRequested(kind) => self.open_window(kind),
            Message::WindowOpened(id) => {
                self.windows.mark_opened(id);
                Task::none()
            }
            Message::WindowSelected(id) => {
                self.windows.select(id);
                Task::none()
            }
            Message::WindowFocusRequested(id) => {
                self.windows.mark_focused(id);
                window::gain_focus(id)
            }
            Message::WindowCloseSelected => {
                if let Some(id) = self.windows.close_selected() {
                    window::close(id)
                } else {
                    Task::none()
                }
            }
            Message::WindowClosed(id) => {
                self.windows.mark_closed(id);
                Task::none()
            }
            Message::WindowIncrementSelectedToolbox => {
                self.windows.increment_selected_toolbox();
                Task::none()
            }
            Message::ResetSandbox => {
                *self = Self::default();
                Task::none()
            }
        };

        self.shared.status_line = self.status_text();
        task
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

    pub fn apply_menu_action(&mut self, action: MenuAction) -> Task<Message> {
        self.shared.last_menu_action = Some(action);

        match action {
            MenuAction::NewSandbox => {
                *self = Self::default();
                Task::none()
            }
            MenuAction::OpenLayoutRecipe => {
                self.active_page = Page::Layouts;
                self.shared
                    .notes
                    .push("Opened the layout recipe lesson from the dashboard menu.".into());
                Task::none()
            }
            MenuAction::OpenDataFlowWalkthrough => {
                self.active_page = Page::DataFlow;
                self.shared.notes.push(
                    "Opened the data flow walkthrough lesson from the File → Open menu.".into(),
                );
                Task::none()
            }
            MenuAction::SaveSnapshot => {
                self.shared.notes.push(format!(
                    "Saved snapshot for {} with counter {}.",
                    self.shared.learner_name, self.shared.shared_counter
                ));
                Task::none()
            }
            MenuAction::ExportRustModule => {
                let export = self.export_rust_module();
                self.shared.notes.push(export.clone());
                self.shared.last_event = format!(
                    "Generated an in-app Rust module export preview ({} bytes).",
                    export.len()
                );
                Task::none()
            }
            MenuAction::ShowTeachingNotes => {
                self.active_page = Page::About;
                self.shared.notes.push(self.teaching_notes_export());
                self.shared.last_event =
                    "Displayed the teaching notes reference in the About page.".into();
                Task::none()
            }
            MenuAction::ToggleSidebarTips => {
                self.shared.show_sidebar_tips = !self.shared.show_sidebar_tips;
                Task::none()
            }
            MenuAction::FocusControlsPage => {
                self.active_page = Page::Controls;
                Task::none()
            }
            MenuAction::OpenInspectorWindow => self.open_window(windows::WindowKind::Inspector),
            MenuAction::ArrangeStudyLayout => self.arrange_study_layout(),
            MenuAction::OpenIcedDocsLesson => {
                self.active_page = Page::Advanced;
                self.shared.last_event =
                    "Opened the in-app Iced docs lesson instead of launching an external browser."
                        .into();
                self.shared.notes.push(
                    "Iced docs lesson: compare this sandbox's Task/update flow with the official Iced guide and API docs.".into(),
                );
                Task::none()
            }
            MenuAction::AboutSandbox => {
                self.active_page = Page::About;
                self.shared.last_event = format!(
                    "Opened About Sandbox for version {} and project context.",
                    env!("CARGO_PKG_VERSION")
                );
                Task::none()
            }
        }
    }

    fn open_window(&mut self, kind: windows::WindowKind) -> Task<Message> {
        self.active_page = Page::Windows;
        windows::open_window_task(&mut self.windows, kind)
    }

    fn arrange_study_layout(&mut self) -> Task<Message> {
        self.active_page = Page::Windows;
        self.windows.status =
            "Arranging study layout: inspector, notes, and preview windows requested in order."
                .into();

        Task::batch([
            self.open_window(windows::WindowKind::Inspector),
            self.open_window(windows::WindowKind::Notes),
            self.open_window(windows::WindowKind::Preview),
        ])
    }

    fn export_rust_module(&self) -> String {
        format!(
            "pub struct SandboxSnapshot {{\n    pub learner: &'static str,\n    pub counter: i32,\n    pub active_page: &'static str,\n}}\n\npub const SNAPSHOT: SandboxSnapshot = SandboxSnapshot {{\n    learner: \"{}\",\n    counter: {},\n    active_page: \"{}\",\n}};",
            self.shared.profile_preview_name(),
            self.shared.shared_counter,
            self.active_page.label(),
        )
    }

    fn teaching_notes_export(&self) -> String {
        format!(
            "Teaching notes\n- Project: {}\n- Version: {}\n- Focus: {}\n- Sidebar tips visible: {}\n- Open windows tracked: {}",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION"),
            self.active_page.lesson(),
            self.shared.show_sidebar_tips,
            self.windows.records.len(),
        )
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
            Page::About => pages::about::view(self),
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
    fn menu_actions_update_expected_pages_without_aliasing() {
        let scenarios = [
            (MenuAction::OpenLayoutRecipe, Page::Layouts),
            (MenuAction::OpenDataFlowWalkthrough, Page::DataFlow),
            (MenuAction::FocusControlsPage, Page::Controls),
            (MenuAction::OpenIcedDocsLesson, Page::Advanced),
            (MenuAction::AboutSandbox, Page::About),
            (MenuAction::ShowTeachingNotes, Page::About),
        ];

        for (action, expected_page) in scenarios {
            let mut app = App::default();
            let _ = app.apply_menu_action(action);
            assert_eq!(app.active_page, expected_page, "action: {action:?}");
            assert_eq!(app.shared.last_menu_action, Some(action));
        }
    }

    #[test]
    fn open_inspector_uses_same_window_open_flow_as_window_message() {
        let mut via_menu = App::default();
        let _ = via_menu.apply_menu_action(MenuAction::OpenInspectorWindow);

        let mut via_message = App::default();
        let _ = via_message.update(Message::WindowOpenRequested(WindowKind::Inspector));

        assert_eq!(via_menu.active_page, Page::Windows);
        assert_eq!(via_menu.active_page, via_message.active_page);
        assert_eq!(via_menu.windows.records.len(), 1);
        assert_eq!(via_menu.windows.records[0].kind, WindowKind::Inspector);
        assert_eq!(via_message.windows.records.len(), 1);
        assert_eq!(via_message.windows.records[0].kind, WindowKind::Inspector);
        assert!(via_menu.windows.selected.is_some());
        assert!(via_message.windows.selected.is_some());
        assert!(via_menu.windows.status.contains("Opening Inspector"));
        assert!(via_message.windows.status.contains("Opening Inspector"));
    }

    #[test]
    fn arrange_study_layout_registers_expected_windows_in_order() {
        let mut app = App::default();
        let _ = app.apply_menu_action(MenuAction::ArrangeStudyLayout);

        let kinds: Vec<_> = app
            .windows
            .records
            .iter()
            .map(|record| record.kind)
            .collect();
        assert_eq!(app.active_page, Page::Windows);
        assert_eq!(
            kinds,
            vec![
                WindowKind::Inspector,
                WindowKind::Notes,
                WindowKind::Preview
            ]
        );
        assert!(app.windows.status.contains("Opening Live Preview"));
        assert_eq!(
            app.windows.selected.map(|id| id),
            app.windows.records.last().map(|record| record.id)
        );
    }

    #[test]
    fn export_actions_have_distinct_semantics() {
        let mut app = App::default();
        app.shared.learner_name = "Ava".into();
        app.shared.shared_counter = 7;

        let _ = app.apply_menu_action(MenuAction::ExportRustModule);
        let rust_export = app.shared.notes.last().cloned().expect("rust export note");
        assert!(rust_export.contains("pub struct SandboxSnapshot"));
        assert!(rust_export.contains("learner: \"Ava\""));
        assert!(app.shared.last_event.contains("Rust module export preview"));

        let _ = app.apply_menu_action(MenuAction::ShowTeachingNotes);
        let teaching_notes = app.shared.notes.last().cloned().expect("teaching notes");
        assert!(teaching_notes.contains("Teaching notes"));
        assert!(teaching_notes.contains("Version:"));
        assert_eq!(app.active_page, Page::About);
    }

    #[test]
    fn about_and_docs_actions_are_distinct_from_dashboard_navigation() {
        let mut app = App::default();

        let _ = app.apply_menu_action(MenuAction::AboutSandbox);
        assert_eq!(app.active_page, Page::About);
        assert!(app.shared.last_event.contains("About Sandbox"));

        let _ = app.apply_menu_action(MenuAction::OpenIcedDocsLesson);
        assert_eq!(app.active_page, Page::Advanced);
        assert!(app.shared.last_event.contains("in-app Iced docs lesson"));
    }

    #[test]
    fn data_flow_messages_apply_shared_state_actions() {
        let mut app = App::default();
        app.apply_data_flow_message(DataFlowMessage::ProfileNameEdited("Mia".into()));

        assert_eq!(
            app.shared.learner_name, "Mia",
            "profile changes should feed through the reducer-backed path"
        );
        assert!(app
            .shared
            .last_event
            .contains("DataFlow updated the shared learner profile"));
    }

    #[test]
    fn shared_state_action_helper_delegates_to_reducer() {
        let mut app = App::default();

        app.apply_shared_state_action(SharedStateAction::SetDashboardStatus {
            value: "Reducer updated dashboard".into(),
            source: Page::Dashboard,
        });

        assert_eq!(app.shared.dashboard_status, "Reducer updated dashboard");
        assert!(app
            .shared
            .last_event
            .contains("Dashboard updated the dashboard status"));
    }
}
