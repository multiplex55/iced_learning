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
        action: MenuAction::OpenLayoutRecipe,
    }),
    MenuNode::Action(MenuLeaf {
        label: "Data Flow Walkthrough",
        action: MenuAction::OpenDataFlowWalkthrough,
    }),
];

const FILE_EXPORT_CHILDREN: &[MenuNode] = &[
    MenuNode::Action(MenuLeaf {
        label: "Rust Module",
        action: MenuAction::ExportRustModule,
    }),
    MenuNode::Action(MenuLeaf {
        label: "Show Teaching Notes",
        action: MenuAction::ShowTeachingNotes,
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
        label: "Open Iced docs lesson",
        action: MenuAction::OpenIcedDocsLesson,
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

pub fn collect_menu_leaves(nodes: &'static [MenuNode]) -> Vec<MenuLeaf> {
    let mut leaves = Vec::new();
    collect_menu_leaves_into(nodes, &mut leaves);
    leaves
}

fn collect_action_labels_into(nodes: &'static [MenuNode], labels: &mut Vec<&'static str>) {
    for node in nodes {
        match node {
            MenuNode::Action(leaf) => labels.push(leaf.label),
            MenuNode::Group { children, .. } => collect_action_labels_into(children, labels),
        }
    }
}

fn collect_menu_leaves_into(nodes: &'static [MenuNode], leaves: &mut Vec<MenuLeaf>) {
    for node in nodes {
        match node {
            MenuNode::Action(leaf) => leaves.push(*leaf),
            MenuNode::Group { children, .. } => collect_menu_leaves_into(children, leaves),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{collect_action_labels, collect_menu_leaves, top_level_menu_summaries, ROOT_MENUS};
    use crate::message::MenuAction;
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

    #[test]
    fn each_menu_label_maps_to_dedicated_action() {
        let leaves = collect_menu_leaves(ROOT_MENUS);

        assert_eq!(
            leaves,
            vec![
                super::MenuLeaf {
                    label: "New Sandbox",
                    action: MenuAction::NewSandbox
                },
                super::MenuLeaf {
                    label: "Layout Recipe",
                    action: MenuAction::OpenLayoutRecipe
                },
                super::MenuLeaf {
                    label: "Data Flow Walkthrough",
                    action: MenuAction::OpenDataFlowWalkthrough
                },
                super::MenuLeaf {
                    label: "Save Snapshot",
                    action: MenuAction::SaveSnapshot
                },
                super::MenuLeaf {
                    label: "Rust Module",
                    action: MenuAction::ExportRustModule
                },
                super::MenuLeaf {
                    label: "Show Teaching Notes",
                    action: MenuAction::ShowTeachingNotes
                },
                super::MenuLeaf {
                    label: "Toggle Sidebar Tips",
                    action: MenuAction::ToggleSidebarTips
                },
                super::MenuLeaf {
                    label: "Jump to Controls",
                    action: MenuAction::FocusControlsPage
                },
                super::MenuLeaf {
                    label: "Open Inspector",
                    action: MenuAction::OpenInspectorWindow
                },
                super::MenuLeaf {
                    label: "Arrange Study Layout",
                    action: MenuAction::ArrangeStudyLayout
                },
                super::MenuLeaf {
                    label: "Open Iced docs lesson",
                    action: MenuAction::OpenIcedDocsLesson
                },
                super::MenuLeaf {
                    label: "About Sandbox",
                    action: MenuAction::AboutSandbox
                },
            ]
        );
    }

    #[test]
    fn formerly_overloaded_actions_are_now_distinct_variants() {
        assert_ne!(
            MenuAction::OpenDataFlowWalkthrough,
            MenuAction::ExportRustModule
        );
        assert_ne!(MenuAction::ShowTeachingNotes, MenuAction::SaveSnapshot);
        assert_ne!(MenuAction::OpenIcedDocsLesson, MenuAction::AboutSandbox);
        assert_ne!(
            MenuAction::OpenInspectorWindow,
            MenuAction::ArrangeStudyLayout
        );
    }
}
