

#[repr(u8)]
#[derive(Copy,Clone,PartialEq,Eq)]
/// Visual style of a [`super::Button`].
///
/// `Normal` uses the themed 3D look, `Flat` draws a simpler caption-only button, and
/// `Raised` uses a more pronounced border.
pub enum Type {
    /// Themed 3D button with a caption, for example `[  OK  ]`.
    Normal,
    /// Caption-only button without a 3D border, for example `  OK  `.
    Flat,
    /// Button with a more pronounced raised border.
    Raised
}