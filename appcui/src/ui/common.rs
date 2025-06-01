//! Common types and utilities used across various UI controls.
//!
//! The common module provides fundamental types, events, and interfaces that are
//! shared between different UI components for consistent behavior and appearance.

pub(crate) mod control_base;
pub(crate) mod control_char_attributes_state;
pub(crate) mod control_event_wrapper;
pub(crate) mod control_manager;
pub mod traits;

pub use control_base::ControlBase;
pub(crate) use control_base::StatusFlags;
pub(crate) use control_char_attributes_state::ControlCharAttributesState;
pub(crate) use control_event_wrapper::ControlEvent;
pub(crate) use control_event_wrapper::ControlEventData;
pub(crate) use control_manager::ControlManager;
