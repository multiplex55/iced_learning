//! Message types describe *events* in Iced.
//!
//! New learners can think of this enum as the contract between the `view`
//! function and the `update` function: widgets emit messages, and `update`
//! handles them.

use crate::pages::{data_flow::DataFlowMessage, windows::WindowKind, Page};
use crate::theme::ThemeChoice;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuAction {
    NewSandbox,
    OpenLayoutRecipe,
    OpenDataFlowWalkthrough,
    SaveSnapshot,
    ExportRustModule,
    ShowTeachingNotes,
    ToggleSidebarTips,
    FocusControlsPage,
    OpenInspectorWindow,
    ArrangeStudyLayout,
    OpenIcedDocsLesson,
    AboutSandbox,
}

impl MenuAction {
    pub fn label(self) -> &'static str {
        match self {
            Self::NewSandbox => "File → New Sandbox",
            Self::OpenLayoutRecipe => "File → Open → Layout Recipe",
            Self::OpenDataFlowWalkthrough => "File → Open → Data Flow Walkthrough",
            Self::SaveSnapshot => "File → Save Snapshot",
            Self::ExportRustModule => "File → Export → Rust Module",
            Self::ShowTeachingNotes => "File → Export → Show Teaching Notes",
            Self::ToggleSidebarTips => "View → Toggle Sidebar Tips",
            Self::FocusControlsPage => "View → Jump to Controls",
            Self::OpenInspectorWindow => "Window → Open Inspector",
            Self::ArrangeStudyLayout => "Window → Arrange Study Layout",
            Self::OpenIcedDocsLesson => "Help → Open Iced docs lesson",
            Self::AboutSandbox => "Help → About Sandbox",
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Navigate(Page),
    MenuSelected(MenuAction),
    SharedTextChanged(String),
    CounterIncremented,
    CounterDecremented,
    ControlsToggled(bool),
    ControlsCheckboxChanged(bool),
    ControlsSliderChanged(u8),
    ControlsChoiceSelected(ControlChoice),
    ProgressStepped,
    ThemeSelected(ThemeChoice),
    DataFlow(DataFlowMessage),
    Tick,
    FormNameChanged(String),
    FormEmailChanged(String),
    FormGoalChanged(String),
    FormLoadExample,
    FormSubmitted,
    AsyncStarted,
    AsyncFinished(String),
    AsyncReset,
    WindowOpenRequested(WindowKind),
    WindowOpened(iced::window::Id),
    WindowSelected(iced::window::Id),
    WindowFocusRequested(iced::window::Id),
    WindowCloseSelected,
    WindowClosed(iced::window::Id),
    WindowIncrementSelectedToolbox,
    ResetSandbox,
}

impl From<DataFlowMessage> for Message {
    fn from(message: DataFlowMessage) -> Self {
        Self::DataFlow(message)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlChoice {
    Button,
    Slider,
    TextInput,
    PickList,
}

impl ControlChoice {
    pub const ALL: [Self; 4] = [Self::Button, Self::Slider, Self::TextInput, Self::PickList];

    pub fn label(self) -> &'static str {
        match self {
            Self::Button => "Buttons",
            Self::Slider => "Sliders",
            Self::TextInput => "Text input",
            Self::PickList => "Pick lists",
        }
    }
}

impl std::fmt::Display for ControlChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.label())
    }
}

#[cfg(test)]
mod tests {
    use super::Message;
    use crate::pages::data_flow::DataFlowMessage;

    #[test]
    fn data_flow_messages_convert_into_root_messages() {
        let message: Message = DataFlowMessage::ProfileNameEdited("Ava".into()).into();

        match message {
            Message::DataFlow(DataFlowMessage::ProfileNameEdited(value)) => {
                assert_eq!(value, "Ava");
            }
            other => panic!("unexpected message: {other:?}"),
        }
    }
}
