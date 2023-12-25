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
