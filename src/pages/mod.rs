//! Demo pages and navigation metadata.
//!
//! The page registry is intentionally data-driven so unit tests can verify the
//! order and labels without needing to render a GUI.

pub mod advanced;
pub mod async_tasks;
pub mod controls;
pub mod dashboard;
pub mod data_flow;
pub mod forms;
pub mod layouts;
pub mod windows;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Page {
    Dashboard,
    Layouts,
    Controls,
    DataFlow,
    Forms,
    AsyncTasks,
    Windows,
    Advanced,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageMeta {
    pub page: Page,
    pub id: &'static str,
    pub label: &'static str,
    pub lesson: &'static str,
}

pub const PAGE_REGISTRY: [PageMeta; 8] = [
    PageMeta {
        page: Page::Dashboard,
        id: "dashboard",
        label: "Dashboard",
        lesson: "Application shell, menu bars, and top-level event handling.",
    },
    PageMeta {
        page: Page::Layouts,
        id: "layouts",
        label: "Layouts",
        lesson: "Rows, columns, containers, spacing, and scrolling.",
    },
    PageMeta {
        page: Page::Controls,
        id: "controls",
        label: "Controls",
        lesson: "Interactive widgets and the state they emit back into update.",
    },
    PageMeta {
        page: Page::DataFlow,
        id: "data-flow",
        label: "DataFlow",
        lesson: "How shared state changes propagate across pages.",
    },
    PageMeta {
        page: Page::Forms,
        id: "forms-validation",
        label: "Forms",
        lesson: "Controlled inputs, validation, and separating pure form logic from view code.",
    },
    PageMeta {
        page: Page::AsyncTasks,
        id: "async-tasks",
        label: "Async/Tasks",
        lesson: "Background work, loading state, and update-driven task completion.",
    },
    PageMeta {
        page: Page::Windows,
        id: "windows",
        label: "Windows",
        lesson: "Window-oriented state, dialogs, and layout composition.",
    },
    PageMeta {
        page: Page::Advanced,
        id: "advanced-theming",
        label: "Advanced",
        lesson: "Styling, theming, subscriptions, overlays, and richer composition patterns.",
    },
];

impl Page {
    pub const ALL: [Self; 8] = [
        Self::Dashboard,
        Self::Layouts,
        Self::Controls,
        Self::DataFlow,
        Self::Forms,
        Self::AsyncTasks,
        Self::Windows,
        Self::Advanced,
    ];

    pub fn label(self) -> &'static str {
        self.meta().label
    }

    pub fn id(self) -> &'static str {
        self.meta().id
    }

    pub fn lesson(self) -> &'static str {
        self.meta().lesson
    }

    pub fn meta(self) -> PageMeta {
        PAGE_REGISTRY
            .into_iter()
            .find(|meta| meta.page == self)
            .expect("every Page variant must be present in PAGE_REGISTRY")
    }
}

#[cfg(test)]
mod tests {
    use super::{Page, PAGE_REGISTRY};
    use std::collections::HashSet;

    #[test]
    fn page_registry_contains_all_expected_pages_in_order() {
        let pages = PAGE_REGISTRY.map(|meta| meta.page);

        assert_eq!(
            pages,
            [
                Page::Dashboard,
                Page::Layouts,
                Page::Controls,
                Page::DataFlow,
                Page::Forms,
                Page::AsyncTasks,
                Page::Windows,
                Page::Advanced,
            ]
        );
    }

    #[test]
    fn labels_are_stable_for_navigation() {
        let labels = PAGE_REGISTRY.map(|meta| meta.label);

        assert_eq!(
            labels,
            [
                "Dashboard",
                "Layouts",
                "Controls",
                "DataFlow",
                "Forms",
                "Async/Tasks",
                "Windows",
                "Advanced"
            ]
        );
    }

    #[test]
    fn demo_metadata_is_internally_consistent() {
        let ids = PAGE_REGISTRY.map(|meta| meta.id);
        let unique = ids.into_iter().collect::<HashSet<_>>();

        assert_eq!(unique.len(), PAGE_REGISTRY.len());
        assert!(PAGE_REGISTRY
            .iter()
            .all(|meta| !meta.label.trim().is_empty()));
    }
}
