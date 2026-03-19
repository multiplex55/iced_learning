//! Application entry point.
//!
//! This file stays intentionally small so new learners can quickly jump from
//! `main` into the `App` state, the `Message` enum, and the page modules.

mod app;
mod forms;
mod menu;
mod message;
mod pages;
mod state;
mod theme;
mod widgets;

use app::App;

fn main() -> iced::Result {
    App::run()
}
