use super::layout_mode::PointAndSizeLayout;
use super::Alignament;
use super::Coordonate;
use super::Size;

use super::LayoutMode;

#[test]
fn layout_mode_xywh() {
    let l1 = LayoutMode::new("x:1,y:1,w:10,h:8");
    assert_eq!(
        l1,
        LayoutMode::PointAndSize(PointAndSizeLayout {
            x: Coordonate::Absolute(1),
            y: Coordonate::Absolute(1),
            align: Alignament::TopLeft,
            anchor: Alignament::TopLeft,
            width: Size::Absolute(10),
            height: Size::Absolute(8)
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
}
#[test]
fn layout_mode_align_center() {
    let l = LayoutMode::new("x:0,y:0,w:100%,h:25%,a:c");
    assert_eq!(
        l,
        LayoutMode::PointAndSize(PointAndSizeLayout {
            x: Coordonate::Absolute(0),
            y: Coordonate::Absolute(0),
            align: Alignament::Center,
            anchor: Alignament::TopLeft,
            width: Size::Percentage(10000),
            height: Size::Percentage(2500)
        })
    );
    let l = LayoutMode::new("x:0,y:0,w:100%,h:25%,a:center");
    assert_eq!(
        l,
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
fn layout_mode_align_top() {
    let l = LayoutMode::new("x:0,y:0,w:100%,h:25%,a:t");
    assert_eq!(
        l,
        LayoutMode::PointAndSize(PointAndSizeLayout {
            x: Coordonate::Absolute(0),
            y: Coordonate::Absolute(0),
            align: Alignament::Top,
            anchor: Alignament::TopLeft,
            width: Size::Percentage(10000),
            height: Size::Percentage(2500)
        })
    );
    let l = LayoutMode::new("x:0,y:0,w:100%,h:25%,a:top");
    assert_eq!(
        l,
        LayoutMode::PointAndSize(PointAndSizeLayout {
            x: Coordonate::Absolute(0),
            y: Coordonate::Absolute(0),
            align: Alignament::Top,
            anchor: Alignament::TopLeft,
            width: Size::Percentage(10000),
            height: Size::Percentage(2500)
        })
    );
}
