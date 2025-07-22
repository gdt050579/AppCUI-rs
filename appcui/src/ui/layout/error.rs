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
        }
    }
}   

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Layout error: {}", self.description())
    }
}