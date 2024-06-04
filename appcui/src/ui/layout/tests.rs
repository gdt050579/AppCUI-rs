use crate::ui::layout::absolute_layout::AbsoluteLayout;

use super::Alignament;
use super::ControlLayout;
use super::Coordonate;
use super::PointAndSizeLayout;
use super::Dimension;

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
                width: Dimension::Absolute($w),
                height: Dimension::Absolute($h)
            })
        );
    };
}

macro_rules! validate_pos {
    ($text:literal, $parent_width:expr, $parent_height:expr, $x:expr,$y:expr,$w:expr,$h:expr) => {
        let mut cl = ControlLayout::new($text);
        cl.update($parent_width, $parent_height);
        assert_eq!(cl.get_width(),$w);
        assert_eq!(cl.get_height(),$h);
        assert_eq!(cl.get_x(),$x);
        assert_eq!(cl.get_y(),$y);
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
            width: Dimension::Percentage(1000),
            height: Dimension::Absolute(8)
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
            width: Dimension::Percentage(10000),
            height: Dimension::Percentage(2500)
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
    validate_pos!("d:c",50,30,0,0,50,30);
    validate_pos!("d:center,w:20,h:10",50,30,15,10,20,10);
    validate_pos!("d:Center,w:20",50,30,15,0,20,30);    
    validate_pos!("d:CENTER,h:10",50,30,0,10,50,10);
    validate_pos!("d:cEnTeR,w:50%,h:25%",60,40,15,15,30,10);
}
#[test]
fn layout_mode_dock_top_left() {
    validate_pos!("d:tl",50,30,0,0,50,30);
    validate_pos!("d:lt,w:20,h:10",50,30,0,0,20,10);
    validate_pos!("d:topleft,w:20",50,30,0,0,20,30);    
    validate_pos!("d:lefttop,h:10",50,30,0,0,50,10);
    validate_pos!("d:TopLeft,w:50%,h:25%",60,40,0,0,30,10);
}
#[test]
fn layout_mode_dock_bottom_left() {
    validate_pos!("d:lb",50,30,0,0,50,30);
    validate_pos!("d:bl,w:20,h:10",50,30,0,20,20,10);
    validate_pos!("d:lb,w:20",50,30,0,0,20,30);    
    validate_pos!("d:bottomleft,h:10",50,30,0,20,50,10);
    validate_pos!("d:leftbottom,w:50%,h:25%",60,40,0,30,30,10);
}
#[test]
fn layout_mode_dock_bottom_right() {
    validate_pos!("d:rb",50,30,0,0,50,30);
    validate_pos!("d:br,w:20,h:10",50,30,30,20,20,10);
    validate_pos!("d:rb,w:20",50,30,30,0,20,30);    
    validate_pos!("d:bottomright,h:10",50,30,0,20,50,10);
    validate_pos!("d:rightbottom,w:50%,h:25%",60,40,30,30,30,10);
}
#[test]
fn layout_mode_dock_top_right() {
    validate_pos!("d:tr",50,30,0,0,50,30);
    validate_pos!("d:rt,w:20,h:10",50,30,30,0,20,10);
    validate_pos!("d:TopRight,w:20",50,30,30,0,20,30);    
    validate_pos!("d:rightTop,h:10",50,30,0,0,50,10);
    validate_pos!("d:topright,w:50%,h:25%",60,40,30,0,30,10);
}
#[test]
fn layout_mode_dock_left() {
    validate_pos!("d:l",50,30,0,0,50,30);
    validate_pos!("d:left,w:20,h:10",50,30,0,10,20,10);
    validate_pos!("d:Left,w:20",50,30,00,0,20,30);    
    validate_pos!("d:l,h:10",50,30,0,10,50,10);
    validate_pos!("d:LeFt,w:50%,h:25%",60,40,0,15,30,10);
}

#[test]
fn layout_mode_dock_top() {
    validate_pos!("d:t",50,30,0,0,50,30);
    validate_pos!("d:top,w:20,h:10",50,30,15,0,20,10);
    validate_pos!("d:Top,w:20",50,30,15,0,20,30);    
    validate_pos!("d:t,h:10",50,30,0,0,50,10);
    validate_pos!("d:ToP,w:50%,h:25%",60,40,15,0,30,10);
}

#[test]
fn layout_mode_dock_right() {
    validate_pos!("d:r",50,30,0,0,50,30);
    validate_pos!("d:Right,w:20,h:10",50,30,30,10,20,10);
    validate_pos!("d:right,w:20",50,30,30,0,20,30);    
    validate_pos!("d:r,h:10",50,30,0,10,50,10);
    validate_pos!("d:rIgHt,w:50%,h:25%",60,40,30,15,30,10);
}
#[test]
fn layout_mode_dock_bottom() {
    validate_pos!("d:b",50,30,0,0,50,30);
    validate_pos!("d:bottom,w:20,h:10",50,30,15,20,20,10);
    validate_pos!("d:Bottom,w:20",50,30,15,0,20,30);    
    validate_pos!("d:b,h:10",50,30,0,20,50,10);
    validate_pos!("d:BoTtOm,w:50%,h:25%",60,40,15,30,30,10);
}
#[test]
fn layout_mode_anchor_lrtb() {
    validate_pos!("l:5,t:6,r:7,b:8",50,30,5,6,38,16);
    validate_pos!("left:5, top:6,  right:7,  bottom:8",50,30,5,6,38,16);
    validate_pos!("l:10%,t:50%,r:20%,b:10%",50,30,5,15,35,12);
}
#[test]
fn layout_mode_anchor_ltr() {
    validate_pos!("l:5,t:6,r:7,h:10",50,30,5,6,38,10);
    validate_pos!("l:10%,t:50%,r:20%,h:2",50,30,5,15,35,2);
    validate_pos!("l:10%,t:50%,r:20%,h:50%",50,30,5,15,35,15);
}
#[test]
fn layout_mode_anchor_lbr() {
    validate_pos!("l:5,b:6,r:7,h:10",50,30,5,14,38,10);
    validate_pos!("l:10%,b:50%,r:20%,h:2",50,30,5,13,35,2);
    validate_pos!("l:10%,bottom:50%,r:20%,h:50%",50,30,5,0,35,15);
}

#[test]
fn dimension_from_basic_type() {
    assert_eq!(Dimension::from(10u8), Dimension::Absolute(10));
    assert_eq!(Dimension::from(10u16), Dimension::Absolute(10));
    assert_eq!(Dimension::from(10u32), Dimension::Absolute(10));
    assert_eq!(Dimension::from(10u64), Dimension::Absolute(10));
}