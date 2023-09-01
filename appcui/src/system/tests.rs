use crate::ui::Desktop;
use crate::ui::command_bar::*;
use super::App;
use super::InitializationFlags;
use super::Theme;
use super::ToolTip;
use crate::graphics::CharFlags;
use crate::graphics::Character;
use crate::graphics::Color;
use crate::graphics::Rect;
use crate::graphics::Size;
use crate::graphics::SpecialChar;
use crate::graphics::SurfaceTester;
use crate::input::Key;
use crate::input::KeyCode;
use crate::input::KeyModifier;
use crate::input::MouseButton;
use crate::terminals::MouseButtonDownEvent;
use crate::terminals::MouseMoveEvent;

fn draw_tool_tip(size: Size, rect: Rect, txt: &str) -> SurfaceTester {
    let mut tooltip = ToolTip::new();
    let theme = Theme::new();
    let mut s = SurfaceTester::new(size.width, size.height);
    
    tooltip.show(txt, &rect, s.get_size(), &theme);
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
    c.set(Key::new(KeyCode::F8, KeyModifier::None), "Delete", 5);
    c.set(Key::new(KeyCode::F2, KeyModifier::Alt), "Save As ...", 12345);
    c.update_positions();
    c
}

#[test]
fn check_command_bar_1() {
    let mut s = SurfaceTester::new(60,5);
    let c = prepare_command_bar(s.get_width(), s.get_height());
    s.clear(Character::new('X',Color::Black,Color::DarkBlue, CharFlags::None));
    c.paint(&mut s, &Theme::new());
    //s.print();
    assert_eq!(s.compute_hash(), 0xD466864BD254E538);
}

#[test]
fn check_command_bar_2() {
    let mut s = SurfaceTester::new(60,5);
    let mut c = prepare_command_bar(s.get_width(), s.get_height());
    s.clear(Character::new('.',Color::Black,Color::DarkBlue, CharFlags::None));
    c.set_key_modifier(KeyModifier::Alt);
    c.paint(&mut s, &Theme::new());
    //s.print();
    assert_eq!(s.compute_hash(), 0x940B30F3F39A2B3A);
}
#[test]
fn check_command_bar_hover() {
    let mut s = SurfaceTester::new(60,5);
    let mut c = prepare_command_bar(s.get_width(), s.get_height());
    s.clear(Character::new(SpecialChar::Block50,Color::Black,Color::DarkBlue, CharFlags::None));
    for x in 0..9 {
        c.on_mouse_move(&MouseMoveEvent{ x, y: 4, button: MouseButton::None });
        c.paint(&mut s, &Theme::new());
        //s.print();
        assert_eq!(s.compute_hash(), 0x6FFD6A9E00B06190);
    }
    c.on_mouse_move(&MouseMoveEvent{ x:9 , y: 4, button: MouseButton::None });
    c.paint(&mut s, &Theme::new());
    //s.print();
    assert_eq!(s.compute_hash(), 0x8FE003D26FC257B8);
    c.on_mouse_move(&MouseMoveEvent{ x:10 , y: 4, button: MouseButton::None });
    c.paint(&mut s, &Theme::new());
    //s.print();
    assert_eq!(s.compute_hash(), 0x24738CE8FFD30F80);
    c.on_mouse_move(&MouseMoveEvent{ x:10 , y: 3, button: MouseButton::None });
    c.paint(&mut s, &Theme::new());
    //s.print();
    assert_eq!(s.compute_hash(), 0x8FE003D26FC257B8);
}

#[test]
fn check_command_bar_click() {
    let mut s = SurfaceTester::new(60,5);
    let mut c = prepare_command_bar(s.get_width(), s.get_height());
    s.clear(Character::new(SpecialChar::Block50,Color::Black,Color::DarkBlue, CharFlags::None));
    c.set_key_modifier(KeyModifier::Alt);
    c.on_mouse_move(&MouseMoveEvent{ x:9 , y: 4, button: MouseButton::None });
    c.paint(&mut s, &Theme::new());
    //s.print();
    assert_eq!(s.compute_hash(), 0xF768DE602AA7C28A);
    c.on_mouse_down(&MouseButtonDownEvent{ x:9 , y: 4, button: MouseButton::Left });
    c.paint(&mut s, &Theme::new());
    //s.print();
    assert_eq!(s.compute_hash(), 0x66FEDFABE303DEF6);
    let result = c.on_mouse_up().unwrap().command_id;
    c.paint(&mut s, &Theme::new());
    //s.print();
    assert_eq!(s.compute_hash(), 0xF768DE602AA7C28A);
    assert_eq!(result, 12345);
}

#[test]
fn check_multiple_apps_started() {
    let a = App::debug(60, 10, InitializationFlags::None, Desktop::new(), "").unwrap();
    a.run();
    let a = App::debug(50, 20, InitializationFlags::None, Desktop::new(), "").unwrap();
    a.run();   
}