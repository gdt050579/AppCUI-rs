//! A pathfinder UI control for navigating and selecting file system paths.
//!
//! The PathFinder control provides an interface for browsing directories and selecting files.
//! It supports both directory tree navigation and path text entry with validation.

pub mod events;
mod initialization_flags;
mod inner_pathfinder;
mod pathfinder;
#[cfg(test)]
mod tests;

pub use self::initialization_flags::Flags;
pub(crate) use self::pathfinder::GenericPathFinder;
pub use self::pathfinder::PathFinder;
