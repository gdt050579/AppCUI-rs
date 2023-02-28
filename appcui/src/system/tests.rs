use super::Theme;
use super::ToolTip;
use crate::graphics::CharFlags;
use crate::graphics::Character;
use crate::graphics::Color;
use crate::graphics::Rect;
use crate::graphics::SurfaceTester;

#[test]
fn check_tooltip_single_line() {
    let mut tooltip = ToolTip::new();
    let r = Rect::new(2, 2, 10, 4);
    let theme = Theme::new();
    let mut s = SurfaceTester::new(40,6);
    tooltip.show(
        "A simple tooltip",
        &r,
        s.get_width(),
        s.get_height(),
        &theme,
    );
    s.clear(Character::new(
        ' ',
        Color::White,
        Color::Black,
        CharFlags::None,
    ));
    s.fill_rect(r, Character::new('X',Color::White,Color::DarkRed,CharFlags::None));
    tooltip.paint(&mut s, &theme);
    //s.print();
    assert_eq!(s.compute_hash(),0xA18B870B1B5423F6);
}

#[test]
fn check_tooltip_multi_line() {
    let mut tooltip = ToolTip::new();
    let r = Rect::new(2, 3, 10, 5);
    let theme = Theme::new();
    let mut s = SurfaceTester::new(40,10);
    tooltip.show(
        "A multi-line tooltip\nto show case",
        &r,
        s.get_width(),
        s.get_height(),
        &theme,
    );
    s.clear(Character::new(
        ' ',
        Color::White,
        Color::Black,
        CharFlags::None,
    ));
    s.fill_rect(r, Character::new('X',Color::White,Color::DarkRed,CharFlags::None));
    tooltip.paint(&mut s, &theme);
    //s.print();
    assert_eq!(s.compute_hash(),0x737C188B334A13C2);
}

#[test]
fn check_tooltip_multi_line_2() {
    let mut tooltip = ToolTip::new();
    let r = Rect::new(2, 4, 10, 5);
    let theme = Theme::new();
    let mut s = SurfaceTester::new(40,15);
    tooltip.show(
        "A multi-line tooltip to show case in this example",
        &r,
        s.get_width(),
        s.get_height(),
        &theme,
    );
    s.clear(Character::new(
        ' ',
        Color::White,
        Color::Black,
        CharFlags::None,
    ));
    s.fill_rect(r, Character::new('X',Color::White,Color::DarkRed,CharFlags::None));
    tooltip.paint(&mut s, &theme);
    //s.print();
    assert_eq!(s.compute_hash(),0x8E67370E48B93A77);
}