use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits = 16)]
/// Initialization flags for a [`struct@super::Window`].
///
/// Combine values with `|`. `Flags::None` is a moveable window with a close button
/// and a fixed size.
pub enum Flags {
    /// Allow the user to resize the window from its edges and corners.
    Sizeable = 0x0001,
    /// Hide the close button in the title bar.
    NoCloseButton = 0x0002,
    /// Prevent the user from dragging the window to a new position.
    FixedPosition = 0x0004,
    /// Hide the title text. The frame, close button, tag, and hotkey stay visible.
    NoTitle = 0x0008,
}
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq, Default)]
/// Border style of a [`struct@super::Window`].
///
/// `Classic` uses a standard frame, `Rounded` uses rounded corners, and `Panel`
/// is a title bar without a surrounding frame. `Borderless` draws no frame and
/// has no inner margin. `Classic`, `Rounded`, and `Panel` center the title.
/// `Borderless` left-aligns it and places the tag on the right.
pub enum Type {
    /// Standard square corners, for example `┌────┐`. The title is centered.
    #[default]
    Classic,
    /// Rounded corners, for example `╭────╮`. The title is centered.
    Rounded,
    /// Title bar with no surrounding frame. The title is centered.
    Panel,
    /// No frame and no inner margin. The title, when shown, is left-aligned, and the tag is placed on the right.
    Borderless,
}

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq, Default)]
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
    Notification,
}
