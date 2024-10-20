mod textarea;
mod initialization_flags;
pub mod events;
#[cfg(test)]
mod tests;

//use self::selection::Selection;
pub use self::textarea::TextArea;
pub use self::initialization_flags::Flags;
//pub(crate) use self::char_class::CharClass;