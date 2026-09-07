#[repr(u8)]
#[derive(Copy,Clone,PartialEq,Eq,Debug)]
/// Border style of a [`super::Panel`].
///
/// Choose a window-like frame, a page, a top bar, or a raised/sunken 3D look.
pub enum Type {
    Border,
    Window,
    Page,
    TopBar,
    Raised,
    Sunken
}