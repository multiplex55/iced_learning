# Iced notes

## Core mental model

1. The user interacts with a widget.
2. The widget emits a `Message`.
3. `update` handles that message and mutates state.
4. `view` runs again and renders the new state.

## Learning advice

- Start with one page and one piece of state.
- Prefer explicit enums and structs while learning.
- Add async tasks only when a demo really needs background work.
- Keep the `view` readable even if it means repeating a little code early on.

## Optimizations and best practices

- Keep `view` functions mostly declarative: they should describe the current UI, not perform business logic.
- Split page-local state from root-global state. Root state is for data shared across pages; local lesson state is better when it only matters to one demo.
- Use helper widgets/components when they remove repetition without hiding the important Iced concepts.
- Avoid deeply nested rows/columns by extracting meaningful sections into local variables.
- Favor deterministic reducers and helper modules for state transitions so unit tests stay fast and stable.
- Minimize unnecessary cloning by storing compact enums and deriving display strings on demand.
- As the app grows, group messages by page or feature so the root `Message` enum remains readable.

## Interesting items to notice in this repository

- `src/menu.rs` mirrors the `iced_aw` dashboard menu with pure metadata. That pattern is educational because it makes menu hierarchy tests trivial.
- `src/forms.rs` keeps validation pure. This is closer to a production recommendation than some of the more heavily annotated demo pages.
- `src/pages/windows.rs` models child-window records explicitly, even though the current Iced builder API still limits distinct per-window widget trees.
- `src/pages/advanced.rs` notes the Iced 0.13 `Task` terminology. Older tutorials may still say `Command`, so version drift is worth watching.
