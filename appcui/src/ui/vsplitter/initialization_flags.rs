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

