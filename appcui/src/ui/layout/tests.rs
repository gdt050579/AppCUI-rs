use crate::ui::layout::absolute_layout::AbsoluteLayout;
use crate::ui::layout::Coordonate;

use super::Alignament;
use super::Anchors;
use super::ControlLayout;
use super::Coordonate16;
use super::Dimension;
use super::Dimension16;
use super::Dock;
use super::LayoutBuilder;
use super::PointAndSizeLayout;
use super::Layout;

use super::LayoutMode;

macro_rules! validate_abs {
    ($text:literal, $x:expr,$y:expr,$w:expr,$h:expr,$a:tt,$anc:tt) => {
        assert_eq!(
            LayoutMode::new($text),
            LayoutMode::PointAndSize(PointAndSizeLayout {
                x: Coordonate16::Absolute($x),
                y: Coordonate16::Absolute($y),
                align: Alignament::$a,
                anchor: Alignament::$anc,
                width: Dimension16::Absolute($w),
                height: Dimension16::Absolute($h)
            })
        );
    };
}

macro_rules! validate_pos {
    ($text:literal, $parent_width:expr, $parent_height:expr, $x:expr,$y:expr,$w:expr,$h:expr) => {
        let mut cl = ControlLayout::from(Layout::new($text));
        cl.update($parent_width, $parent_height);
        assert_eq!(cl.get_width(), $w);
        assert_eq!(cl.get_height(), $h);
        assert_eq!(cl.get_x(), $x);
        assert_eq!(cl.get_y(), $y);
    };
}

#[test]
fn layout_mode_xywh() {
    let l1 = LayoutMode::new("x:1,y:1,w:10,h:8");
    assert_eq!(
        l1,
        LayoutMode::Absolute(AbsoluteLayout {
            x: 1,
            y: 1,
            width: 10,
            height: 8
        })
    );
    let l2 = LayoutMode::new("x:-4,y:10%,w:10%,h:8");
    assert_eq!(
        l2,
        LayoutMode::PointAndSize(PointAndSizeLayout {
            x: Coordonate16::Absolute(-4),
            y: Coordonate16::Percentage(1000),
            align: Alignament::TopLeft,
            anchor: Alignament::TopLeft,
            width: Dimension16::Percentage(1000),
            height: Dimension16::Absolute(8)
        })
    );
    let l3 = LayoutMode::new("x:0,y:0,w:100%,h:25%,a:c");
    assert_eq!(
        l3,
        LayoutMode::PointAndSize(PointAndSizeLayout {
            x: Coordonate16::Absolute(0),
            y: Coordonate16::Absolute(0),
            align: Alignament::Center,
            anchor: Alignament::TopLeft,
            width: Dimension16::Percentage(10000),
            height: Dimension16::Percentage(2500)
        })
    );
}
#[test]
fn layout_mode_align_center() {
    validate_abs!("x:0,y:0,w:12,h:15,a:center", 0, 0, 12, 15, Center, TopLeft);
    validate_abs!("x:0,y:0,w:12,h:15,a:c", 0, 0, 12, 15, Center, TopLeft);
    validate_abs!("x:0,y:0,w:12,h:15,align:c", 0, 0, 12, 15, Center, TopLeft);
    validate_abs!("x:0,y:0,w:12,h:15,align:center", 0, 0, 12, 15, Center, TopLeft);
}
#[test]
fn layout_mode_align_top_left() {
    validate_abs!("x:0,y:0,w:12,h:15,a:tl", 0, 0, 12, 15, TopLeft, TopLeft);
    validate_abs!("x:0,y:0,w:12,h:15,a:lt", 0, 0, 12, 15, TopLeft, TopLeft);
    validate_abs!("x:0,y:0,w:12,h:15,align:topleft", 0, 0, 12, 15, TopLeft, TopLeft);
    validate_abs!("x:0,y:0,w:12,h:15,align:lefttop", 0, 0, 12, 15, TopLeft, TopLeft);
}
#[test]
fn layout_mode_align_top_right() {
    validate_abs!("x:1,y:0,w:12,h:15,a:tr", 1, 0, 12, 15, TopRight, TopLeft);
    validate_abs!("x:2,y:0,w:12,h:15,a:rt", 2, 0, 12, 15, TopRight, TopLeft);
    validate_abs!("x:3,y:0,w:12,h:15,align:topright", 3, 0, 12, 15, TopRight, TopLeft);
    validate_abs!("x:4,y:0,w:12,h:15,align:righttop", 4, 0, 12, 15, TopRight, TopLeft);
}
#[test]
fn layout_mode_align_bottom_right() {
    validate_abs!("x:1,y:-2,w:12,h:15,a:br", 1, -2, 12, 15, BottomRight, TopLeft);
    validate_abs!("x:2,y:-1,w:12,h:15,a:rb", 2, -1, 12, 15, BottomRight, TopLeft);
    validate_abs!("x:3,y: 1,w:12,h:15,align:bottomright", 3, 1, 12, 15, BottomRight, TopLeft);
    validate_abs!("x:4,y: 2,w:12,h:15,align:rightbottom", 4, 2, 12, 15, BottomRight, TopLeft);
}
#[test]
fn layout_mode_align_bottom_left() {
    validate_abs!("x:1,y:-2,w:12,h:15,a:bl", 1, -2, 12, 15, BottomLeft, TopLeft);
    validate_abs!("x:2,y:-1,w:12,h:15,a:lb", 2, -1, 12, 15, BottomLeft, TopLeft);
    validate_abs!("x:3,y: 1,w:12,h:15,align:bottomleft", 3, 1, 12, 15, BottomLeft, TopLeft);
    validate_abs!("x:4,y: 2,w:12,h:15,align:leftbottom", 4, 2, 12, 15, BottomLeft, TopLeft);
}
#[test]
fn layout_mode_align_left() {
    validate_abs!("w:12,h:15,x:1,y:-2,a:l", 1, -2, 12, 15, Left, TopLeft);
    validate_abs!("y: 1,x:3,w:12,h:15,a:left", 3, 1, 12, 15, Left, TopLeft);
    validate_abs!("x:4,w:12,y: 2,h:15,align:left", 4, 2, 12, 15, Left, TopLeft);
}
#[test]
fn layout_mode_align_right() {
    validate_abs!("w:12,h:15,x:1,y:-2,a:r", 1, -2, 12, 15, Right, TopLeft);
    validate_abs!("y: 1,x:3,width:12,height:15,a:right", 3, 1, 12, 15, Right, TopLeft);
    validate_abs!("x:4,w:12,y: 2,h:15,align:right", 4, 2, 12, 15, Right, TopLeft);
}
#[test]
fn layout_mode_align_top() {
    validate_abs!("w:12,h:15,x:1,y:-2,a:t", 1, -2, 12, 15, Top, TopLeft);
    validate_abs!("y: 1,x:3,width:12,height:15,a:top", 3, 1, 12, 15, Top, TopLeft);
    validate_abs!("x:4,w:12,y: 2,h:15,align:top", 4, 2, 12, 15, Top, TopLeft);
}
#[test]
fn layout_mode_align_bottom() {
    validate_abs!("width:12,h:15,x:1,y:-2,a:b", 1, -2, 12, 15, Bottom, TopLeft);
    validate_abs!("y: 1,x:3,width:12,height:15,a:bottom", 3, 1, 12, 15, Bottom, TopLeft);
    validate_abs!("x:4,w:12,y: 2,HEIGHT:15,align:bottom", 4, 2, 12, 15, Bottom, TopLeft);
}
#[test]
fn layout_mode_dock_center() {
    validate_pos!("d:c", 50, 30, 0, 0, 50, 30);
    validate_pos!("d:center,w:20,h:10", 50, 30, 15, 10, 20, 10);
    validate_pos!("d:Center,w:20", 50, 30, 15, 0, 20, 30);
    validate_pos!("d:CENTER,h:10", 50, 30, 0, 10, 50, 10);
    validate_pos!("d:cEnTeR,w:50%,h:25%", 60, 40, 15, 15, 30, 10);
}
#[test]
fn layout_mode_dock_top_left() {
    validate_pos!("d:tl", 50, 30, 0, 0, 50, 30);
    validate_pos!("d:lt,w:20,h:10", 50, 30, 0, 0, 20, 10);
    validate_pos!("d:topleft,w:20", 50, 30, 0, 0, 20, 30);
    validate_pos!("d:lefttop,h:10", 50, 30, 0, 0, 50, 10);
    validate_pos!("d:TopLeft,w:50%,h:25%", 60, 40, 0, 0, 30, 10);
}
#[test]
fn layout_mode_dock_bottom_left() {
    validate_pos!("d:lb", 50, 30, 0, 0, 50, 30);
    validate_pos!("d:bl,w:20,h:10", 50, 30, 0, 20, 20, 10);
    validate_pos!("d:lb,w:20", 50, 30, 0, 0, 20, 30);
    validate_pos!("d:bottomleft,h:10", 50, 30, 0, 20, 50, 10);
    validate_pos!("d:leftbottom,w:50%,h:25%", 60, 40, 0, 30, 30, 10);
}
#[test]
fn layout_mode_dock_bottom_right() {
    validate_pos!("d:rb", 50, 30, 0, 0, 50, 30);
    validate_pos!("d:br,w:20,h:10", 50, 30, 30, 20, 20, 10);
    validate_pos!("d:rb,w:20", 50, 30, 30, 0, 20, 30);
    validate_pos!("d:bottomright,h:10", 50, 30, 0, 20, 50, 10);
    validate_pos!("d:rightbottom,w:50%,h:25%", 60, 40, 30, 30, 30, 10);
}
#[test]
fn layout_mode_dock_top_right() {
    validate_pos!("d:tr", 50, 30, 0, 0, 50, 30);
    validate_pos!("d:rt,w:20,h:10", 50, 30, 30, 0, 20, 10);
    validate_pos!("d:TopRight,w:20", 50, 30, 30, 0, 20, 30);
    validate_pos!("d:rightTop,h:10", 50, 30, 0, 0, 50, 10);
    validate_pos!("d:topright,w:50%,h:25%", 60, 40, 30, 0, 30, 10);
}
#[test]
fn layout_mode_dock_left() {
    validate_pos!("d:l", 50, 30, 0, 0, 50, 30);
    validate_pos!("d:left,w:20,h:10", 50, 30, 0, 10, 20, 10);
    validate_pos!("d:Left,w:20", 50, 30, 00, 0, 20, 30);
    validate_pos!("d:l,h:10", 50, 30, 0, 10, 50, 10);
    validate_pos!("d:LeFt,w:50%,h:25%", 60, 40, 0, 15, 30, 10);
}

#[test]
fn layout_mode_dock_top() {
    validate_pos!("d:t", 50, 30, 0, 0, 50, 30);
    validate_pos!("d:top,w:20,h:10", 50, 30, 15, 0, 20, 10);
    validate_pos!("d:Top,w:20", 50, 30, 15, 0, 20, 30);
    validate_pos!("d:t,h:10", 50, 30, 0, 0, 50, 10);
    validate_pos!("d:ToP,w:50%,h:25%", 60, 40, 15, 0, 30, 10);
}

#[test]
fn layout_mode_dock_right() {
    validate_pos!("d:r", 50, 30, 0, 0, 50, 30);
    validate_pos!("d:Right,w:20,h:10", 50, 30, 30, 10, 20, 10);
    validate_pos!("d:right,w:20", 50, 30, 30, 0, 20, 30);
    validate_pos!("d:r,h:10", 50, 30, 0, 10, 50, 10);
    validate_pos!("d:rIgHt,w:50%,h:25%", 60, 40, 30, 15, 30, 10);
}
#[test]
fn layout_mode_dock_bottom() {
    validate_pos!("d:b", 50, 30, 0, 0, 50, 30);
    validate_pos!("d:bottom,w:20,h:10", 50, 30, 15, 20, 20, 10);
    validate_pos!("d:Bottom,w:20", 50, 30, 15, 0, 20, 30);
    validate_pos!("d:b,h:10", 50, 30, 0, 20, 50, 10);
    validate_pos!("d:BoTtOm,w:50%,h:25%", 60, 40, 15, 30, 30, 10);
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
    validate_pos!("l:5,r:7,y:0,h:10,a:t", 50, 30, 5, 0, 38, 10);
    validate_pos!("l:5,r:7,y:10,h:10,a:c", 50, 30, 5, 5, 38, 10);
    validate_pos!("l:5,r:7,y:20,h:10,a:b", 50, 30, 5, 10, 38, 10);
    // no alignament - default is center
    validate_pos!("l:5,r:7,y:0,h:10", 50, 30, 5, -5, 38, 10);

    validate_pos!("l:10%,a:t,y:50%,r:20%,h:4", 50, 30, 5, 15, 35, 4);
    validate_pos!("l:10%,a:c,y:50%,r:20%,h:4", 50, 30, 5, 13, 35, 4);
    validate_pos!("l:10%,a:b,y:50%,r:20%,h:4", 50, 30, 5, 11, 35, 4);

    validate_pos!("l:10%,a:t,y:50%,r:20%,h:50%", 50, 30, 5, 15, 35, 15);
    validate_pos!("l:10%,a:c,y:50%,r:20%,h:50%", 50, 30, 5, 8, 35, 15);
    validate_pos!("l:10%,a:b,y:50%,r:20%,h:50%", 50, 30, 5, 0, 35, 15);
}

#[test]
#[should_panic]
fn layout_mode_anchor_lr_dont_allow_x() {
    // this code should panic because 'x' can not be used in a Left-Right layout mode
    validate_pos!("l:5,r:7,y:0,h:10,a:t,x:10", 50, 30, 5, 0, 38, 10);
}
#[test]
#[should_panic]
fn layout_mode_anchor_lr_invalid_alignament() {
    // this code should panic because only (top,bottom and center) alignaments can not be used in a Left-Right layout mode
    validate_pos!("l:5,r:7,y:0,h:10,a:left", 50, 30, 5, 0, 38, 10);
}

#[test]
fn layout_mode_anchor_tb() {
    validate_pos!("t:5,b:7,x:0,w:10,a:l", 30, 50, 0, 5, 10, 38);
    validate_pos!("t:5,b:7,x:10,w:10,a:c", 30, 50, 5, 5, 10, 38);
    validate_pos!("t:5,b:7,x:20,w:10,a:r", 30, 50, 10, 5, 10, 38);
    // no alignament - default is center
    validate_pos!("t:5,b:7,x:0,w:10", 30, 50, -5, 5, 10, 38);

    validate_pos!("t:10%,a:l,x:50%,b:20%,w:4", 30, 50, 15, 5, 4, 35);
    validate_pos!("t:10%,a:c,x:50%,b:20%,w:4", 30, 50, 13, 5, 4, 35);
    validate_pos!("t:10%,a:r,x:50%,b:20%,w:4", 30, 50, 11, 5, 4, 35);

    validate_pos!("t:10%,a:l,x:50%,b:20%,w:50%", 30, 50, 15, 5, 15, 35);
    validate_pos!("t:10%,a:c,x:50%,b:20%,w:50%", 30, 50, 8, 5, 15, 35);
    validate_pos!("t:10%,a:r,x:50%,b:20%,w:50%", 30, 50, 0, 5, 15, 35);
}

#[test]
#[should_panic]
fn layout_mode_anchor_tb_dont_allow_y() {
    // this code should panic because 'y' can not be used in a Top-Down layout mode
    validate_pos!("t:5,b:7,y:0,w:10,a:l", 30, 50, 0, 5, 10, 38);
}
#[test]
#[should_panic]
fn layout_mode_anchor_td_invalid_alignament() {
    // this code should panic because only (left,right and center) alignaments can not be used in a Top-Down layout mode
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
    validate_pos!("d:c,w:100%,h:100%,l:1", 50, 30, 5, 0, 38, 10);
}
#[test]
#[should_panic]
fn layout_mode_dock_dont_allow_right() {
    // this code should panic because 'r' can not be used in a Top-Left-Right layout mode
    validate_pos!("d:c,w:100%,h:100%,r:1", 50, 30, 5, 0, 38, 10);
}
#[test]
#[should_panic]
fn layout_mode_dock_dont_allow_top() {
    // this code should panic because 'lt' can not be used in a Top-Left-Right layout mode
    validate_pos!("d:c,w:100%,h:100%,t:1", 50, 30, 5, 0, 38, 10);
}
#[test]
#[should_panic]
fn layout_mode_dock_dont_allow_bottom() {
    // this code should panic because 'b' can not be used in a Top-Left-Right layout mode
    validate_pos!("d:c,w:100%,h:100%,b:1", 50, 30, 5, 0, 38, 10);
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
    validate_pos!("d:blablabla", 50, 30, 5, 0, 38, 10);
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
    let mut c = Coordonate::Absolute(0);
    c.update_with_absolute_value(10, 20);
    assert_eq!(c, Coordonate::Absolute(10));

    let mut c = Coordonate::Percentage(0.0f32);
    c.update_with_absolute_value(10, 20);
    assert_eq!(c, Coordonate::Percentage(0.5f32));
}

#[test]
fn check_layout_builder() {
    let lb = LayoutBuilder::new()
        .x(100)
        .y(1.25)
        .width(100)
        .height(1.0)
        .alignament(Alignament::Center)
        .dock(Dock::TopLeft)
        .left_anchor(0.5)
        .right_anchor(0.25)
        .top_anchor(5)
        .bottom_anchor(7);
    assert_eq!(lb.params.x, Some(Coordonate16::Absolute(100)));
    assert_eq!(lb.params.y, Some(Coordonate16::Percentage(12500)));
    assert_eq!(lb.params.width, Some(Dimension16::Absolute(100)));
    assert_eq!(lb.params.height, Some(Dimension16::Percentage(10000)));
    assert_eq!(lb.params.align, Some(Alignament::Center));
    assert_eq!(lb.params.dock, Some(Dock::TopLeft));
    assert_eq!(lb.params.a_left, Some(Coordonate16::Percentage(5000)));
    assert_eq!(lb.params.a_right, Some(Coordonate16::Percentage(2500)));
    assert_eq!(lb.params.a_top, Some(Coordonate16::Absolute(5)));
    assert_eq!(lb.params.a_bottom, Some(Coordonate16::Absolute(7)));
}
