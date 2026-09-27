//! The application bar: a unique top strip for menus and app-wide items.
//!
//! There is one application bar per process. Enable it with
//! [`app_bar`](crate::system::MultiWindowAppBuilder::app_bar) when building the
//! [`crate::system::App`]. Controls then obtain it with `appbar()`, add items with
//! [`AppBar::add`], and show them from [`events::AppBarEvents::on_update`].
//!
//! When focus changes, every item is hidden and `on_update` is called from the
//! focused control up to the desktop. Call [`AppBar::show`] for each item that
//! should be visible. Items are then laid out by `order` (lower first) and
//! [`Side`] (`Left` or `Right`). If the terminal is too narrow, trailing items
//! are omitted.
//!
//! # Items
//!
//! | Type | Role |
//! |------|------|
//! | [`MenuButton`] | Opens a [`crate::ui::Menu`] |
//! | [`Button`] | Clickable caption |
//! | [`ToggleButton`] | On/off button |
//! | [`SwitchButton`] | Two captions (and optional symbol) |
//! | [`Label`] | Static text |
//! | [`Separator`] | Vertical `|` divider |
//!
//! # Examples
//!
//! ```rust, no_run
//! use appcui::prelude::*;
//!
//! fn main() -> Result<(), appcui::system::Error> {
//!     App::new().app_bar().window(MyWin::new).run()
//! }
//!
//! #[Window(events = AppBarEvents)]
//! struct MyWin {
//!     h_save: Handle<appbar::Button>,
//! }
//!
//! impl MyWin {
//!     fn new() -> Self {
//!         let mut w = Self {
//!             base: window!("'Demo',a:c,w:40,h:8"),
//!             h_save: Handle::None,
//!         };
//!         w.h_save = w.appbar().add(appbar::Button::new("&Save", 0, appbar::Side::Left));
//!         w
//!     }
//! }
//!
//! impl AppBarEvents for MyWin {
//!     fn on_update(&self, appbar: &mut AppBar) {
//!         appbar.show(self.h_save);
//!     }
//! }
//! ```

pub mod events;
mod app_bar;
mod app_bar_item;
mod item_base;
mod menu_button;
mod toggle_button;
mod switch_button;
mod separator;
mod button;
mod label;
mod side;
mod item_status;
#[cfg(test)]
mod tests;


use self::app_bar_item::AppBarItem;
use self::item_base::ItemBase;
use self::item_status::ItemStatus;
pub use self::menu_button::MenuButton;
pub use self::separator::Separator;
pub use self::label::Label;
pub use self::button::Button;
pub use self::toggle_button::ToggleButton;
pub use self::switch_button::SwitchButton;
pub use self::switch_button::SwitchButtonSymbol;
pub use self::app_bar::AppBar;
pub use self::side::Side;