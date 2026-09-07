use EnumBitFlags::EnumBitFlags;

#[derive(Copy,Clone,PartialEq,Eq)]
/// Identifies one of the two panes of a [`super::VSplitter`].
///
/// `Left` is the left panel, `Right` the right panel.
pub enum Panel {
    Left,
    Right,
}

#[derive(Copy,Clone,PartialEq,Eq)]
/// How a [`super::VSplitter`] redistributes space when its parent is resized.
///
/// Preserve the relative split, keep the left panel's size, or keep the right
/// panel's size.
pub enum ResizeBehavior {
    PreserveAspectRatio,
    PreserveLeftPanelSize,
    PreserveRightPanelSize,
}

#[EnumBitFlags(bits = 8)]
pub enum Flags {
    MergeBorders = 0x0001,
}
