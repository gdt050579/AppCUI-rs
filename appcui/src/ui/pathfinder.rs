//! A pathfinder UI control for navigating and selecting file system paths.
//!
//! The PathFinder control provides an interface for browsing directories and selecting files.
//! It supports both directory tree navigation and path text entry with validation.

mod pathfinder;
mod inner_pathfinder;
mod initialization_flags;
pub mod events;
#[cfg(test)]
mod tests;

pub use self::pathfinder::PathFinder;
pub(crate) use self::pathfinder::GenericPathFinder; 
pub use self::initialization_flags::Flags;