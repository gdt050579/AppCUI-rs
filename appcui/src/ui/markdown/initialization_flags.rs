#[repr(u8)]
#[derive(Eq,PartialEq, Copy, Clone)]
/// Optional features for a [`super::Markdown`] control.
///
/// `None` renders the document only. `ScrollBars` adds bars when the content is
/// larger than the control.
pub enum Flags {
    None,
    ScrollBars
}