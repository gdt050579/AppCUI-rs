//! A menu UI control that provides hierarchical command options.
//!
//! The Menu control displays a list of commands or options that users can select.
//! It supports nested submenus, separators, and keyboard shortcuts for navigation.

pub mod events;
mod menu;
mod menu_bar;
mod menu_bar_item;
mod menu_button_state;
pub(crate) mod menu_item;
mod mouse_position_info;
#[cfg(test)]
mod tests;
mod utils;

pub mod checkbox;
pub mod command;
pub mod separator;
pub mod single_choice;
pub mod submenu;

pub use self::menu::Menu;
pub use self::menu_bar::MenuBar;
use self::menu_bar_item::MenuBarItem;
pub(crate) use self::menu_item::MenuItem;
use self::menu_item::MenuItemWrapper;

pub use self::checkbox::CheckBox;
pub use self::command::Command;
pub use self::separator::Separator;
pub use self::single_choice::SingleChoice;
pub use self::submenu::SubMenu;
