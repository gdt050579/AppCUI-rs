//! # Toolbar
//! 
//! The toolbar module provides components for creating and managing toolbar elements in a window.
//! 
//! A toolbar is an area over the margins of a window (top or bottom) where various interactive
//! and non-interactive controls reside. Toolbar items are simpler controls that convey their events
//! directly to the window via the `ToolBarEvents` trait.
//! 
//! ## Toolbar Organization
//! 
//! All toolbar items are organized into groups. A group is similar to a flow layout where
//! all toolbar items are aligned one after another, depending on the direction of the flow.
//! Toolbar items don't have a layout of themselves - their position is determined by the group
//! they belong to.
//! 
//! Groups can be positioned in one of four locations:
//! - `TopLeft`: Items flow from left to right at the top of the window
//! - `BottomLeft`: Items flow from left to right at the bottom of the window
//! - `TopRight`: Items flow from right to left at the top of the window
//! - `BottomRight`: Items flow from right to left at the bottom of the window
//! 
//! ## Toolbar Items
//! 
//! The module provides several types of toolbar items:
//! 
//! - `Button`: A clickable item that triggers actions when pressed
//! - `CheckBox`: A toggleable item with checked/unchecked states
//! - `Label`: A non-interactive item for displaying text
//! - `SingleChoice`: A selectable item where only one can be selected in a group
//! 
//! ## Using Toolbars
//! 
//! To add toolbar items to a window:
//! 
//! 1. Create a group with `window.toolbar().create_group(GroupPosition::...)`
//! 2. Create toolbar items with their respective constructors
//! 3. Add items to the group with `window.toolbar().add(group, item)`
//! 4. Implement appropriate event handlers in your window

#[macro_use]
mod item_base;
mod group;
mod toolbar_item;
#[macro_use]
mod toolbar;
mod position_helper;
mod paint_data;
mod label;
mod hotkey;
mod tag;
mod close_button;
mod maximize_restore_button;
mod resize_corner;
mod button;
mod checkbox;
mod single_choice;
mod symbol_attr_state;


use self::item_base::ItemBase;
pub (super) use self::toolbar_item::ToolBarItem;
use self::toolbar::AddToToolbar;
use self::position_helper::PositionHelper;
use self::paint_data::PaintData;
use self::symbol_attr_state::SymbolAttrState;
pub (super) use self::toolbar::ToolBar;
pub use self::group::GroupPosition;
pub use self::group::Group;

// tool bar items (public)
pub use self::label::Label;
pub use self::button::Button;
pub use self::checkbox::CheckBox;
pub use self::single_choice::SingleChoice;

// tool bar items (internal)
pub(super) use self::hotkey::HotKey;
pub(super) use self::tag::Tag;
pub(super) use self::close_button::CloseButton;
pub(super) use self::maximize_restore_button::MaximizeRestoreButton;
pub(super) use self::resize_corner::ResizeCorner;