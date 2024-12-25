use AppCUIProcMacro::*;

use super::App;
use super::Theme;
use super::Themes;
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
use crate::ui::command_bar::*;
use crate::ui::common::traits::*;

fn draw_tool_tip(size: Size, rect: Rect, txt: &str) -> SurfaceTester {
    let mut tooltip = ToolTip::new();
    let theme = Theme::new(Themes::Default);
    let mut s = SurfaceTester::new(size.width, size.height);

    tooltip.show(txt, &rect, s.size(), &theme);
    s.clear(Character::new(' ', Color::White, Color::Black, CharFlags::None));
    s.fill_rect(rect, Character::new('X', Color::White, Color::DarkRed, CharFlags::None));
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
    let s = draw_tool_tip(Size::new(40, 10), Rect::new(2, 3, 10, 5), "A multi-line tooltip\nto show case");
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
#[derive(Copy, Clone)]
struct Command {
    value: u32,
}
impl TryFrom<u32> for Command {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(Self { value })
    }
}
impl From<Command> for u32 {
    fn from(value: Command) -> Self {
        value.value
    }
}
impl CommandID for Command {}
impl Command {
    fn new(value: u32) -> Self {
        Self { value }
    }
}

fn prepare_command_bar(size: Size) -> CommandBar {
    let mut c = CommandBar::new(size.width, size.height);
    c.set(Key::new(KeyCode::F2, KeyModifier::None), "Save", Command::new(1));
    c.set(Key::new(KeyCode::F3, KeyModifier::None), "Open", Command::new(2));
    c.set(Key::new(KeyCode::F5, KeyModifier::None), "Run", Command::new(3));
    c.set(Key::new(KeyCode::F7, KeyModifier::None), "Compile", Command::new(4));
    c.set(Key::new(KeyCode::F8, KeyModifier::None), "Delete", Command::new(5));
    c.set(Key::new(KeyCode::F2, KeyModifier::Alt), "Save As ...", Command::new(12345));
    c.update_positions();
    c
}

#[test]
fn check_command_bar_1() {
    let mut s = SurfaceTester::new(60, 5);
    let c = prepare_command_bar(s.size());
    s.clear(Character::new('X', Color::Black, Color::DarkBlue, CharFlags::None));
    c.paint(&mut s, &Theme::new(Themes::Default));
    //s.print();
    assert_eq!(s.compute_hash(), 0xD466864BD254E538);
}

#[test]
fn check_command_bar_2() {
    let mut s = SurfaceTester::new(60, 5);
    let mut c = prepare_command_bar(s.size());
    s.clear(Character::new('.', Color::Black, Color::DarkBlue, CharFlags::None));
    c.set_key_modifier(KeyModifier::Alt);
    c.paint(&mut s, &Theme::new(Themes::Default));
    //s.print();
    assert_eq!(s.compute_hash(), 0x940B30F3F39A2B3A);
}
#[test]
fn check_command_bar_hover() {
    let mut s = SurfaceTester::new(60, 5);
    let mut c = prepare_command_bar(s.size());
    s.clear(Character::new(SpecialChar::Block50, Color::Black, Color::DarkBlue, CharFlags::None));
    for x in 0..9 {
        c.on_mouse_move(&MouseMoveEvent {
            x,
            y: 4,
            button: MouseButton::None,
        });
        c.paint(&mut s, &Theme::new(Themes::Default));
        //s.print();
        assert_eq!(s.compute_hash(), 0x6FFD6A9E00B06190);
    }
    c.on_mouse_move(&MouseMoveEvent {
        x: 9,
        y: 4,
        button: MouseButton::None,
    });
    c.paint(&mut s, &Theme::new(Themes::Default));
    //s.print();
    assert_eq!(s.compute_hash(), 0x8FE003D26FC257B8);
    c.on_mouse_move(&MouseMoveEvent {
        x: 10,
        y: 4,
        button: MouseButton::None,
    });
    c.paint(&mut s, &Theme::new(Themes::Default));
    //s.print();
    assert_eq!(s.compute_hash(), 0x24738CE8FFD30F80);
    c.on_mouse_move(&MouseMoveEvent {
        x: 10,
        y: 3,
        button: MouseButton::None,
    });
    c.paint(&mut s, &Theme::new(Themes::Default));
    //s.print();
    assert_eq!(s.compute_hash(), 0x8FE003D26FC257B8);
}

#[test]
fn check_command_bar_click() {
    let mut s = SurfaceTester::new(60, 5);
    let mut c = prepare_command_bar(s.size());
    s.clear(Character::new(SpecialChar::Block50, Color::Black, Color::DarkBlue, CharFlags::None));
    c.set_key_modifier(KeyModifier::Alt);
    c.on_mouse_move(&MouseMoveEvent {
        x: 9,
        y: 4,
        button: MouseButton::None,
    });
    c.paint(&mut s, &Theme::new(Themes::Default));
    //s.print();
    assert_eq!(s.compute_hash(), 0xF768DE602AA7C28A);
    c.on_mouse_down(&MouseButtonDownEvent {
        x: 9,
        y: 4,
        button: MouseButton::Left,
    });
    c.paint(&mut s, &Theme::new(Themes::Default));
    //s.print();
    assert_eq!(s.compute_hash(), 0x66FEDFABE303DEF6);
    let result = c.on_mouse_up().unwrap().command_id;
    c.paint(&mut s, &Theme::new(Themes::Default));
    //s.print();
    assert_eq!(s.compute_hash(), 0xF768DE602AA7C28A);
    assert_eq!(result, 12345);
}

#[test]
fn check_multiple_apps_started() {
    let a = App::debug(60, 10, "").build().unwrap();
    a.run();
    let a = App::debug(50, 20, "").build().unwrap();
    a.run();
}

#[test]
fn check_mouse_keymodifier_mouse() {
    #[CustomControl(overwrite:OnPaint+OnMouseEvent, internal: true)]
    struct TestControl {
        txt: String,
    }
    impl TestControl {
        fn new() -> Self {
            Self {
                base: ControlBase::new(Layout::new("d:c,w:100%,h:100%"), true),
                txt: String::new(),
            }
        }
    }
    impl OnPaint for TestControl {
        fn on_paint(&self, surface: &mut crate::prelude::Surface, _theme: &Theme) {
            surface.clear(char!("' ',red,black"));
            surface.write_string(0, 0, &self.txt, CharAttribute::new(Color::White, Color::Black, CharFlags::None), true);
        }
    }
    impl OnMouseEvent for TestControl {
        fn on_mouse_event(&mut self, event: &crate::prelude::MouseEvent) -> EventProcessStatus {
            match event {
                crate::prelude::MouseEvent::Enter => {}
                crate::prelude::MouseEvent::Leave => {}
                crate::prelude::MouseEvent::Over(_) => {}
                crate::prelude::MouseEvent::Pressed(data) => {
                    self.txt = format!("Pressed: x:{},y:{}\nbutton:{:?},modif:{:?}", data.x, data.y, data.button, data.modifier);
                }
                crate::prelude::MouseEvent::Released(data) => {
                    self.txt = format!("Pressed: x:{},y:{}\nbutton:{:?},modif:{:?}", data.x, data.y, data.button, data.modifier);
                }
                crate::prelude::MouseEvent::DoubleClick(data) => {
                    self.txt = format!("Pressed: x:{},y:{}\nbutton:{:?},modif:{:?}", data.x, data.y, data.button, data.modifier);
                }
                crate::prelude::MouseEvent::Drag(data) => {
                    self.txt = format!("Pressed: x:{},y:{}\nbutton:{:?},modif:{:?}", data.x, data.y, data.button, data.modifier);
                }
                crate::prelude::MouseEvent::Wheel(_) => {}
            }
            EventProcessStatus::Processed
        }
    }
    let script = "
        Paint.Enable(false)
        Paint('Initial state')   
        CheckHash(0x614245CC84C42969)   
        Mouse.Drag(1,1,3,3);
        Paint('Simple Drag')   
        CheckHash(0x72F1BAC32695526E)  
        Key.Modifier(Ctrl) 
        Mouse.Drag(3,3,5,5);
        Paint('Simple Drag (with ctrl - value 2)')   
        CheckHash(0x6595486EEC0578BC)  
        Key.Modifier(Ctrl+Shift) 
        Mouse.Drag(5,5,7,7);
        Paint('Simple Drag (with ctrl+shift - value 6)')   
        CheckHash(0x81760858B7A9C498)  
        Key.Modifier(None) 
        Mouse.Drag(7,7,1,1);
        Paint('Simple Drag (with no modifier - value 0)')   
        CheckHash(0x6959C1EA263F8E6E)          
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = window!("Test,d:c");
    w.add(TestControl::new());
    a.add_window(w);
    a.run();
}

