//! Application builders used to configure and start an AppCUI application.
//!
//! Obtain a builder from the matching [`App`](crate::system::App) constructor, chain
//! configuration methods, then call `run` to start the event loop.
//!
//! | Constructor | Builder | Typical use |
//! |-------------|---------|-------------|
//! | [`App::new`](crate::system::App::new) | [`MultiWindowAppBuilder`] | Desktop with one or more windows |
//! | [`App::single_window`](crate::system::App::single_window) | [`SingleWindowAppBuilder`] | A single window that fills the desktop |
//! | [`App::frame_app`](crate::system::App::frame_app) | [`FrameAppBuilder`] | Games or animations that update every frame |
//! | [`App::input_app`](crate::system::App::input_app) | [`InputAppBuilder`] | Custom paint and input without window chrome |
//!
//! Every builder shares terminal settings such as size, title, backend, logging,
//! color schema, and screen restore. Window-based builders also expose desktop
//! UI settings (app bar, command bar, theme, timers, and a custom desktop).

mod internal_builder;
mod app_desktop;
mod multi_window_app_builder;
mod single_window_app_builder;
mod frame_app_builder;
mod input_app_builder;
#[cfg(test)]
mod tests;

pub(crate) use internal_builder::impl_terminal_builder_methods;
pub(crate) use internal_builder::impl_ui_builder_methods;
pub(crate) use app_desktop::impl_app_desktop_methods;
pub(crate) use internal_builder::InternalBuilder;

pub use multi_window_app_builder::MultiWindowAppBuilder;
pub use single_window_app_builder::SingleWindowAppBuilder;
pub use frame_app_builder::FrameAppBuilder;
pub use frame_app_builder::FrameApp;
pub use input_app_builder::InputAppBuilder;
pub use input_app_builder::InputApp;
