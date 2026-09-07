#[repr(u8)]
#[derive(Copy,Clone,PartialEq,Eq,Debug)]
/// Border style of a [`super::Panel`].
///
/// Choose a window-like frame, a page, a top bar, or a raised/sunken 3D look.
pub enum Type {
    /// A simple rectangular frame around the children, for example `┌────┐`.
    Border,
    /// A window-like frame with a title area.
    Window,
    /// A page-style border that suggests a sheet of content.
    Page,
    /// A bar along the top edge only, with no full frame.
    TopBar,
    /// A raised 3D border that stands out from the parent.
    Raised,
    /// A sunken 3D border that appears inset into the parent.
    Sunken
}