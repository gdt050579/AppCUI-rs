//! Rich text single-line field: same behavior as [`TextField`](super::TextField) with optional per-character styling via `on_color`.

mod attribute_text;
mod richtextfield;
pub mod events;

pub use self::attribute_text::AttributeText;
pub use self::richtextfield::RichTextField;
pub use super::textfield::Flags;
