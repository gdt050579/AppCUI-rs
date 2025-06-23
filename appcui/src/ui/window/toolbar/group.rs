#[repr(u8)]
#[derive(Clone, Copy)]
/// Defines the position of a toolbar group within a window.
///
/// Toolbar groups can be positioned in one of four locations: top-left, bottom-left,
/// top-right, or bottom-right of the window. Each group can contain multiple toolbar
/// items (like buttons, checkboxes, labels, or single choice items) which are arranged
/// in the order they're added to the group.
///
/// When multiple toolbar items are added to a group, they're arranged horizontally:
/// - For left-aligned groups (TopLeft, BottomLeft), items are arranged from left to right
/// - For right-aligned groups (TopRight, BottomRight), items are arranged from right to left
pub enum GroupPosition {
    /// Position the group at the top-left corner of the window
    TopLeft,
    /// Position the group at the bottom-left corner of the window
    BottomLeft,
    /// Position the group at the top-right corner of the window
    TopRight,
    /// Position the group at the bottom-right corner of the window
    BottomRight,
}

#[derive(Clone, Copy)]
/// A group of toolbar items positioned at a specific location in a window.
///
/// Groups are used to organize toolbar items (like buttons, checkboxes, labels, etc.)
/// into collections that share the same position on the window's toolbar area.
/// Multiple groups can exist in the same position, in which case they'll be arranged
/// one after another.
///
/// Groups are created using the `toolbar().create_group()` method on a window,
/// and toolbar items are added to a group using the `toolbar().add()` method.
pub struct Group {
    pub(super) pos: GroupPosition,
    pub(super) id: u8,
}
impl Default for Group {
    fn default() -> Self {
        Self {
            pos: GroupPosition::TopLeft,
            id: 255,
        }
    }
}
