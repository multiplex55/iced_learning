//! Message types describe *events* in Iced.
//!
//! New learners can think of this enum as the contract between the `view`
//! function and the `update` function: widgets emit messages, and `update`
//! handles them.

use crate::pages::Page;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuAction {
    NewSandbox,
    OpenRecipe,
    SaveSnapshot,
    ExportCode,
    ToggleSidebarTips,
    FocusControlsPage,
    OpenInspectorWindow,
    ArrangeStudyLayout,
    ViewDocs,
    AboutSandbox,
}

impl MenuAction {
    pub fn label(self) -> &'static str {
        match self {
            Self::NewSandbox => "File → New Sandbox",
            Self::OpenRecipe => "File → Open → Layout Recipe",
            Self::SaveSnapshot => "File → Save Snapshot",
            Self::ExportCode => "File → Export → Rust Module",
            Self::ToggleSidebarTips => "View → Toggle Sidebar Tips",
            Self::FocusControlsPage => "View → Jump to Controls",
            Self::OpenInspectorWindow => "Window → Open Inspector",
            Self::ArrangeStudyLayout => "Window → Arrange Study Layout",
            Self::ViewDocs => "Help → View Iced Docs",
            Self::AboutSandbox => "Help → About",
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
    AdvancedThemeToggled(bool),
    Tick,
    ToggleChildWindow,
    ResetSandbox,
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
