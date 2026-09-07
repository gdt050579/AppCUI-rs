#[repr(u8)]
#[derive(Eq,PartialEq, Copy, Clone)]
/// Optional features for a [`super::Canvas`].
///
/// `None` draws only the surface. `ScrollBars` adds bars when the canvas content is
/// larger than the control.
pub enum Flags {
    None,
    ScrollBars,
}