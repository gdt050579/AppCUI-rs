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

#[test]
fn check_horizontal_scroll_keys() {
    let script = "
        Paint.Enable(false)
        Mouse.Click(20,3,left)
        Paint('Initial state')
        CheckHash(0x48187EA7A323BEB9)
        Key.Pressed(Right,3)
        Paint('Text: long...')
        CheckHash(0x91A7B533331C0C5E)
        Key.Pressed(Right,4)
        Paint('Text ends with number 0 plus 2 spaces')
        CheckHash(0x7CB43A82FA003A0F)
        Key.Pressed(Left,2)
        Paint('Text ends with number 0')
        CheckHash(0x4D0E74FA48AD993A)
        Key.Pressed(Left,10)
        Paint('Back to initial state')
        CheckHash(0x48187EA7A323BEB9)
        Key.Pressed(Ctrl+Alt+Right)
        Paint('(2) Text ends with number 0 plus two space')
        CheckHash(0x7CB43A82FA003A0F)
        Key.Pressed(End)
        Paint('(2) Text ends with number 99 plus one space')
        CheckHash(0x65673E7122B34563)
        Key.Pressed(Ctrl+Alt+Left)
        Paint('Text: My long 0 textual ...')
        CheckHash(0xF552C5DD89A89A58)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Test,d:c,w:35,h:11,flags: Sizeable");
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
fn check_horizontal_scroll_keys_no_checkboxes() {
    let script = "
        Paint.Enable(false)
        Mouse.Click(20,3,left)
        Paint('Initial state')
        CheckHash(0x8E353010059659F2)
        Key.Pressed(Right,3)
        Paint('Text: long...')
        CheckHash(0xF37B858D1F32F6E6)
        Key.Pressed(Right,4)
        Paint('Text ends with number 0 plus 2 spaces')
        CheckHash(0xA35208251E5B262)
        Key.Pressed(Left,2)
        Paint('Text ends with number 0')
        CheckHash(0xEFA93D65A82F9DDE)
        Key.Pressed(Left,10)
        Paint('Back to initial state')
        CheckHash(0x8E353010059659F2)
        Key.Pressed(Ctrl+Alt+Right)
        Paint('(2) Text ends with number 0 plus two space')
        CheckHash(0xA35208251E5B262)
        Key.Pressed(End)
        Paint('(2) Text ends with number 99 plus one space')
        CheckHash(0x275F91C95C024736)
        Key.Pressed(Ctrl+Alt+Left)
        Paint('Text: My long 0 textual ...')
        CheckHash(0xB5EF8D0830462D7F)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Test,d:c,w:35,h:11,flags: Sizeable");
    let mut p = panel!("Test,l:1,t:1,b:1,r:1");
    let mut l = ListBox::new(Layout::new("d:c,w:100%,h:100%"),listbox::Flags::ScrollBars);
    for i in 0..100 {
        l.add(&format!("My long {} textual item number {}",i%11,i));
    }
    l.set_components_toolbar_margins(2, 0);
    p.add(l);
    w.add(p);
    a.add_window(w);
    a.run();
}