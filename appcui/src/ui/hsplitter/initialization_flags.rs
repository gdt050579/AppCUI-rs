use EnumBitFlags::EnumBitFlags;

#[derive(Copy,Clone,PartialEq,Eq)]
pub enum Panel {
    Top,
    Bottom,
}

#[derive(Copy,Clone,PartialEq,Eq)]
pub enum ResizeBehavior {
    PreserveAspectRatio,
    PreserveTopPanelSize,
    PreserveBottomPanelSize,
}

#[EnumBitFlags(bits = 8)]
pub enum Flags {
    MergeBorders = 0x0001,
}
