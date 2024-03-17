use crate::prelude::*;

#[test]
fn check_behavior() {
    let script = "
        Paint.Enable(false)
        Paint('initial state')   
        CheckHash(0x7E4294B3BDFB617F)
        CheckCursor(13,3)
        Key.TypeText(mypass)
        Paint('mypass password typed')
        CheckHash(0x1D678EBEA102AF7F)
        CheckCursor(19,3)
        Mouse.Move(20,3)
        Paint('mouse hover - tooltip visible')
        CheckHash(0xB170FB8F3BA64841)
        Mouse.Hold(20,3,left)
        Paint('Show password, no tooltip')
        CheckHash(0x7FBDE9790EAFF51C)
        Mouse.Release(20,3,left)
        Paint('Pass is hidden again, no tooltip')
        CheckHash(0x1D678EBEA102AF7F)
        Key.Pressed(Backspace,3)
        Paint('delete last 3 chars')
        CheckHash(0x4DDE3EBA59D1D595)
        Mouse.Hold(20,3,left)
        Paint('Show password (myp), no tooltip')
        CheckHash(0x2205012F056CA905)
        Mouse.Release(20,3,left)
        Paint('Hide password')
        CheckHash(0x4DDE3EBA59D1D595)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:40,h:9"), window::Flags::None);
    w.add(Password::new(Layout::new("x:1,y:1,w:30")));

    a.add_window(w);
    a.run();
}

#[test]
fn check_macro() {
    let script = "
        Paint.Enable(false)
        Paint('initial state')   
        CheckHash(0xA761B1792C262886)
        CheckCursor(16,5)
        Mouse.Move(20,5)
        Mouse.Hold(20,5,left)
        Paint('Pass: abc visible')
        CheckHash(0xBA3DB90189C89092)
        Mouse.Release(20,5,left)
        Paint('pass hidden')
        CheckHash(0xA761B1792C262886)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:40,h:9"), window::Flags::None);
    w.add(password!("x:1,y:1,w:30"));
    w.add(password!("x:1,y:3,w:30,pass:123"));

    a.add_window(w);
    a.run();
}