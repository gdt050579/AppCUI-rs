use std::fmt::*;

#[derive(Debug)]
pub enum Error {
    XYParameterUsedWithDock,
    AnchorParameterUsedWithDock,
    PivotParameterUsedWithDock,
    AlignParameterUsedWithDock,
    WidthParameterUsedWithTopOrBottomDock,
    HeightParameterUsedWithLeftOrRightDock,
    WidthOrHeightParameterUsedWithDockFill,
    XYParameterUsedWithAlign,
    AnchorParameterUsedWithAlign,
    PivotParameterUsedWithAlign,
    DockParameterUsedWithAlign,
    AnchorParameterUsedWithXY,
    CornerAnchorParameterUsedWithXY,
    CornerAnchorParameterUsedWithPivot,
    AllAnchorsParameterUsedWithXY,
    AllAnchorsParameterUsedWithSize,
    AllAnchorsParameterUsedWithPivot,
    LeftTopRightAnchorsUsedWithXY,
    LeftTopRightAnchorsUsedWithWidth,
    LeftTopRightAnchorsUsedWithPivot,
    LeftRightAnchorsUsedWithX,
    LeftRightAnchorsUsedWithWidth,
    LeftRightAnchorsUsedWithoutPivot,
    LeftRightAnchorsUsedWithoutY,
    LeftRightAnchorsUsedWithInvalidPivot,
    LeftBottomRightAnchorsUsedWithXY,
    LeftBottomRightAnchorsUsedWithWidth,
    LeftBottomRightAnchorsUsedWithPivot,
    TopBottomAnchorsUsedWithY,
    TopBottomAnchorsUsedWithHeight,
    TopBottomAnchorsUsedWithoutX,
    TopBottomAnchorsUsedWithoutPivot,
    TopBottomAnchorsUsedWithInvalidPivot,
    TopLeftBottomAnchorsUsedWithXY,
    TopLeftBottomAnchorsUsedWithHeight,
    TopLeftBottomAnchorsUsedWithPivot,
    NoParameters,
    InvalidLayoutRule
}

impl Error {
    fn description(&self) -> &'static str {
        match self {
            Error::NoParameters => "No parameters provided to the LayoutBuilder method ! Please provide either an absolute layout, a docked layout, a pivot layout, an alignment layout or an anchored-based  layout !",
            Error::XYParameterUsedWithDock => "When ('dock') parameter is used, 'x' and 'y' parameters can not be used !",
            Error::AnchorParameterUsedWithDock => "When ('dock') parameter is used, anchor parameters ('top', 'bottom', 'left' and 'right') can not be used !",
            Error::PivotParameterUsedWithDock => "When ('dock') parameter is used, 'pivot' parameter can not be used !",
            Error::AlignParameterUsedWithDock => "When ('dock') parameter is used, 'align' parameter can not be used !",
            Error::WidthParameterUsedWithTopOrBottomDock => "When ('dock') parameter is used with the value 'Dock:Top' or 'Dock:Bottom', the 'width' parameter can not be used as it is infered from the parent's width !",
            Error::HeightParameterUsedWithLeftOrRightDock => "When ('dock') parameter is used with the value 'Dock:Left' or 'Dock:Right', the 'height' parameter can not be used as it is infered from the parent's height !",
            Error::WidthOrHeightParameterUsedWithDockFill => "When ('dock') parameter is used with the value 'Dock:Fill', the 'width' and 'height' parameters can not be used as they are infered from the parent's width and height !",
            Error::XYParameterUsedWithAlign => "When ('align') parameter is used,'x' and 'y' parameters can not be used !",
            Error::AnchorParameterUsedWithAlign => "When ('align') parameter is used, anchor parameters ('top', 'bottom', 'left' and 'right') can not be used !",
            Error::PivotParameterUsedWithAlign => "When ('align') parameter is used, 'pivot' parameter can not be used !",
            Error::DockParameterUsedWithAlign => "When ('align') parameter is used, 'dock' parameter can not be used !",
            Error::AnchorParameterUsedWithXY => "When ('x' and 'y') parameters are used, anchor parameters ('top', 'bottom', 'left' and 'right') can not be used !",
            Error::CornerAnchorParameterUsedWithXY => "When a corner anchor is provided - e.g ('top' with `left`, 'top' with `right`, 'bottom' with `left` or 'bottom' with `right`) - 'x' and 'y' parameters can not be used as they are infered from the anchor !",
            Error::CornerAnchorParameterUsedWithPivot => "When a corner anchor is provided - e.g ('top' with `left`, 'top' with `right`, 'bottom' with `left` or 'bottom' with `right`) - 'pivot' parameter can not be used as it is infered from the anchor !",
            Error::InvalidLayoutRule => "The layout rule (combination of parameters) is invalid !",
            Error::AllAnchorsParameterUsedWithXY => "When all anchor parameters ('left', 'top', 'right' and 'bottom') are used, 'x' and 'y' parameters can not be used as they are infered from the anchors !",
            Error::AllAnchorsParameterUsedWithSize => "When all anchor parameters ('left', 'top', 'right' and 'bottom') are used, 'width' and 'height' parameters can not be used as they are infered from the anchors !",
            Error::AllAnchorsParameterUsedWithPivot => "When all anchor parameters ('left', 'top', 'right' and 'bottom') are used, 'pivot' parameter can not be used as it is infered from the anchors !",
            Error::LeftTopRightAnchorsUsedWithXY => "When (left,top,right) anchors are used together, 'x' and 'y' parameter can not be used as they are infered from the anchors !",
            Error::LeftTopRightAnchorsUsedWithWidth => "When (left,top,right) anchors are used together, 'width' parameter can not be used as it is infered from the anchors !",
            Error::LeftTopRightAnchorsUsedWithPivot => "When (left,top,right) anchors are used together, 'pivot' parameter can not be used as it is infered from the anchors !",
            Error::LeftRightAnchorsUsedWithX => "When (left,right) anchors are used together, 'x' parameter can not be used as it is infered from the anchors !",
            Error::LeftRightAnchorsUsedWithWidth => "When (left,right) anchors are used together, 'width' parameter can not be used as it is infered from the anchors !",
            Error::LeftRightAnchorsUsedWithoutPivot => "When (left,right) anchors are used together, 'pivot' parameter must be provided !",
            Error::LeftRightAnchorsUsedWithoutY => "When (left,right) anchors are used together, 'y' parameter must be provided !",
            Error::LeftRightAnchorsUsedWithInvalidPivot => "When (left,right) anchors are used together, only Pivot::TopCenter, Pivot::Center and Pivot::BottomCenter pivot values are allowed !",
            Error::LeftBottomRightAnchorsUsedWithXY => "When (left,bottom,right) anchors are used together, 'x' and 'y' parameter can not be used as they are infered from the anchors !",
            Error::LeftBottomRightAnchorsUsedWithWidth => "When (left,bottom,right) anchors are used together, 'width' parameter can not be used as it is infered from the anchors !",
            Error::LeftBottomRightAnchorsUsedWithPivot => "When (left,bottom,right) anchors are used together, 'pivot' parameter can not be used as it is infered from the anchors !",
            Error::TopBottomAnchorsUsedWithY => "When (top,bottom) anchors are used together, 'y' parameter can not be used as it is infered from the anchors !",
            Error::TopBottomAnchorsUsedWithHeight => "When (top,bottom) anchors are used together, 'height' parameter can not be used as it is infered from the anchors !",
            Error::TopBottomAnchorsUsedWithoutX => "When (top,bottom) anchors are used together, 'x' parameter must be provided !",
            Error::TopBottomAnchorsUsedWithoutPivot => "When (top,bottom) anchors are used together, 'pivot' parameter must be provided !",
            Error::TopBottomAnchorsUsedWithInvalidPivot => "When (top,bottom) anchors are used together, only Pivot::LeftCenter, Pivot::Center and Pivot::RightCenter pivot values are allowed !",
            Error::TopLeftBottomAnchorsUsedWithXY => "When (top,left,bottom) anchors are used together, 'x' and 'y' parameter can not be used as they are infered from the anchors !",
            Error::TopLeftBottomAnchorsUsedWithHeight => "When (top,left,bottom) anchors are used together, 'height' parameter can not be used as it is infered from the anchors !",
            Error::TopLeftBottomAnchorsUsedWithPivot => "When (top,left,bottom) anchors are used together, 'pivot' parameter can not be used as it is infered from the anchors !",
        }
    }
}   

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Layout error: {}", self.description())
    }
}