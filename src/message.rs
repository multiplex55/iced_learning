//! Message types describe *events* in Iced.
//!
//! New learners can think of this enum as the contract between the `view`
//! function and the `update` function: widgets emit messages, and `update`
//! handles them.

use crate::app::Page;

#[derive(Debug, Clone)]
pub enum Message {
    Navigate(Page),
    SharedTextChanged(String),
    CounterIncremented,
    CounterDecremented,
    ToggleMenu,
    ToggleChildWindow,
    ResetSandbox,
}
