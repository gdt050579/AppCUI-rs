//! # System Module
//!
//! The system module provides core functionality for managing the application runtime, including:
//! - Application lifecycle management
//! - Event handling and processing
//! - Timer management
//! - Background task execution
//! - Theme management
//! - Menu and control handle management
//!
//! ## Key Components
//!
//! ### Application Structure
//! The `App` structure is the main entry point for creating and managing applications:
//! - Provides a builder pattern for application configuration
//! - Manages the application's lifecycle (initialization, running, shutdown)
//! - Handles window management and modal dialogs
//! - Controls the application's event loop
//! - Manages application-wide resources and state
//!
//! ### Handle System
//! The `Handle<T>` generic type provides a type-safe way to reference and manage UI components:
//! - Type-safe references to UI controls, menus, and other components
//! - Unique identification of components across the application
//! - Safe component lifecycle management
//! - Support for component hierarchy and relationships
//! - Thread-safe component access
//!
//! ### Clipboard System
//! The `Clipboard` structure provides system-wide clipboard functionality:
//! - Text-based clipboard operations
//! - Thread-safe clipboard access
//! - Platform-independent clipboard management
//! - Integration with terminal capabilities
//!
//! ### Timer System
//! The timer system provides functionality for scheduling and managing timed events:
//! - Create and manage multiple timers
//! - Control timer states (running, paused, stopped)
//! - Handle timer events and callbacks
//! - Thread-safe timer operations
//!
//! ### Background Tasks
//! The background task system enables asynchronous operations:
//! - Execute tasks in separate threads
//! - Communicate between main and background threads
//! - Handle task lifecycle events
//! - Manage task state and results
//!
//! ### Theme Management
//! The theme system provides consistent styling across the application:
//! - Predefined themes (Default, Dark Gray, Light)
//! - Custom theme creation
//! - Menu-specific theming
//! - Color and style management
//!
//! ## Usage Example
//! ```rust, no_compile
//! use appcui::prelude::*;
//! 
//! // Create a new application with system configuration
//! let app = App::new()
//!     .size(Size::new(80, 25))
//!     .title("My App")
//!     .menu_bar()
//!     .command_bar()
//!     .theme(Theme::new(Themes::DarkGray))
//!     .timers_count(4)
//!     .build()?;
//! 
//! // Start the application's main loop
//! app.run();
//! ```
//!
//! ## Error Handling
//! The system module provides error handling through the `Error` type:
//! - `InitializationFailure`: Failed to initialize system components
//! - `InvalidFeature`: Requested feature is not available
//! - `InvalidParameter`: Invalid parameter provided to system functions
//!
//! ## Thread Safety
//! The system module is designed to be thread-safe:
//! - Safe concurrent access to shared resources
//! - Thread-safe event handling
//! - Synchronized timer operations
//! - Protected background task execution

mod theme;
mod app;
mod clipboard;
mod runtime_manager;
mod runtime_manager_traits;
mod tooltip;
mod error;
mod handle;
mod control_handle_manager;
mod menu_handle_manager;
mod timer;
mod background_task;
#[cfg(test)]
mod tests;
mod builder;
#[cfg(feature="EVENT_RECORDER")]
mod event_recorder;

pub use self::theme::Theme;
pub use self::theme::Themes;
pub(crate) use self::runtime_manager_traits::ThemeMethods;
pub use self::handle::Handle;
pub (crate) use self::handle::HandleSupport;
pub (crate) use self::theme::MenuTheme;
pub (crate) use self::control_handle_manager::ControlHandleManager;
pub (crate) use self::menu_handle_manager::MenuHandleManager;

pub use self::app::App;
pub use self::clipboard::Clipboard;
pub use self::error::Error;
pub use self::error::ErrorKind;
pub (crate) use self::runtime_manager::RuntimeManager;
pub (crate) use self::runtime_manager_traits::LayoutMethods;
pub (crate) use self::runtime_manager_traits::PaintMethods;
pub (crate) use self::runtime_manager_traits::TimerMethods;
pub (crate) use self::tooltip::ToolTip;
pub use self::builder::Builder;
pub use self::timer::Timer;
pub use self::background_task::BackgroundTask;
pub use self::background_task::BackgroundTaskConector;