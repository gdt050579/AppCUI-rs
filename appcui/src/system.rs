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

mod app;
mod background_task;
mod builder;
mod clipboard;
mod control_handle_manager;
mod error;
#[cfg(feature = "EVENT_RECORDER")]
mod event_recorder;
mod handle;
mod menu_handle_manager;
mod runtime_manager;
mod runtime_manager_traits;
#[cfg(test)]
mod tests;
mod theme;
mod timer;
mod tooltip;
mod system_event;

pub(crate) use self::control_handle_manager::ControlHandleManager;
pub use self::handle::Handle;
pub(crate) use self::handle::HandleSupport;
pub(crate) use self::menu_handle_manager::MenuHandleManager;
pub(crate) use self::runtime_manager_traits::ThemeMethods;
pub(crate) use self::theme::MenuTheme;
pub use self::theme::Theme;
pub use self::theme::Themes;

pub use self::app::App;
pub use self::background_task::BackgroundTask;
pub use self::background_task::BackgroundTaskConector;
pub use self::builder::Builder;
pub use self::clipboard::Clipboard;
pub use self::error::Error;
pub use self::error::ErrorKind;
pub(crate) use self::runtime_manager::RuntimeManager;
pub(crate) use self::runtime_manager_traits::LayoutMethods;
pub(crate) use self::runtime_manager_traits::PaintMethods;
pub(crate) use self::runtime_manager_traits::TimerMethods;
pub use self::timer::Timer;
pub(crate) use self::tooltip::ToolTip;

pub(crate) use self::system_event::KeyPressedEvent;
pub(crate) use self::system_event::KeyModifierChangedEvent;
pub(crate) use self::system_event::MouseButtonDownEvent;
pub(crate) use self::system_event::MouseButtonUpEvent;
pub(crate) use self::system_event::MouseDoubleClickEvent;
pub(crate) use self::system_event::MouseMoveEvent;
pub(crate) use self::system_event::MouseWheelEvent;
pub(crate) use self::system_event::SystemEvent;
pub(crate) use self::system_event::TimerPausedEvent;
pub(crate) use self::system_event::TimerStartEvent;
pub(crate) use self::system_event::TimerTickUpdateEvent;
