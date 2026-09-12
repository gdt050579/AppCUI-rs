use EnumBitFlags::EnumBitFlags;

#[derive(Copy,Clone,PartialEq,Eq)]
/// Identifies one of the two panes of an [`super::HSplitter`].
///
/// `Top` is the upper panel, `Bottom` the lower panel.
pub enum Panel {
    /// The upper pane.
    Top,
    /// The lower pane.
    Bottom,
}

#[derive(Copy,Clone,PartialEq,Eq)]
/// How an [`super::HSplitter`] redistributes space when its parent is resized.
///
/// Preserve the relative split, keep the top panel's size, or keep the bottom
/// panel's size.
pub enum ResizeBehavior {
    /// Keep the same relative split (for example 30% / 70%) when the parent grows or shrinks.
    PreserveAspectRatio,
    /// Keep the top pane's height in character cells; the bottom pane absorbs the change.
    PreserveTopPanelSize,
    /// Keep the bottom pane's height in character cells; the top pane absorbs the change.
    PreserveBottomPanelSize,
}

#[EnumBitFlags(bits = 8)]
/// Initialization flags for an [`struct@super::HSplitter`].
///
/// Combine values with `|`. `Flags::None` draws the splitter independently of the
/// parent window frame.
pub enum Flags {
    /// Join the splitter line with neighboring window or panel borders.
    MergeBorders = 0x0001,
}
