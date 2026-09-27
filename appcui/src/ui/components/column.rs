//! A named, sized, aligned header cell used by list and tree views.
//!
//! [`Column`] stores the caption, width in characters, [`crate::graphics::TextAlignment`],
//! and an optional tooltip. [`ListItem::column`](super::listitem::ListItem::column)
//! returns one per header cell; [`crate::ui::ListView`] and [`crate::ui::TreeView`]
//! also accept columns from `add_column` or the `columns:[...]` macro parameter.
//! [`crate::ui::components::ColumnsHeader`] paints the row and handles resize and sort.
//!
//! [`Type`] classifies a column as text or boolean (True/False, Yes/No, or a check mark).
//!
//! # Examples
//!
//! ```rust
//! use appcui::prelude::*;
//!
//! let mut col = Column::new("&Name", 20, TextAlignment::Left);
//! col.set_tooltip("Student name");
//! assert_eq!(col.width(), 20);
//! assert_eq!(col.alignment(), TextAlignment::Left);
//! ```

mod column;
mod data_type;

pub use column::Column;
pub use data_type::Type;
