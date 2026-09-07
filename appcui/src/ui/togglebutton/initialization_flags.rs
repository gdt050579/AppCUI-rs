#[repr(u8)]
#[derive(Copy,Clone,PartialEq,Eq)]
/// Visual style of a [`super::ToggleButton`].
///
/// `Normal` uses the themed button look; `Underlined` emphasizes the caption with
/// an underline.
pub enum Type {
    /// Themed toggle that looks like a regular button, for example `[ Bold ]`.
    Normal,
    /// Caption with an underline to show the toggled state, for example `Bold` / `B̲o̲l̲d̲`.
    Underlined,
}