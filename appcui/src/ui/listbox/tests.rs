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
fn check_create_with_macro_1() {
    let script = "
        Paint.Enable(false)
        Mouse.Click(10,3,left)
        Paint('Initial state')
        CheckHash(0x6063E984F2B99F35)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Test,d:c,w:50,h:11,flags: Sizeable");
    let mut p = panel!("Test,l:1,t:1,b:1,r:1");
    let mut l = listbox!("d:c,w:100%,h:100%,flags: ScrollBars+CheckBoxes+SearchBar, lsm:2");
    for i in 0..100 {
        l.add(&format!("My long {} textual item number {}",i%11,i));
    }
    p.add(l);
    w.add(p);
    a.add_window(w);
    a.run();
}

#[test]
fn check_create_with_macro_2() {
    let script = "
        Paint.Enable(false)
        Mouse.Click(10,3,left)
        Paint('Initial state')
        CheckHash(0xB00B42A9B1771A31)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Test,d:c,w:50,h:11,flags: Sizeable");
    let mut p = panel!("Test,l:1,t:1,b:1,r:1");
    let l = listbox!("d:c,w:100%,h:100%,flags: ScrollBars, lsm:2, items:[Red,Gree,Blue,White,Black,Orange,Yellow,Purple]");
    p.add(l);
    w.add(p);
    a.add_window(w);
    a.run();
}

#[test]
fn check_create_with_macro_3() {
    let script = "
        Paint.Enable(false)
        Paint('Initial state')
        CheckHash(0x7C90380CED317D30)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Test,d:c,w:40,h:8,flags: Sizeable");
    let l = listbox!("d:c,w:100%,h:100%,index:3,flags: ScrollBars, lsm:2, items:[Red,Gree,Blue,White,Black,Orange,Yellow,Purple]");
    w.add(l);
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

#[test]
fn check_search() {
    let script = "
        Paint.Enable(false)
        Mouse.Click(10,3,left)
        Paint('Initial state')
        CheckHash(0x6063E984F2B99F35)
        Key.Pressed(3)
        Paint('item with number 3 selected')
        CheckHash(0x77F2B2A16DC57592)
        Key.Pressed(Enter)
        Paint('item with number 13 selected')
        CheckHash(0x6A06D517F0AFD720)
        Key.Pressed(Enter)
        Paint('My long 3 textual item number 14')
        CheckHash(0x839A3B3FE1C88B62)
        Key.Pressed(Enter)
        Paint('item with number 23 selected')
        CheckHash(0xF8FAEE7802252B2B)
        CheckCursor(12,8)
        Key.Pressed(4)
        Paint('item with number 34 selected')
        CheckHash(0xA310DECEF6DAD532)
        CheckCursor(13,8)
        Key.Pressed(Enter)
        Paint('item with number 34 remains selected')
        CheckHash(0xA310DECEF6DAD532)
        CheckCursor(13,8)
        Key.Pressed(Up)
        Paint('item with number 33 selected')
        CheckHash(0x321DDD364829236)
        CheckCursor(hidden)
        Key.Pressed(Backspace,2)
        Paint('item with number 33 selected, nothing on filter')
        CheckHash(0xC67D8535BB675FCC)
        CheckCursor(11,8)
        Key.TypeText('textual')
        Paint('99+ matches, item with number 33 remains selected')
        CheckHash(0xB326140771C7B175)
        CheckCursor(18,8)
        Key.TypeText(' item')
        Paint('99+ matches, filter contains: ual item')
        CheckHash(0xB9D4D59DB11028AD)
        CheckCursor(19,8)
        Key.Pressed(Backspace)
        Paint('99+ matches, filter contains: tual ite')
        CheckHash(0x557FB0D9EF43F0D4)
        CheckCursor(19,8)
        Key.Pressed(Escape)
        Paint('Filter is cleared, item with number 33 remains selected')
        CheckHash(0xD14E71E597E07A1C)
        CheckCursor(hidden)
        Key.Pressed(Escape)
        Paint('Window is closed (in non-edit mode Escape is not proccessed)')
        CheckHash(0x3900AF2CBDF4157D)
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
fn check_resize() {
    let script = "
        Paint.Enable(false)
        Mouse.Click(10,3,left)
        Paint('Initial state')
        CheckHash(0x6063E984F2B99F35)
        Key.TypeText('textual item')
        Paint('99+ matches, filter contains: ual item')
        CheckHash(0x1AAEFD071EA0A6AC)
        CheckCursor(19,8)    
        Mouse.Hold(54,10,left)    
        Mouse.Move(53,10)
        Paint('1. resize -1')
        CheckHash(0x62CCB6C4ADF7A944)
        Mouse.Move(43,10)
        Paint('2. resize -11 (hscrollbar enabeled)')
        CheckHash(0xA613C1648737F792)
        Mouse.Move(33,10)
        Paint('3. resize -21 (hscrollbar 4 characters)')
        CheckHash(0xB26C43B1BE3E2AAC)
        Mouse.Move(32,10)
        Paint('4. resize -22 (searchbar: |al item 99+|)')
        CheckHash(0xA79F143C4D6A2A24)
        CheckCursor(18,8)
        Mouse.Move(30,10)
        Paint('5. resize -24 (searchbar: | item 99+|)')
        CheckHash(0xCC8BDA112FEA5274)
        CheckCursor(16,8)
        Mouse.Move(29,10)
        Paint('6. resize -25 (searchbar: |item 99+|)')
        CheckHash(0x8AA1B5E6E52D57FC)
        CheckCursor(15,8)
        Mouse.Move(28,10)
        Paint('7. resize -26 (searchbar: |tem 99+|)')
        CheckHash(0xA5EFB2617AC4A6D9)
        CheckCursor(14,8)
        Mouse.Move(27,10)
        Paint('8. resize -27 (searchbar: |l item| - count is not shown)')
        CheckHash(0xBD85603D712F9F21)
        CheckCursor(17,8)
        Mouse.Move(25,10)
        Paint('9. resize -29 (searchbar: |item| - count is not shown)')
        CheckHash(0x10203BA9F674373C)
        CheckCursor(15,8)
        Mouse.Move(23,10)
        Paint('10. resize -31 (searchbar is not visible)')
        CheckHash(0x815B0867843C1DE0)
        CheckCursor(hidden)
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
fn check_search_ignore_case() {
    let script = "
        Paint.Enable(false)
        Paint('Initial state')
        CheckHash(0xB158DD83CDC989A2)
        Key.TypeText('green')
        Paint('4 matches, all green (now at Green)')
        CheckHash(0x82A41F9E2DCA2055)
        CheckCursor(14,10)
        Key.Pressed(Enter)
        Paint('4 matches, all green (now at dark green)')
        CheckHash(0x1A61E75B7D994F7E)
        CheckCursor(14,10)
        Key.Pressed(Enter)
        Paint('4 matches, all green (now at light GREEn)')
        CheckHash(0x8C1D65C5086980E9)
        CheckCursor(14,10)
        Key.Pressed(Enter)
        Paint('4 matches, all green (now at Special GrEeN)')
        CheckHash(0xB1A0CB915D5EC26D)
        CheckCursor(14,10)
        Key.Pressed(Enter)
        Paint('4 matches, all green (back to initial Green)')
        CheckHash(0x82A41F9E2DCA2055)
        CheckCursor(14,10)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Test,d:c,w:50,h:11,flags: Sizeable");
    let l = listbox!("d:c,w:100%,h:100%,flags: ScrollBars+CheckBoxes+SearchBar, lsm:2, items:[Red,Green,'dark green', 'light GREEn',White,'Special GrEeN',Black,Orange,Yellow,Purple]");    
    w.add(l);
    a.add_window(w);
    a.run();
}
