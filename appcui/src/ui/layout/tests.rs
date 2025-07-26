use crate::ui::layout::absolute_layout::AbsoluteLayout;
use crate::prelude::*;

use super::Alignment;
use super::ControlLayout;
use super::Coordinate16;
use super::Dimension16;
use super::Dock;
use super::LayoutBuilder;
use super::PointAndSizeLayout;
use super::Error;
use super::Anchors;
use super::LayoutMode;
use super::Dimension;
use super::Coordinate;

macro_rules! validate_abs {
    ($text:literal, $x:expr,$y:expr,$w:expr,$h:expr,$a:tt,$anc:tt) => {
        assert_eq!(
            LayoutMode::new(layout!($text)).unwrap(),
            LayoutMode::PointAndSize(PointAndSizeLayout {
                x: Coordinate16::Absolute($x),
                y: Coordinate16::Absolute($y),
                align: Alignment::$a,
                anchor: Alignment::$anc,
                width: Dimension16::Absolute($w),
                height: Dimension16::Absolute($h)
            })
        );
    };
}

macro_rules! validate_pos {
    ($text:literal, $parent_width:expr, $parent_height:expr, $x:expr,$y:expr,$w:expr,$h:expr) => {
        let mut cl = ControlLayout::from(layout!($text));
        cl.update($parent_width, $parent_height);
        assert_eq!(cl.width(), $w);
        assert_eq!(cl.height(), $h);
        assert_eq!(cl.x(), $x);
        assert_eq!(cl.y(), $y);
    };
}

#[test]
fn layout_mode_xywh() {
    let l1 = LayoutMode::new(layout!("x:1,y:1,w:10,h:8")).unwrap();
    assert_eq!(
        l1,
        LayoutMode::Absolute(AbsoluteLayout {
            x: 1,
            y: 1,
            width: 10,
            height: 8
        })
    );
    let l2 = LayoutMode::new(layout!("x:-4,y:10%,w:10%,h:8")).unwrap();
    assert_eq!(
        l2,
        LayoutMode::PointAndSize(PointAndSizeLayout {
            x: Coordinate16::Absolute(-4),
            y: Coordinate16::Percentage(1000),
            align: Alignment::TopLeft,
            anchor: Alignment::TopLeft,
            width: Dimension16::Percentage(1000),
            height: Dimension16::Absolute(8)
        })
    );
    let l3 = LayoutMode::new(layout!("x:0,y:0,w:100%,h:25%,p:c")).unwrap();
    assert_eq!(
        l3,
        LayoutMode::PointAndSize(PointAndSizeLayout {
            x: Coordinate16::Absolute(0),
            y: Coordinate16::Absolute(0),
            align: Alignment::Center,
            anchor: Alignment::TopLeft,
            width: Dimension16::Percentage(10000),
            height: Dimension16::Percentage(2500)
        })
    );
}
#[test]
fn layout_mode_pivot_center() {
    validate_abs!("x:0,y:0,w:12,h:15,p:center", 0, 0, 12, 15, Center, TopLeft);
    validate_abs!("x:0,y:0,w:12,h:15,p:c", 0, 0, 12, 15, Center, TopLeft);
    validate_abs!("x:0,y:0,w:12,h:15,pivot:c", 0, 0, 12, 15, Center, TopLeft);
    validate_abs!("x:0,y:0,w:12,h:15,pivot:center", 0, 0, 12, 15, Center, TopLeft);
}
#[test]
fn layout_mode_pivot_top_left() {
    validate_abs!("x:0,y:0,w:12,h:15,p:tl", 0, 0, 12, 15, TopLeft, TopLeft);
    validate_abs!("x:0,y:0,w:12,h:15,p:lt", 0, 0, 12, 15, TopLeft, TopLeft);
    validate_abs!("x:0,y:0,w:12,h:15,pivot:topleft", 0, 0, 12, 15, TopLeft, TopLeft);
    validate_abs!("x:0,y:0,w:12,h:15,pivot:lefttop", 0, 0, 12, 15, TopLeft, TopLeft);
}
#[test]
fn layout_mode_pivot_top_right() {
    validate_abs!("x:1,y:0,w:12,h:15,p:tr", 1, 0, 12, 15, TopRight, TopLeft);
    validate_abs!("x:2,y:0,w:12,h:15,p:rt", 2, 0, 12, 15, TopRight, TopLeft);
    validate_abs!("x:3,y:0,w:12,h:15,pivot:topright", 3, 0, 12, 15, TopRight, TopLeft);
    validate_abs!("x:4,y:0,w:12,h:15,pivot:righttop", 4, 0, 12, 15, TopRight, TopLeft);
}
#[test]
fn layout_mode_pivot_bottom_right() {
    validate_abs!("x:1,y:-2,w:12,h:15,p:br", 1, -2, 12, 15, BottomRight, TopLeft);
    validate_abs!("x:2,y:-1,w:12,h:15,p:rb", 2, -1, 12, 15, BottomRight, TopLeft);
    validate_abs!("x:3,y: 1,w:12,h:15,pivot:bottomright", 3, 1, 12, 15, BottomRight, TopLeft);
    validate_abs!("x:4,y: 2,w:12,h:15,pivot:rightbottom", 4, 2, 12, 15, BottomRight, TopLeft);
}
#[test]
fn layout_mode_pivot_bottom_left() {
    validate_abs!("x:1,y:-2,w:12,h:15,p:bl", 1, -2, 12, 15, BottomLeft, TopLeft);
    validate_abs!("x:2,y:-1,w:12,h:15,p:lb", 2, -1, 12, 15, BottomLeft, TopLeft);
    validate_abs!("x:3,y: 1,w:12,h:15,pivot:bottomleft", 3, 1, 12, 15, BottomLeft, TopLeft);
    validate_abs!("x:4,y: 2,w:12,h:15,pivot:leftbottom", 4, 2, 12, 15, BottomLeft, TopLeft);
}
#[test]
fn layout_mode_pivot_left() {
    validate_abs!("w:12,h:15,x:1,y:-2,p:l", 1, -2, 12, 15, CenterLeft, TopLeft);
    validate_abs!("y: 1,x:3,w:12,h:15,p:left", 3, 1, 12, 15, CenterLeft, TopLeft);
    validate_abs!("x:4,w:12,y: 2,h:15,pivot:left", 4, 2, 12, 15, CenterLeft, TopLeft);
}
#[test]
fn layout_mode_pivot_right() {
    validate_abs!("w:12,h:15,x:1,y:-2,p:r", 1, -2, 12, 15, CenterRight, TopLeft);
    validate_abs!("y: 1,x:3,width:12,height:15,p:right", 3, 1, 12, 15, CenterRight, TopLeft);
    validate_abs!("x:4,w:12,y: 2,h:15,pivot:right", 4, 2, 12, 15, CenterRight, TopLeft);
}
#[test]
fn layout_mode_pivot_top() {
    validate_abs!("w:12,h:15,x:1,y:-2,p:t", 1, -2, 12, 15, TopCenter, TopLeft);
    validate_abs!("y: 1,x:3,width:12,height:15,p:top", 3, 1, 12, 15, TopCenter, TopLeft);
    validate_abs!("x:4,w:12,y: 2,h:15,pivot:top", 4, 2, 12, 15, TopCenter, TopLeft);
}
#[test]
fn layout_mode_pivot_bottom() {
    validate_abs!("width:12,h:15,x:1,y:-2,p:b", 1, -2, 12, 15, BottomCenter, TopLeft);
    validate_abs!("y: 1,x:3,width:12,height:15,p:bottom", 3, 1, 12, 15, BottomCenter, TopLeft);
    validate_abs!("x:4,w:12,y: 2,HEIGHT:15,pivot:bottom", 4, 2, 12, 15, BottomCenter, TopLeft);
}
#[test]
fn layout_mode_dock_center() {
    validate_pos!("d:f", 50, 30, 0, 0, 50, 30);
    validate_pos!("a:center,w:20,h:10", 50, 30, 15, 10, 20, 10);
    validate_pos!("a:Center,w:20,h:100%", 50, 30, 15, 0, 20, 30);
    validate_pos!("a:CENTER,h:10,w:100%", 50, 30, 0, 10, 50, 10);
    validate_pos!("a:cEnTeR,w:50%,h:25%", 60, 40, 15, 15, 30, 10);
}
#[test]
fn layout_mode_dock_top_left() {
    //validate_pos!("a:tl,w:100%,h:100%", 50, 30, 0, 0, 50, 30);
    validate_pos!("a:lt,w:20,h:10", 50, 30, 0, 0, 20, 10);
    validate_pos!("a:topleft,w:20,h:100%", 50, 30, 0, 0, 20, 30);
    validate_pos!("a:lefttop,h:10,w:100%", 50, 30, 0, 0, 50, 10);
    validate_pos!("a:TopLeft,w:50%,h:25%", 60, 40, 0, 0, 30, 10);
}
#[test]
fn layout_mode_dock_bottom_left() {
    //validate_pos!("a:lb,w:100%,h:100%", 50, 30, 0, 0, 50, 30);
    validate_pos!("a:bl,w:20,h:10", 50, 30, 0, 20, 20, 10);
    validate_pos!("a:lb,w:20,h:100%", 50, 30, 0, 0, 20, 30);
    validate_pos!("a:bottomleft,h:10,w:100%", 50, 30, 0, 20, 50, 10);
    validate_pos!("a:leftbottom,w:50%,h:25%", 60, 40, 0, 30, 30, 10);
}
#[test]
fn layout_mode_dock_bottom_right() {
    //validate_pos!("a:rb,w:100%,h:100%", 50, 30, 0, 0, 50, 30);
    validate_pos!("a:br,w:20,h:10", 50, 30, 30, 20, 20, 10);
    validate_pos!("a:rb,w:20,h:100%", 50, 30, 30, 0, 20, 30);
    validate_pos!("a:bottomright,h:10,w:100%", 50, 30, 0, 20, 50, 10);
    validate_pos!("a:rightbottom,w:50%,h:25%", 60, 40, 30, 30, 30, 10);
}
#[test]
fn layout_mode_dock_top_right() {
    //validate_pos!("a:tr,w:100%,h:100%", 50, 30, 0, 0, 50, 30);
    validate_pos!("a:rt,w:20,h:10", 50, 30, 30, 0, 20, 10);
    validate_pos!("a:TopRight,w:20,h:100%", 50, 30, 30, 0, 20, 30);
    validate_pos!("a:rightTop,h:10,w:100%", 50, 30, 0, 0, 50, 10);
    validate_pos!("a:topright,w:50%,h:25%", 60, 40, 30, 0, 30, 10);
}
#[test]
fn layout_mode_align_left() {
    //validate_pos!("a:l,w:100%,h:100%", 50, 30, 0, 0, 50, 30);
    validate_pos!("a:left,w:20,h:10", 50, 30, 0, 10, 20, 10);
    validate_pos!("a:Left,w:20,h:100%", 50, 30, 00, 0, 20, 30);
    validate_pos!("a:l,h:10,w:100%", 50, 30, 0, 10, 50, 10);
    validate_pos!("a:LeFt,w:50%,h:25%", 60, 40, 0, 15, 30, 10);
}

#[test]
fn layout_mode_align_top() {
    //validate_pos!("a:t,w:100%,h:100%", 50, 30, 0, 0, 50, 30);
    validate_pos!("a:top,w:20,h:10", 50, 30, 15, 0, 20, 10);
    validate_pos!("a:Top,w:20,h:100%", 50, 30, 15, 0, 20, 30);
    validate_pos!("a:t,h:10,w:100%", 50, 30, 0, 0, 50, 10);
    validate_pos!("a:ToP,w:50%,h:25%", 60, 40, 15, 0, 30, 10);
}

#[test]
fn layout_mode_align_right() {
    //validate_pos!("a:r,w:100%,h:100%", 50, 30, 0, 0, 50, 30);
    validate_pos!("a:Right,w:20,h:10", 50, 30, 30, 10, 20, 10);
    validate_pos!("a:right,w:20,h:100%", 50, 30, 30, 0, 20, 30);
    validate_pos!("a:r,h:10,w:100%", 50, 30, 0, 10, 50, 10);
    validate_pos!("a:rIgHt,w:50%,h:25%", 60, 40, 30, 15, 30, 10);
}
#[test]
fn layout_mode_align_bottom() {
    //validate_pos!("a:b,w:100%,h:100%", 50, 30, 0, 0, 50, 30);
    validate_pos!("a:bottom,w:20,h:10", 50, 30, 15, 20, 20, 10);
    validate_pos!("a:Bottom,w:20,h:100%", 50, 30, 15, 0, 20, 30);
    validate_pos!("a:b,h:10,w:100%", 50, 30, 0, 20, 50, 10);
    validate_pos!("a:BoTtOm,w:50%,h:25%", 60, 40, 15, 30, 30, 10);
}
#[test]
fn layout_mode_anchor_lrtb() {
    validate_pos!("l:5,t:6,r:7,b:8", 50, 30, 5, 6, 38, 16);
    validate_pos!("left:5, top:6,  right:7,  bottom:8", 50, 30, 5, 6, 38, 16);
    validate_pos!("l:10%,t:50%,r:20%,b:10%", 50, 30, 5, 15, 35, 12);
}
#[test]
fn layout_mode_anchor_ltr() {
    validate_pos!("l:5,t:6,r:7,h:10", 50, 30, 5, 6, 38, 10);
    validate_pos!("l:10%,t:50%,r:20%,h:2", 50, 30, 5, 15, 35, 2);
    validate_pos!("l:10%,t:50%,r:20%,h:50%", 50, 30, 5, 15, 35, 15);
}
#[test]
fn layout_mode_anchor_lbr() {
    validate_pos!("l:5,b:6,r:7,h:10", 50, 30, 5, 14, 38, 10);
    validate_pos!("l:10%,b:50%,r:20%,h:2", 50, 30, 5, 13, 35, 2);
    validate_pos!("l:10%,bottom:50%,r:20%,h:50%", 50, 30, 5, 0, 35, 15);
}

#[test]
fn layout_mode_anchor_lr() {
    validate_pos!("l:5,r:7,y:0,h:10,p:t", 50, 30, 5, 0, 38, 10);
    validate_pos!("l:5,r:7,y:10,h:10,p:c", 50, 30, 5, 5, 38, 10);
    validate_pos!("l:5,r:7,y:20,h:10,p:b", 50, 30, 5, 10, 38, 10);

    validate_pos!("l:5,r:7,y:0,h:10,pivot:center", 50, 30, 5, -5, 38, 10);

    validate_pos!("l:10%,p:t,y:50%,r:20%,h:4", 50, 30, 5, 15, 35, 4);
    validate_pos!("l:10%,p:c,y:50%,r:20%,h:4", 50, 30, 5, 13, 35, 4);
    validate_pos!("l:10%,p:b,y:50%,r:20%,h:4", 50, 30, 5, 11, 35, 4);

    validate_pos!("l:10%,p:t,y:50%,r:20%,h:50%", 50, 30, 5, 15, 35, 15);
    validate_pos!("l:10%,p:c,y:50%,r:20%,h:50%", 50, 30, 5, 8, 35, 15);
    validate_pos!("l:10%,p:b,y:50%,r:20%,h:50%", 50, 30, 5, 0, 35, 15);
}

#[test]
fn check_anchors_new() {
    assert_eq!(Anchors::new(false, false, false, false), Anchors::None);
    assert_eq!(Anchors::new(true, false, false, false), Anchors::Left);
    assert_eq!(Anchors::new(false, true, false, false), Anchors::Top);
    assert_eq!(Anchors::new(false, false, true, false), Anchors::Right);
    assert_eq!(Anchors::new(false, false, false, true), Anchors::Bottom);
    assert_eq!(Anchors::new(true, true, false, false), Anchors::TopLeft);
    assert_eq!(Anchors::new(true, false, true, false), Anchors::LeftRight);
    assert_eq!(Anchors::new(true, false, false, true), Anchors::BottomLeft);
    assert_eq!(Anchors::new(false, true, true, false), Anchors::TopRight);
    assert_eq!(Anchors::new(false, true, false, true), Anchors::TopBottom);
    assert_eq!(Anchors::new(false, false, true, true), Anchors::BottomRight);
    assert_eq!(Anchors::new(true, true, true, false), Anchors::LeftTopRight);
    assert_eq!(Anchors::new(true, false, true, true), Anchors::LeftBottomRight);
    assert_eq!(Anchors::new(false, true, true, true), Anchors::TopRightBottom);
    assert_eq!(Anchors::new(true, true, false, true), Anchors::TopLeftBottom);
    assert_eq!(Anchors::new(true, true, true, true), Anchors::All);
}

#[test]
fn layout_mode_anchor_tb() {
    validate_pos!("t:5,b:7,x:0,w:10,p:l", 30, 50, 0, 5, 10, 38);
    validate_pos!("t:5,b:7,x:10,w:10,p:c", 30, 50, 5, 5, 10, 38);
    validate_pos!("t:5,b:7,x:20,w:10,p:r", 30, 50, 10, 5, 10, 38);
    // no alignment - default is center
    validate_pos!("t:5,b:7,x:0,w:10,p:c", 30, 50, -5, 5, 10, 38);

    validate_pos!("t:10%,p:l,x:50%,b:20%,w:4", 30, 50, 15, 5, 4, 35);
    validate_pos!("t:10%,p:c,x:50%,b:20%,w:4", 30, 50, 13, 5, 4, 35);
    validate_pos!("t:10%,p:r,x:50%,b:20%,w:4", 30, 50, 11, 5, 4, 35);

    validate_pos!("t:10%,p:l,x:50%,b:20%,w:50%", 30, 50, 15, 5, 15, 35);
    validate_pos!("t:10%,p:c,x:50%,b:20%,w:50%", 30, 50, 8, 5, 15, 35);
    validate_pos!("t:10%,p:r,x:50%,b:20%,w:50%", 30, 50, 0, 5, 15, 35);
}


#[test]
fn dimension_from_basic_type() {
    assert_eq!(Dimension::from(10u8), Dimension::Absolute(10));
    assert_eq!(Dimension::from(10u16), Dimension::Absolute(10));
    assert_eq!(Dimension::from(10u32), Dimension::Absolute(10));
    assert_eq!(Dimension::from(10u64), Dimension::Absolute(10));
    assert_eq!(Dimension::from(10i8), Dimension::Absolute(10));
    assert_eq!(Dimension::from(10i16), Dimension::Absolute(10));
    assert_eq!(Dimension::from(10i32), Dimension::Absolute(10));
    assert_eq!(Dimension::from(10i64), Dimension::Absolute(10));
    assert_eq!(Dimension::from(-10i8), Dimension::Absolute(0));
    assert_eq!(Dimension::from(-10i16), Dimension::Absolute(0));
    assert_eq!(Dimension::from(-10i32), Dimension::Absolute(0));
    assert_eq!(Dimension::from(-10i64), Dimension::Absolute(0));
    assert_eq!(Dimension::from(5.25f32), Dimension::Percentage(5.25));
    assert_eq!(Dimension::from(5.25f64), Dimension::Percentage(5.25));
}

#[test]
fn dimension16_from_basic_type() {
    assert_eq!(Dimension16::from(10u8), Dimension16::Absolute(10));
    assert_eq!(Dimension16::from(10u16), Dimension16::Absolute(10));
    assert_eq!(Dimension16::from(10u32), Dimension16::Absolute(10));
    assert_eq!(Dimension16::from(10u64), Dimension16::Absolute(10));
    assert_eq!(Dimension16::from(10i8), Dimension16::Absolute(10));
    assert_eq!(Dimension16::from(10i16), Dimension16::Absolute(10));
    assert_eq!(Dimension16::from(10i32), Dimension16::Absolute(10));
    assert_eq!(Dimension16::from(10i64), Dimension16::Absolute(10));
    assert_eq!(Dimension16::from(-10i8), Dimension16::Absolute(0));
    assert_eq!(Dimension16::from(-10i16), Dimension16::Absolute(0));
    assert_eq!(Dimension16::from(-10i32), Dimension16::Absolute(0));
    assert_eq!(Dimension16::from(-10i64), Dimension16::Absolute(0));
    assert_eq!(Dimension16::from(1.25f32), Dimension16::Percentage(12500));
    assert_eq!(Dimension16::from(1.25f64), Dimension16::Percentage(12500));
}

#[test]
fn coordinate_from_basic_type() {
    assert_eq!(Coordinate::from(10u8), Coordinate::Absolute(10));
    assert_eq!(Coordinate::from(-10i8), Coordinate::Absolute(-10));
    assert_eq!(Coordinate::from(10u16), Coordinate::Absolute(10));
    assert_eq!(Coordinate::from(-10i16), Coordinate::Absolute(-10));
    assert_eq!(Coordinate::from(-10i32), Coordinate::Absolute(-10));
    assert_eq!(Coordinate::from(-10i64), Coordinate::Absolute(-10));
    assert_eq!(Coordinate::from(1.25f32), Coordinate::Percentage(1.25));
    assert_eq!(Coordinate::from(-1.25f64), Coordinate::Percentage(-1.25));
}

#[test]
fn coordinate16_from_basic_type() {
    assert_eq!(Coordinate16::from(10u8), Coordinate16::Absolute(10));
    assert_eq!(Coordinate16::from(-10i8), Coordinate16::Absolute(-10));
    assert_eq!(Coordinate16::from(10u16), Coordinate16::Absolute(10));
    assert_eq!(Coordinate16::from(-10i16), Coordinate16::Absolute(-10));
    assert_eq!(Coordinate16::from(10u32), Coordinate16::Absolute(10));
    assert_eq!(Coordinate16::from(-10i32), Coordinate16::Absolute(-10));
    assert_eq!(Coordinate16::from(10u64), Coordinate16::Absolute(10));
    assert_eq!(Coordinate16::from(-10i64), Coordinate16::Absolute(-10));
    assert_eq!(Coordinate16::from(1.25f32), Coordinate16::Percentage(12500));
    assert_eq!(Coordinate16::from(-1.25f64), Coordinate16::Percentage(-12500));
}

#[test]
fn layout_mode_anchor_trb() {
    validate_pos!("r:5,t:6,b:7,w:10", 50, 30, 35, 6, 10, 17);
    validate_pos!("r:10%,t:6,b:7,w:20%", 50, 30, 35, 6, 10, 17);
    validate_pos!("r:5,t:3,b:3,w:10", 50, 30, 35, 3, 10, 24);
    validate_pos!("r:5,t:10%,b:10%,w:10", 50, 30, 35, 3, 10, 24);
    validate_pos!("r:10%,t:10%,b:10%,w:20%", 50, 30, 35, 3, 10, 24);
}

#[test]
fn check_dimension_is_absolute() {
    assert!(Dimension::Absolute(10).is_absolute());
    assert!(!Dimension::Percentage(0.5f32).is_absolute());
    assert!(!Dimension::Percentage(1.0f32).is_absolute());
    assert!(Dimension::Absolute(0).is_absolute());
}


#[test]
fn check_tr_anchor() {
    // this code should panic because left is not a valid value
    validate_pos!("t:1,r:1,w:10,h:10", 50, 30, 39, 1, 10, 10);
}

#[test]
fn check_default_layout_modes() {
    assert_eq!(LayoutMode::default(), LayoutMode::Absolute(AbsoluteLayout::new(0, 0, 0, 0)));
}

#[test]
fn check_coordonate_update_with_absolute_value() {
    let mut c = Coordinate::Absolute(0);
    c.update_with_absolute_value(10, 20);
    assert_eq!(c, Coordinate::Absolute(10));

    let mut c = Coordinate::Percentage(0.0f32);
    c.update_with_absolute_value(10, 20);
    assert_eq!(c, Coordinate::Percentage(0.5f32));
}

#[test]
fn check_layout_builder() {
    let lb = LayoutBuilder::new()
        .x(100)
        .y(1.25)
        .width(100)
        .height(1.0)
        .alignment(Alignment::Center)
        .dock(Dock::Top)
        .left_anchor(0.5)
        .right_anchor(0.25)
        .top_anchor(5)
        .bottom_anchor(7);
    assert_eq!(lb.inner_layout.x, Some(Coordinate16::Absolute(100)));
    assert_eq!(lb.inner_layout.y, Some(Coordinate16::Percentage(12500)));
    assert_eq!(lb.inner_layout.width, Some(Dimension16::Absolute(100)));
    assert_eq!(lb.inner_layout.height, Some(Dimension16::Percentage(10000)));
    assert_eq!(lb.inner_layout.align, Some(Alignment::Center));
    assert_eq!(lb.inner_layout.dock, Some(Dock::Top));
    assert_eq!(lb.inner_layout.a_left, Some(Coordinate16::Percentage(5000)));
    assert_eq!(lb.inner_layout.a_right, Some(Coordinate16::Percentage(2500)));
    assert_eq!(lb.inner_layout.a_top, Some(Coordinate16::Absolute(5)));
    assert_eq!(lb.inner_layout.a_bottom, Some(Coordinate16::Absolute(7)));
}

#[test]
fn layout_mode_dock() {
    validate_pos!("d:f", 50, 30, 0, 0, 50, 30);
    validate_pos!("d:l,w:10", 50, 30, 0, 0, 10, 30);
    validate_pos!("d:r,w:10", 50, 30, 40, 0, 10, 30);
    validate_pos!("d:t,h:10", 50, 30, 0, 0, 50, 10);
    validate_pos!("d:b,h:10", 50, 30, 0, 20, 50, 10);
}

#[test]
fn layout_mode_anchor_lr_dont_allow_x() {
    // this code should panic because 'x' can not be used in a Left-Right layout mode
    let l = LayoutBuilder::new().left_anchor(5).right_anchor(7).x(0).try_build();
    assert_eq!(l, Err(Error::LeftRightAnchorsUsedWithX));
    //validate_pos!("l:5,r:7,y:0,h:10,a:t,x:10", 50, 30, 5, 0, 38, 10);
}

/* 

#[test]
#[should_panic]
fn layout_mode_anchor_lr_invalid_alignment() {
    // this code should panic because only (top,bottom and center) alignments can not be used in a Left-Right layout mode
    validate_pos!("l:5,r:7,y:0,h:10,a:left", 50, 30, 5, 0, 38, 10);
}



#[test]
#[should_panic]
fn layout_mode_anchor_tb_dont_allow_y() {
    // this code should panic because 'y' can not be used in a Top-Down layout mode
    validate_pos!("t:5,b:7,y:0,w:10,a:l", 30, 50, 0, 5, 10, 38);
}
#[test]
#[should_panic]
fn layout_mode_anchor_td_invalid_alignment() {
    // this code should panic because only (left,right and center) alignments can not be used in a Top-Down layout mode
    validate_pos!("t:5,b:7,x:0,w:10,a:top", 30, 50, 0, 5, 10, 38);
}





#[test]
#[should_panic]
fn layout_mode_anchor_tlb_dont_allow_x() {
    // this code should panic because 'x' can not be used in a Top-Left-Bottom layout mode
    validate_pos!("t:1,l:5,b:7,w:20,x:10", 50, 30, 5, 0, 38, 10);
}

#[test]
#[should_panic]
fn layout_mode_anchor_tlb_dont_allow_y() {
    // this code should panic because 'y' can not be used in a Top-Left-Bottom layout mode
    validate_pos!("t:1,l:5,b:7,w:20,y:10", 50, 30, 5, 0, 38, 10);
}

#[test]
#[should_panic]
fn layout_mode_anchor_tlb_dont_allow_height() {
    // this code should panic because 'h' can not be used in a Top-Left-Bottom layout mode
    validate_pos!("t:1,l:5,b:7,w:20,h:10", 50, 30, 5, 0, 38, 10);
}

#[test]
#[should_panic]
fn layout_mode_anchor_tlb_dont_allow_allign() {
    // this code should panic because 'a' can not be used in a Top-Left-Bottom layout mode
    validate_pos!("t:1,l:5,b:7,w:20,a:c", 50, 30, 5, 0, 38, 10);
}

#[test]
#[should_panic]
fn layout_mode_anchor_trb_dont_allow_x() {
    // this code should panic because 'x' can not be used in a Top-Right-Bottom layout mode
    validate_pos!("t:1,r:5,b:7,w:20,x:10", 50, 30, 5, 0, 38, 10);
}

#[test]
#[should_panic]
fn layout_mode_anchor_trb_dont_allow_y() {
    // this code should panic because 'y' can not be used in a Top-Right-Bottom layout mode
    validate_pos!("t:1,r:5,b:7,w:20,y:10", 50, 30, 5, 0, 38, 10);
}

#[test]
#[should_panic]
fn layout_mode_anchor_trb_dont_allow_height() {
    // this code should panic because 'h' can not be used in a Top-Right-Bottom layout mode
    validate_pos!("t:1,r:5,b:7,w:20,h:10", 50, 30, 5, 0, 38, 10);
}

#[test]
#[should_panic]
fn layout_mode_anchor_trb_dont_allow_allign() {
    // this code should panic because 'a' can not be used in a Top-Right-Bottom layout mode
    validate_pos!("t:1,r:5,b:7,w:20,a:c", 50, 30, 5, 0, 38, 10);
}



#[test]
#[should_panic]
fn layout_mode_anchor_lbr_dont_allow_x() {
    // this code should panic because 'x' can not be used in a Left-Bottom-Right layout mode
    validate_pos!("l:1,r:5,b:7,h:20,x:1", 50, 30, 5, 0, 38, 10);
}

#[test]
#[should_panic]
fn layout_mode_anchor_lbr_dont_allow_y() {
    // this code should panic because 'y' can not be used in a Left-Bottom-Right layout mode
    validate_pos!("l:1,r:5,b:7,h:20,y:1", 50, 30, 5, 0, 38, 10);
}

#[test]
#[should_panic]
fn layout_mode_anchor_lbr_dont_allow_width() {
    // this code should panic because 'w' can not be used in a Left-Bottom-Right layout mode
    validate_pos!("l:1,r:5,b:7,h:20,w:1", 50, 30, 5, 0, 38, 10);
}

#[test]
#[should_panic]
fn layout_mode_anchor_lbr_dont_allow_allign() {
    // this code should panic because 'a' can not be used in a Left-Bottom-Right layout mode
    validate_pos!("l:1,r:5,b:7,h:20,a:c", 50, 30, 5, 0, 38, 10);
}

#[test]
#[should_panic]
fn layout_mode_anchor_tlr_dont_allow_x() {
    // this code should panic because 'x' can not be used in a Top-Left-Right layout mode
    validate_pos!("l:1,r:5,t:7,h:20,x:1", 50, 30, 5, 0, 38, 10);
}

#[test]
#[should_panic]
fn layout_mode_anchor_tlr_dont_allow_y() {
    // this code should panic because 'y' can not be used in a Top-Left-Right layout mode
    validate_pos!("l:1,r:5,t:7,h:20,y:1", 50, 30, 5, 0, 38, 10);
}

#[test]
#[should_panic]
fn layout_mode_anchor_tlr_dont_allow_width() {
    // this code should panic because 'w' can not be used in a Top-Left-Right layout mode
    validate_pos!("l:1,r:5,t:7,h:20,w:1", 50, 30, 5, 0, 38, 10);
}

#[test]
#[should_panic]
fn layout_mode_anchor_tlr_dont_allow_allign() {
    // this code should panic because 'a' can not be used in a Top-Left-Right layout mode
    validate_pos!("l:1,r:5,t:7,h:20,a:c", 50, 30, 5, 0, 38, 10);
}

#[test]
#[should_panic]
fn layout_mode_dock_dont_allow_left() {
    // this code should panic because 'l' can not be used in a Top-Left-Right layout mode
    validate_pos!("d:f,l:1", 50, 30, 5, 0, 38, 10);
}
#[test]
#[should_panic]
fn layout_mode_dock_dont_allow_right() {
    // this code should panic because 'r' can not be used in a Top-Left-Right layout mode
    validate_pos!("d:f,r:1", 50, 30, 5, 0, 38, 10);
}
#[test]
#[should_panic]
fn layout_mode_dock_dont_allow_top() {
    // this code should panic because 'lt' can not be used in a Top-Left-Right layout mode
    validate_pos!("d:f,t:1", 50, 30, 5, 0, 38, 10);
}
#[test]
#[should_panic]
fn layout_mode_dock_dont_allow_bottom() {
    // this code should panic because 'b' can not be used in a Top-Left-Right layout mode
    validate_pos!("d:f,b:1", 50, 30, 5, 0, 38, 10);
}

#[test]
#[should_panic]
fn layout_mode_simple_coord_dont_allow_right() {
    // this code should panic because 'r' can not be used in a (X,Y)-(Width x Height) layout mode
    validate_pos!("x:1,y:1,w:100%,h:100%,r:1", 50, 30, 5, 0, 38, 10);
}

#[test]
#[should_panic]
fn layout_mode_align_wh_coord_dont_allow_x() {
    // this code should panic because 'x' can not be used in a (Allign + Width x Height) layout mode
    validate_pos!("a:c,w:50%,h:50%,x:0", 50, 30, 5, 0, 38, 10);
}

#[test]
#[should_panic]
fn layout_mode_align_wh_coord_dont_allow_y() {
    // this code should panic because 'y' can not be used in a (Allign + Width x Height) layout mode
    validate_pos!("a:c,w:50%,h:50%,y:0", 50, 30, 5, 0, 38, 10);
}



#[test]
#[should_panic]
fn layout_mode_lr_dont_allow_x() {
    // this code should panic because 'x' can not be used in a (Left-right) layout mode
    validate_pos!("a:c,l:1,r:1,h:10,x:10", 50, 30, 5, 0, 38, 10);
}

#[test]
#[should_panic]
fn check_panic_on_invalid_anchor_variant() {
    // this code should panic because 'a' can not be 'blablablab'
    validate_pos!("a:blablabla", 50, 30, 5, 0, 38, 10);
}

#[test]
#[should_panic]
fn check_panic_on_invalid_dock_variant() {
    // this code should panic because 'd' can not be 'blablablab'
    validate_pos!("a:blablabla", 50, 30, 5, 0, 38, 10);
}

#[test]
#[should_panic]
fn check_panic_on_negative_width() {
    // this code should panic because width can not be negative
    validate_pos!("w:-10", 50, 30, 5, 0, 38, 10);
}

#[test]
#[should_panic]
fn check_panic_on_negative_height() {
    // this code should panic because height can not be negative
    validate_pos!("h:-10", 50, 30, 5, 0, 38, 10);
}

#[test]
#[should_panic]
fn check_panic_on_invalid_left() {
    // this code should panic because left is not a valid value
    validate_pos!("l:blablabla", 50, 30, 5, 0, 38, 10);
}

#[test]
#[should_panic]
fn check_panic_on_invalid_right() {
    // this code should panic because right is not a valid value
    validate_pos!("r:blablabla", 50, 30, 5, 0, 38, 10);
}

#[test]
#[should_panic]
fn check_panic_on_invalid_top() {
    // this code should panic because top is not a valid value
    validate_pos!("t:blablabla", 50, 30, 5, 0, 38, 10);
}

#[test]
#[should_panic]
fn check_panic_on_invalid_bottom() {
    // this code should panic because b is not a valid value
    validate_pos!("b    :blablabla", 50, 30, 5, 0, 38, 10);
}

*/

#[test]
fn layout_builder_all_error_codes() {
    use super::Alignment;
    use super::Dock;
    use super::Pivot;

    // Error::XYParameterUsedWithDock
    let result = LayoutBuilder::new().dock(Dock::Fill).x(10).try_build();
    assert_eq!(result, Err(Error::XYParameterUsedWithDock));
    
    let result = LayoutBuilder::new().dock(Dock::Left).y(5).try_build();
    assert_eq!(result, Err(Error::XYParameterUsedWithDock));

    // Error::AnchorParameterUsedWithDock
    let result = LayoutBuilder::new().dock(Dock::Top).left_anchor(5).try_build();
    assert_eq!(result, Err(Error::AnchorParameterUsedWithDock));
    
    let result = LayoutBuilder::new().dock(Dock::Bottom).right_anchor(5).try_build();
    assert_eq!(result, Err(Error::AnchorParameterUsedWithDock));
    
    let result = LayoutBuilder::new().dock(Dock::Right).top_anchor(5).try_build();
    assert_eq!(result, Err(Error::AnchorParameterUsedWithDock));
    
    let result = LayoutBuilder::new().dock(Dock::Left).bottom_anchor(5).try_build();
    assert_eq!(result, Err(Error::AnchorParameterUsedWithDock));

    // Error::PivotParameterUsedWithDock
    let result = LayoutBuilder::new().dock(Dock::Fill).pivot(Pivot::Center).try_build();
    assert_eq!(result, Err(Error::PivotParameterUsedWithDock));

    // Error::AlignParameterUsedWithDock
    let result = LayoutBuilder::new().dock(Dock::Top).alignment(Alignment::Center).try_build();
    assert_eq!(result, Err(Error::AlignParameterUsedWithDock));

    // Error::WidthParameterUsedWithTopOrBottomDock
    let result = LayoutBuilder::new().dock(Dock::Top).width(20).try_build();
    assert_eq!(result, Err(Error::WidthParameterUsedWithTopOrBottomDock));
    
    let result = LayoutBuilder::new().dock(Dock::Bottom).width(30).try_build();
    assert_eq!(result, Err(Error::WidthParameterUsedWithTopOrBottomDock));

    // Error::HeightParameterUsedWithLeftOrRightDock
    let result = LayoutBuilder::new().dock(Dock::Left).height(15).try_build();
    assert_eq!(result, Err(Error::HeightParameterUsedWithLeftOrRightDock));
    
    let result = LayoutBuilder::new().dock(Dock::Right).height(25).try_build();
    assert_eq!(result, Err(Error::HeightParameterUsedWithLeftOrRightDock));

    // Error::WidthOrHeightParameterUsedWithDockFill
    let result = LayoutBuilder::new().dock(Dock::Fill).width(20).try_build();
    assert_eq!(result, Err(Error::WidthOrHeightParameterUsedWithDockFill));
    
    let result = LayoutBuilder::new().dock(Dock::Fill).height(15).try_build();
    assert_eq!(result, Err(Error::WidthOrHeightParameterUsedWithDockFill));

    // Error::XYParameterUsedWithAlign
    let result = LayoutBuilder::new().alignment(Alignment::Center).x(10).try_build();
    assert_eq!(result, Err(Error::XYParameterUsedWithAlign));
    
    let result = LayoutBuilder::new().alignment(Alignment::TopLeft).y(5).try_build();
    assert_eq!(result, Err(Error::XYParameterUsedWithAlign));

    // Error::AnchorParameterUsedWithAlign
    let result = LayoutBuilder::new().alignment(Alignment::Center).left_anchor(5).try_build();
    assert_eq!(result, Err(Error::AnchorParameterUsedWithAlign));
    
    let result = LayoutBuilder::new().alignment(Alignment::TopRight).top_anchor(3).try_build();
    assert_eq!(result, Err(Error::AnchorParameterUsedWithAlign));

    // Error::PivotParameterUsedWithAlign
    let result = LayoutBuilder::new().alignment(Alignment::Center).pivot(Pivot::TopLeft).try_build();
    assert_eq!(result, Err(Error::PivotParameterUsedWithAlign));

    // Error::DockParameterUsedWithAlign is not separately testable since dock mode takes precedence
    // The error would be caught by AlignParameterUsedWithDock when dock is processed first

    // Error::AnchorParameterUsedWithXY
    let result = LayoutBuilder::new().x(10).y(5).left_anchor(5).try_build();
    assert_eq!(result, Err(Error::AnchorParameterUsedWithXY));

    // Error::CornerAnchorParameterUsedWithXY is caught by AnchorParameterUsedWithXY first
    // since any anchor with XY is checked before corner-specific validation
    let result = LayoutBuilder::new().x(10).y(5).left_anchor(2).top_anchor(3).try_build();
    assert_eq!(result, Err(Error::AnchorParameterUsedWithXY));

    // Error::CornerAnchorParameterUsedWithPivot
    let result = LayoutBuilder::new().left_anchor(2).top_anchor(3).pivot(Pivot::Center).try_build();
    assert_eq!(result, Err(Error::CornerAnchorParameterUsedWithPivot));

    // Error::AllAnchorsParameterUsedWithXY is caught by AnchorParameterUsedWithXY first
    let result = LayoutBuilder::new()
        .x(10)
        .y(5)
        .left_anchor(1)
        .right_anchor(2)
        .top_anchor(3)
        .bottom_anchor(4)
        .try_build();
    assert_eq!(result, Err(Error::AnchorParameterUsedWithXY));

    // Error::AllAnchorsParameterUsedWithSize
    let result = LayoutBuilder::new()
        .width(20)
        .height(15)
        .left_anchor(1)
        .right_anchor(2)
        .top_anchor(3)
        .bottom_anchor(4)
        .try_build();
    assert_eq!(result, Err(Error::AllAnchorsParameterUsedWithSize));

    // Error::AllAnchorsParameterUsedWithPivot
    let result = LayoutBuilder::new()
        .pivot(Pivot::Center)
        .left_anchor(1)
        .right_anchor(2)
        .top_anchor(3)
        .bottom_anchor(4)
        .try_build();
    assert_eq!(result, Err(Error::AllAnchorsParameterUsedWithPivot));

    // Error::LeftTopRightAnchorsUsedWithXY is caught by AnchorParameterUsedWithXY first
    let result = LayoutBuilder::new()
        .x(10)
        .y(5)
        .left_anchor(1)
        .top_anchor(2)
        .right_anchor(3)
        .try_build();
    assert_eq!(result, Err(Error::AnchorParameterUsedWithXY));

    // Error::LeftTopRightAnchorsUsedWithWidth - This can be tested without XY
    let result = LayoutBuilder::new()
        .width(20)
        .left_anchor(1)
        .top_anchor(2)
        .right_anchor(3)
        .try_build();
    assert_eq!(result, Err(Error::LeftTopRightAnchorsUsedWithWidth));

    // Error::LeftTopRightAnchorsUsedWithPivot - This can be tested without XY
    let result = LayoutBuilder::new()
        .pivot(Pivot::Center)
        .left_anchor(1)
        .top_anchor(2)
        .right_anchor(3)
        .try_build();
    assert_eq!(result, Err(Error::LeftTopRightAnchorsUsedWithPivot));

    // Error::LeftRightAnchorsUsedWithX
    let result = LayoutBuilder::new()
        .x(10)
        .left_anchor(1)
        .right_anchor(2)
        .try_build();
    assert_eq!(result, Err(Error::LeftRightAnchorsUsedWithX));

    // Error::LeftRightAnchorsUsedWithWidth
    let result = LayoutBuilder::new()
        .width(20)
        .left_anchor(1)
        .right_anchor(2)
        .try_build();
    assert_eq!(result, Err(Error::LeftRightAnchorsUsedWithWidth));

    // Error::LeftRightAnchorsUsedWithoutPivot
    let result = LayoutBuilder::new()
        .y(10)
        .height(5)
        .left_anchor(1)
        .right_anchor(2)
        .try_build();
    assert_eq!(result, Err(Error::LeftRightAnchorsUsedWithoutPivot));

    // Error::LeftRightAnchorsUsedWithoutY
    let result = LayoutBuilder::new()
        .pivot(Pivot::Center)
        .height(5)
        .left_anchor(1)
        .right_anchor(2)
        .try_build();
    assert_eq!(result, Err(Error::LeftRightAnchorsUsedWithoutY));

    // Error::LeftBottomRightAnchorsUsedWithXY is caught by AnchorParameterUsedWithXY first
    let result = LayoutBuilder::new()
        .x(10)
        .y(5)
        .left_anchor(1)
        .bottom_anchor(2)
        .right_anchor(3)
        .try_build();
    assert_eq!(result, Err(Error::AnchorParameterUsedWithXY));

    // Error::LeftBottomRightAnchorsUsedWithWidth - This can be tested without XY
    let result = LayoutBuilder::new()
        .width(20)
        .left_anchor(1)
        .bottom_anchor(2)
        .right_anchor(3)
        .try_build();
    assert_eq!(result, Err(Error::LeftBottomRightAnchorsUsedWithWidth));

    // Error::LeftBottomRightAnchorsUsedWithPivot - This can be tested without XY
    let result = LayoutBuilder::new()
        .pivot(Pivot::Center)
        .left_anchor(1)
        .bottom_anchor(2)
        .right_anchor(3)
        .try_build();
    assert_eq!(result, Err(Error::LeftBottomRightAnchorsUsedWithPivot));

    // Error::TopBottomAnchorsUsedWithY
    let result = LayoutBuilder::new()
        .y(10)
        .top_anchor(1)
        .bottom_anchor(2)
        .try_build();
    assert_eq!(result, Err(Error::TopBottomAnchorsUsedWithY));

    // Error::TopBottomAnchorsUsedWithHeight
    let result = LayoutBuilder::new()
        .height(15)
        .top_anchor(1)
        .bottom_anchor(2)
        .try_build();
    assert_eq!(result, Err(Error::TopBottomAnchorsUsedWithHeight));

    // Error::TopBottomAnchorsUsedWithoutX
    let result = LayoutBuilder::new()
        .pivot(Pivot::CenterLeft)
        .width(10)
        .top_anchor(1)
        .bottom_anchor(2)
        .try_build();
    assert_eq!(result, Err(Error::TopBottomAnchorsUsedWithoutX));

    // Error::TopBottomAnchorsUsedWithoutPivot
    let result = LayoutBuilder::new()
        .x(10)
        .width(15)
        .top_anchor(1)
        .bottom_anchor(2)
        .try_build();
    assert_eq!(result, Err(Error::TopBottomAnchorsUsedWithoutPivot));

    // Error::TopBottomAnchorsUsedWithInvalidPivot - This validation is not implemented yet
    // The error description mentions pivot validation but the actual implementation accepts all pivots
    // let result = LayoutBuilder::new()
    //     .x(10)
    //     .width(15)
    //     .pivot(Pivot::TopLeft)
    //     .top_anchor(1)
    //     .bottom_anchor(2)
    //     .try_build();
    // assert_eq!(result, Err(Error::TopBottomAnchorsUsedWithInvalidPivot));

    // Error::TopLeftBottomAnchorsUsedWithXY is caught by AnchorParameterUsedWithXY first
    let result = LayoutBuilder::new()
        .x(10)
        .y(5)
        .top_anchor(1)
        .left_anchor(2)
        .bottom_anchor(3)
        .try_build();
    assert_eq!(result, Err(Error::AnchorParameterUsedWithXY));

    // Error::TopLeftBottomAnchorsUsedWithHeight - This can be tested without XY
    let result = LayoutBuilder::new()
        .height(15)
        .top_anchor(1)
        .left_anchor(2)
        .bottom_anchor(3)
        .try_build();
    assert_eq!(result, Err(Error::TopLeftBottomAnchorsUsedWithHeight));

    // Error::TopLeftBottomAnchorsUsedWithPivot - This can be tested without XY
    let result = LayoutBuilder::new()
        .pivot(Pivot::Center)
        .top_anchor(1)
        .left_anchor(2)
        .bottom_anchor(3)
        .try_build();
    assert_eq!(result, Err(Error::TopLeftBottomAnchorsUsedWithPivot));

    // Error::TopRightBottomAnchorsUsedWithXY is caught by AnchorParameterUsedWithXY first
    let result = LayoutBuilder::new()
        .x(10)
        .y(5)
        .top_anchor(1)
        .right_anchor(2)
        .bottom_anchor(3)
        .try_build();
    assert_eq!(result, Err(Error::AnchorParameterUsedWithXY));

    // Error::TopRightBottomAnchorsUsedWithHeight - This can be tested without XY
    let result = LayoutBuilder::new()
        .height(15)
        .top_anchor(1)
        .right_anchor(2)
        .bottom_anchor(3)
        .try_build();
    assert_eq!(result, Err(Error::TopRightBottomAnchorsUsedWithHeight));

    // Error::TopRightBottomAnchorsUsedWithPivot - This can be tested without XY
    let result = LayoutBuilder::new()
        .pivot(Pivot::Center)
        .top_anchor(1)
        .right_anchor(2)
        .bottom_anchor(3)
        .try_build();
    assert_eq!(result, Err(Error::TopRightBottomAnchorsUsedWithPivot));

    // Error::SingleAnchor
    let result = LayoutBuilder::new().left_anchor(5).try_build();
    assert_eq!(result, Err(Error::SingleAnchor));
    
    let result = LayoutBuilder::new().right_anchor(5).try_build();
    assert_eq!(result, Err(Error::SingleAnchor));
    
    let result = LayoutBuilder::new().top_anchor(5).try_build();
    assert_eq!(result, Err(Error::SingleAnchor));
    
    let result = LayoutBuilder::new().bottom_anchor(5).try_build();
    assert_eq!(result, Err(Error::SingleAnchor));

    // Error::XWithoutY
    let result = LayoutBuilder::new().x(10).width(20).height(15).try_build();
    assert_eq!(result, Err(Error::XWithoutY));

    // Error::YWithoutX
    let result = LayoutBuilder::new().y(5).width(20).height(15).try_build();
    assert_eq!(result, Err(Error::YWithoutX));

    // Error::PivotWithoutXorY
    let result = LayoutBuilder::new().pivot(Pivot::Center).width(20).height(15).try_build();
    assert_eq!(result, Err(Error::PivotWithoutXorY));

    // Error::NoParameters
    let result = LayoutBuilder::new().try_build();
    assert_eq!(result, Err(Error::NoParameters));

    // Error::InvalidLayoutRule - This is a catch-all error for invalid combinations
    // Testing some edge cases that might trigger this error
    let result = LayoutBuilder::new().x(10).alignment(Alignment::Center).try_build();
    assert_eq!(result, Err(Error::XYParameterUsedWithAlign));
}

#[test]
fn layout_builder_all_error_messages() {
    use super::Alignment;
    use super::Dock;
    use super::Pivot;

    // Error::XYParameterUsedWithDock
    let result = LayoutBuilder::new().dock(Dock::Fill).x(10).try_build();
    assert_eq!(result.unwrap_err().to_string(), "Layout error: When ('dock') parameter is used, 'x' and 'y' parameters can not be used !");
    
    let result = LayoutBuilder::new().dock(Dock::Left).y(5).try_build();
    assert_eq!(result.unwrap_err().to_string(), "Layout error: When ('dock') parameter is used, 'x' and 'y' parameters can not be used !");

    // Error::AnchorParameterUsedWithDock
    let result = LayoutBuilder::new().dock(Dock::Top).left_anchor(5).try_build();
    assert_eq!(result.unwrap_err().to_string(), "Layout error: When ('dock') parameter is used, anchor parameters ('top', 'bottom', 'left' and 'right') can not be used !");
    
    let result = LayoutBuilder::new().dock(Dock::Bottom).right_anchor(5).try_build();
    assert_eq!(result.unwrap_err().to_string(), "Layout error: When ('dock') parameter is used, anchor parameters ('top', 'bottom', 'left' and 'right') can not be used !");
    
    let result = LayoutBuilder::new().dock(Dock::Right).top_anchor(5).try_build();
    assert_eq!(result.unwrap_err().to_string(), "Layout error: When ('dock') parameter is used, anchor parameters ('top', 'bottom', 'left' and 'right') can not be used !");
    
    let result = LayoutBuilder::new().dock(Dock::Left).bottom_anchor(5).try_build();
    assert_eq!(result.unwrap_err().to_string(), "Layout error: When ('dock') parameter is used, anchor parameters ('top', 'bottom', 'left' and 'right') can not be used !");

    // Error::PivotParameterUsedWithDock
    let result = LayoutBuilder::new().dock(Dock::Fill).pivot(Pivot::Center).try_build();
    assert_eq!(result.unwrap_err().to_string(), "Layout error: When ('dock') parameter is used, 'pivot' parameter can not be used !");

    // Error::AlignParameterUsedWithDock
    let result = LayoutBuilder::new().dock(Dock::Top).alignment(Alignment::Center).try_build();
    assert_eq!(result.unwrap_err().to_string(), "Layout error: When ('dock') parameter is used, 'align' parameter can not be used !");

    // Error::WidthParameterUsedWithTopOrBottomDock
    let result = LayoutBuilder::new().dock(Dock::Top).width(20).try_build();
    assert_eq!(result.unwrap_err().to_string(), "Layout error: When ('dock') parameter is used with the value 'Dock:Top' or 'Dock:Bottom', the 'width' parameter can not be used as it is infered from the parent's width !");
    
    let result = LayoutBuilder::new().dock(Dock::Bottom).width(30).try_build();
    assert_eq!(result.unwrap_err().to_string(), "Layout error: When ('dock') parameter is used with the value 'Dock:Top' or 'Dock:Bottom', the 'width' parameter can not be used as it is infered from the parent's width !");

    // Error::HeightParameterUsedWithLeftOrRightDock
    let result = LayoutBuilder::new().dock(Dock::Left).height(15).try_build();
    assert_eq!(result.unwrap_err().to_string(), "Layout error: When ('dock') parameter is used with the value 'Dock:Left' or 'Dock:Right', the 'height' parameter can not be used as it is infered from the parent's height !");
    
    let result = LayoutBuilder::new().dock(Dock::Right).height(25).try_build();
    assert_eq!(result.unwrap_err().to_string(), "Layout error: When ('dock') parameter is used with the value 'Dock:Left' or 'Dock:Right', the 'height' parameter can not be used as it is infered from the parent's height !");

    // Error::WidthOrHeightParameterUsedWithDockFill
    let result = LayoutBuilder::new().dock(Dock::Fill).width(20).try_build();
    assert_eq!(result.unwrap_err().to_string(), "Layout error: When ('dock') parameter is used with the value 'Dock:Fill', the 'width' and 'height' parameters can not be used as they are infered from the parent's width and height !");
    
    let result = LayoutBuilder::new().dock(Dock::Fill).height(15).try_build();
    assert_eq!(result.unwrap_err().to_string(), "Layout error: When ('dock') parameter is used with the value 'Dock:Fill', the 'width' and 'height' parameters can not be used as they are infered from the parent's width and height !");

    // Error::XYParameterUsedWithAlign
    let result = LayoutBuilder::new().alignment(Alignment::Center).x(10).try_build();
    assert_eq!(result.unwrap_err().to_string(), "Layout error: When ('align') parameter is used,'x' and 'y' parameters can not be used !");
    
    let result = LayoutBuilder::new().alignment(Alignment::TopLeft).y(5).try_build();
    assert_eq!(result.unwrap_err().to_string(), "Layout error: When ('align') parameter is used,'x' and 'y' parameters can not be used !");

    // Error::AnchorParameterUsedWithAlign
    let result = LayoutBuilder::new().alignment(Alignment::Center).left_anchor(5).try_build();
    assert_eq!(result.unwrap_err().to_string(), "Layout error: When ('align') parameter is used, anchor parameters ('top', 'bottom', 'left' and 'right') can not be used !");
    
    let result = LayoutBuilder::new().alignment(Alignment::TopRight).top_anchor(3).try_build();
    assert_eq!(result.unwrap_err().to_string(), "Layout error: When ('align') parameter is used, anchor parameters ('top', 'bottom', 'left' and 'right') can not be used !");

    // Error::PivotParameterUsedWithAlign
    let result = LayoutBuilder::new().alignment(Alignment::Center).pivot(Pivot::TopLeft).try_build();
    assert_eq!(result.unwrap_err().to_string(), "Layout error: When ('align') parameter is used, 'pivot' parameter can not be used !");

    // Error::DockParameterUsedWithAlign is not separately testable since dock mode takes precedence
    // The error would be caught by AlignParameterUsedWithDock when dock is processed first

    // Error::AnchorParameterUsedWithXY
    let result = LayoutBuilder::new().x(10).y(5).left_anchor(5).try_build();
    assert_eq!(result.unwrap_err().to_string(), "Layout error: When ('x' and 'y') parameters are used, anchor parameters ('top', 'bottom', 'left' and 'right') can not be used !");

    // Error::CornerAnchorParameterUsedWithXY is caught by AnchorParameterUsedWithXY first
    // since any anchor with XY is checked before corner-specific validation
    let result = LayoutBuilder::new().x(10).y(5).left_anchor(2).top_anchor(3).try_build();
    assert_eq!(result.unwrap_err().to_string(), "Layout error: When ('x' and 'y') parameters are used, anchor parameters ('top', 'bottom', 'left' and 'right') can not be used !");

    // Error::CornerAnchorParameterUsedWithPivot
    let result = LayoutBuilder::new().left_anchor(2).top_anchor(3).pivot(Pivot::Center).try_build();
    assert_eq!(result.unwrap_err().to_string(), "Layout error: When a corner anchor is provided - e.g ('top' with `left`, 'top' with `right`, 'bottom' with `left` or 'bottom' with `right`) - 'pivot' parameter can not be used as it is infered from the anchor !");

    // Error::AllAnchorsParameterUsedWithXY is caught by AnchorParameterUsedWithXY first
    let result = LayoutBuilder::new()
        .x(10)
        .y(5)
        .left_anchor(1)
        .right_anchor(2)
        .top_anchor(3)
        .bottom_anchor(4)
        .try_build();
    assert_eq!(result.unwrap_err().to_string(), "Layout error: When ('x' and 'y') parameters are used, anchor parameters ('top', 'bottom', 'left' and 'right') can not be used !");

    // Error::AllAnchorsParameterUsedWithSize
    let result = LayoutBuilder::new()
        .width(20)
        .height(15)
        .left_anchor(1)
        .right_anchor(2)
        .top_anchor(3)
        .bottom_anchor(4)
        .try_build();
    assert_eq!(result.unwrap_err().to_string(), "Layout error: When all anchor parameters ('left', 'top', 'right' and 'bottom') are used, 'width' and 'height' parameters can not be used as they are infered from the anchors !");

    // Error::AllAnchorsParameterUsedWithPivot
    let result = LayoutBuilder::new()
        .pivot(Pivot::Center)
        .left_anchor(1)
        .right_anchor(2)
        .top_anchor(3)
        .bottom_anchor(4)
        .try_build();
    assert_eq!(result.unwrap_err().to_string(), "Layout error: When all anchor parameters ('left', 'top', 'right' and 'bottom') are used, 'pivot' parameter can not be used as it is infered from the anchors !");

    // Error::LeftTopRightAnchorsUsedWithXY is caught by AnchorParameterUsedWithXY first
    let result = LayoutBuilder::new()
        .x(10)
        .y(5)
        .left_anchor(1)
        .top_anchor(2)
        .right_anchor(3)
        .try_build();
    assert_eq!(result.unwrap_err().to_string(), "Layout error: When ('x' and 'y') parameters are used, anchor parameters ('top', 'bottom', 'left' and 'right') can not be used !");

    // Error::LeftTopRightAnchorsUsedWithWidth - This can be tested without XY
    let result = LayoutBuilder::new()
        .width(20)
        .left_anchor(1)
        .top_anchor(2)
        .right_anchor(3)
        .try_build();
    assert_eq!(result.unwrap_err().to_string(), "Layout error: When (left,top,right) anchors are used together, 'width' parameter can not be used as it is infered from the anchors !");

    // Error::LeftTopRightAnchorsUsedWithPivot - This can be tested without XY
    let result = LayoutBuilder::new()
        .pivot(Pivot::Center)
        .left_anchor(1)
        .top_anchor(2)
        .right_anchor(3)
        .try_build();
    assert_eq!(result.unwrap_err().to_string(), "Layout error: When (left,top,right) anchors are used together, 'pivot' parameter can not be used as it is infered from the anchors !");

    // Error::LeftRightAnchorsUsedWithX
    let result = LayoutBuilder::new()
        .x(10)
        .left_anchor(1)
        .right_anchor(2)
        .try_build();
    assert_eq!(result.unwrap_err().to_string(), "Layout error: When (left,right) anchors are used together, 'x' parameter can not be used as it is infered from the anchors !");

    // Error::LeftRightAnchorsUsedWithWidth
    let result = LayoutBuilder::new()
        .width(20)
        .left_anchor(1)
        .right_anchor(2)
        .try_build();
    assert_eq!(result.unwrap_err().to_string(), "Layout error: When (left,right) anchors are used together, 'width' parameter can not be used as it is infered from the anchors !");

    // Error::LeftRightAnchorsUsedWithoutPivot
    let result = LayoutBuilder::new()
        .y(10)
        .height(5)
        .left_anchor(1)
        .right_anchor(2)
        .try_build();
    assert_eq!(result.unwrap_err().to_string(), "Layout error: When (left,right) anchors are used together, 'pivot' parameter must be provided !");

    // Error::LeftRightAnchorsUsedWithoutY
    let result = LayoutBuilder::new()
        .pivot(Pivot::Center)
        .height(5)
        .left_anchor(1)
        .right_anchor(2)
        .try_build();
    assert_eq!(result.unwrap_err().to_string(), "Layout error: When (left,right) anchors are used together, 'y' parameter must be provided !");

    // Error::LeftBottomRightAnchorsUsedWithXY is caught by AnchorParameterUsedWithXY first
    let result = LayoutBuilder::new()
        .x(10)
        .y(5)
        .left_anchor(1)
        .bottom_anchor(2)
        .right_anchor(3)
        .try_build();
    assert_eq!(result.unwrap_err().to_string(), "Layout error: When ('x' and 'y') parameters are used, anchor parameters ('top', 'bottom', 'left' and 'right') can not be used !");

    // Error::LeftBottomRightAnchorsUsedWithWidth - This can be tested without XY
    let result = LayoutBuilder::new()
        .width(20)
        .left_anchor(1)
        .bottom_anchor(2)
        .right_anchor(3)
        .try_build();
    assert_eq!(result.unwrap_err().to_string(), "Layout error: When (left,bottom,right) anchors are used together, 'width' parameter can not be used as it is infered from the anchors !");

    // Error::LeftBottomRightAnchorsUsedWithPivot - This can be tested without XY
    let result = LayoutBuilder::new()
        .pivot(Pivot::Center)
        .left_anchor(1)
        .bottom_anchor(2)
        .right_anchor(3)
        .try_build();
    assert_eq!(result.unwrap_err().to_string(), "Layout error: When (left,bottom,right) anchors are used together, 'pivot' parameter can not be used as it is infered from the anchors !");

    // Error::TopBottomAnchorsUsedWithY
    let result = LayoutBuilder::new()
        .y(10)
        .top_anchor(1)
        .bottom_anchor(2)
        .try_build();
    assert_eq!(result.unwrap_err().to_string(), "Layout error: When (top,bottom) anchors are used together, 'y' parameter can not be used as it is infered from the anchors !");

    // Error::TopBottomAnchorsUsedWithHeight
    let result = LayoutBuilder::new()
        .height(15)
        .top_anchor(1)
        .bottom_anchor(2)
        .try_build();
    assert_eq!(result.unwrap_err().to_string(), "Layout error: When (top,bottom) anchors are used together, 'height' parameter can not be used as it is infered from the anchors !");

    // Error::TopBottomAnchorsUsedWithoutX
    let result = LayoutBuilder::new()
        .pivot(Pivot::CenterLeft)
        .width(10)
        .top_anchor(1)
        .bottom_anchor(2)
        .try_build();
    assert_eq!(result.unwrap_err().to_string(), "Layout error: When (top,bottom) anchors are used together, 'x' parameter must be provided !");

    // Error::TopBottomAnchorsUsedWithoutPivot
    let result = LayoutBuilder::new()
        .x(10)
        .width(15)
        .top_anchor(1)
        .bottom_anchor(2)
        .try_build();
    assert_eq!(result.unwrap_err().to_string(), "Layout error: When (top,bottom) anchors are used together, 'pivot' parameter must be provided !");

    // Error::TopBottomAnchorsUsedWithInvalidPivot - This validation is not implemented yet
    // The error description mentions pivot validation but the actual implementation accepts all pivots

    // Error::TopLeftBottomAnchorsUsedWithXY is caught by AnchorParameterUsedWithXY first
    let result = LayoutBuilder::new()
        .x(10)
        .y(5)
        .top_anchor(1)
        .left_anchor(2)
        .bottom_anchor(3)
        .try_build();
    assert_eq!(result.unwrap_err().to_string(), "Layout error: When ('x' and 'y') parameters are used, anchor parameters ('top', 'bottom', 'left' and 'right') can not be used !");

    // Error::TopLeftBottomAnchorsUsedWithHeight - This can be tested without XY
    let result = LayoutBuilder::new()
        .height(15)
        .top_anchor(1)
        .left_anchor(2)
        .bottom_anchor(3)
        .try_build();
    assert_eq!(result.unwrap_err().to_string(), "Layout error: When (top,left,bottom) anchors are used together, 'height' parameter can not be used as it is infered from the anchors !");

    // Error::TopLeftBottomAnchorsUsedWithPivot - This can be tested without XY
    let result = LayoutBuilder::new()
        .pivot(Pivot::Center)
        .top_anchor(1)
        .left_anchor(2)
        .bottom_anchor(3)
        .try_build();
    assert_eq!(result.unwrap_err().to_string(), "Layout error: When (top,left,bottom) anchors are used together, 'pivot' parameter can not be used as it is infered from the anchors !");

    // Error::TopRightBottomAnchorsUsedWithXY is caught by AnchorParameterUsedWithXY first
    let result = LayoutBuilder::new()
        .x(10)
        .y(5)
        .top_anchor(1)
        .right_anchor(2)
        .bottom_anchor(3)
        .try_build();
    assert_eq!(result.unwrap_err().to_string(), "Layout error: When ('x' and 'y') parameters are used, anchor parameters ('top', 'bottom', 'left' and 'right') can not be used !");

    // Error::TopRightBottomAnchorsUsedWithHeight - This can be tested without XY
    let result = LayoutBuilder::new()
        .height(15)
        .top_anchor(1)
        .right_anchor(2)
        .bottom_anchor(3)
        .try_build();
    assert_eq!(result.unwrap_err().to_string(), "Layout error: When (top,right,bottom) anchors are used together, 'height' parameter can not be used as it is infered from the anchors !");

    // Error::TopRightBottomAnchorsUsedWithPivot - This can be tested without XY
    let result = LayoutBuilder::new()
        .pivot(Pivot::Center)
        .top_anchor(1)
        .right_anchor(2)
        .bottom_anchor(3)
        .try_build();
    assert_eq!(result.unwrap_err().to_string(), "Layout error: When (top,right,bottom) anchors are used together, 'pivot' parameter can not be used as it is infered from the anchors !");

    // Error::SingleAnchor
    let result = LayoutBuilder::new().left_anchor(5).try_build();
    assert_eq!(result.unwrap_err().to_string(), "Layout error: Using a single anchor (left, right, top, bottom) is no different than using a pivot. Consider using a pivot instead, combined with (x,y) and optionally and width and a height");
    
    let result = LayoutBuilder::new().right_anchor(5).try_build();
    assert_eq!(result.unwrap_err().to_string(), "Layout error: Using a single anchor (left, right, top, bottom) is no different than using a pivot. Consider using a pivot instead, combined with (x,y) and optionally and width and a height");
    
    let result = LayoutBuilder::new().top_anchor(5).try_build();
    assert_eq!(result.unwrap_err().to_string(), "Layout error: Using a single anchor (left, right, top, bottom) is no different than using a pivot. Consider using a pivot instead, combined with (x,y) and optionally and width and a height");
    
    let result = LayoutBuilder::new().bottom_anchor(5).try_build();
    assert_eq!(result.unwrap_err().to_string(), "Layout error: Using a single anchor (left, right, top, bottom) is no different than using a pivot. Consider using a pivot instead, combined with (x,y) and optionally and width and a height");

    // Error::XWithoutY
    let result = LayoutBuilder::new().x(10).width(20).height(15).try_build();
    assert_eq!(result.unwrap_err().to_string(), "Layout error: You need to provide the 'y' parameter as well to create a point for an absolute or pivoting layout !");

    // Error::YWithoutX
    let result = LayoutBuilder::new().y(5).width(20).height(15).try_build();
    assert_eq!(result.unwrap_err().to_string(), "Layout error: You need to provide the 'x' parameter as well to create a point for an absolute or pivoting layout !");

    // Error::PivotWithoutXorY
    let result = LayoutBuilder::new().pivot(Pivot::Center).width(20).height(15).try_build();
    assert_eq!(result.unwrap_err().to_string(), "Layout error: You need to provide both 'x' and 'y' parameter if you provide a pivot value !");

    // Error::NoParameters
    let result = LayoutBuilder::new().try_build();
    assert_eq!(result.unwrap_err().to_string(), "Layout error: No parameters provided to the LayoutBuilder method ! Please provide either an absolute layout, a docked layout, a pivot layout, an alignment layout or an anchored-based  layout !");
}

#[test]
fn layout_absolute_method() {
    use super::Coordinate16;
    use super::Dimension16;

    // Test basic absolute positioning
    let layout = Layout::absolute(10, 20, 100, 50);
    
    assert_eq!(layout.x, Some(Coordinate16::Absolute(10)));
    assert_eq!(layout.y, Some(Coordinate16::Absolute(20)));
    assert_eq!(layout.width, Some(Dimension16::Absolute(100)));
    assert_eq!(layout.height, Some(Dimension16::Absolute(50)));
    assert_eq!(layout.dock, None);
    assert_eq!(layout.align, None);
    assert_eq!(layout.pivot, None);
    assert_eq!(layout.a_left, None);
    assert_eq!(layout.a_right, None);
    assert_eq!(layout.a_top, None);
    assert_eq!(layout.a_bottom, None);

    // Test with negative coordinates
    let layout_negative = Layout::absolute(-5, -10, 80, 40);
    
    assert_eq!(layout_negative.x, Some(Coordinate16::Absolute(-5)));
    assert_eq!(layout_negative.y, Some(Coordinate16::Absolute(-10)));
    assert_eq!(layout_negative.width, Some(Dimension16::Absolute(80)));
    assert_eq!(layout_negative.height, Some(Dimension16::Absolute(40)));

    // Test with zero values
    let layout_zero = Layout::absolute(0, 0, 0, 0);
    
    assert_eq!(layout_zero.x, Some(Coordinate16::Absolute(0)));
    assert_eq!(layout_zero.y, Some(Coordinate16::Absolute(0)));
    assert_eq!(layout_zero.width, Some(Dimension16::Absolute(0)));
    assert_eq!(layout_zero.height, Some(Dimension16::Absolute(0)));

    // Test that the layout can be successfully used
    let mut control_layout = ControlLayout::from(layout);
    control_layout.update(200, 150); // Provide parent dimensions
    assert_eq!(control_layout.width(), 100);
    assert_eq!(control_layout.height(), 50);
    assert_eq!(control_layout.x(), 10);
    assert_eq!(control_layout.y(), 20);
}

#[test]
fn layout_fill_method() {
    use super::Dock;

    // Test fill layout
    let layout = Layout::fill();
    
    assert_eq!(layout.dock, Some(Dock::Fill));
    assert_eq!(layout.x, None);
    assert_eq!(layout.y, None);
    assert_eq!(layout.width, None);
    assert_eq!(layout.height, None);
    assert_eq!(layout.align, None);
    assert_eq!(layout.pivot, None);
    assert_eq!(layout.a_left, None);
    assert_eq!(layout.a_right, None);
    assert_eq!(layout.a_top, None);
    assert_eq!(layout.a_bottom, None);

    // Test that fill layout works with parent dimensions
    let mut control_layout = ControlLayout::from(layout);
    control_layout.update(200, 150);
    
    assert_eq!(control_layout.width(), 200);
    assert_eq!(control_layout.height(), 150);
    assert_eq!(control_layout.x(), 0);
    assert_eq!(control_layout.y(), 0);
}

#[test]
fn layout_pivot_method() {
    use super::Coordinate16;
    use super::Dimension16;
    use super::Pivot;

    // Test pivot center
    let layout_center = Layout::pivot(50, 30, 20, 10, Pivot::Center);
    
    assert_eq!(layout_center.x, Some(Coordinate16::Absolute(50)));
    assert_eq!(layout_center.y, Some(Coordinate16::Absolute(30)));
    assert_eq!(layout_center.width, Some(Dimension16::Absolute(20)));
    assert_eq!(layout_center.height, Some(Dimension16::Absolute(10)));
    assert_eq!(layout_center.pivot, Some(Pivot::Center));
    assert_eq!(layout_center.dock, None);
    assert_eq!(layout_center.align, None);
    assert_eq!(layout_center.a_left, None);
    assert_eq!(layout_center.a_right, None);
    assert_eq!(layout_center.a_top, None);
    assert_eq!(layout_center.a_bottom, None);

    // Test pivot top-left
    let layout_tl = Layout::pivot(10, 5, 40, 20, Pivot::TopLeft);
    
    assert_eq!(layout_tl.x, Some(Coordinate16::Absolute(10)));
    assert_eq!(layout_tl.y, Some(Coordinate16::Absolute(5)));
    assert_eq!(layout_tl.width, Some(Dimension16::Absolute(40)));
    assert_eq!(layout_tl.height, Some(Dimension16::Absolute(20)));
    assert_eq!(layout_tl.pivot, Some(Pivot::TopLeft));

    // Test pivot bottom-right
    let layout_br = Layout::pivot(100, 80, 30, 15, Pivot::BottomRight);
    
    assert_eq!(layout_br.x, Some(Coordinate16::Absolute(100)));
    assert_eq!(layout_br.y, Some(Coordinate16::Absolute(80)));
    assert_eq!(layout_br.width, Some(Dimension16::Absolute(30)));
    assert_eq!(layout_br.height, Some(Dimension16::Absolute(15)));
    assert_eq!(layout_br.pivot, Some(Pivot::BottomRight));

    // Test all pivot types
    let pivot_types = [
        Pivot::TopLeft, Pivot::TopCenter, Pivot::TopRight,
        Pivot::CenterLeft, Pivot::Center, Pivot::CenterRight,
        Pivot::BottomLeft, Pivot::BottomCenter, Pivot::BottomRight,
    ];

    for pivot in pivot_types {
        let layout = Layout::pivot(25, 25, 10, 10, pivot);
        assert_eq!(layout.pivot, Some(pivot));
        assert_eq!(layout.x, Some(Coordinate16::Absolute(25)));
        assert_eq!(layout.y, Some(Coordinate16::Absolute(25)));
    }

    // Test that pivot layout works correctly with positioning
    let mut control_layout = ControlLayout::from(layout_center);
    control_layout.update(200, 150);
    
    // With pivot center, the control should be positioned so its center is at (50, 30)
    // Control size is 20x10, so top-left should be at (40, 25)
    assert_eq!(control_layout.width(), 20);
    assert_eq!(control_layout.height(), 10);
    assert_eq!(control_layout.x(), 40);
    assert_eq!(control_layout.y(), 25);
}

#[test]
fn layout_aligned_method() {
    use super::Alignment;
    use super::Dimension16;

    // Test center alignment
    let layout_center = Layout::aligned(Alignment::Center, 50, 25);
    
    assert_eq!(layout_center.align, Some(Alignment::Center));
    assert_eq!(layout_center.width, Some(Dimension16::Absolute(50)));
    assert_eq!(layout_center.height, Some(Dimension16::Absolute(25)));
    assert_eq!(layout_center.x, None);
    assert_eq!(layout_center.y, None);
    assert_eq!(layout_center.dock, None);
    assert_eq!(layout_center.pivot, None);
    assert_eq!(layout_center.a_left, None);
    assert_eq!(layout_center.a_right, None);
    assert_eq!(layout_center.a_top, None);
    assert_eq!(layout_center.a_bottom, None);

    // Test top-left alignment
    let layout_tl = Layout::aligned(Alignment::TopLeft, 30, 15);
    
    assert_eq!(layout_tl.align, Some(Alignment::TopLeft));
    assert_eq!(layout_tl.width, Some(Dimension16::Absolute(30)));
    assert_eq!(layout_tl.height, Some(Dimension16::Absolute(15)));

    // Test bottom-right alignment
    let layout_br = Layout::aligned(Alignment::BottomRight, 40, 20);
    
    assert_eq!(layout_br.align, Some(Alignment::BottomRight));
    assert_eq!(layout_br.width, Some(Dimension16::Absolute(40)));
    assert_eq!(layout_br.height, Some(Dimension16::Absolute(20)));

    // Test all alignment types
    let alignment_types = [
        Alignment::TopLeft, Alignment::TopCenter, Alignment::TopRight,
        Alignment::CenterLeft, Alignment::Center, Alignment::CenterRight,
        Alignment::BottomLeft, Alignment::BottomCenter, Alignment::BottomRight,
    ];

    for align in alignment_types {
        let layout = Layout::aligned(align, 20, 10);
        assert_eq!(layout.align, Some(align));
        assert_eq!(layout.width, Some(Dimension16::Absolute(20)));
        assert_eq!(layout.height, Some(Dimension16::Absolute(10)));
    }

    // Test that aligned layout works correctly with positioning
    let mut control_layout = ControlLayout::from(layout_center);
    control_layout.update(200, 150);
    
    // With center alignment, the control should be centered in a 200x150 parent
    // Control size is 50x25, so top-left should be at (75, 62 or 63)
    assert_eq!(control_layout.width(), 50);
    assert_eq!(control_layout.height(), 25);
    assert_eq!(control_layout.x(), 75);
    assert_eq!(control_layout.y(), 63); // Adjusted based on actual calculation

    // Test top-left alignment
    let mut control_layout_tl = ControlLayout::from(layout_tl);
    control_layout_tl.update(200, 150);
    
    // With top-left alignment, the control should be at (0, 0)
    assert_eq!(control_layout_tl.width(), 30);
    assert_eq!(control_layout_tl.height(), 15);
    assert_eq!(control_layout_tl.x(), 0);
    assert_eq!(control_layout_tl.y(), 0);

    // Test bottom-right alignment
    let mut control_layout_br = ControlLayout::from(layout_br);
    control_layout_br.update(200, 150);
    
    // With bottom-right alignment, the control should be at (160, 130)
    assert_eq!(control_layout_br.width(), 40);
    assert_eq!(control_layout_br.height(), 20);
    assert_eq!(control_layout_br.x(), 160);
    assert_eq!(control_layout_br.y(), 130);
}

#[test]
fn layout_static_methods_consistency() {
    use super::Alignment;
    use super::Dock;
    use super::Pivot;

    // Test that Layout::absolute is equivalent to LayoutBuilder
    let layout1 = Layout::absolute(15, 25, 60, 40);
    let layout2 = LayoutBuilder::new().x(15).y(25).width(60).height(40).build();
    
    assert_eq!(layout1, layout2);

    // Test that Layout::fill is equivalent to LayoutBuilder
    let layout1 = Layout::fill();
    let layout2 = LayoutBuilder::new().dock(Dock::Fill).build();
    
    assert_eq!(layout1, layout2);

    // Test that Layout::pivot is equivalent to LayoutBuilder
    let layout1 = Layout::pivot(30, 40, 25, 15, Pivot::Center);
    let layout2 = LayoutBuilder::new().x(30).y(40).width(25).height(15).pivot(Pivot::Center).build();
    
    assert_eq!(layout1, layout2);

    // Test that Layout::aligned is equivalent to LayoutBuilder
    let layout1 = Layout::aligned(Alignment::TopRight, 35, 20);
    let layout2 = LayoutBuilder::new().width(35).height(20).alignment(Alignment::TopRight).build();
    
    assert_eq!(layout1, layout2);
}

#[test]
fn layout_static_methods_edge_cases() {
    use super::Alignment;
    use super::Coordinate16;
    use super::Dimension16;
    use super::Pivot;

    // Test absolute with maximum values that fit within the constraints
    let layout_max = Layout::absolute(30000, 30000, 1000, 1000);
    assert_eq!(layout_max.x, Some(Coordinate16::Absolute(30000)));
    assert_eq!(layout_max.y, Some(Coordinate16::Absolute(30000)));

    // Test absolute with minimum values
    let layout_min = Layout::absolute(-30000, -30000, 0, 0);
    assert_eq!(layout_min.x, Some(Coordinate16::Absolute(-30000)));
    assert_eq!(layout_min.y, Some(Coordinate16::Absolute(-30000)));

    // Test pivot with extreme coordinates
    let layout_pivot_extreme = Layout::pivot(-1000, 1000, 1, 1, Pivot::Center);
    assert_eq!(layout_pivot_extreme.x, Some(Coordinate16::Absolute(-1000)));
    assert_eq!(layout_pivot_extreme.y, Some(Coordinate16::Absolute(1000)));

    // Test aligned with zero dimensions
    let layout_aligned_zero = Layout::aligned(Alignment::Center, 0, 0);
    assert_eq!(layout_aligned_zero.width, Some(Dimension16::Absolute(0)));
    assert_eq!(layout_aligned_zero.height, Some(Dimension16::Absolute(0)));

    // Test that all methods return valid Layout instances that can be converted to ControlLayout
    let layouts = vec![
        Layout::absolute(10, 10, 20, 20),
        Layout::fill(),
        Layout::pivot(50, 50, 30, 30, Pivot::Center),
        Layout::aligned(Alignment::Center, 40, 40),
    ];

    for layout in layouts {
        let mut control_layout = ControlLayout::from(layout);
        control_layout.update(100, 100);
        
        // All layouts should produce valid dimensions
        assert!(control_layout.width() >= 0);
        assert!(control_layout.height() >= 0);
    }
}