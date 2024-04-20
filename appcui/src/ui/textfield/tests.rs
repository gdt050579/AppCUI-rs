use crate::prelude::*;

#[test]
fn check_move_left_right() {
    let script = "
        Paint.Enable(false)
        Paint('cursor at end')   
        CheckHash(0x52EEFBBE06A52F24)
        CheckCursor(24,3)
        Key.Pressed(Left,4)
        Paint('cursor at: Hello w|o|rld')   
        CheckHash(0x52EEFBBE06A52F24)
        CheckCursor(20,3)
        Key.Pressed(Home)
        Paint('cursor at: |H|ello world')   
        CheckHash(0x52EEFBBE06A52F24)
        CheckCursor(13,3)
        Key.Pressed(Right,3)
        Paint('cursor at: Hel|l|o world')   
        CheckHash(0x52EEFBBE06A52F24)
        CheckCursor(16,3)
        Key.Pressed(End)
        Paint('cursor at: Hello world||')   
        CheckHash(0x52EEFBBE06A52F24)
        CheckCursor(24,3)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:40,h:9"), window::Flags::None);
    w.add(textfield!("'Hello world',x:1,y:1,w:38,h:1"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_select_all() {
    let script = "
        Paint.Enable(false)
        Paint('cursor at end')   
        CheckHash(0x52EEFBBE06A52F24)
        CheckCursor(24,3)
        Key.Pressed(Left,4)
        Paint('cursor at: Hello w|o|rld')   
        CheckHash(0x52EEFBBE06A52F24)
        CheckCursor(20,3)
        Key.Pressed(Ctrl+A)
        Paint('All selected - cursor at end')   
        CheckHash(0x6CF3F30A185BD123)
        CheckCursor(24,3)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:40,h:9"), window::Flags::None);
    w.add(textfield!("'Hello world',x:1,y:1,w:38,h:1"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_select_left_right() {
    let script = "
        Paint.Enable(false)
        Paint('cursor at end')   
        CheckHash(0x118C7686A80A8D9E)
        CheckCursor(24,3)
        Key.Pressed(Shift+Left,4)
        Paint('last 4 chars selected:  123❤╬▶-|〓GDT| ')   
        CheckHash(0x8AE976432AC0CA2E)
        CheckCursor(20,3)
        Key.Pressed(Home)
        Paint('No selection, cursor on first character')   
        CheckHash(0x118C7686A80A8D9E)
        CheckCursor(13,3)
        Key.Pressed(Shift+Right,5)
        Paint('First 5 characters selected')   
        CheckHash(0xA4D17EB2D03D98B5)
        CheckCursor(18,3)
        Key.Pressed(Right)
        Key.Pressed(Shift+End)
        Paint('Last 5 characters selectedL 123❤╬▶|-〓GDT|')   
        CheckHash(0xABDC8B8ADDB65A89)
        CheckCursor(24,3)
        Key.Pressed(Left,3)
        Key.Pressed(Shift+Home)
        Paint('All but last 3 chars selected |123❤╬▶-〓|GDT')   
        CheckHash(0xDCE11FF9FF553196)
        CheckCursor(13,3)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:40,h:9"), window::Flags::None);
    w.add(textfield!("'123❤️╬▶-〓GDT',x:1,y:1,w:38,h:1"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_move_up_down() {
    let script = "
        Paint.Enable(false)
        //Error.Disable(true)
        Paint('Text: [orld ... field], cursor after field')   
        CheckHash(0x1F3E1601C2AD9D28)
        CheckCursor(22,5)
        Key.Pressed(Left,4)
        Paint('Text: [orld ... field], cursor at f|i|eld')   
        CheckHash(0x1F3E1601C2AD9D28)
        CheckCursor(18,5)
        Key.Pressed(Up)
        Paint('Text: [orld ... field], cursor at r|u|st')   
        CheckHash(0x1F3E1601C2AD9D28)
        CheckCursor(18,4)
        Key.Pressed(Up)
        Paint('Text: [orld ... field], cursor at |!|')   
        CheckHash(0x1F3E1601C2AD9D28)
        CheckCursor(18,3)
        Key.Pressed(Right,3)
        Paint('Text: [orld ... field], cursor at f|r|')   
        CheckHash(0x1F3E1601C2AD9D28)
        CheckCursor(21,3)
        Key.Pressed(Right,1)
        Paint('Text: [orld ... field], cursor at |o|m')   
        CheckHash(0x1F3E1601C2AD9D28)
        CheckCursor(22,3)
        Key.Pressed(Right,5)
        Paint('Text: [orld ... field], cursor at |r|ust')   
        CheckHash(0x1F3E1601C2AD9D28)
        CheckCursor(17,4)
        Key.Pressed(Down)
        Paint('Text: [orld ... field], cursor at |f|ield')   
        CheckHash(0x1F3E1601C2AD9D28)
        CheckCursor(17,5)
        Key.Pressed(Down)
        Paint('Text: [orld ... field], cursor after field')   
        CheckHash(0x1F3E1601C2AD9D28)
        CheckCursor(22,5)
        Key.Pressed(Up)
        Paint('Text: [orld ... field], cursor on |t|ext')   
        CheckHash(0x1F3E1601C2AD9D28)
        CheckCursor(22,4)
        Key.Pressed(Right,2)
        Key.Pressed(Up,2)
        Paint('Text: [orld ... field], cursor on wo|r|ld')   
        CheckHash(0x1F3E1601C2AD9D28)
        CheckCursor(14,3)
        Key.Pressed(Left,4)
        Paint('Text: [o world ... fie], cursor on |o| world')   
        CheckHash(0xC5BAFB611B5D4DC0)
        CheckCursor(13,3)
        Key.Pressed(Up)
        Paint('Text: [Hello world ... text], cursor on |H|ello')   
        CheckHash(0xA9FB2699A34265B7)
        CheckCursor(13,3)
        Key.Pressed(Down,2)
        Paint('Text: [Hello world ... text], cursor on |H|ello')   
        CheckHash(0xA9FB2699A34265B7)
        CheckCursor(13,5)
        Key.Pressed(Down)
        Paint('Text: [ello world ... text], cursor after text')   
        CheckHash(0xF085F207FD65B0EF)
        CheckCursor(22,5)
        Key.Pressed(Down)
        Paint('Text: [orld ... field], cursor after field')   
        CheckHash(0x1F3E1601C2AD9D28)
        CheckCursor(22,5)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:40,h:9"), window::Flags::None);
    w.add(textfield!("'Hello world ! from a rust text field',x:1,y:1,w:12,h:3"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_scroll_left_right() {
    let script = "
        Paint.Enable(false)
        //Error.Disable(true)
        Paint('Text: [ext field], cursor after field')   
        CheckHash(0x17543EE2D2FC227)
        CheckCursor(22,3)
        Key.Pressed(Left,10)
        Paint('Show: |t|ext field')   
        CheckHash(0xF71A995E91A8C393)
        CheckCursor(13,3)
        Key.Pressed(Left)
        Paint('Show: ||text fiel')   
        CheckHash(0x7E5170FC47AC8297)
        CheckCursor(13,3)
        Key.Pressed(Left)
        Paint('Show: |t| text fie')   
        CheckHash(0x7259CCA61A53D42F)
        CheckCursor(13,3)
        Key.Pressed(Left)
        Paint('Show: |s|t text fi')   
        CheckHash(0x3834B1F4E61ED745)
        CheckCursor(13,3)
        Key.Pressed(Home)
        Paint('Show: |H|ello worl')   
        CheckHash(0xCC5E5369E794A320)
        CheckCursor(13,3)
        Key.Pressed(Right,8)
        Paint('Show: Hello wo|r|l')   
        CheckHash(0xCC5E5369E794A320)
        CheckCursor(21,3)
        Key.Pressed(Right)
        Paint('Show: Hello wor|l|')   
        CheckHash(0xCC5E5369E794A320)
        CheckCursor(22,3)
        Key.Pressed(Right)
        Paint('Show: ello worl|d|')   
        CheckHash(0x98959B2900E8C32C)
        CheckCursor(22,3)
        Key.Pressed(End)
        Paint('Show: [ext field], cursor after field (2)')   
        CheckHash(0x17543EE2D2FC227)
        CheckCursor(22,3)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:40,h:9"), window::Flags::None);
    w.add(textfield!("'Hello world ! from a rust text field',x:1,y:1,w:12,h:1"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_home_end() {
    let script = "
        Paint.Enable(false)
        //Error.Disable(true)
        Paint('Text:  ▶-〓 world|| (initial state - cursor is last)')   
        CheckHash(0xB5F2856A17C1B50D)
        CheckCursor(22,3)
        Key.Pressed(Home)
        Paint('Text: |H|ello |❤|╬▶-')   
        CheckHash(0x5B8CEEE9E9B5B8F8)
        CheckCursor(13,3)
        Key.Pressed(End)
        Paint('Text:  ▶-〓 world||')   
        CheckHash(0xB5F2856A17C1B50D)
        CheckCursor(22,3)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:40,h:9"), window::Flags::None);
    w.add(textfield!("'Hello ❤️╬▶-〓 world',x:1,y:1,w:12,h:1"));
    a.add_window(w);
    a.run();
}


#[test]
fn check_delete() {
    let script = "
        Paint.Enable(false)
        //Error.Disable(true)
        Key.Pressed(Home)
        Paint('Text: [Hello ❤╬▶-]')   
        CheckHash(0x5B8CEEE9E9B5B8F8)
        CheckCursor(13,3)
        Key.Pressed(Right,4)
        Paint('Show: Hell|o| ❤╬▶-')   
        CheckHash(0x5B8CEEE9E9B5B8F8)
        CheckCursor(17,3)
        Key.Pressed(Delete)
        Paint('Show: Hell| |❤╬▶-〓')   
        CheckHash(0x9D7057333DEA1294)
        CheckCursor(17,3)
        Key.Pressed(Delete)
        Paint('Show: Hell|❤|╬▶-〓 ')   
        CheckHash(0x21EE5121805EEE60)
        CheckCursor(17,3)
        Key.Pressed(Delete)
        Paint('Show: Hell|╬|▶-〓 w')   
        CheckHash(0xE17209AE4688CF0A)
        CheckCursor(17,3)
        Key.Pressed(Delete)
        Paint('Show: Hell|▶|-〓 wo')   
        CheckHash(0xD496D44D8168F24E)
        CheckCursor(17,3)
        Key.Pressed(Delete,3)
        Paint('Show: Hell world')   
        CheckHash(0x44E4F736AFDEAEC3)
        CheckCursor(17,3)
        Key.Pressed(Delete,5)
        Paint('Show: Helld')   
        CheckHash(0xB916CB18AB8F111)
        CheckCursor(17,3)
        Key.Pressed(Delete)
        Paint('Show: Hell')   
        CheckHash(0xA6A83D08E7B5430D)
        CheckCursor(17,3)
        Key.Pressed(Home)
        Paint('Show: Hell')   
        CheckHash(0xA6A83D08E7B5430D)
        CheckCursor(13,3)
        Key.Pressed(End)
        Paint('Show: Hell')   
        CheckHash(0xA6A83D08E7B5430D)
        CheckCursor(17,3)
        Key.Pressed(Home)
        Key.Pressed(Delete,2)
        Paint('Show: ll')   
        CheckHash(0xD37A8CFC9107AD14)
        CheckCursor(13,3)
        Key.Pressed(End)
        Paint('Show: ll')   
        CheckHash(0xD37A8CFC9107AD14)
        CheckCursor(15,3)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:40,h:9"), window::Flags::None);
    w.add(textfield!("'Hello ❤️╬▶-〓 world',x:1,y:1,w:12,h:1"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_delete_from_end() {
    let script = "
        Paint.Enable(false)
        //Error.Disable(true)
        Key.Pressed(Left)
        Paint('Text: ▶-〓 worl|d|')   
        CheckHash(0xB5F2856A17C1B50D)
        CheckCursor(21,3)
        Key.Pressed(Delete)
        Paint('Text: ╬▶-〓 worl, cursor last')   
        CheckHash(0x57F85A60FF685391)
        CheckCursor(21,3)
        Key.Pressed(Delete,10)
        Paint('Text: ╬▶-〓 worl, cursor last, nothing changes')   
        CheckHash(0x57F85A60FF685391)
        CheckCursor(21,3)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:40,h:9"), window::Flags::None);
    w.add(textfield!("'Hello ❤️╬▶-〓 world',x:1,y:1,w:12,h:1"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_delete_after_selection() {
    let script = "
        Paint.Enable(false)
        //Error.Disable(true)
        Key.Pressed(Home)
        Key.Pressed(Right,2)
        Key.Pressed(Shift+Right,4)
        Paint('Text: Hello |❤|╬▶-')   
        CheckHash(0x473ABCF8BA20B908)
        CheckCursor(19,3)
        Key.Pressed(Delete)
        Paint('Text:  He|❤|╬▶-〓 wo')   
        CheckHash(0xE80B6D9DE88C7B68)
        CheckCursor(15,3)
        Key.Pressed(Delete,10)
        Paint('Text: He|d|')   
        CheckHash(0xE76A8A2CB6353D91)
        CheckCursor(15,3)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:40,h:9"), window::Flags::None);
    w.add(textfield!("'Hello ❤️╬▶-〓 world',x:1,y:1,w:12,h:1"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_backspace() {
    let script = "
        Paint.Enable(false)
        //Error.Disable(true)
        Key.Pressed(Home)
        Key.Pressed(Right,3)
        Paint('Text: Hel|l|o ❤╬▶-')   
        CheckHash(0x5B8CEEE9E9B5B8F8)
        CheckCursor(16,3)
        Key.Pressed(Backspace)
        Paint('Text:  He|l|o ❤╬▶-〓')   
        CheckHash(0xF711197C690A2243)
        CheckCursor(15,3)
        Key.Pressed(Backspace)
        Paint('Text:  H|l|o ❤╬▶-〓 ')   
        CheckHash(0x218C39E1236E15A6)
        CheckCursor(14,3)
        Key.Pressed(Backspace)
        Paint('Text:  |l|o ❤╬▶-〓 w')   
        CheckHash(0xDF2768238E2F92D5)
        CheckCursor(13,3)
        Key.Pressed(Backspace,10)
        Paint('Text:  |l|o ❤╬▶-〓 w => nothing changes')   
        CheckHash(0xDF2768238E2F92D5)
        CheckCursor(13,3)
        Key.Pressed(Right,5)
        Paint('Text: lo ❤╬|▶|-〓 w')   
        CheckHash(0xDF2768238E2F92D5)
        CheckCursor(18,3)
        Key.Pressed(Backspace)
        Paint('Text: lo ❤|▶|-〓 wo')   
        CheckHash(0x29812F27AF27CF21)
        CheckCursor(17,3)
        Key.Pressed(Backspace)
        Paint('Text: lo |▶|-〓 wor')   
        CheckHash(0xF757EDD3740156BE)
        CheckCursor(16,3)
        Key.Pressed(End)
        Paint('Text: ▶-〓 world||')   
        CheckHash(0xB5F2856A17C1B50D)
        CheckCursor(22,3)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:40,h:9"), window::Flags::None);
    w.add(textfield!("'Hello ❤️╬▶-〓 world',x:1,y:1,w:12,h:1"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_backspace_from_end() {
    let script = "
        Paint.Enable(false)
        //Error.Disable(true)
        Paint('1. Text: ▶-〓 world||')   
        CheckHash(0xB5F2856A17C1B50D)
        CheckCursor(22,3)
        Key.Pressed(Backspace)
        Paint('2.Text: ▶-〓 worl||')   
        CheckHash(0x57F85A60FF685391)
        CheckCursor(21,3)
        Key.Pressed(Backspace,5)
        Paint('3.Text: ▶-〓||')   
        CheckHash(0xDCF04CF5A595D7C3)
        CheckCursor(16,3)
        Key.Pressed(Left,7)
        Paint('4.Text: o ❤╬▶-〓')   
        CheckHash(0xEDBA26DAAFF5F06A)
        CheckCursor(13,3)
        Key.Pressed(End)
        Paint('5.Text: llo ❤╬▶-〓||')   
        CheckHash(0x777BAF7290F92F2A)
        CheckCursor(22,3)
        Key.Pressed(Backspace,3)
        Paint('6.Text: llo ❤╬||')   
        CheckHash(0x1CCCA3758557EAD9)
        CheckCursor(19,3)
        Key.Pressed(Backspace,3)
        Paint('7.Text: llo||')   
        CheckHash(0x120F4BA0A2FAD8A3)
        CheckCursor(16,3)
        Key.Pressed(Backspace)
        Paint('8.Text: ll||')   
        CheckHash(0xD37A8CFC9107AD14)
        CheckCursor(15,3)
        Key.Pressed(Backspace)
        Paint('9.Text: l||')   
        CheckHash(0x6F226838BB58E638)
        CheckCursor(14,3)
        Key.Pressed(Home)
        Paint('10.Text: |H|el')   
        CheckHash(0xD8844B5C8926B539)
        CheckCursor(13,3)
        Key.Pressed(Backspace,5)
        Paint('11.Text: |H|el (nothing changes)')   
        CheckHash(0xD8844B5C8926B539)
        CheckCursor(13,3)
        Key.Pressed(End)
        Paint('12.Text: Hel||')   
        CheckHash(0xD8844B5C8926B539)
        CheckCursor(16,3)
        Key.Pressed(Backspace,2)
        Paint('13.Text: H||')   
        CheckHash(0xD2F96D0190922BCC)
        CheckCursor(14,3)
        Key.Pressed(Backspace)
        Paint('14.Text: || (text completely deleted)')   
        CheckHash(0xA4EDA87645FBF114)
        CheckCursor(13,3)
        Key.Pressed(Backspace,10)
        Paint('15.Text: || (nothing changes)')   
        CheckHash(0xA4EDA87645FBF114)
        CheckCursor(13,3)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:40,h:9"), window::Flags::None);
    w.add(textfield!("'Hello ❤️╬▶-〓 world',x:1,y:1,w:12,h:1"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_backspace_after_selection() {
    let script = "
        Paint.Enable(false)
        Key.Pressed(Home)
        Key.Pressed(Right,2)
        Key.Pressed(Shift+Right,4)
        Paint('Text: Hello |❤|╬▶-')   
        CheckHash(0x473ABCF8BA20B908)
        CheckCursor(19,3)
        Key.Pressed(Backspace)
        Paint('Text:  He|❤|╬▶-〓 wo')   
        CheckHash(0xE80B6D9DE88C7B68)
        CheckCursor(15,3)
        Key.Pressed(Backspace,2)
        Paint('Text: ❤╬▶-〓 worl')   
        CheckHash(0xDA69921E3679D663)
        CheckCursor(13,3)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:40,h:9"), window::Flags::None);
    w.add(textfield!("'Hello ❤️╬▶-〓 world',x:1,y:1,w:12,h:1"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_write_text() {
    let script = "
        Paint.Enable(false)
        Paint('1.No text')   
        CheckHash(0x45BA2529A16D4D14)
        CheckCursor(13,3)
        Key.TypeText('Hello')
        Paint('2.Text is: Hello')   
        CheckHash(0xF2A1C652DC792B72)
        CheckCursor(18,3)
        Key.TypeText('_')
        Paint('3.Text is: ello,')   
        CheckHash(0xD94D182D41DB085)
        CheckCursor(18,3)
        Key.TypeText('world')
        Paint('4.Text is: world')   
        CheckHash(0x7C2C49BC32FC4A52)
        CheckCursor(18,3)
        Key.Pressed(Home)
        Paint('5.Text is: Hello_')   
        CheckHash(0xD77BA73FFBC7795D)
        CheckCursor(13,3)
        Key.Pressed(End)
        Paint('6.Text is: world')   
        CheckHash(0x7C2C49BC32FC4A52)
        CheckCursor(18,3)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:40,h:9"), window::Flags::None);
    w.add(textfield!("x:1,y:1,w:8,h:1"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_write_unicode_text() {
    let script = "
        Paint.Enable(false)
        Paint('1.No text')   
        CheckHash(0x45BA2529A16D4D14)
        CheckCursor(13,3)
        Key.TypeText('Hello')
        Paint('2.Text is: Hello')   
        CheckHash(0xF2A1C652DC792B72)
        CheckCursor(18,3)
        Key.TypeText('❤️')
        Paint('3.Text is: ello❤️')   
        CheckHash(0xF1B75E37ED9C08B)
        CheckCursor(18,3)
        Key.TypeText('〓rl❤️')
        Paint('4.Text is: ❤️〓rl❤️')   
        CheckHash(0x3CCD819EB95146CD)
        CheckCursor(18,3)
        Key.Pressed(Home)
        Paint('5.Text is: Hello❤️')   
        CheckHash(0x8931746B3B63B087)
        CheckCursor(13,3)
        Key.Pressed(End)
        Paint('6.Text is: ❤️〓rl❤️')   
        CheckHash(0x3CCD819EB95146CD)
        CheckCursor(18,3)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:40,h:9"), window::Flags::None);
    w.add(textfield!("x:1,y:1,w:8,h:1"));
    a.add_window(w);
    a.run();
}
#[test]
fn check_write_multiline_text() {
    let script = "
        Paint.Enable(false)
        Paint('1.No text')   
        CheckHash(0x51B7DD250957FD14)
        CheckCursor(13,3)
        Key.TypeText('Hello_world')
        Paint('2.Text is: Hello_word')   
        CheckHash(0xF1C8594065D9D73)
        CheckCursor(18,4)
        Key.TypeText('❤️')
        Paint('3.Text is: Hello_word❤️')   
        CheckHash(0x300DCC6FF4C1CB86)
        CheckCursor(13,5)
        Key.TypeText('〓rl❤️')
        Paint('4.Text is: Hello_word❤️❤️〓rl❤️')   
        CheckHash(0x770216FAF138E89A)
        CheckCursor(17,5)
        Key.TypeText('❤️')
        Paint('5.Text is: Hello_word❤️❤️〓rl❤️❤️')   
        CheckHash(0x2B30AC2B28C6526B)
        CheckCursor(18,5)
        Key.TypeText('❤️')
        Paint('6.Text is: ello_word❤️❤️〓rl❤️❤️❤️')   
        CheckHash(0x2476BC2CC3EF5C9E)
        CheckCursor(18,5)
        Key.TypeText('12345')
        Paint('7.Text is: word❤️❤️〓rl❤️❤️❤️12345')   
        CheckHash(0x581C58B9161C94E2)
        CheckCursor(18,5)
        Key.Pressed(Home)
        Paint('8.Text is: Hello_word❤️❤️〓rl❤️❤️❤️')   
        CheckHash(0xE60DAD19F94F1D86)
        CheckCursor(13,3)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:40,h:9"), window::Flags::None);
    w.add(textfield!("x:1,y:1,w:8,h:3"));
    a.add_window(w);
    a.run();
}