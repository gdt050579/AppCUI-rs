use EnumBitFlags::EnumBitFlags;

#[derive(Copy,Clone,PartialEq,Eq)]
pub enum Panel {
    Left,
    Right,
}

#[derive(Copy,Clone,PartialEq,Eq)]
pub enum ResizeBehavior {
    PreserveAspectRatio,
    PreserveLeftPanelSize,
    PreserveRightPanelSize,
}

#[EnumBitFlags(bits = 8)]
pub enum Flags {
    MergeBorders = 0x0001,
}
