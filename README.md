# iced_learning

`iced_learning` is an intentionally small Rust GUI application for learning the [Iced](https://iced.rs) ecosystem.
The repository is structured as a long-lived sandbox: you can add pages, swap widgets, and experiment with state changes without first designing a full production architecture.

## Purpose

Use this project to learn how Iced applications are typically organized:

- a top-level app state,
- a `Message` enum that represents user and system events,
- an `update` function that mutates state, and
- a `view` function that renders widgets from the latest state.

The first pass keeps these concepts obvious instead of abstracting them behind services or complex async layers.

## Demo pages

The app starts on **Dashboard** and includes these extendable demo areas:

- **Dashboard**: a lightweight landing page that summarizes shared state and the purpose of the sandbox.
- **Layouts**: simple `row`, `column`, spacing, and container composition examples.
- **Controls**: text input and button-driven state updates.
- **Data flow**: how shared state moves through the update/view loop.
- **Windows**: placeholder state for future multi-window and modal experiments.
- **Advanced**: notes for future markdown, SVG, menu, tabs, and richer `iced_aw` widget demos.

## Running the app

```bash
cargo run
```

Format and test as you iterate:

```bash
cargo fmt
cargo test
```

## Code organization

- `src/main.rs`: tiny entry point that launches the sandbox.
- `src/app.rs`: the top-level `App` state, navigation, boot logic, and the central update/view flow.
- `src/message.rs`: the messages emitted by widgets and handled by `App::update`.
- `src/pages/`: focused demo pages grouped by learning topic.
- `src/widgets/`: reusable helper widgets and view builders.
- `src/state/`: shared state types that can outgrow `App` later.
- `docs/`: lightweight notes for architecture, Iced concepts, and future experiments.

## Where to look first

If you are new to Iced, open files in this order:

1. `src/main.rs`
2. `src/app.rs`
3. `src/message.rs`
4. `src/pages/dashboard.rs`
5. `src/pages/controls.rs`

That path shows the core Iced loop before you dive into extra pages.

## Features enabled in `Cargo.toml`

The dependency feature flags are written explicitly so learners can see which capabilities are being prepared for future demos:

- `iced`: `advanced`, `svg`, and `markdown` for richer widget examples.
- `iced_aw`: `tabs`, `menu`, `sidebar`, `card`, and `badge` for ecosystem widget exploration.

Add more crates only when a page actually needs them.
