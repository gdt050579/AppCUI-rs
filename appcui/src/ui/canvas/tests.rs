use crate::prelude::*;

#[test]
fn check_keyboard() {
    let script = "
        Paint.Enable(false)
        Paint('Initial state')
        CheckHash(0xDD19FB5257AF4C39)
        Key.Pressed(Right)
        Paint('2')
        CheckHash(0x9072EEB8678E62E)
        Key.Pressed(Right,3)
        Paint('3')
        CheckHash(0xCDCA2B1AA0D0ED78)
        Key.Pressed(Right,2)
        Paint('4')
        CheckHash(0x8D74117DDF999E46)
        Key.Pressed(Right,100)
        Paint('4 (already at margin)')
        CheckHash(0x8D74117DDF999E46)
        Key.Pressed(Left,3)
        Paint('5')
        CheckHash(0x7AEB60F729867DDE)
        Key.Pressed(Left,2)
        Paint('Back to initial state')
        CheckHash(0xDD19FB5257AF4C39)
        Key.Pressed(Down,2)
        Paint('6')
        CheckHash(0x33CCFDF3A125050)
        Key.Pressed(Down,4)
        Paint('7')
        CheckHash(0xAE6E99E7D14B45D0)
        Key.Pressed(Up,2)
        Paint('8')
        CheckHash(0x17A3D40C19A04DA8)
        Key.Pressed(Shift+Up)
        Paint('Back to initial state')
        CheckHash(0xDD19FB5257AF4C39)
        Key.Pressed(Shift+Right)
        Paint('Right top margin')
        CheckHash(0x8D74117DDF999E46)
        Key.Pressed(Shift+Down)
        Paint('9')
        CheckHash(0x94C6FF9EDFF78BD0)
        Key.Pressed(Shift+Left)
        Paint('10')
        CheckHash(0xAE6E99E7D14B45D0)
        Key.Pressed(Home)
        Paint('Back to initial state')
        CheckHash(0xDD19FB5257AF4C39)
        Key.Pressed(End)
        Paint('11')
        CheckHash(0x94C6FF9EDFF78BD0)
";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = window!("Title,d:c,w:40,h:8");
    let mut c = Canvas::new(Size::new(20,10),Layout::new("x:1,y:1,w:15,h:4"),canvas::Flags::None);
    let s = c.get_drawing_surface();
    s.write_string(0, 0, "         11111111112", CharAttribute::with_color(Color::White, Color::DarkRed), false);
    s.write_string(0, 1, "12345678901234567890", CharAttribute::with_color(Color::White, Color::DarkRed), false);
    s.fill_horizontal_line(0, 9, 20, Character::new('=',Color::White,Color::DarkRed,CharFlags::None));
    for i in 2..10 {
        s.write_char(0, i, Character::new(((i+48) as u8) as char, Color::White, Color::DarkRed, CharFlags::None));
        s.write_char(7, i, Character::new(((i+48) as u8) as char, Color::White, Color::DarkRed, CharFlags::None));
        s.write_char(14, i, Character::new(((i+48) as u8) as char, Color::White, Color::DarkRed, CharFlags::None));
        s.write_char(19, i, Character::new(((i+48) as u8) as char, Color::White, Color::DarkRed, CharFlags::None));
    }
    w.add(c);
    a.add_window(w);
    a.run();
}

#[test]
fn check_keyboard_2() {
    let script = "
        Paint.Enable(false)
        Paint('Initial state')
        CheckHash(0xECD64DA8EC4ABB22)
        Key.Pressed(Ctrl+Right)
        Paint('2')
        CheckHash(0x2F6B2D52610BE94A)
        Key.Pressed(Ctrl+Right,2)
        Paint('3')
        Key.Pressed(Ctrl+Right,3)
        Paint('4')
        CheckHash(0x91FA33F25664115F)
        Key.Pressed(Ctrl+Left)
        Paint('5')
        CheckHash(0x7939B4BF72AAB9CA)
        Key.Pressed(Ctrl+Left,2)
        Paint('6')
        Key.Pressed(Ctrl+Left,1)
        Paint('Back Initial state')
        CheckHash(0xECD64DA8EC4ABB22)
        Key.Pressed(Ctrl+Down)
        Paint('7')
        CheckHash(0x8C4B46EB47038066)
        Key.Pressed(Ctrl+Down)
        Paint('8')
        CheckHash(0x88133037D51CD5B5)
        Key.Pressed(Ctrl+Down)
        Paint('9')
        CheckHash(0x7D07BFDFFDFE8F0F)
        Key.Pressed(Ctrl+Up,2)
        Paint('10')
        CheckHash(0x6B582357BD9EB6F5)
        Key.Pressed(Ctrl+Up)
        Paint('Back Initial state')
        CheckHash(0xECD64DA8EC4ABB22)
";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = window!("Title,d:c,w:40,h:8");
    let mut c = Canvas::new(Size::new(20,10),Layout::new("x:1,y:1,w:4,h:3"),canvas::Flags::None);
    let s = c.get_drawing_surface();
    s.write_string(0, 0, "         11111111112", CharAttribute::with_color(Color::White, Color::DarkRed), false);
    s.write_string(0, 1, "12345678901234567890", CharAttribute::with_color(Color::White, Color::DarkRed), false);
    s.fill_horizontal_line(0, 9, 20, Character::new('=',Color::White,Color::DarkRed,CharFlags::None));
    for i in 2..10 {
        s.write_char(0, i, Character::new(((i+48) as u8) as char, Color::White, Color::DarkRed, CharFlags::None));
        s.write_char(7, i, Character::new(((i+48) as u8) as char, Color::White, Color::DarkRed, CharFlags::None));
        s.write_char(14, i, Character::new(((i+48) as u8) as char, Color::White, Color::DarkRed, CharFlags::None));
        s.write_char(19, i, Character::new(((i+48) as u8) as char, Color::White, Color::DarkRed, CharFlags::None));
    }
    w.add(c);
    a.add_window(w);
    a.run();
}

#[test]
fn check_background_char() {
    let script = "
        Paint.Enable(false)
        Paint('Initial state')
        CheckHash(0x209459BC48B6383C)
";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = window!("Title,d:c,w:40,h:8");
    let mut c = Canvas::new(Size::new(2,2),Layout::new("x:1,y:1,w:15,h:4"),canvas::Flags::None);
    let s = c.get_drawing_surface();
    s.clear(Character::new('X',Color::Yellow,Color::DarkRed,CharFlags::None));
    c.set_backgound(Character::new('.',Color::White,Color::Black,CharFlags::None));
    w.add(c);
    a.add_window(w);
    a.run();
}

