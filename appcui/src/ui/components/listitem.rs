//! How a row is described for list and tree controls.
//!
//! Implement [`ListItem`] for your item type so [`crate::ui::ListView`] and
//! [`crate::ui::TreeView`] can paint cells, sort, filter, and build the header.
//! [`columns_count`](ListItem::columns_count) and [`column`](ListItem::column)
//! define the [`super::Column`]s. [`render_method`](ListItem::render_method)
//! returns a [`RenderMethod`] for each cell (or [`RenderMethod::Custom`] to use
//! [`paint`](ListItem::paint)). [`compare`](ListItem::compare) sorts;
//! [`matches`](ListItem::matches) filters when `CustomFilter` is set.
//!
//! Alternatively, `#[derive(ListItem)]` with `#[Column(...)]` on each displayed
//! field generates the same implementation.
//!
//! # Cell formats
//!
//! | Type | Typical [`RenderMethod`] |
//! |------|--------------------------|
//! | [`NumericFormat`] | `Int64`, `UInt64` |
//! | [`FloatFormat`] | `Float` |
//! | [`BoolFormat`] | `Bool` |
//! | [`DateFormat`] / [`TimeFormat`] / [`DateTimeFormat`] / [`DurationFormat`] | date and time variants |
//! | [`SizeFormat`] / [`PercentageFormat`] / [`RatingFormat`] | `Size`, `Percentage`, `Rating` |
//! | [`CurrencyFormat`] | `Currency` (`$ 12.50`) |
//! | [`Status`] / [`StatusFormat`] | `Status` (running, queued, …) |
//! | [`TemperatureFormat`] / [`AreaFormat`] / [`DistanceFormat`] / [`VolumeFormat`] / [`WeightFormat`] / [`SpeedFormat`] | unit values |
//!
//! # Examples
//!
//! ```rust
//! use appcui::prelude::*;
//!
//! #[derive(ListItem)]
//! struct Student {
//!     #[Column(name: "&Name", width: 20, align: Left)]
//!     name: String,
//!     #[Column(name: "&Grade", width: 5, align: Center)]
//!     grade: u8,
//! }
//! ```
//!
//! A manual implementation can return [`RenderMethod`] values directly:
//!
//! ```rust
//! use appcui::prelude::*;
//! use std::cmp::Ordering;
//!
//! struct Student {
//!     name: String,
//!     grade: u8,
//! }
//!
//! impl ListItem for Student {
//!     fn columns_count() -> u16 {
//!         2
//!     }
//!     fn column(index: u16) -> Column {
//!         match index {
//!             0 => Column::new("&Name", 20, TextAlignment::Left),
//!             1 => Column::new("&Grade", 5, TextAlignment::Center),
//!             _ => Column::new("", 10, TextAlignment::Left),
//!         }
//!     }
//!     fn render_method(&self, column_index: u16) -> Option<RenderMethod<'_>> {
//!         match column_index {
//!             0 => Some(RenderMethod::Text(&self.name)),
//!             1 => Some(RenderMethod::UInt64(self.grade as u64, NumericFormat::Normal)),
//!             _ => None,
//!         }
//!     }
//!     fn compare(&self, other: &Self, column_index: u16) -> Ordering {
//!         match column_index {
//!             0 => self.name.cmp(&other.name),
//!             1 => self.grade.cmp(&other.grade),
//!             _ => Ordering::Equal,
//!         }
//!     }
//! }
//! ```

pub(in crate::ui) mod listitem;
pub(in crate::ui) mod formats;
pub(in crate::ui) mod render_method;

pub use self::listitem::ListItem;
pub use self::render_method::RenderMethod;

pub use self::formats::AreaFormat;
pub use self::formats::BoolFormat;
pub use self::formats::CurrencyFormat;
pub use self::formats::DateFormat;
pub use self::formats::DateTimeFormat;
pub use self::formats::DurationFormat;
pub use self::formats::TimeFormat;
pub use self::formats::DistanceFormat;
pub use self::formats::FloatFormat;
pub use self::formats::NumericFormat;
pub use self::formats::PercentageFormat;
pub use self::formats::RatingFormat;
pub use self::formats::SizeFormat;
pub use self::formats::SpeedFormat;
pub use self::formats::Status;
pub use self::formats::StatusFormat;
pub use self::formats::TemperatureFormat;
pub use self::formats::VolumeFormat;
pub use self::formats::WeightFormat;
