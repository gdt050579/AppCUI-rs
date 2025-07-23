use crate::ui::layout::absolute_layout::AbsoluteLayout;
use crate::ui::layout::Coordinate;
use crate::prelude::*;

use super::Alignment;
use super::ControlLayout;
use super::Coordinate16;
use super::Dimension16;
use super::Dock;
use super::LayoutBuilder;
use super::PointAndSizeLayout;

use super::LayoutMode;

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
        assert_eq!(cl.get_width(), $w);
        assert_eq!(cl.get_height(), $h);
        assert_eq!(cl.get_x(), $x);
        assert_eq!(cl.get_y(), $y);
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
    validate_pos!("a:Center,w:20", 50, 30, 15, 0, 20, 30);
    validate_pos!("a:CENTER,h:10", 50, 30, 0, 10, 50, 10);
    validate_pos!("a:cEnTeR,w:50%,h:25%", 60, 40, 15, 15, 30, 10);
}
#[test]
fn layout_mode_dock_top_left() {
    validate_pos!("a:tl", 50, 30, 0, 0, 50, 30);
    validate_pos!("a:lt,w:20,h:10", 50, 30, 0, 0, 20, 10);
    validate_pos!("a:topleft,w:20", 50, 30, 0, 0, 20, 30);
    validate_pos!("a:lefttop,h:10", 50, 30, 0, 0, 50, 10);
    validate_pos!("a:TopLeft,w:50%,h:25%", 60, 40, 0, 0, 30, 10);
}
#[test]
fn layout_mode_dock_bottom_left() {
    validate_pos!("a:lb", 50, 30, 0, 0, 50, 30);
    validate_pos!("a:bl,w:20,h:10", 50, 30, 0, 20, 20, 10);
    validate_pos!("a:lb,w:20", 50, 30, 0, 0, 20, 30);
    validate_pos!("a:bottomleft,h:10", 50, 30, 0, 20, 50, 10);
    validate_pos!("a:leftbottom,w:50%,h:25%", 60, 40, 0, 30, 30, 10);
}
#[test]
fn layout_mode_dock_bottom_right() {
    validate_pos!("a:rb", 50, 30, 0, 0, 50, 30);
    validate_pos!("a:br,w:20,h:10", 50, 30, 30, 20, 20, 10);
    validate_pos!("a:rb,w:20", 50, 30, 30, 0, 20, 30);
    validate_pos!("a:bottomright,h:10", 50, 30, 0, 20, 50, 10);
    validate_pos!("a:rightbottom,w:50%,h:25%", 60, 40, 30, 30, 30, 10);
}
#[test]
fn layout_mode_dock_top_right() {
    validate_pos!("a:tr", 50, 30, 0, 0, 50, 30);
    validate_pos!("a:rt,w:20,h:10", 50, 30, 30, 0, 20, 10);
    validate_pos!("a:TopRight,w:20", 50, 30, 30, 0, 20, 30);
    validate_pos!("a:rightTop,h:10", 50, 30, 0, 0, 50, 10);
    validate_pos!("a:topright,w:50%,h:25%", 60, 40, 30, 0, 30, 10);
}
#[test]
fn layout_mode_dock_left() {
    validate_pos!("a:l", 50, 30, 0, 0, 50, 30);
    validate_pos!("a:left,w:20,h:10", 50, 30, 0, 10, 20, 10);
    validate_pos!("a:Left,w:20", 50, 30, 00, 0, 20, 30);
    validate_pos!("a:l,h:10", 50, 30, 0, 10, 50, 10);
    validate_pos!("a:LeFt,w:50%,h:25%", 60, 40, 0, 15, 30, 10);
}

#[test]
fn layout_mode_dock_top() {
    validate_pos!("a:t", 50, 30, 0, 0, 50, 30);
    validate_pos!("a:top,w:20,h:10", 50, 30, 15, 0, 20, 10);
    validate_pos!("a:Top,w:20", 50, 30, 15, 0, 20, 30);
    validate_pos!("a:t,h:10", 50, 30, 0, 0, 50, 10);
    validate_pos!("a:ToP,w:50%,h:25%", 60, 40, 15, 0, 30, 10);
}

#[test]
fn layout_mode_dock_right() {
    validate_pos!("a:r", 50, 30, 0, 0, 50, 30);
    validate_pos!("a:Right,w:20,h:10", 50, 30, 30, 10, 20, 10);
    validate_pos!("a:right,w:20", 50, 30, 30, 0, 20, 30);
    validate_pos!("a:r,h:10", 50, 30, 0, 10, 50, 10);
    validate_pos!("a:rIgHt,w:50%,h:25%", 60, 40, 30, 15, 30, 10);
}
#[test]
fn layout_mode_dock_bottom() {
    validate_pos!("a:b", 50, 30, 0, 0, 50, 30);
    validate_pos!("a:bottom,w:20,h:10", 50, 30, 15, 20, 20, 10);
    validate_pos!("a:Bottom,w:20", 50, 30, 15, 0, 20, 30);
    validate_pos!("a:b,h:10", 50, 30, 0, 20, 50, 10);
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

/* 
#[test]
#[should_panic]
fn layout_mode_anchor_lr_dont_allow_x() {
    // this code should panic because 'x' can not be used in a Left-Right layout mode
    validate_pos!("l:5,r:7,y:0,h:10,a:t,x:10", 50, 30, 5, 0, 38, 10);
}
#[test]
#[should_panic]
fn layout_mode_anchor_lr_invalid_alignment() {
    // this code should panic because only (top,bottom and center) alignments can not be used in a Left-Right layout mode
    validate_pos!("l:5,r:7,y:0,h:10,a:left", 50, 30, 5, 0, 38, 10);
}

#[test]
fn layout_mode_anchor_tb() {
    validate_pos!("t:5,b:7,x:0,w:10,p:l", 30, 50, 0, 5, 10, 38);
    validate_pos!("t:5,b:7,x:10,w:10,p:c", 30, 50, 5, 5, 10, 38);
    validate_pos!("t:5,b:7,x:20,w:10,p:r", 30, 50, 10, 5, 10, 38);
    // no alignment - default is center
    validate_pos!("t:5,b:7,x:0,w:10", 30, 50, -5, 5, 10, 38);

    validate_pos!("t:10%,p:l,x:50%,b:20%,w:4", 30, 50, 15, 5, 4, 35);
    validate_pos!("t:10%,p:c,x:50%,b:20%,w:4", 30, 50, 13, 5, 4, 35);
    validate_pos!("t:10%,p:r,x:50%,b:20%,w:4", 30, 50, 11, 5, 4, 35);

    validate_pos!("t:10%,p:l,x:50%,b:20%,w:50%", 30, 50, 15, 5, 15, 35);
    validate_pos!("t:10%,p:c,x:50%,b:20%,w:50%", 30, 50, 8, 5, 15, 35);
    validate_pos!("t:10%,p:r,x:50%,b:20%,w:50%", 30, 50, 0, 5, 15, 35);
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
fn coordonate_from_basic_type() {
    assert_eq!(Coordonate::from(10u8), Coordonate::Absolute(10));
    assert_eq!(Coordonate::from(-10i8), Coordonate::Absolute(-10));
    assert_eq!(Coordonate::from(10u16), Coordonate::Absolute(10));
    assert_eq!(Coordonate::from(-10i16), Coordonate::Absolute(-10));
    assert_eq!(Coordonate::from(-10i32), Coordonate::Absolute(-10));
    assert_eq!(Coordonate::from(-10i64), Coordonate::Absolute(-10));
    assert_eq!(Coordonate::from(1.25f32), Coordonate::Percentage(1.25));
    assert_eq!(Coordonate::from(-1.25f64), Coordonate::Percentage(-1.25));
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
fn layout_mode_anchor_trb() {
    validate_pos!("r:5,t:6,b:7,w:10", 50, 30, 35, 6, 10, 17);
    validate_pos!("r:10%,t:6,b:7,w:20%", 50, 30, 35, 6, 10, 17);
    validate_pos!("r:5,t:3,b:3,w:10", 50, 30, 35, 3, 10, 24);
    validate_pos!("r:5,t:10%,b:10%,w:10", 50, 30, 35, 3, 10, 24);
    validate_pos!("r:10%,t:10%,b:10%,w:20%", 50, 30, 35, 3, 10, 24);
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
    validate_pos!("a:c,w:100%,h:100%,l:1", 50, 30, 5, 0, 38, 10);
}
#[test]
#[should_panic]
fn layout_mode_dock_dont_allow_right() {
    // this code should panic because 'r' can not be used in a Top-Left-Right layout mode
    validate_pos!("a:c,w:100%,h:100%,r:1", 50, 30, 5, 0, 38, 10);
}
#[test]
#[should_panic]
fn layout_mode_dock_dont_allow_top() {
    // this code should panic because 'lt' can not be used in a Top-Left-Right layout mode
    validate_pos!("a:c,w:100%,h:100%,t:1", 50, 30, 5, 0, 38, 10);
}
#[test]
#[should_panic]
fn layout_mode_dock_dont_allow_bottom() {
    // this code should panic because 'b' can not be used in a Top-Left-Right layout mode
    validate_pos!("a:c,w:100%,h:100%,b:1", 50, 30, 5, 0, 38, 10);
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
fn check_dimension_is_absolute() {
    assert!(Dimension::Absolute(10).is_absolute());
    assert!(!Dimension::Percentage(0.5f32).is_absolute());
    assert!(!Dimension::Percentage(1.0f32).is_absolute());
    assert!(Dimension::Absolute(0).is_absolute());
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
