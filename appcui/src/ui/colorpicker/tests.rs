use crate::prelude::*;

#[test]
fn check_colorpicker_header_size() {
    let script = "
        Paint.Enable(false)
        // |  0 | ▒▒▒▒▒╔═══════════════════ Title ═══════════════════[x]╗▒▒▒▒▒ |
        // |  1 | ▒▒▒▒▒║                                                ║▒▒▒▒▒ |
        // |  2 | ▒▒▒▒▒║  ■ DarkGreen      ▼                            ║▒▒▒▒▒ |
        // |  3 | ▒▒▒▒▒║                                                ║▒▒▒▒▒ |
        // |  4 | ▒▒▒▒▒║  ■ DarkGree   ▼     ■ D   ▼        ■   ▼       ║▒▒▒▒▒ |
        // |  5 | ▒▒▒▒▒║                                                ║▒▒▒▒▒ |
        // |  6 | ▒▒▒▒▒║  ■ DarkGre   ▼      ■    ▼         ■   ▼       ║▒▒▒▒▒ |
        // |  7 | ▒▒▒▒▒║                                                ║▒▒▒▒▒ |
        // |  8 | ▒▒▒▒▒║  ■ Da   ▼           ■   ▼          ■   ▼       ║▒▒▒▒▒ |
        // |  9 | ▒▒▒▒▒║                                                ║▒▒▒▒▒ |
        // | 10 | ▒▒▒▒▒╚════════════════════════════════════════════════╝▒▒▒▒▒ |
        Paint('Focus on top colorpicker')
        CheckHash(0x292A2027DCD39746)   
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Title,d:c,w:50,h:11");
    w.add(ColorPicker::new(Color::DarkGreen, Layout::new("x:1,y:3,w:16")));
    w.add(ColorPicker::new(Color::DarkGreen, Layout::new("x:1,y:5,w:15")));
    w.add(ColorPicker::new(Color::DarkGreen, Layout::new("x:1,y:7,w:10")));

    w.add(ColorPicker::new(Color::DarkGreen, Layout::new("x:20,y:3,w:9")));
    w.add(ColorPicker::new(Color::DarkGreen, Layout::new("x:20,y:5,w:8")));
    w.add(ColorPicker::new(Color::DarkGreen, Layout::new("x:20,y:7,w:7")));

    w.add(ColorPicker::new(Color::DarkGreen, Layout::new("x:35,y:3,w:6")));
    w.add(ColorPicker::new(Color::DarkGreen, Layout::new("x:35,y:5,w:5")));
    w.add(ColorPicker::new(Color::DarkGreen, Layout::new("x:35,y:7,w:4")));

    w.add(ColorPicker::new(Color::DarkGreen, Layout::new("x:1,y:1,w:20")));
    a.add_window(w);
    a.run();
}

#[test]
fn check_colorpicker_expand() {
    let script = "
        Paint.Enable(false)
        Paint('Focus on bottom-right control')
        CheckHash(0x69AC235869E0C834)   
        Key.Pressed(Space)
        Paint('Bottom up control expanded');
        CheckHash(0xE01B4D7F5A5D0921)
        Key.Pressed(Left,2)
        Paint('Color is now dark-green')
        CheckHash(0x3999BB81560252E9)
        Key.Pressed(Enter)
        Paint('Close again')
        CheckHash(0xFC931A2DAD72A509)
        Key.Pressed(Tab)
        Key.Pressed(Enter)
        Paint('Top-left colorpicker opened')
        CheckHash(0xC9EEC177D8E36F5C)
        Key.Pressed(Tab)
        Paint('Focus on second colopicker')
        CheckHash(0xFDB4B5D817961295)
        Key.Pressed(Enter)
        // there is no space on top or bottom to open the second color picker
        Paint('Second colopicker is not opened')
        CheckHash(0xFDB4B5D817961295)
        Key.Pressed(Tab,2)
        Key.Pressed(Enter)
        Paint('Bottom-left colorpicker opened')
        CheckHash(0x3C5092DFC5D9281F)
        Key.Pressed(Escape)
        Paint('Bottom-left colorpicker closed')
        CheckHash(0xFF26EBFE96A11715)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Title,d:c,w:50,h:11");
    w.add(ColorPicker::new(Color::DarkGreen, Layout::new("x:1,y:1,w:46")));
    w.add(colorpicker!("Red,x:1,y:3,w:12"));
    w.add(colorpicker!("Black,x:1,y:5,w:7"));
    w.add(colorpicker!("color:pink,x:1,y:7,w:20"));
    w.add(colorpicker!("transparent,x:37,y:7,w:10"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_colorpicker_keys() {
    let script = "
        Paint.Enable(false)
        Paint('initial state')
        CheckHash(0xABC9AC41ED0390F3)   
        Key.Pressed(Down)
        Paint('Color=Pink')
        CheckHash(0x6D86A9AD2EF51D9)  
        Key.Pressed(Left,20)
        Paint('Color=Black')
        CheckHash(0x1CE9940D71F41ED7)
        Key.Pressed(Right,1)
        Paint('Color=DarkBlue')
        CheckHash(0xC744F43FB93B523A)
        Key.Pressed(Down,100);
        // after this we should be on transparent
        Key.Pressed(Up,2)
        Paint('Color=Yellow')
        CheckHash(0x72D633CE280C1ACE)
        Key.Pressed(Enter)
        Paint('Expanded')
        CheckHash(0x7765B355C90D8C36)  
        Key.Pressed(Left)
        Paint('Color=Pink')
        CheckHash(0x1930A74DB0BEA79E)
        Key.Pressed(Left,2)
        Paint('Color=Aqua')
        CheckHash(0xAEE0020F8B87B003)
        Key.Pressed(Up,2)
        Paint('Color=Teal')
        CheckHash(0xCA074786AF1579B4)
        Key.Pressed(Right)
        Paint('Color=Transparent')
        CheckHash(0x81B57ECA929B2BAD)
        Key.Pressed(Left,4)
        Paint('Color=Black')
        CheckHash(0x23D72BCE00DD50F8)
        Key.Pressed(Left,3)
        Paint('Color=DarkGreen')
        CheckHash(0x66EDB2D77177B1ED)
        Key.Pressed(Down,2)
        Paint('Color=Green')
        CheckHash(0xBAD5C0EB916CD335)
        Key.Pressed(Escape)
        Paint('Closed')
        CheckHash(0xF0CE1F2D9AF9C7A1)
        Key.Pressed(Enter)
        Paint('Re-opened')
        CheckHash(0xBAD5C0EB916CD335)
        Key.Pressed(Enter)
        Paint('Closed again')
        CheckHash(0xF0CE1F2D9AF9C7A1)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Title,d:c,w:50,h:11");
    w.add(colorpicker!("Red,x:1,y:1,w:15"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_colorpicker_cursor_on_transparent() {
    let script = "
        Paint.Enable(false)
        Paint('initial state')
        CheckHash(0x62D1C24D1EB6AB17)
        Key.Pressed(Space)
        Paint('Opened')
        CheckHash(0x81B57ECA929B2BAD)
        CheckCursor(23,4)
        Key.Pressed(Left)
        Paint('Teal color')
        CheckHash(0xCA074786AF1579B4)
        CheckCursor(hidden)    
        Key.Pressed(Right) 
        Key.Pressed(Escape)  
        Paint('Transparent color') 
        CheckHash(0x62D1C24D1EB6AB17)
        CheckCursor(hidden)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Title,d:c,w:50,h:11");
    w.add(colorpicker!("Transparent,x:1,y:1,w:15"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_colorpicker_mouse() {
    let script = "
        Paint.Enable(false)
        Paint('initial state')
        CheckHash(0x8CE7D62D827A3765)
        Mouse.Move(10,2)
        Paint('Hover over first colorpicker')
        CheckHash(0x288048EBEB6DB121)
        Mouse.Click(10,2,left)
        Paint('Opened')
        CheckHash(0x78999B76B30F4422)
        Mouse.Move(12,4)
        Paint('Over blue color')
        CheckHash(0xF689A841176DFC04)
        Mouse.Move(15,7)
        Paint('Over Yellow color')
        CheckHash(0xEE67C6A417AAECE0)
        Mouse.Click(15,7,left)
        Paint('Closed & color=yellow')
        CheckHash(0x51551A218F024821)
        Mouse.Click(10,2,left)
        Paint('re-Opened')
        CheckHash(0xAE2D7D4B7C775901)
        Mouse.Move(36,4)
        Paint('Over transparent color')
        CheckHash(0x14A104B2E8257F79)
        Mouse.Click(36,4,left)
        Paint('transparent color')
        CheckHash(0x6A5BA6F70F76D2E4)
        Mouse.Click(20,8,left)
        Paint('Second opened')
        CheckHash(0x10C2B16AC33F9326)
        Mouse.Move(16,3)
        Paint('Over green')
        CheckHash(0x1718FEFC03A4EC8C)
        Mouse.Click(16,3,left)
        Paint('Green color')
        CheckHash(0xDF9C98AEDE4BBC7)
        Mouse.Click(40,2,left)
        Paint('3rd colorpicker opened')
        CheckHash(0xB5DF98E7C0E5F5A)
        Mouse.Move(10,8)
        Paint('Hover over second')
        CheckHash(0x4EB5F2B604D672DA)
        Mouse.Click(10,8,left)
        Paint('second re-opened')
        CheckHash(0x46F9F469CA6B6F00)
        Mouse.Click(40,5,left)
        Paint('all closed')
        CheckHash(0xDF9C98AEDE4BBC7)
        Mouse.Click(10,8,left)
        Paint('second re-opened (3rd time)')
        CheckHash(0x46F9F469CA6B6F00)
        // now we click on the desktop
        Mouse.Click(0,0,left)
        Paint('all closed')
        CheckHash(0xDF9C98AEDE4BBC7)
        Mouse.Click(10,8,left)
        Paint('second re-opened (4th time)')
        CheckHash(0x46F9F469CA6B6F00)
        // click on the expanded panel, but not on a color
        Mouse.Click(24,6,left)
        Paint('all closed')
        CheckHash(0xDF9C98AEDE4BBC7)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Title,d:c,w:50,h:11");
    w.add(colorpicker!("Transparent,x:1,y:1,w:15"));
    w.add(colorpicker!("Red,x:30,y:1,w:15"));
    w.add(colorpicker!("Blue,x:1,y:7,w:30"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_colorpicker_window_move() {
    let script = "
        Paint.Enable(false)
        Paint('initial state')
        CheckHash(0x581F2761E3AAC48B)
        Key.Pressed(Space)
        Paint('Opened')
        CheckHash(0x8916E8419C72033F)
        Mouse.Drag(30,0,28,4)
        Paint('window move, colorpicker should be closed')
        CheckHash(0x1750D911F6D971AB)
        Mouse.Drag(28,4,26,7)
        Paint('window move over desktop with bottom side')
        CheckHash(0xE2404991A12EB4BC)
        Mouse.Click(10,9,left)
        Paint('Re-opened on top')
        CheckHash(0xCD1E85F61C0DD1F5)
        Mouse.Hold(1,7,left)
        Paint('Click on window (colorpicker is closed)')
        CheckHash(0x21C93A3BEEFA875F)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Title,d:t,w:50,h:6");
    w.add(colorpicker!("Transparent,x:1,y:1,w:15"));
    a.add_window(w);
    a.run();
}