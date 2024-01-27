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
    let mut c = Canvas::new(Size::new(20, 10), Layout::new("x:1,y:1,w:15,h:4"), canvas::Flags::None);
    let s = c.get_drawing_surface();
    s.write_string(
        0,
        0,
        "         11111111112",
        CharAttribute::with_color(Color::White, Color::DarkRed),
        false,
    );
    s.write_string(
        0,
        1,
        "12345678901234567890",
        CharAttribute::with_color(Color::White, Color::DarkRed),
        false,
    );
    s.fill_horizontal_line(0, 9, 20, Character::new('=', Color::White, Color::DarkRed, CharFlags::None));
    for i in 2..10 {
        s.write_char(
            0,
            i,
            Character::new(((i + 48) as u8) as char, Color::White, Color::DarkRed, CharFlags::None),
        );
        s.write_char(
            7,
            i,
            Character::new(((i + 48) as u8) as char, Color::White, Color::DarkRed, CharFlags::None),
        );
        s.write_char(
            14,
            i,
            Character::new(((i + 48) as u8) as char, Color::White, Color::DarkRed, CharFlags::None),
        );
        s.write_char(
            19,
            i,
            Character::new(((i + 48) as u8) as char, Color::White, Color::DarkRed, CharFlags::None),
        );
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
    let mut c = Canvas::new(Size::new(20, 10), Layout::new("x:1,y:1,w:4,h:3"), canvas::Flags::None);
    let s = c.get_drawing_surface();
    s.write_string(
        0,
        0,
        "         11111111112",
        CharAttribute::with_color(Color::White, Color::DarkRed),
        false,
    );
    s.write_string(
        0,
        1,
        "12345678901234567890",
        CharAttribute::with_color(Color::White, Color::DarkRed),
        false,
    );
    s.fill_horizontal_line(0, 9, 20, Character::new('=', Color::White, Color::DarkRed, CharFlags::None));
    for i in 2..10 {
        s.write_char(
            0,
            i,
            Character::new(((i + 48) as u8) as char, Color::White, Color::DarkRed, CharFlags::None),
        );
        s.write_char(
            7,
            i,
            Character::new(((i + 48) as u8) as char, Color::White, Color::DarkRed, CharFlags::None),
        );
        s.write_char(
            14,
            i,
            Character::new(((i + 48) as u8) as char, Color::White, Color::DarkRed, CharFlags::None),
        );
        s.write_char(
            19,
            i,
            Character::new(((i + 48) as u8) as char, Color::White, Color::DarkRed, CharFlags::None),
        );
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
    let mut c = Canvas::new(Size::new(2, 2), Layout::new("x:1,y:1,w:15,h:4"), canvas::Flags::None);
    let s = c.get_drawing_surface();
    s.clear(Character::new('X', Color::Yellow, Color::DarkRed, CharFlags::None));
    c.set_backgound(Character::new('.', Color::White, Color::Black, CharFlags::None));
    w.add(c);
    a.add_window(w);
    a.run();
}

#[test]
fn check_mouse_on_scrollbars() {
    let script = "
        Paint.Enable(false)
        Paint('Initial state')
        CheckHash(0x5482fcc49857230a)
        Key.Pressed(Tab)
        Paint('Canvas focused')
        CheckHash(0x688773f2f69a827b)
        Key.Pressed(Right,5)
        Paint('bottom scrollbar moved with keys')
        CheckHash(0x824c6aac59e00fc0)
        Key.Pressed(Down,3)
        Paint('vertical scrollbar moved with keys')
        CheckHash(0x65f048227a2f5326)
        Key.Pressed(Right,20)
        Key.Pressed(Down,20)
        Paint('Scrollbars on bottom-right')
        CheckHash(0x2001b1a95626920f)
        Mouse.Move(49,7)
        Mouse.Click(49,7,left)
        Mouse.Move(31,13)
        Mouse.Click(31,13,left)
        Mouse.Move(44,16)
        Paint('scroll bars moved with mouse')
        CheckHash(0x58c2e5881e384722)
        Mouse.Move(31,13)
        Mouse.Click(31,13,left)
        Mouse.Click(31,13,left)
        Mouse.Click(31,13,left)
        Mouse.Click(31,13,left)
        Mouse.Click(31,13,left)
        Mouse.Click(31,13,left)
        Mouse.Click(31,13,left)
        Mouse.Click(31,13,left)
        Paint('move even more through left')
        CheckHash(0x94a715f8bdb37e60)
        Mouse.Move(49,7)
        Mouse.Click(49,7,left)
        Mouse.Click(49,7,left)
        Mouse.Click(49,7,left)
        Mouse.Click(49,7,left)
        Mouse.Click(49,7,left)
        Mouse.Click(49,7,left)
        Paint('move to top')
        CheckHash(0xd602abd0107382d7)
        Mouse.Move(31,13)
        Mouse.Click(31,13,left)
        Mouse.Click(31,13,left)
        Mouse.Click(31,13,left)
        Mouse.Click(31,13,left)
        Mouse.Click(31,13,left)
        Paint('Move to left')
        CheckHash(0xda9461e4fdcc6780)
        Mouse.Move(32,13)
        Paint('Mouse hover over bottom scrollbar')
        CheckHash(0xa8a93bbcfdf66b2d)
        Mouse.Hold(32,13,left)
        Mouse.Move(41,13)
        Mouse.Release(41,13,left)
        Paint('Scroll moved via drag')
        CheckHash(0x4dc5e7857a64e487)
        Mouse.Move(49,8)
        Mouse.Hold(49,8,left)
        Mouse.Move(49,10)
        Mouse.Release(49,10,left)
        Paint('Scroll move via drag (vertical)')
        CheckHash(0xd70e2e289ca96e54)
        Mouse.Move(48,13)
        Paint('Hover over window with scroll enabled')
        CheckHash(0x51e83736974739af)
        Mouse.Move(40,9)
        Mouse.Hold(40,9,left)
        Mouse.Move(36,11)
        Mouse.Release(36,11,left)
        Paint('Move via direct drag from surface')
        CheckHash(0xaf34cf74037cd71e)
        Key.Pressed(Tab)
        Mouse.Move(49,13)
        Paint('Hover over window (but with focus on button)')
        CheckHash(0x4e8b05d573a197ab)    
";
    let text: &str = r"012345678901234567890123456789
/- Some Text To Test -\
\=====================/
| () () () () () () ()| => 123
|---------------------|
\=-=-=-=-=-=-=-=-=-=-=/
 \-=-=-=-=-=-=-=-=-=-/
  \-=-=-=-=-=-=-=-=-/
   \===============/
    \ooooooooooooo/ => 1234567
";
    let mut a = App::debug(60, 20, script).build().unwrap();
    let mut w = window!("Title,d:c,w:40,h:8,flags:Sizeable");
    let mut c = Canvas::new(Size::new(30, 10), Layout::new("l:20,t:0,r:0,b:0"), canvas::Flags::ScrollBars);
    let s = c.get_drawing_surface();
    s.write_string(0, 0, text, CharAttribute::with_color(Color::White, Color::Black), true);
    w.add(c);
    w.add(button!("Test,l:1,t:1,a:tl,w:10"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_mouse_on_scrollbars_resize() {
    let script = "
    Paint.Enable(false)
    Paint('Initial State')
    CheckHash(0x5482fcc49857230a)
    Key.Pressed(Tab)
    Paint('Focus on Canvas')
    CheckHash(0x966ee395984d4a52)
    Mouse.Move(48,13)
    Mouse.Drag(48,13,53,18)
    Mouse.Move(56,18)
    Paint('Right scroll bar disabled')
    CheckHash(0x16c9777fb66b6367)
    Mouse.Move(53,18)
    Mouse.Drag(53,18,58,18)
    Mouse.Move(34,6)
    Mouse.Drag(34,6,27,1)
    Mouse.Move(50,13)
    Mouse.Drag(50,13,58,13)
    Paint('Bottom scroll bar disabled')
    CheckHash(0x8f9d0e2f5da48de2)
    Mouse.Move(41,13)
    Mouse.Click(41,13,left)
    Mouse.Move(31,13)
    Mouse.Click(31,13,left)
    Mouse.Move(52,13)
    Mouse.Click(52,13,left)
    Mouse.Move(58,11)
    Mouse.Click(58,11,left)
    Mouse.Move(58,7)
    Mouse.Click(58,7,left)
    Mouse.Move(58,4)
    Mouse.Click(58,4,left)
    Mouse.Move(28,13)
    Mouse.Click(28,13,left)
    Paint('Nothing hapens after click on scrollbars')
    CheckHash(0x41f0bfa8a7682c72)
    Mouse.Move(58,13)
    Mouse.Drag(58,13,38,7)
    Mouse.Move(41,10)
    Paint('right scrollbar is hidden')
    CheckHash(0xc5fbc8e124456a4c)
    Mouse.Move(38,7)
    Mouse.Drag(38,7,38,9)
    Mouse.Move(39,12)
    Paint('right scrollbar visible')
    CheckHash(0xc7827de70d4271d4)
    Mouse.Move(37,9)
    Mouse.Drag(37,9,31,9)
    Paint('bottom scollbar hidden')
    CheckHash(0x78790845f2cd8e40)        
";
    let text: &str = r"012345678901234567890123456789
/- Some Text To Test -\
\=====================/
| () () () () () () ()| => 123
|---------------------|
\=-=-=-=-=-=-=-=-=-=-=/
 \-=-=-=-=-=-=-=-=-=-/
  \-=-=-=-=-=-=-=-=-/
   \===============/
    \ooooooooooooo/ => 1234567
";
    let mut a = App::debug(60, 20, script).build().unwrap();
    let mut w = window!("Title,d:c,w:40,h:8,flags:Sizeable");
    let mut c = Canvas::new(Size::new(30, 10), Layout::new("l:20,t:0,r:0,b:0"), canvas::Flags::ScrollBars);
    c.set_components_toolbar_margins(2, 1);
    let s = c.get_drawing_surface();
    s.write_string(0, 0, text, CharAttribute::with_color(Color::White, Color::Black), true);
    w.add(c);
    w.add(button!("Test,l:1,t:1,a:tl,w:10"));
    a.add_window(w);
    a.run();
}


#[test]
fn check_macro_init_1() {
    let script = "
    Paint.Enable(false)
    Paint('Initial State')
    CheckHash(0x5482fcc49857230a)
    Key.Pressed(Tab)
    Paint('Focus')
    CheckHash(0x966EE395984D4A52)
";
    let text: &str = r"012345678901234567890123456789
/- Some Text To Test -\
\=====================/
| () () () () () () ()| => 123
|---------------------|
\=-=-=-=-=-=-=-=-=-=-=/
 \-=-=-=-=-=-=-=-=-=-/
  \-=-=-=-=-=-=-=-=-/
   \===============/
    \ooooooooooooo/ => 1234567
";
    let mut a = App::debug(60, 20, script).build().unwrap();
    let mut w = window!("Title,d:c,w:40,h:8,flags:Sizeable");
    let mut c = canvas!("'30 x 10',l:20,t:0,r:0,b:0,flags=Scrollbars,lsm=2,tsm=1");
    let s = c.get_drawing_surface();
    s.write_string(0, 0, text, CharAttribute::with_color(Color::White, Color::Black), true);
    w.add(c);
    w.add(button!("Test,l:1,t:1,a:tl,w:10"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_macro_init_2() {
    let script = "
    Paint.Enable(false)
    Paint('Initial State')
    CheckHash(0x30DC80BF114CEFBE)     
    Key.Pressed(Tab)
    Paint('With focus')
    CheckHash(0x4709CADE1C0994D3)
";
    let mut a = App::debug(60, 20, script).build().unwrap();
    let mut w = window!("Title,d:c,w:40,h:8,flags:Sizeable");
    let mut c = canvas!("'4 x 2',l:20,t:0,r:0,b:0,flags=Scrollbars,lsm=3,tsm=1,back={X,fore:Green,Back:Yellow}");
    let s = c.get_drawing_surface();
    s.clear(char!("<->,r,black"));
    w.add(c);
    w.add(button!("Test,l:1,t:1,a:tl,w:10"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_pageup_pagedown() {
    let text = r"--- From Wiki ----
Rust is a multi-paradigm, general-purpose 
programming language that emphasizes performance, 
type safety, and concurrency. It enforces memory 
safety—meaning that all references point to valid 
memory—without a garbage collector. To 
simultaneously enforce memory safety and prevent 
data races, its 'borrow checker' tracks the object 
lifetime of all references in a program during 
compilation. Rust was influenced by ideas from 
functional programming, including immutability, 
higher-order functions, and algebraic data types. 
It is popular for systems programming.

From: https://en.wikipedia.org/wiki/Rust_(programming_language)
";
let script = "
Paint.Enable(false)
Resize(60,20)
Paint('Initial state')
CheckHash(0xc9196f52d863ff88)
Key.Pressed(PageDown)
Paint('Page down (first line: simultabeously ...)')
CheckHash(0x27f6cbecd53c03f8)
Key.Pressed(PageDown)
Paint('Page Down (first line: compilation. ....)')
CheckHash(0xe1527c32d87dfd3b)
Key.Pressed(PageDown)
Paint('Another page down (nothing changes)')
CheckHash(0xe1527c32d87dfd3b)
Key.Pressed(PageUp)
Paint('Page Up (first line: type safety, and ...)')
CheckHash(0x9e29d2f1826c129)
Key.Pressed(PageUp)
Paint('Back to the initial state')
CheckHash(0xc9196f52d863ff88)
";
    let mut a = App::debug(60, 20, script).build().unwrap();
    let mut w = window!("Title,d:c,w:40,h:8,flags:Sizeable");
    let mut c = canvas!("'60x15',d:c,w:100%,h:100%,flags=ScrollBars,lsm:3,tsm:1");
    let s = c.get_drawing_surface();
    s.write_string(0, 0, text, CharAttribute::with_color(Color::White, Color::Black), true);
    w.add(c);
    a.add_window(w);
    a.run();
}