//! Dashboard page.
//!
//! Start here if you are new to Iced: it shows how an app shell can host a
//! dashboard, a menu bar, and visible event output in one place.

use iced::widget::{button, column, container, row, text};
use iced::{Element, Length};
use iced_aw::menu::{Item, Menu};
use iced_aw::{menu_bar, menu_items};

use crate::app::App;
use crate::menu;
use crate::message::{MenuAction, Message};
use crate::theme;
use crate::widgets;

#[cfg(test)]
pub fn menu_groups() -> Vec<(&'static str, usize)> {
    menu::top_level_menu_summaries()
}

pub fn view(app: &App) -> Element<'_, Message> {
    let menu = build_menu_bar();
    let last_action = app
        .shared
        .last_menu_action
        .map(MenuAction::label)
        .unwrap_or("Nothing selected yet; choose a menu item to trigger an app message.");

    let summary = widgets::section_card(
        "Dashboard overview",
        "This page teaches menu composition, top-level shell layout, and visible message handling.",
        column![
            text(format!(
                "Accent helper in theme.rs: {}",
                theme::accent_hex()
            )),
            text(format!(
                "Learner profile: {}",
                app.shared.profile_preview_name()
            )),
            text(format!(
                "Shared dashboard status: {}",
                app.shared.dashboard_status
            )),
            text(format!(
                "Theme choice metadata: {}",
                app.shared.theme_choice.label()
            )),
            text(format!("Shared counter: {}", app.shared.shared_counter)),
            text(format!(
                "Derived dashboard summary: {}",
                app.shared.dashboard_summary()
            )),
            text(format!(
                "Sidebar teaching tips visible: {}",
                app.shared.show_sidebar_tips
            )),
            text(format!(
                "Open child windows tracked: {}",
                app.windows.open_count()
            )),
        ]
        .spacing(8),
    );

    let output = widgets::section_card(
        "Visible event output",
        "When a menu item emits a Message::MenuSelected event, App::update mutates state and the dashboard re-renders this text.",
        column![
            text(format!("Last selected action: {last_action}")),
            button("Open the inspector via a normal button")
                .on_press(Message::MenuSelected(MenuAction::OpenInspectorWindow)),
            text("Interesting item to notice: the pure menu hierarchy lives in src/menu.rs so tests can verify structure without depending on iced_aw widgets."),
        ]
        .spacing(10),
    );

    let content = column![
        widgets::section_title("Dashboard"),
        widgets::note("The dashboard places an iced_aw menu bar above explanatory cards so event flow stays visible."),
        menu,
        row![summary, output].spacing(16),
    ]
    .spacing(16);

    container(content).width(Length::Fill).into()
}

fn build_menu_bar<'a>() -> Element<'a, Message> {
    let root = |items| Menu::new(items).max_width(220.0).spacing(4.0).offset(12.0);
    let nested = |items| Menu::new(items).max_width(220.0).spacing(4.0).offset(0.0);

    // `iced_aw` adds value here by providing a desktop-like menu bar quickly.
    // The structure is still mirrored in src/menu.rs because educational demos
    // benefit from pure metadata you can inspect and test separately.
    #[rustfmt::skip]
    let bar = menu_bar!(
        (
            button("File"),
            root(menu_items!(
                (button("New Sandbox").on_press(Message::MenuSelected(MenuAction::NewSandbox)))
                (
                    button("Open"),
                    nested(menu_items!(
                        (button("Layout Recipe").on_press(Message::MenuSelected(MenuAction::OpenRecipe)))
                        (button("Data Flow Walkthrough").on_press(Message::MenuSelected(MenuAction::ExportCode)))
                    ))
                )
                (button("Save Snapshot").on_press(Message::MenuSelected(MenuAction::SaveSnapshot)))
                (
                    button("Export"),
                    nested(menu_items!(
                        (button("Rust Module").on_press(Message::MenuSelected(MenuAction::ExportCode)))
                        (button("Teaching Notes").on_press(Message::MenuSelected(MenuAction::SaveSnapshot)))
                    ))
                )
            ))
        )
        (
            button("View"),
            root(menu_items!(
                (button("Toggle Sidebar Tips").on_press(Message::MenuSelected(MenuAction::ToggleSidebarTips)))
                (button("Jump to Controls").on_press(Message::MenuSelected(MenuAction::FocusControlsPage)))
            ))
        )
        (
            button("Window"),
            root(menu_items!(
                (button("Open Inspector").on_press(Message::MenuSelected(MenuAction::OpenInspectorWindow)))
                (button("Arrange Study Layout").on_press(Message::MenuSelected(MenuAction::ArrangeStudyLayout)))
            ))
        )
        (
            button("Help"),
            root(menu_items!(
                (button("View Iced Docs").on_press(Message::MenuSelected(MenuAction::ViewDocs)))
                (button("About Sandbox").on_press(Message::MenuSelected(MenuAction::AboutSandbox)))
            ))
        )
    );

    container(bar).into()
}

#[cfg(test)]
mod tests {
    use super::menu_groups;

    #[test]
    fn dashboard_menu_groups_match_expected_labels_and_sizes() {
        assert_eq!(
            menu_groups(),
            vec![("File", 4), ("View", 2), ("Window", 2), ("Help", 2)]
        );
    }
}
