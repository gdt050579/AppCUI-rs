//! A window UI control that serves as a container for other controls.
//!
//! The Window control provides a movable, resizable container with a title bar and borders.
//! It can contain multiple child controls and manages their layout within its boundaries.

mod border;
pub mod events;
mod initialization_flags;
mod modal_window;
mod resize_move_status;
#[cfg(test)]
mod tests;
mod title;
pub mod toolbar;
mod window;

pub use self::initialization_flags::Background;
pub use self::initialization_flags::Flags;
pub use self::initialization_flags::Type;
use border::Border;
pub use modal_window::ModalWindow;
use resize_move_status::ResizeMoveStatus;
use title::Title;
pub use window::Window;
