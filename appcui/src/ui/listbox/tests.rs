use crate::prelude::*;


#[test]
fn check_create() {
    let script = "
        Paint.Enable(false)
        Mouse.Click(10,3,left)
        Paint('Initial state')
        CheckHash(0x6063E984F2B99F35)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Test,d:c,w:50,h:11,flags: Sizeable");
    let mut p = panel!("Test,l:1,t:1,b:1,r:1");
    let mut l = ListBox::new(Layout::new("d:c,w:100%,h:100%"),listbox::Flags::ScrollBars|listbox::Flags::CheckBoxes|listbox::Flags::SearchBar);
    for i in 0..100 {
        l.add(&format!("My long {} textual item number {}",i%11,i));
    }
    l.set_components_toolbar_margins(2, 0);
    p.add(l);
    w.add(p);
    a.add_window(w);
    a.run();
}

#[test]
fn check_movement_keys() {
    let script = "
        Paint.Enable(false)
        Mouse.Click(10,3,left)
        Paint('Initial state')
        CheckHash(0x6063E984F2B99F35)
        Key.Pressed(Down)
        Paint('Item 1 selected')
        CheckHash(0xAC9889BE6AB84F91)
        Key.Pressed(Down,10)
        Paint('Item 11 selected')
        CheckHash(0xA3883875FC9A3271)
        Key.Pressed(PageDown)
        Paint('Item 16 selected')
        CheckHash(0x2FBC55EEB1DDF27F)
        Key.Pressed(PageUp)
        Paint('Item 11 selected (first from the page)')
        CheckHash(0xA1ED77DFFAE5B01)
        Key.Pressed(Home)
        Paint('Back to initial state')
        CheckHash(0x6063E984F2B99F35)
        Key.Pressed(End)
        Paint('Item 99 selected')
        CheckHash(0xBB5E6CFADECD2695)
        Key.Pressed(Up,2)
        Paint('Item 97 selected')
        CheckHash(0x7D4E807C521DA5D)
        Key.Pressed(Enter)
        Paint('Item 97 Checked')
        CheckHash(0xBC6601936E1716C1)
        Key.Pressed(Space)
        Paint('Item 97 un-Checked')
        CheckHash(0x7D4E807C521DA5D)
        Key.Pressed(Ctrl+Alt+Up)
        Paint('Item 97 selected, scroll starts from item 94')
        CheckHash(0x485E17D81967435A)
        Key.Pressed(Ctrl+Alt+Up,2)
        Paint('No visible selection, scroll starts from item 92')
        CheckHash(0x8434A20459C1FA52)
        Key.Pressed(Ctrl+Alt+Down,4)
        Paint('Item 97 selected, scroll starts from item 95')
        CheckHash(0x7D4E807C521DA5D)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Test,d:c,w:50,h:11,flags: Sizeable");
    let mut p = panel!("Test,l:1,t:1,b:1,r:1");
    let mut l = ListBox::new(Layout::new("d:c,w:100%,h:100%"),listbox::Flags::ScrollBars|listbox::Flags::CheckBoxes|listbox::Flags::SearchBar);
    for i in 0..100 {
        l.add(&format!("My long {} textual item number {}",i%11,i));
    }
    l.set_components_toolbar_margins(2, 0);
    p.add(l);
    w.add(p);
    a.add_window(w);
    a.run();
}