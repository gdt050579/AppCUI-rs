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

