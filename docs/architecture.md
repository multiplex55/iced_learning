# Architecture notes

This sandbox uses a deliberately small architecture:

- `App` owns the global state.
- `Message` represents user intent or app events.
- page modules render focused examples.
- shared state stays in plain structs and enums.

## Why this shape?

Learners can understand the whole app without tracing through service locators, background workers, or dependency injection.
Once a future page genuinely needs async loading or a more advanced subsystem, it can be introduced in isolation.
