use crate::parameter_parser::{alignament::Alignament, NamedParamsMap};

static LAYOUT_PARAMS: [&str; 10] = ["x", "y", "left", "top", "right", "bottom", "align", "dock", "width", "height"];

macro_rules! should_not_use {
    ($param:expr, $msg:literal) => {
        if $param {
            panic!($msg);
        }
    };
}
#[repr(u8)]
#[derive(Copy, Clone)]
pub(super) enum Anchors {
    None = 0,

    Left = 0x01,
    Top = 0x02,
    Right = 0x04,
    Bottom = 0x08,

    // 2 anchors
    LeftRight = 0x05, // Left | Right
    TopBottom = 0x0A, // Top  | Bottom,

    // Corners
    TopLeft = 0x03,     // Top | Left
    TopRight = 0x06,    // Top | Right
    BottomLeft = 0x09,  // Bottom | Left
    BottomRight = 0x0C, // Bottom | Right

    // Three
    LeftTopRight = 0x07,    // Left | Top | Right
    LeftBottomRight = 0x0D, // Left | Bottom | Right
    TopLeftBottom = 0x0B,   // Top | Left | Bottom
    TopRightBottom = 0x0E,  // Top | Right | Bottom

    // All
    All = 0x0F,
}
impl Anchors {
    fn new(left: bool, top: bool, right: bool, bottom: bool) -> Anchors {
        let mut flags = 0u8;
        flags |= if left { Anchors::Left as u8 } else { 0 };
        flags |= if right { Anchors::Right as u8 } else { 0 };
        flags |= if top { Anchors::Top as u8 } else { 0 };
        flags |= if bottom { Anchors::Bottom as u8 } else { 0 };
        match flags {
            0 => Anchors::None,

            0x01 => Anchors::Left,
            0x02 => Anchors::Top,
            0x03 => Anchors::TopLeft,
            0x04 => Anchors::Right,
            0x05 => Anchors::LeftRight,
            0x06 => Anchors::TopRight,
            0x07 => Anchors::LeftTopRight,
            0x08 => Anchors::Bottom,
            0x09 => Anchors::BottomLeft,
            0x0A => Anchors::TopBottom,
            0x0B => Anchors::TopLeftBottom,
            0x0C => Anchors::BottomRight,
            0x0D => Anchors::LeftBottomRight,
            0x0E => Anchors::TopRightBottom,
            0x0F => Anchors::All,

            _ => Anchors::None,
        }
    }
}
fn copy_layout_params(s: &mut String, params: &NamedParamsMap) {
    let mut one_added = false;
    for p in LAYOUT_PARAMS {
        if let Some(value) = params.get(p) {
            if one_added {
                s.push_str(" , ");
            }
            s.push_str(p);
            s.push(':');
            s.push_str(value.get_string());
            one_added = true;
        }
    }
}

fn analyze_layout_validity(params: &NamedParamsMap) {
    let x = params.contains("x");
    let y = params.contains("y");
    let left = params.contains("left");
    let right = params.contains("right");
    let top = params.contains("top");
    let bottom = params.contains("bottom");
    let align = params.contains("align");
    let dock = params.contains("dock");
    let width = params.contains("width");
    let height = params.contains("height");
    // same logic as the one from layout mode
    if dock {
        should_not_use!(x, "When ('dock' or 'd') parameter is used,'x' parameter can not be used !");
        should_not_use!(y, "When ('dock' or 'd') parameter is used,'y' parameter can not be used !");
        should_not_use!(top, "When ('dock' or 'd') parameter is used,('top' or 't') parameters can not be used !");
        should_not_use!(
            bottom,
            "When ('dock' or 'd') parameter is used,('bottom' or 'b') parameters can not be used !"
        );
        should_not_use!(
            left,
            "When ('dock' or 'd') parameter is used,('left' or 'l') parameters can not be used !"
        );
        should_not_use!(
            right,
            "When ('dock' or 'd') parameter is used,('right' or 'r') parameters can not be used !"
        );
        should_not_use!(
            align,
            "When ('dock' or 'd') parameter is used,('align' or 'a') parameters can not be used !"
        );
        return;
    }
    // x , y
    if x && y {
        should_not_use!(left, "When (x,y) parameters are used, ('left' or 'l') parameter can not be used !");
        should_not_use!(right, "When (x,y) parameters are used, ('right' or 'r') parameter can not be used !");
        should_not_use!(top, "When (x,y) parameters are used, ('top' or 't') parameter can not be used !");
        should_not_use!(bottom, "When (x,y) parameters are used, ('bottom' or 'b') parameter can not be used !");
        return;
    }
    let anchors = Anchors::new(left, top, right, bottom);
    match anchors {
        Anchors::LeftRight => {
            should_not_use!(x, "When (left,right) parameters are used together, 'X' parameter can not be used");
            should_not_use!(width,"When (left,right) parameters are used toghere, ('width' or 'w') parameters can not be used as the width is deduced from left-right difference");

            if let Some(value) = params.get("align") {
                match value.to_align() {
                    Alignament::Top | Alignament::Center | Alignament::Bottom => {}
                    _ => panic!("When (left,right) are provided, only Top(t), Center(c) and Bottom(b) alignament values are allowed !"),
                }
            }
        }
        Anchors::TopBottom => {
            should_not_use!(y, "When (top,bottom) parameters are used together, 'Y' parameter can not be used");
            should_not_use!(height,"When (top,bottom) parameters are used toghere, ('height' or 'h') parameters can not be used as the width is deduced from bottom-top difference");

            if let Some(value) = params.get("align") {
                match value.to_align() {
                    Alignament::Top | Alignament::Center | Alignament::Bottom => {}
                    _ => panic!("When (top,bottom) are provided, only Left(l), Center(c) and Right(r) alignament values are allowed !"),
                }
            }
        }
        Anchors::TopLeft | Anchors::TopRight | Anchors::BottomLeft | Anchors::BottomRight => {
            should_not_use!(x, "When a corner anchor is being use (top,left,righ,bottom), 'x' can bot be used !");
            should_not_use!(y, "When a corner anchor is being use (top,left,righ,bottom), 'y' can bot be used !");
        }
        Anchors::LeftTopRight => {
            should_not_use!(x, "When (left,top,right) parameters are used together, 'X' parameter can not be used");
            should_not_use!(y, "When (left,top,right) parameters are used together, 'Y' parameter can not be used");
            should_not_use!(
                width,
                "When (left,top,right) parameters are used together, 'width' parameter can not be used"
            );
            should_not_use!(
                align,
                "When (left,top,right) parameters are used together, 'align' parameter can not be used"
            );
        }
        Anchors::LeftBottomRight => {
            should_not_use!(x, "When (left,bottom,right) parameters are used together, 'X' parameter can not be used");
            should_not_use!(y, "When (left,bottom,right) parameters are used together, 'Y' parameter can not be used");
            should_not_use!(
                width,
                "When (left,bottom,right) parameters are used together, 'width' parameter can not be used"
            );
            should_not_use!(
                align,
                "When (left,bottom,right) parameters are used together, 'align' parameter can not be used"
            );
        }
        Anchors::TopLeftBottom => {
            should_not_use!(x, "When (top,left,bottom) parameters are used together, 'X' parameter can not be used");
            should_not_use!(y, "When (top,left,bottom) parameters are used together, 'Y' parameter can not be used");
            should_not_use!(
                height,
                "When (top,left,bottom) parameters are used together, 'height' parameter can not be used"
            );
            should_not_use!(
                align,
                "When (top,left,bottom) parameters are used together, 'align' parameter can not be used"
            );
        }
        Anchors::TopRightBottom => {
            should_not_use!(x, "When (top,right,bottom) parameters are used together, 'X' parameter can not be used");
            should_not_use!(y, "When (top,right,bottom) parameters are used together, 'Y' parameter can not be used");
            should_not_use!(
                height,
                "When (top,right,bottom) parameters are used together, 'height' parameter can not be used"
            );
            should_not_use!(
                align,
                "When (top,right,bottom) parameters are used together, 'align' parameter can not be used"
            );
        }
        Anchors::All => {
            should_not_use!(
                x,
                "When (left,top,right,bottom) parameters are used together, 'X' parameter can not be used"
            );
            should_not_use!(
                y,
                "When (left,top,right,bottom) parameters are used together, 'Y' parameter can not be used"
            );
            should_not_use!(
                height,
                "When (left,top,right,bottom) parameters are used together, 'height' parameter can not be used"
            );
            should_not_use!(
                width,
                "When (left,top,right,bottom) parameters are used together, 'widyj' parameter can not be used"
            );
            should_not_use!(
                align,
                "When (left,top,right,bottom) parameters are used together, 'align' parameter can not be used"
            );
        }
        _ => {
            panic!("Invalid layout format --> this combination can not be used to create a layout for a control ");
        }
    }
}
pub(super) fn add_layout(s: &mut String, params: &NamedParamsMap) {
    s.push_str("Layout::new(\"");
    analyze_layout_validity(params);
    copy_layout_params(s, params);
    s.push_str("\")");
}
