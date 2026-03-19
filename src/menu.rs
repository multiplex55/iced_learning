//! Pure menu metadata used by the dashboard demo.
//!
//! The `iced_aw` widget tree still lives in `pages/dashboard.rs`, but the
//! hierarchy is described here so tests can validate labels and nesting without
//! touching GUI code.

use crate::message::MenuAction;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MenuLeaf {
    pub label: &'static str,
    pub action: MenuAction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuNode {
    Action(MenuLeaf),
    Group {
        label: &'static str,
        children: &'static [MenuNode],
    },
}

const FILE_OPEN_CHILDREN: &[MenuNode] = &[
    MenuNode::Action(MenuLeaf {
        label: "Layout Recipe",
        action: MenuAction::OpenRecipe,
    }),
    MenuNode::Action(MenuLeaf {
        label: "Data Flow Walkthrough",
        action: MenuAction::ExportCode,
    }),
];

const FILE_EXPORT_CHILDREN: &[MenuNode] = &[
    MenuNode::Action(MenuLeaf {
        label: "Rust Module",
        action: MenuAction::ExportCode,
    }),
    MenuNode::Action(MenuLeaf {
        label: "Teaching Notes",
        action: MenuAction::SaveSnapshot,
    }),
];

const FILE_CHILDREN: &[MenuNode] = &[
    MenuNode::Action(MenuLeaf {
        label: "New Sandbox",
        action: MenuAction::NewSandbox,
    }),
    MenuNode::Group {
        label: "Open",
        children: FILE_OPEN_CHILDREN,
    },
    MenuNode::Action(MenuLeaf {
        label: "Save Snapshot",
        action: MenuAction::SaveSnapshot,
    }),
    MenuNode::Group {
        label: "Export",
        children: FILE_EXPORT_CHILDREN,
    },
];

const VIEW_CHILDREN: &[MenuNode] = &[
    MenuNode::Action(MenuLeaf {
        label: "Toggle Sidebar Tips",
        action: MenuAction::ToggleSidebarTips,
    }),
    MenuNode::Action(MenuLeaf {
        label: "Jump to Controls",
        action: MenuAction::FocusControlsPage,
    }),
];

const WINDOW_CHILDREN: &[MenuNode] = &[
    MenuNode::Action(MenuLeaf {
        label: "Open Inspector",
        action: MenuAction::OpenInspectorWindow,
    }),
    MenuNode::Action(MenuLeaf {
        label: "Arrange Study Layout",
        action: MenuAction::ArrangeStudyLayout,
    }),
];

const HELP_CHILDREN: &[MenuNode] = &[
    MenuNode::Action(MenuLeaf {
        label: "View Iced Docs",
        action: MenuAction::ViewDocs,
    }),
    MenuNode::Action(MenuLeaf {
        label: "About Sandbox",
        action: MenuAction::AboutSandbox,
    }),
];

pub const ROOT_MENUS: &[MenuNode] = &[
    MenuNode::Group {
        label: "File",
        children: FILE_CHILDREN,
    },
    MenuNode::Group {
        label: "View",
        children: VIEW_CHILDREN,
    },
    MenuNode::Group {
        label: "Window",
        children: WINDOW_CHILDREN,
    },
    MenuNode::Group {
        label: "Help",
        children: HELP_CHILDREN,
    },
];

pub fn top_level_menu_summaries() -> Vec<(&'static str, usize)> {
    ROOT_MENUS
        .iter()
        .map(|node| match node {
            MenuNode::Group { label, children } => (*label, children.len()),
            MenuNode::Action(_) => unreachable!("root menu entries must be groups"),
        })
        .collect()
}

pub fn collect_action_labels(nodes: &'static [MenuNode]) -> Vec<&'static str> {
    let mut labels = Vec::new();
    collect_action_labels_into(nodes, &mut labels);
    labels
}

fn collect_action_labels_into(nodes: &'static [MenuNode], labels: &mut Vec<&'static str>) {
    for node in nodes {
        match node {
            MenuNode::Action(leaf) => labels.push(leaf.label),
            MenuNode::Group { children, .. } => collect_action_labels_into(children, labels),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{collect_action_labels, top_level_menu_summaries, ROOT_MENUS};
    use std::collections::HashSet;

    #[test]
    fn top_level_menu_groups_match_dashboard_layout() {
        assert_eq!(
            top_level_menu_summaries(),
            vec![("File", 4), ("View", 2), ("Window", 2), ("Help", 2)]
        );
    }

    #[test]
    fn menu_action_labels_are_non_empty_and_unique() {
        let labels = collect_action_labels(ROOT_MENUS);
        assert!(labels.iter().all(|label| !label.trim().is_empty()));

        let unique = labels.iter().copied().collect::<HashSet<_>>();
        assert_eq!(labels.len(), unique.len());
    }
}
