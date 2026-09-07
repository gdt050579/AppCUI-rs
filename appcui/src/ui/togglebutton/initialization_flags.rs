#[repr(u8)]
#[derive(Copy,Clone,PartialEq,Eq)]
/// Visual style of a [`super::ToggleButton`].
///
/// `Normal` uses the themed button look; `Underlined` emphasizes the caption with
/// an underline.
pub enum Type {
    Normal,
    Underlined,
}