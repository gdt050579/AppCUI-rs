use EnumBitFlags::EnumBitFlags;

#[derive(Copy,Clone,PartialEq,Eq)]
/// Identifies one of the two panes of a [`super::VSplitter`].
///
/// `Left` is the left panel, `Right` the right panel.
pub enum Panel {
    /// The left pane.
    Left,
    /// The right pane.
    Right,
}

#[derive(Copy,Clone,PartialEq,Eq)]
/// How a [`super::VSplitter`] redistributes space when its parent is resized.
///
/// Preserve the relative split, keep the left panel's size, or keep the right
/// panel's size.
pub enum ResizeBehavior {
    /// Keep the same relative split (for example 30% / 70%) when the parent grows or shrinks.
    PreserveAspectRatio,
    /// Keep the left pane's width in character cells; the right pane absorbs the change.
    PreserveLeftPanelSize,
    /// Keep the right pane's width in character cells; the left pane absorbs the change.
    PreserveRightPanelSize,
}

#[EnumBitFlags(bits = 8)]
pub enum Flags {
    MergeBorders = 0x0001,
}
