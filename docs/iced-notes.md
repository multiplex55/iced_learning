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
