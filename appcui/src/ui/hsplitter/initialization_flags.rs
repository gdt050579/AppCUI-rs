use EnumBitFlags::EnumBitFlags;

#[derive(Copy,Clone,PartialEq,Eq)]
/// Identifies one of the two panes of an [`super::HSplitter`].
///
/// `Top` is the upper panel, `Bottom` the lower panel.
pub enum Panel {
    Top,
    Bottom,
}

#[derive(Copy,Clone,PartialEq,Eq)]
/// How an [`super::HSplitter`] redistributes space when its parent is resized.
///
/// Preserve the relative split, keep the top panel's size, or keep the bottom
/// panel's size.
pub enum ResizeBehavior {
    PreserveAspectRatio,
    PreserveTopPanelSize,
    PreserveBottomPanelSize,
}

#[EnumBitFlags(bits = 8)]
pub enum Flags {
    MergeBorders = 0x0001,
}
