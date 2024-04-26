use crate::prelude::*;

#[test]
fn check_move_left_right() {
    let script = "
        Paint.Enable(false)
        // pressed Right arrow to remove selection
        Key.Pressed(Right)
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
        // pressed Right arrow to remove selection
        Key.Pressed(Right)
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
        // pressed Right arrow to remove selection
        Key.Pressed(Right)
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
        // pressed Right arrow to remove selection
        Key.Pressed(Right)
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
        // pressed Right arrow to remove selection
        Key.Pressed(Right)
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
        // pressed Right arrow to remove selection
        Key.Pressed(Right)
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
        // pressed Right arrow to remove selection
        Key.Pressed(Right)
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

#[test]
fn check_insert_text() {
    let script = "
        Paint.Enable(false)
        Paint('1.Initial state (llo world)')   
        CheckHash(0x483E309A3C03D9D2)
        CheckCursor(22,3)
        Key.Pressed(Home)
        Key.Pressed(Right,2)        
        Key.TypeText('123456')
        Paint('2.Text: He123456|l|l')   
        CheckHash(0x616CCD19CA5D1D2A)
        CheckCursor(21,3)
        Key.Pressed(Backspace,6)
        Paint('3.Text: He|l|lo worl')   
        CheckHash(0xCC5E5369E794A320)
        CheckCursor(15,3)
        Key.TypeText('〓rl❤️❤️❤️')
        Paint('4.Text: He〓rl❤❤❤|l|l')   
        CheckHash(0x9A47A09DA4CFEA69)
        CheckCursor(21,3)
        Key.Pressed(End)
        Paint('4.Text: llo world||')   
        CheckHash(0x13D74C30632D286D)
        CheckCursor(22,3)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:40,h:9"), window::Flags::None);
    w.add(textfield!("'Hello world',x:1,y:1,w:12,h:1"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_insert_text_over_selection() {
    let script = "
        Paint.Enable(false)
        //Error.Disable(true)
        Key.Pressed(Home)
        Key.Pressed(Right,2) 
        Paint('1.Initial state (He|l|lo worl)')   
        CheckHash(0xCC5E5369E794A320)
        CheckCursor(15,3)
        Key.Pressed(Shift+Right,4)
        Paint('2.Selected [llo ] (Hello |w|orl)')   
        CheckHash(0xA8CA2E147025C590)
        CheckCursor(19,3)
        Key.TypeText('〓❤️❤️❤️')
        Paint('3.Text replaces (He〓❤❤❤|w|orl)')   
        CheckHash(0x2D25F2AE8CA02B9D)
        CheckCursor(19,3)
        Key.Pressed(Right)
        Paint('4.Move cursor to right (He〓❤❤❤w|o|rl)')   
        CheckHash(0x2D25F2AE8CA02B9D)
        CheckCursor(20,3)
        Key.Pressed(Right)
        Paint('5.Move cursor to right (He〓❤❤❤wo|r|l)')   
        CheckHash(0x2D25F2AE8CA02B9D)
        CheckCursor(21,3)
        Key.Pressed(Right)
        Paint('6.Move cursor to right (He〓❤❤❤wor|l|)')   
        CheckHash(0x2D25F2AE8CA02B9D)
        CheckCursor(22,3)
        Key.Pressed(Right)
        Paint('7.Move cursor to right (e〓❤❤❤worl|d|)')   
        CheckHash(0x3D405AF32EA72405)
        CheckCursor(22,3)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:40,h:9"), window::Flags::None);
    w.add(textfield!("'Hello world',x:1,y:1,w:12,h:1"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_move_to_next_word() {
    let script = "
        Paint.Enable(false)
        Key.Pressed(Home)
        Paint('1.Visible:  |H|ello   world,〓❤,123,  test.set')   
        CheckHash(0x2D7A431271FF0AE0)
        CheckCursor(13,3)
        Key.Pressed(Ctrl+Right)
        Paint('2.Visible:  Hello   |w|orld,〓❤,123,  test.set')   
        CheckHash(0x2D7A431271FF0AE0)
        CheckCursor(21,3)
        Key.Pressed(Left,2)
        Paint('3.Visible:  Hello | | world,〓❤,123,  test.set')   
        CheckHash(0x2D7A431271FF0AE0)
        CheckCursor(19,3)
        Key.Pressed(Ctrl+Right)
        Paint('4.Visible:  Hello   |w|orld,〓❤,123,  test.set')   
        CheckHash(0x2D7A431271FF0AE0)
        CheckCursor(21,3)
        Key.Pressed(Ctrl+Right)
        Paint('5.Visible:  Hello   world|,|〓❤,123,  test.set')   
        CheckHash(0x2D7A431271FF0AE0)
        CheckCursor(26,3)
        Key.Pressed(Ctrl+Right)
        Paint('6.Visible:  Hello   world,|〓|❤,123,  test.set')   
        CheckHash(0x2D7A431271FF0AE0)
        CheckCursor(27,3)
        Key.Pressed(Ctrl+Right)
        Paint('7.Visible:  Hello   world,〓❤|,|123,  test.set')   
        CheckHash(0x2D7A431271FF0AE0)
        CheckCursor(29,3)
        Key.Pressed(Ctrl+Right)
        Paint('8.Visible:  Hello   world,〓❤,|1|23,  test.set')   
        CheckHash(0x2D7A431271FF0AE0)
        CheckCursor(30,3)
        Key.Pressed(Ctrl+Right)
        Paint('9.Visible:  Hello   world,〓❤,123|,|  test.set')   
        CheckHash(0x2D7A431271FF0AE0)
        CheckCursor(33,3)
        Key.Pressed(Ctrl+Right)
        Paint('10.Visible:  Hello   world,〓❤,123,  |t|est.set')   
        CheckHash(0x2D7A431271FF0AE0)
        CheckCursor(36,3)
        Key.Pressed(Ctrl+Right)
        Paint('11.Visible:  Hello   world,〓❤,123,  test|.|set')   
        CheckHash(0x2D7A431271FF0AE0)
        CheckCursor(40,3)
        Key.Pressed(Ctrl+Right)
        Paint('12.Visible:  Hello   world,〓❤,123,  test.|s|et')   
        CheckHash(0x2D7A431271FF0AE0)
        CheckCursor(41,3)
        Key.Pressed(Ctrl+Right)
        Paint('13.Visible:  lo   world,〓❤,123,  test.set    |u|')   
        CheckHash(0xE5B31249CF6A1A98)
        CheckCursor(45,3)
        Key.Pressed(Right,2)
        Paint('14.Visible:   world,〓❤,123,  test.set    un|i|')   
        CheckHash(0xC78D0EC4D48FBC30)
        CheckCursor(45,3)
        Key.Pressed(Ctrl+Right)
        Paint('15.Visible:  ,〓❤,123,  test.set    uni〓code  |t|')   
        CheckHash(0x1F540C31A3C9129C)
        CheckCursor(45,3)
        Key.Pressed(Ctrl+Right)
        // reach the end of the text
        Paint('16.Visible:   test.set    uni〓code  twice   ')   
        CheckHash(0x188AC9EB95C89126)
        CheckCursor(45,3)
        Key.Pressed(Ctrl+Right)
        Paint('17.Visible:   test.set    uni〓code  twice   (nothing happens)')   
        CheckHash(0x188AC9EB95C89126)
        CheckCursor(45,3)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:40,h:9"), window::Flags::None);
    w.add(textfield!("'Hello   world,〓❤,123,  test.set    uni〓code  twice   ',x:1,y:1,w:35,h:1"));
    a.add_window(w);
    a.run();
}


#[test]
fn check_move_to_previous_word() {
    let script = "
        Paint.Enable(false)
        //Error.Disable(true)
        Key.Pressed(End)
        Paint('1.Visible:    test.set    uni〓code  twice    ||')   
        CheckHash(0x188AC9EB95C89126)
        CheckCursor(45,3)
        Key.Pressed(Ctrl+Left)
        Paint('2.Visible:   test.set    uni〓code  |t|wice    ')   
        CheckHash(0x188AC9EB95C89126)
        CheckCursor(37,3)
        Key.Pressed(Left,1)
        Paint('3.Visible:   test.set    uni〓code  | |twice    ')   
        CheckHash(0x188AC9EB95C89126)
        CheckCursor(36,3)
        Key.Pressed(Ctrl+Left)
        Paint('4.Visible:   test.set    |u|ni〓code  twice   ')   
        CheckHash(0x188AC9EB95C89126)
        CheckCursor(27,3)
        Key.Pressed(Ctrl+Left)
        Paint('5.Visible:   test.|s|et    uni〓code  twice   ')   
        CheckHash(0x188AC9EB95C89126)
        CheckCursor(20,3)
        Key.Pressed(Ctrl+Left)
        Paint('6.Visible:   test|.|set    uni〓code  twice   ')   
        CheckHash(0x188AC9EB95C89126)
        CheckCursor(19,3)
        Key.Pressed(Ctrl+Left)
        Paint('7.Visible:  |t|est.set    uni〓code  twice   ')   
        CheckHash(0x188AC9EB95C89126)
        CheckCursor(15,3)
        Key.Pressed(Ctrl+Left)
        Paint('8.Visible:  |,|  test.set    uni〓code  twice   ')   
        CheckHash(0x85C9E5D26DF4B442)
        CheckCursor(13,3)
        Key.Pressed(Ctrl+Left)
        Paint('9.Visible:  |1|23,  test.set    uni〓code  twicet')   
        CheckHash(0x1F31A3D060CF832)
        CheckCursor(13,3)
        Key.Pressed(Ctrl+Left)
        Paint('10.Visible: |,|123,  test.set    uni〓code  twic')   
        CheckHash(0xC9CD5299AA782443)
        CheckCursor(13,3)
        Key.Pressed(Ctrl+Left)
        Paint('11.Visible:  |〓|❤,123,  test.set    uni〓code  tw')   
        CheckHash(0xD8427A56B47AB583)
        CheckCursor(13,3)
        Key.Pressed(Ctrl+Left)
        Paint('12.Visible:  |,|〓❤,123,  test.set    uni〓code  t')   
        CheckHash(0x1F540C31A3C9129C)
        CheckCursor(13,3)
        Key.Pressed(Ctrl+Left)
        Paint('13.Visible:  |w|orld,〓❤,123,  test.set    uni〓co')   
        CheckHash(0x39319158125831AB)
        CheckCursor(13,3)
        Key.Pressed(Left,2)
        Paint('14.Visible: || world,〓❤,123,  test.set    uni〓')   
        CheckHash(0x7B07EF88A003B02F)
        CheckCursor(13,3)
        Key.Pressed(Ctrl+Left)
        Paint('15.Visible:  |H|ello   world,〓❤,123,  test.set')   
        CheckHash(0x2D7A431271FF0AE0)
        CheckCursor(13,3)
        Key.Pressed(Ctrl+Left)
        // now we reach the start of the test
        Paint('16.Visible: ||   Hello   world,〓❤,123,  test.set')   
        CheckHash(0xC7E06F2FFB5E0060)
        CheckCursor(13,3)
        Key.Pressed(Ctrl+Left)
        Paint('17.Visible: ||   Hello   world,〓❤,123,  test.set (nothing happens)')   
        CheckHash(0xC7E06F2FFB5E0060)
        CheckCursor(13,3)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:40,h:9"), window::Flags::None);
    w.add(textfield!("'  Hello   world,〓❤,123,  test.set    uni〓code  twice   ',x:1,y:1,w:35,h:1"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_readonly_flag() {
    let script = "
        Paint.Enable(false)
        Key.Pressed(Left,4)
        Paint('1.Visible: Hello   w|o|rld')   
        CheckHash(0x320354CF1FEFAEB4)
        CheckCursor(22,3)
        Key.Pressed(Backspace,10)
        Key.Pressed(Delete,10)
        Key.TypeText('hello')
        Paint('2.Visible: Hello   w|o|rld (nothing changes)')   
        CheckHash(0x320354CF1FEFAEB4)
        CheckCursor(22,3)
        Key.Pressed(Shift+Right,2)
        Key.Pressed(Delete,10)
        Key.TypeText('hello')
        Paint('3.Visible: Hello   wor|l|d (text not modified)')   
        CheckHash(0x1CD049E1CD927118)
        CheckCursor(24,3)
        Key.Pressed(Ctrl+Shift+U)
        Paint('4.Visible: Hello   wor|l|d (text not modified) no UPCASE')   
        CheckHash(0x1CD049E1CD927118)
        CheckCursor(24,3)
        Key.Pressed(Ctrl+U)
        Paint('5.Visible: Hello   wor|l|d (text not modified) no LOWERCASE')   
        CheckHash(0x1CD049E1CD927118)
        CheckCursor(24,3)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:40,h:9"), window::Flags::None);
    w.add(textfield!("'Hello   world',x:1,y:1,w:20,h:1,flags: ReadOnly"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_autoselect_on_focus() {
    let script = "
        Paint.Enable(false)
        Paint('Text-3 focused')   
        CheckHash(0xBD293603FEBD93F4)
        Key.Pressed(Tab)
        Paint('Text-1 focused (and selected)')   
        CheckHash(0x67FEAAC03BEBCF24)
        Key.Pressed(Tab)
        Paint('Text-2 focused (and selected)')   
        CheckHash(0x318BE55CDA07E3FC)
        Key.Pressed(Tab)
        Paint('Text-3 focused (no selection)')   
        CheckHash(0xBD293603FEBD93F4)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:40,h:9"), window::Flags::None);
    w.add(textfield!("'Text-1',x:1,y:1,w:30,h:1,flags: ReadOnly"));
    w.add(textfield!("'Text-2',x:1,y:3,w:30,h:1"));
    w.add(textfield!("'Text-3',x:1,y:5,w:30,h:1,flags: DisableAutoSelectOnFocus"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_validation_event() {
    #[Window(events = TextFieldEvents, internal=true)]
    struct MyWin {
        info: Handle<Label>,
        txt: Handle<TextField>,
    }
    impl MyWin {
        fn new() -> Self {
            let mut me = Self {
                base: Window::new("Win", Layout::new("d:c,w:47,h:7"), window::Flags::None),
                info: Handle::None,
                txt: Handle::None,
            };
            me.info = me.add(label!("'',x:1,y:1,w:35"));
            me.txt = me.add(textfield!("x:1,y:3,w:35,flags:ProcessEnter"));
            me
        }
        fn set_info(&mut self, txt: &str) {
            let h_label = self.info;
            if let Some(label) = self.control_mut(h_label) {
                label.set_caption(txt);
            }
        }
    }
    impl TextFieldEvents for MyWin {
        fn on_validate(&mut self, _handle: Handle<TextField>, text: &str) -> EventProcessStatus {
            self.set_info(text);
            EventProcessStatus::Processed
        }
    }

    let script = "
        Paint.Enable(false)
        Paint('Initial state')   
        CheckHash(0x615B7D42C0680A1E)   
        Key.TypeText('Hello world')
        Paint('Hello world') 
        CheckHash(0x7762DE5EE067C87E) 
        Key.Pressed(Enter)
        Paint('Label contains: Hello world')
        CheckHash(0x5E0D88141ECF26FD) 
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}


#[test]
fn check_mouse_click() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')   
        CheckHash(0xCA13991A964D78E7)
        CheckCursor(25,3)
        Mouse.Click(17,3,left)
        Paint('2. Nothing selected, cursor on |o|')   
        CheckHash(0x9D7022C6AA5D3FFB)
        CheckCursor(17,3)
        Mouse.Click(30,3,left)
        Paint('3. Nothing selected, cursor at the end')   
        CheckHash(0x9D7022C6AA5D3FFB)
        CheckCursor(25,3)
        Key.TypeText('Rust is a great language')
        Paint('4. Text: a great language')   
        CheckHash(0xCA75D9C93B27BEBE)
        CheckCursor(30,3)
        Mouse.Click(16,3,left)
        Paint('5. Text: a |g|reat language')   
        CheckHash(0xCA75D9C93B27BEBE)
        CheckCursor(16,3)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:40,h:9"), window::Flags::None);
    w.add(textfield!("'Hello w❤️rl❤️d',x:1,y:1,w:20,h:1"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_mouse_click_multi_line() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')   
        CheckHash(0x1D939C7615CD409F)
        CheckCursor(17,4)
        Mouse.Click(17,3,left)
        Paint('2. Nothing selected, cursor on |o|')   
        CheckHash(0xF2E5B0D71CC946A3)
        CheckCursor(17,3)
        Mouse.Click(20,5,left)
        Paint('3. Nothing selected, cursor at the end')   
        CheckHash(0xF2E5B0D71CC946A3)
        CheckCursor(17,4)
        Key.TypeText('Rust is a great language')
        Paint('4. Text: ust is a great language')   
        CheckHash(0x82AAADB5012BC38E)
        CheckCursor(20,5)
        Mouse.Click(17,4,left)
        Paint('5. Text: ust is a gre|a|t language')   
        CheckHash(0x82AAADB5012BC38E)
        CheckCursor(17,4)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:40,h:9"), window::Flags::None);
    w.add(textfield!("'Hello w❤️rl❤️d',x:1,y:1,w:10,h:3"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_mouse_click_outside_bounds() {
    let script = "
        Paint.Enable(false)
        //Error.Disable(true)
        Paint('1. Initial state')   
        CheckHash(0xCA13991A964D78E7)
        CheckCursor(25,3)
        Mouse.Click(17,3,left)
        Paint('2. Nothing selected, cursor on |o|')   
        CheckHash(0x9D7022C6AA5D3FFB)
        CheckCursor(17,3)
        Mouse.Click(12,3,left)
        Paint('3. Nothing happens (click outside bounds) cursor remains the same')   
        CheckHash(0x9D7022C6AA5D3FFB)
        CheckCursor(17,3)
        Mouse.Click(31,3,left)
        Paint('4. Nothing happens (click outside bounds) cursor remains the same')   
        CheckHash(0x9D7022C6AA5D3FFB)
        CheckCursor(17,3)
        Mouse.Click(30,3,left)
        Paint('5. Now cursor at end')   
        CheckHash(0x9D7022C6AA5D3FFB)
        CheckCursor(25,3)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:40,h:9"), window::Flags::None);
    w.add(textfield!("'Hello w❤️rl❤️d',x:1,y:1,w:20,h:1"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_mouse_selection() {
    let script = "
        Paint.Enable(false)
        Key.Pressed(Right)
        Paint('1. Text: - I ❤️ Rust Language :)')   
        CheckHash(0x480A380B10C2094C)
        CheckCursor(20,5)
        Mouse.Drag(20,3,17,5)
        Paint('2. Selected text: Rust Language')   
        CheckHash(0xD374BBDDCBFB9EFB)
        CheckCursor(17,5)
        Mouse.Drag(18,3,10,2)
        Paint('3. Selected text: ello w❤️rl❤️d - I ')   
        CheckHash(0x57E8868A7830C0DE)
        CheckCursor(13,3)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:40,h:9"), window::Flags::None);
    w.add(textfield!("'Hello w❤️rl❤️d - I ❤️ Rust Language :)',x:1,y:1,w:10,h:3"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_double_click_selection() {
    let script = "
        Paint.Enable(false)
        Key.Pressed(Right)
        Paint('1. Text: - I ❤️ Rust Language :)')   
        CheckHash(0xC612D5EC66D395F)
        CheckCursor(24,5)
        Mouse.DoubleClick(17,5,left)
        Paint('2. Selected text: Language')   
        CheckHash(0xC500029C8BF4A50B)
        CheckCursor(21,5)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:40,h:9"), window::Flags::None);
    w.add(textfield!("'Hello w❤️rl❤️d - I ❤️ Rust Language :)',x:1,y:1,w:14,h:3"));
    a.add_window(w);
    a.run();
}


#[test]
fn check_select_word_for_upper_and_lowercase() {
    let script = "
        Paint.Enable(false)
        Key.Pressed(Home)
        Key.Pressed(Right,8)
        Paint('1. Hello world, I ❤ Rust Language :)')   
        CheckHash(0x933F4645D825CC91 )
        CheckCursor(21,3)
        Key.Pressed(Ctrl+U)
        Paint('2. Hello world, I ❤ Rust Language :)')   
        CheckHash(0xAED543050B90DB6)
        CheckCursor(24,3)
        Key.Pressed(Ctrl+Shift+U)
        Paint('3. Hello WORLD, I ❤ Rust Language :)')   
        CheckHash(0x350A1BC25797ADD6)
        CheckCursor(24,3)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:40,h:9"), window::Flags::None);
    w.add(textfield!("'Hello wOrLd, I ❤️ Rust Language :)',x:1,y:1,w:36,h:1"));
    a.add_window(w);
    a.run();
}