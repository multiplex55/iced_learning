//! Demo pages and navigation metadata.
//!
//! The page registry is intentionally data-driven so unit tests can verify the
//! order and labels without needing to render a GUI.

pub mod advanced;
pub mod controls;
pub mod dashboard;
pub mod data_flow;
pub mod layouts;
pub mod windows;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Page {
    Dashboard,
    Layouts,
    Controls,
    DataFlow,
    Windows,
    Advanced,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageMeta {
    pub page: Page,
    pub label: &'static str,
    pub lesson: &'static str,
}

pub const PAGE_REGISTRY: [PageMeta; 6] = [
    PageMeta {
        page: Page::Dashboard,
        label: "Dashboard",
        lesson: "Application shell, menu bars, and top-level event handling.",
    },
    PageMeta {
        page: Page::Layouts,
        label: "Layouts",
        lesson: "Rows, columns, containers, spacing, and scrolling.",
    },
    PageMeta {
        page: Page::Controls,
        label: "Controls",
        lesson: "Interactive widgets and the state they emit back into update.",
    },
    PageMeta {
        page: Page::DataFlow,
        label: "DataFlow",
        lesson: "How shared state changes propagate across pages.",
    },
    PageMeta {
        page: Page::Windows,
        label: "Windows",
        lesson: "Window-oriented state, dialogs, and layout composition.",
    },
    PageMeta {
        page: Page::Advanced,
        label: "Advanced",
        lesson: "Styling, subscriptions, overlays, and richer composition patterns.",
    },
];

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
        self.meta().label
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
                "Windows",
                "Advanced"
            ]
        );
    }
}
