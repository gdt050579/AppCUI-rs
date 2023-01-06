use super::layout_mode::PointAndSizeLayout;
use super::Coordonate;
use super::Size;
use super::Alignament;

use super::LayoutMode;

#[test]
fn xywh_layout() {
    let l = LayoutMode::new("x:1,y:1,w:10,h:8");
    assert_eq!(l,LayoutMode::PointAndSize(PointAndSizeLayout{
        x:Coordonate::Absolute(1),
        y: Coordonate::Absolute(1),
        align: Alignament::TopLeft,
        anchor: Alignament::TopLeft,
        width: Size::Absolute(10),
        height: Size::Absolute(8)
    }));
}
