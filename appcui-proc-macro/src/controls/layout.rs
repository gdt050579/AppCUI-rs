use crate::controls::control_builder;
use crate::parameter_parser;
use crate::parameter_parser::{alignment::Alignment, NamedParamsMap};
use crate::token_stream_to_string::TokenStreamToString;
use proc_macro::*;
use std::fmt::Write;
use std::str::FromStr;

macro_rules! should_not_use {
    ($param:expr, $msg:literal) => {
        if $param {
            panic!($msg);
        }
    };
}
macro_rules! should_use {
    ($param:expr, $msg:literal) => {
        if !$param {
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

struct LayoutParams {
    x: bool,
    y: bool,
    left: bool,
    right: bool,
    top: bool,
    bottom: bool,
    align: bool,
    pivot: bool,
    dock: bool,
    width: bool,
    height: bool,
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

fn validate_dock_layout(lp: &LayoutParams, params: &NamedParamsMap) {
    should_not_use!(lp.x, "When ('dock' or 'd') parameter is used, 'x' paramete can not be used !");
    should_not_use!(lp.y, "When ('dock' or 'd') parameter is used, 'y' paramete can not be used !");
    should_not_use!(
        lp.top,
        "When ('dock' or 'd') parameter is used, ('top' or 't') parameter can not be used !"
    );
    should_not_use!(
        lp.bottom,
        "When ('dock' or 'd') parameter is used, ('bottom' or 'b') parameter can not be used !"
    );
    should_not_use!(
        lp.left,
        "When ('dock' or 'd') parameter is used, ('left' or 'l') parameter can not be used !"
    );
    should_not_use!(
        lp.right,
        "When ('dock' or 'd') parameter is used, ('right' or 'r') parameter can not be used !"
    );
    should_not_use!(
        lp.align,
        "When ('dock' or 'd') parameter is used, ('align' or 'a') parameter can not be used !"
    );
    should_not_use!(
        lp.pivot,
        "When ('dock' or 'd') parameter is used, ('pivot' or 'p') parameter can not be used !"
    );
    should_not_use!(
        lp.align,
        "When ('dock' or 'd') parameter is used, ('align' or 'a') parameter can not be used !"
    );
    let dock = params.get("dock").unwrap().to_dock();
    match dock {
        parameter_parser::dock::Dock::Left | parameter_parser::dock::Dock::Right => {
            should_not_use!(
                lp.height,
                "When ('dock' or 'd') parameter is used with the value `left' or 'right` ('d:left' or 'd:right') the `height` parameter is not neccesary as the dock will expand the current control height to the height of its parent !"
            );
        }
        parameter_parser::dock::Dock::Top | parameter_parser::dock::Dock::Bottom => {
            should_not_use!(
                lp.width,
                "When ('dock' or 'd') parameter is used with the value `top' or 'bottom` ('d:top' or 'd:bottom') the `width` parameter is not neccesary as the dock will expand the current control width to the width of its parent !"
            );
        }
        parameter_parser::dock::Dock::Fill => {
            should_not_use!(
                lp.width,
                "When ('dock' or 'd') parameter is used with the value `fill` ('d:fill') the `width` parameter is not neccesary as the dock will expand the current control width to the width of its parent !"
            );
            should_not_use!(
                lp.height,
                "When ('dock' or 'd') parameter is used with the value `fill` ('d:fill') the `height` parameter is not neccesary as the dock will expand the current control height to the height of its parent !"
            );
        }
    }
}

fn validate_align_layout(lp: &LayoutParams, params: &mut NamedParamsMap) {
    should_not_use!(lp.x, "When ('align' or 'a') parameter is used,'x' parameter can not be used !");
    should_not_use!(lp.y, "When ('align' or 'a') parameter is used,'y' parameter can not be used !");
    should_not_use!(
        lp.top,
        "When ('align' or 'a') parameter is used,('top' or 't') parameters can not be used !"
    );
    should_not_use!(
        lp.bottom,
        "When ('align' or 'a') parameter is used,('bottom' or 'b') parameters can not be used !"
    );
    should_not_use!(
        lp.left,
        "When ('align' or 'a') parameter is used,('left' or 'l') parameters can not be used !"
    );
    should_not_use!(
        lp.right,
        "When ('align' or 'a') parameter is used,('right' or 'r') parameters can not be used !"
    );
    should_not_use!(
        lp.dock,
        "When ('align' or 'a') parameter is used,('dock' or 'd') parameters can not be used !"
    );
    should_not_use!(
        lp.pivot,
        "When ('align' or 'a') parameter is used,('pivot' or 'p') parameters can not be used !"
    );
    if lp.width && lp.height {
        let w = params.get_mut("width").map(|v| { v.get_percentage().unwrap_or(0.0) }).unwrap_or(0.0);
        let h = params.get_mut("height").map(|v| { v.get_percentage().unwrap_or(0.0) }).unwrap_or(0.0);
        if (w==100.0f32) && (h==100.0f32) {
            panic!("Using 'align' with a width and height of 100% of the parent is equivalent to using dock with the value 'fill'. Remove the `align`/`a`, `width`/`w` and `height`/`h` parameters and replace them with `dock:fill` ");
        }
    }

    // temporary
    // if !lp.width || !lp.height {
    //     panic!("Missig either width or height for align !");
    // }
}

fn validate_xy_layout(lp: &LayoutParams, _params: &NamedParamsMap) {
    should_not_use!(
        lp.left,
        "When both (`x` and `y`) parameters are used, the left anchor parameter ('left' or 'l') can not be used !"
    );
    should_not_use!(
        lp.right,
        "When both (`x` and `y`) parameters are used, the right anchor parameter ('right' or 'r') can not be used !"
    );
    should_not_use!(
        lp.top,
        "When both (`x` and `y`) parameters are used, the top anchor parameter ('top' or 't') can not be used !"
    );
    should_not_use!(
        lp.bottom,
        "When both (`x` and `y`) parameters are used, the bottom anchor parameter ('bottom' or 'b') can not be used !"
    );
}

fn validate_left_right_layout(lp: &LayoutParams, params: &NamedParamsMap) {
    should_not_use!(
        lp.x,
        "When (left,right) anchors are used together, 'X' parameter can not be used as it is infered from the anchors !"
    );
    should_not_use!(
        lp.width,
        "When (left,right) anchors are used together, ('width' or 'w') parameter can not be used as the width is deduced from left-right difference"
    );
    should_use!(lp.y, "When (left,right) anchors are used together, 'Y' parameter is required !");
    should_use!(lp.pivot, "When (left,right) anchors are used together, 'pivot' parameter is required !");    
}

fn validate_top_bottom_layout(lp: &LayoutParams, params: &NamedParamsMap) {
    should_not_use!(
        lp.y,
        "When (top,bottom) anchors are used together, 'Y' parameter can not be used as it is infered from the anchors !"
    );
    should_not_use!(lp.height, "When (top,bottom) anchors are used together, ('height' or 'h') parameter can not be used as the height is deduced from top-bottom difference");
    should_use!(lp.x, "When (top,bottom) anchors are used together, 'X' parameter is required !");
    should_use!(lp.pivot, "When (top,bottom) anchors are used together, 'pivot' parameter is required !");

    match params.get("pivot").unwrap().to_align() {
        Alignment::CenterLeft | Alignment::Center | Alignment::CenterRight => {}
        _ => panic!(
            "When (top,bottom) anchors are used together, only CenterLeft (cl or c), Center (c) and CenterRight(cr or c) pivot values are allowed !"
        ),
    }
}

fn validate_corner_anchor_layout(lp: &LayoutParams, _params: &NamedParamsMap) {
    should_not_use!(lp.x, "When a corner anchor is being used (top,left,righ,bottom), 'x' can bot be used !");
    should_not_use!(lp.y, "When a corner anchor is being used (top,left,righ,bottom), 'y' can bot be used !");
}

fn validate_left_top_right_layout(lp: &LayoutParams, _params: &NamedParamsMap) {
    should_not_use!(
        lp.x,
        "When (left,top,right) anchors are used together, 'x' can bot be used as it is infered from the anchors !"
    );
    should_not_use!(
        lp.y,
        "When (left,top,right) anchors are used together, 'y' can bot be used as it is infered from the anchors !"
    );
    should_not_use!(
        lp.width,
        "When (left,top,right) anchors are used together, 'width' can bot be used as it is infered from the anchors !"
    );
    should_not_use!(lp.pivot, "When (left,top,right) anchors are used together, 'pivot' can bot be used !");
}

fn validate_left_bottom_right_layout(lp: &LayoutParams, _params: &NamedParamsMap) {
    should_not_use!(
        lp.x,
        "When (left,bottom,right) anchors are used together, 'x' can bot be used as it is infered from the anchors !"
    );
    should_not_use!(
        lp.y,
        "When (left,bottom,right) anchors are used together, 'y' can bot be used as it is infered from the anchors !"
    );
    should_not_use!(
        lp.width,
        "When (left,bottom,right) anchors are used together, 'width' can bot be used as it is infered from the anchors !"
    );
    should_not_use!(lp.pivot, "When (left,bottom,right) anchors are used together, 'pivot' can bot be used !");
}

fn validate_top_left_bottom_layout(lp: &LayoutParams, _params: &NamedParamsMap) {
    should_not_use!(
        lp.x,
        "When (top,left,bottom) anchors are used together, 'x' can bot be used as it is infered from the anchors !"
    );
    should_not_use!(
        lp.y,
        "When (top,left,bottom) anchors are used together, 'y' can bot be used as it is infered from the anchors !"
    );
    should_not_use!(
        lp.height,
        "When (top,left,bottom) anchors are used together, 'height' can bot be used as it is infered from the anchors !"
    );
    should_not_use!(lp.pivot, "When (top,left,bottom) anchors are used together, 'pivot' can bot be used !");
}

fn validate_top_right_bottom_layout(lp: &LayoutParams, _params: &NamedParamsMap) {
    should_not_use!(
        lp.x,
        "When (top,right,bottom) anchors are used together, 'x' can bot be used as it is infered from the anchors !"
    );
    should_not_use!(
        lp.y,
        "When (top,right,bottom) anchors are used together, 'y' can bot be used as it is infered from the anchors !"
    );
    should_not_use!(
        lp.height,
        "When (top,right,bottom) anchors are used together, 'height' can bot be used as it is infered from the anchors !"
    );
    should_not_use!(lp.pivot, "When (top,right,bottom) anchors are used together, 'pivot' can bot be used !");
}

fn validate_all_anchors_layout(lp: &LayoutParams, _params: &NamedParamsMap) {
    should_not_use!(
        lp.x,
        "When (left,top,right,bottom) parameters are used together, 'x' parameter can not be used as it is infered from the anchors !"
    );
    should_not_use!(
        lp.y,
        "When (left,top,right,bottom) parameters are used together, 'y' parameter can not be used as it is infered from the anchors !"
    );
    should_not_use!(
        lp.width,
        "When (left,top,right,bottom) parameters are used together, 'width' parameter can not be used as it is infered from the anchors !"
    );
    should_not_use!(
        lp.height,
        "When (left,top,right,bottom) parameters are used together, 'height' parameter can not be used as it is infered from the anchors !"
    );
    should_not_use!(
        lp.pivot,
        "When (left,top,right,bottom) parameters are used together, 'pivot' parameter can not be used !"
    );
}

fn validate_layout(params: &mut NamedParamsMap) {
    let lp = LayoutParams {
        x: params.contains("x"),
        y: params.contains("y"),
        left: params.contains("left"),
        right: params.contains("right"),
        top: params.contains("top"),
        bottom: params.contains("bottom"),
        align: params.contains("align"),
        pivot: params.contains("pivot"),
        dock: params.contains("dock"),
        width: params.contains("width"),
        height: params.contains("height"),
    };
    // all are missing
    if !(lp.x || lp.y || lp.left || lp.top || lp.right || lp.bottom || lp.align || lp.pivot || lp.dock || lp.width || lp.height) {
        panic!("You need to provide one or some combination of the following parameters: 'x', 'y', 'width'/'w', 'height'/'h', 'left'/'l', 'top'/'t', 'right'/'r', 'bottom'/'b', 'align'/'a', 'dock'/'d' or 'pivot'/'p' !");
    }
    // same logic as the one from layout mode
    if lp.dock {
        validate_dock_layout(&lp, params);
        return;
    }
    // align
    if lp.align {
        validate_align_layout(&lp, params);
        return;
    }
    // x , y
    if lp.x && lp.y {
        validate_xy_layout(&lp, params);
        return;
    }

    let anchors = Anchors::new(lp.left, lp.top, lp.right, lp.bottom);
    match anchors {
        Anchors::LeftRight => validate_left_right_layout(&lp, params),
        Anchors::TopBottom => validate_top_bottom_layout(&lp, params),
        Anchors::TopLeft | Anchors::TopRight | Anchors::BottomLeft | Anchors::BottomRight => validate_corner_anchor_layout(&lp, params),
        Anchors::LeftTopRight => validate_left_top_right_layout(&lp, params),
        Anchors::LeftBottomRight => validate_left_bottom_right_layout(&lp, params),
        Anchors::TopLeftBottom => validate_top_left_bottom_layout(&lp, params),
        Anchors::TopRightBottom => validate_top_right_bottom_layout(&lp, params),
        Anchors::All => validate_all_anchors_layout(&lp, params),
        Anchors::Left => {
            panic!("Using only the 'left'/'l' anchor is no different than using a pivot. Consider using a pivot instead, combined with (x,y) and optionally and width and a height)");
        } 
        Anchors::Right => {
            panic!("Using only the 'right'/'r' anchor is no different than using a pivot. Consider using a pivot instead, combined with (x,y) and optionally and width and a height)");
        } 
        Anchors::Top => {
            panic!("Using only the 'top'/'t' anchor is no different than using a pivot. Consider using a pivot instead, combined with (x,y) and optionally and width and a height)");
        }
        Anchors::Bottom => {
            panic!("Using only the 'bottom'/'b' anchor is no different than using a pivot. Consider using a pivot instead, combined with (x,y) and optionally and width and a height)");
        }
        Anchors::None => {
            if lp.x && !lp.y {
                panic!("You need 'y' parameter as well if you want to define a pivot point or the top-left corner of a control !");
            }
            if !lp.x && lp.y {
                panic!("You need 'x' parameter as well if you want to define a pivot point or the top-left corner of a control !");
            }
            if lp.pivot && !lp.x && !lp.y {
                panic!("You can not use pivot standalone - you also need 'x' and 'y' parameters !");
            }
            panic!("Invalid layout format --> this combination can not be used to create a layout for a control ");
        }
    }
}
fn add_number(output: &mut String, method: &'static str, key: &'static str, params: &mut NamedParamsMap) {
    if let Some(v) = params.get_mut(key) {
        output.push_str(method);
        output.push('(');
        if let Some(value) = v.get_i32() {
            let _ = write!(output, "{value}");
        } else if let Some(proc) = v.get_percentage() {
            let _ = write!(output, "{}f32", proc / 100.0f32);
        } else {
            panic!("Invalid value for parameter `{key}` -> expecting either a number (e.g. {key}: 10) or a percentage (e.g. {key}: 7.5%) but got the following value: '{}')",v.get_string());
        }
        output.push(')');
    }
}
fn add_alignment(output: &mut String, method: &'static str, enum_name: &'static str, key: &'static str, params: &mut NamedParamsMap) {
    if let Some(v) = params.get_mut(key) {
        let _ = write!(output, "{method}({enum_name}::");
        if let Some(a) = v.get_alignment() {
            output.push_str(a.name());
        }
        output.push(')');
    }
}
fn add_dock(output: &mut String, params: &mut NamedParamsMap) {
    if let Some(v) = params.get_mut("dock") {
        output.push_str(".dock(Dock::");
        if let Some(d) = v.get_dock() {
            output.push_str(d.name());
        }
        output.push(')');
    }
}
pub(super) fn add_layout(output: &mut String, params: &mut NamedParamsMap) {
    // s.push_str("Layout::new(\"");
    // copy_layout_params(s, params);
    // s.push_str("\")");
    validate_layout(params);
    output.push_str("LayoutBuilder::new()");
    add_number(output, ".x", "x", params);
    add_number(output, ".y", "y", params);
    add_number(output, ".width", "width", params);
    add_number(output, ".height", "height", params);
    add_number(output, ".left_anchor", "left", params);
    add_number(output, ".right_anchor", "right", params);
    add_number(output, ".top_anchor", "top", params);
    add_number(output, ".bottom_anchor", "bottom", params);
    add_alignment(output, ".alignment", "Alignment", "align", params);
    add_alignment(output, ".pivot", "Pivot", "pivot", params);
    add_dock(output, params);
    output.push_str(".build()");
}

pub(crate) fn create(input: TokenStream) -> TokenStream {
    let s = input.validate_one_string_parameter("layout");
    let mut d = parameter_parser::parse(&s).unwrap();
    if let Err(e) = d.validate_named_parameters(&s, control_builder::CONTROL_NAMED_PARAMATERS) {
        e.panic();
    }
    let mut res = String::with_capacity(128);
    add_layout(&mut res, &mut d);
    TokenStream::from_str(&res).expect("Fail to convert 'layout!' macro content to token stream")
}
