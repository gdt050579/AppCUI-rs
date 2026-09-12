#[repr(u8)]
#[derive(Eq,PartialEq, Copy, Clone)]
/// Optional features for an [`super::ImageViewer`].
///
/// `None` draws only the image. `ScrollBars` adds bars when the image is larger
/// than the control.
pub enum Flags {
    /// Draw only the image, with no scroll bars.
    None,
    /// Show horizontal and vertical scroll bars when the image is larger than the control.
    ScrollBars,
}