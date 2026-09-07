

#[repr(u8)]
#[derive(Copy,Clone,PartialEq,Eq)]
/// Visual style of a [`super::Button`].
///
/// `Normal` uses the themed 3D look, `Flat` draws a simpler caption-only button, and
/// `Raised` uses a more pronounced border.
pub enum Type {
    Normal,
    Flat,
    Raised
}