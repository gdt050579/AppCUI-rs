//! A window UI control that serves as a container for other controls.
//!
//! The Window control provides a movable, resizable container with a title bar and borders.
//! It can contain multiple child controls and manages their layout within its boundaries.

mod initialization_flags;
mod window;
mod modal_window;
mod title;
mod border;
mod drag_status;
pub mod toolbar;
pub mod events;
#[cfg(test)]
mod tests;

pub use window::Window;
pub use modal_window::ModalWindow;
pub use self::initialization_flags::Flags;
pub use self::initialization_flags::Type;
pub use self::initialization_flags::Background;
use drag_status::DragStatus;
use title::Title;
use border::Border;