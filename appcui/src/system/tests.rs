use super::CommandBar;
use super::Theme;
use super::ToolTip;
use crate::graphics::CharFlags;
use crate::graphics::Character;
use crate::graphics::Color;
use crate::graphics::Rect;
use crate::graphics::Size;
use crate::graphics::SurfaceTester;
use crate::input::Key;
use crate::input::KeyCode;
use crate::input::KeyModifier;

fn draw_tool_tip(size: Size, rect: Rect, txt: &str) -> SurfaceTester {
    let mut tooltip = ToolTip::new();
    let theme = Theme::new();
    let mut s = SurfaceTester::new(size.width, size.height);
    tooltip.show(txt, &rect, s.get_width(), s.get_height(), &theme);
    s.clear(Character::new(
        ' ',
        Color::White,
        Color::Black,
        CharFlags::None,
    ));
    s.fill_rect(
        rect,
        Character::new('X', Color::White, Color::DarkRed, CharFlags::None),
    );
    tooltip.paint(&mut s, &theme);
    s
}

#[test]
fn check_tooltip_single_line() {
    let s = draw_tool_tip(Size::new(40, 6), Rect::new(2, 2, 10, 4), "A simple tooltip");
    //s.print();
    assert_eq!(s.compute_hash(), 0xA18B870B1B5423F6);
}

#[test]
fn check_tooltip_multi_line() {
    let s = draw_tool_tip(
        Size::new(40, 10),
        Rect::new(2, 3, 10, 5),
        "A multi-line tooltip\nto show case",
    );
    //s.print();
    assert_eq!(s.compute_hash(), 0x737C188B334A13C2);
}
#[test]
fn check_tooltip_multi_line_2() {
    let s = draw_tool_tip(
        Size::new(40, 15),
        Rect::new(2, 4, 10, 5),
        "A multi-line tooltip to show case in this example",
    );
    //s.print();
    assert_eq!(s.compute_hash(), 0x8E67370E48B93A77);
}
#[test]
fn check_tooltip_multi_line_3() {
    let s = draw_tool_tip(
        Size::new(40, 15),
        Rect::new(0, 4, 5, 5),
        "A multi-line tooltip to show case in this example",
    );
    //s.print();
    assert_eq!(s.compute_hash(), 0x6F0C45230D2BDDE7);
}
#[test]
fn check_tooltip_bottom_pos() {
    let s = draw_tool_tip(
        Size::new(40, 10),
        Rect::new(3, 0, 10, 5),
        "A multi-line tooltip to show case in this example",
    );
    //s.print();
    assert_eq!(s.compute_hash(), 0xD12BB7D1C8BA1281);
}
#[test]
fn check_tooltip_bottom_pos_no_show() {
    let s = draw_tool_tip(
        Size::new(40, 10),
        Rect::new(3, 0, 10, 7),
        "A multi-line tooltip to show case in this example",
    );
    //s.print();
    assert_eq!(s.compute_hash(), 0x9F6184450761DB25);
}

fn prepare_command_bar(width: u32, height: u32) -> CommandBar {
    let mut c = CommandBar::new(width,height);
    c.set(Key::new(KeyCode::F2, KeyModifier::None), "Save", 1);
    c.set(Key::new(KeyCode::F3, KeyModifier::None), "Open", 2);
    c.set(Key::new(KeyCode::F5, KeyModifier::None), "Run", 3);
    c.set(Key::new(KeyCode::F7, KeyModifier::None), "Compile", 4);
    c.set(Key::new(KeyCode::F2, KeyModifier::Alt), "Save As ...", 5);
    c
}