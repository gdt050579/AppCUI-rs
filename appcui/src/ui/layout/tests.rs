use crate::ui::layout::absolute_layout::AbsoluteLayout;

use super::Alignament;
use super::Coordonate;
use super::PointAndSizeLayout;
use super::Size;

use super::LayoutMode;

macro_rules! validate_abs {
    ($text:literal, $x:expr,$y:expr,$w:expr,$h:expr,$a:tt,$anc:tt) => {
        assert_eq!(
            LayoutMode::new($text),
            LayoutMode::PointAndSize(PointAndSizeLayout {
                x: Coordonate::Absolute($x),
                y: Coordonate::Absolute($y),
                align: Alignament::$a,
                anchor: Alignament::$anc,
                width: Size::Absolute($w),
                height: Size::Absolute($h)
            })
        );
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
            x: Coordonate::Absolute(-4),
            y: Coordonate::Percentage(1000),
            align: Alignament::TopLeft,
            anchor: Alignament::TopLeft,
            width: Size::Percentage(1000),
            height: Size::Absolute(8)
        })
    );
    let l3 = LayoutMode::new("x:0,y:0,w:100%,h:25%,a:c");
    assert_eq!(
        l3,
        LayoutMode::PointAndSize(PointAndSizeLayout {
            x: Coordonate::Absolute(0),
            y: Coordonate::Absolute(0),
            align: Alignament::Center,
            anchor: Alignament::TopLeft,
            width: Size::Percentage(10000),
            height: Size::Percentage(2500)
        })
    );
}
#[test]
fn layout_mode_align_center() {
    validate_abs!("x:0,y:0,w:12,h:15,a:center", 0, 0, 12, 15, Center, TopLeft);
    validate_abs!("x:0,y:0,w:12,h:15,a:c", 0, 0, 12, 15, Center, TopLeft);
    validate_abs!("x:0,y:0,w:12,h:15,align:c", 0, 0, 12, 15, Center, TopLeft);
    validate_abs!(
        "x:0,y:0,w:12,h:15,align:center",
        0,
        0,
        12,
        15,
        Center,
        TopLeft
    );
}
#[test]
fn layout_mode_align_top_left() {
    validate_abs!("x:0,y:0,w:12,h:15,a:tl", 0, 0, 12, 15, TopLeft, TopLeft);
    validate_abs!("x:0,y:0,w:12,h:15,a:lt", 0, 0, 12, 15, TopLeft, TopLeft);
    validate_abs!(
        "x:0,y:0,w:12,h:15,align:topleft",
        0,
        0,
        12,
        15,
        TopLeft,
        TopLeft
    );
    validate_abs!(
        "x:0,y:0,w:12,h:15,align:lefttop",
        0,
        0,
        12,
        15,
        TopLeft,
        TopLeft
    );
}
#[test]
fn layout_mode_align_top_right() {
    validate_abs!("x:1,y:0,w:12,h:15,a:tr", 1, 0, 12, 15, TopRight, TopLeft);
    validate_abs!("x:2,y:0,w:12,h:15,a:rt", 2, 0, 12, 15, TopRight, TopLeft);
    validate_abs!(
        "x:3,y:0,w:12,h:15,align:topright",
        3,
        0,
        12,
        15,
        TopRight,
        TopLeft
    );
    validate_abs!(
        "x:4,y:0,w:12,h:15,align:righttop",
        4,
        0,
        12,
        15,
        TopRight,
        TopLeft
    );
}
#[test]
fn layout_mode_align_bottom_right() {
    validate_abs!(
        "x:1,y:-2,w:12,h:15,a:br",
        1,
        -2,
        12,
        15,
        BottomRight,
        TopLeft
    );
    validate_abs!(
        "x:2,y:-1,w:12,h:15,a:rb",
        2,
        -1,
        12,
        15,
        BottomRight,
        TopLeft
    );
    validate_abs!(
        "x:3,y: 1,w:12,h:15,align:bottomright",
        3,
        1,
        12,
        15,
        BottomRight,
        TopLeft
    );
    validate_abs!(
        "x:4,y: 2,w:12,h:15,align:rightbottom",
        4,
        2,
        12,
        15,
        BottomRight,
        TopLeft
    );
}
#[test]
fn layout_mode_align_bottom_left() {
    validate_abs!(
        "x:1,y:-2,w:12,h:15,a:bl",
        1,
        -2,
        12,
        15,
        BottomLeft,
        TopLeft
    );
    validate_abs!(
        "x:2,y:-1,w:12,h:15,a:lb",
        2,
        -1,
        12,
        15,
        BottomLeft,
        TopLeft
    );
    validate_abs!(
        "x:3,y: 1,w:12,h:15,align:bottomleft",
        3,
        1,
        12,
        15,
        BottomLeft,
        TopLeft
    );
    validate_abs!(
        "x:4,y: 2,w:12,h:15,align:leftbottom",
        4,
        2,
        12,
        15,
        BottomLeft,
        TopLeft
    );
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
    validate_abs!(
        "y: 1,x:3,width:12,height:15,a:right",
        3,
        1,
        12,
        15,
        Right,
        TopLeft
    );
    validate_abs!(
        "x:4,w:12,y: 2,h:15,align:right",
        4,
        2,
        12,
        15,
        Right,
        TopLeft
    );
}
#[test]
fn layout_mode_align_top() {
    validate_abs!("w:12,h:15,x:1,y:-2,a:t", 1, -2, 12, 15, Top, TopLeft);
    validate_abs!(
        "y: 1,x:3,width:12,height:15,a:top",
        3,
        1,
        12,
        15,
        Top,
        TopLeft
    );
    validate_abs!("x:4,w:12,y: 2,h:15,align:top", 4, 2, 12, 15, Top, TopLeft);
}
#[test]
fn layout_mode_align_bottom() {
    validate_abs!("width:12,h:15,x:1,y:-2,a:b", 1, -2, 12, 15, Bottom, TopLeft);
    validate_abs!(
        "y: 1,x:3,width:12,height:15,a:bottom",
        3,
        1,
        12,
        15,
        Bottom,
        TopLeft
    );
    validate_abs!(
        "x:4,w:12,y: 2,HEIGHT:15,align:bottom",
        4,
        2,
        12,
        15,
        Bottom,
        TopLeft
    );
}
