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