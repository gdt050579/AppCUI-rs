use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits=16)]
pub enum Flags {
    Sizeable      = 0x0001,
    NoCloseButton = 0x0002,
    FixedPosition = 0x0004,
}
#[repr(u8)]
#[derive(Copy,Clone,PartialEq,Eq,Default)]
/// Border style of a [`struct@super::Window`].
///
/// `Classic` uses a standard frame, `Rounded` uses rounded corners, and `Panel`
/// looks like a borderless panel.
pub enum Type {
    #[default]
    Classic,
    Rounded,
    Panel,
}


#[repr(u8)]
#[derive(Copy,Clone,PartialEq,Eq,Default)]
/// Semantic background color of a [`struct@super::Window`].
///
/// `Normal` uses the theme default. `Error`, `Warning`, and `Notification` tint the
/// window to indicate severity.
pub enum Background {
    #[default]
    Normal,
    Error,
    Warning,
    Notification
}