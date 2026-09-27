use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits=16)]
/// Initialization flags for a [`struct@super::Window`].
///
/// Combine values with `|`. `Flags::None` is a moveable window with a close button
/// and a fixed size.
pub enum Flags {
    /// Allow the user to resize the window from its edges and corners.
    Sizeable      = 0x0001,
    /// Hide the close button in the title bar.
    NoCloseButton = 0x0002,
    /// Prevent the user from dragging the window to a new position.
    FixedPosition = 0x0004,
}
#[repr(u8)]
#[derive(Copy,Clone,PartialEq,Eq,Default)]
/// Border style of a [`struct@super::Window`].
///
/// `Classic` uses a standard frame, `Rounded` uses rounded corners, and `Panel`
/// looks like a borderless panel.
pub enum Type {
    /// Standard square corners, for example `┌────┐`.
    #[default]
    Classic,
    /// Rounded corners, for example `╭────╮`.
    Rounded,
    /// Borderless panel look without window chrome.
    Panel,
}


#[repr(u8)]
#[derive(Copy,Clone,PartialEq,Eq,Default)]
/// Semantic background color of a [`struct@super::Window`].
///
/// `Normal` uses the theme default. `Error`, `Warning`, and `Notification` tint the
/// window to indicate severity.
pub enum Background {
    /// Theme default window background.
    #[default]
    Normal,
    /// Error tint (typically red) for failure dialogs.
    Error,
    /// Warning tint (typically yellow/olive) for caution dialogs.
    Warning,
    /// Notification tint for informational dialogs.
    Notification
}