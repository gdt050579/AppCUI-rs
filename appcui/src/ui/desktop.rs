//! A desktop UI control that serves as the root container for an application.
//!
//! The Desktop control provides the main surface for rendering an application's interface.
//! It manages windows, dialogs, and global key bindings for the entire application.

mod desktop;
mod empty_desktop;
pub mod events;
#[cfg(test)]
mod tests;

pub use self::desktop::ArrangeWindowsMethod;
pub use self::desktop::Desktop;
pub(crate) use self::empty_desktop::EmptyDesktop;
