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
    let mut l = ListBox::new(
        Layout::new("d:c,w:100%,h:100%"),
        listbox::Flags::ScrollBars | listbox::Flags::CheckBoxes | listbox::Flags::SearchBar,
    );
    for i in 0..100 {
        l.add(&format!("My long {} textual item number {}", i % 11, i));
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
        l.add(&format!("My long {} textual item number {}", i % 11, i));
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
        Paint('Initial state (White is selected,Red is the first item)')
        CheckHash(0xE2D9A09340BE3EFD)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Test,d:c,w:40,h:8,flags: Sizeable");
    let l = listbox!("d:c,w:100%,h:100%,index:3,flags: ScrollBars, lsm:2, items:[Red,Gree,Blue,White,Black,Orange,Yellow,Purple]");
    w.add(l);
    a.add_window(w);
    a.run();
}

#[test]
fn check_create_with_macro_4() {
    let script = "
        Paint.Enable(false)
        Paint('Initial state (White is selected,Red is the first item)')
        CheckHash(0xAD52FAF15DEBDF73)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Test,d:c,w:40,h:8,flags: Sizeable");
    let l = listbox!("d:c,w:100%,h:100%,index:3,flags: SearchBar, lsm:2, items:[Red,Gree,Blue,White,Black,Orange,Yellow,Purple]");
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
    let mut l = ListBox::new(
        Layout::new("d:c,w:100%,h:100%"),
        listbox::Flags::ScrollBars | listbox::Flags::CheckBoxes | listbox::Flags::SearchBar,
    );
    for i in 0..100 {
        l.add(&format!("My long {} textual item number {}", i % 11, i));
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
    let mut l = ListBox::new(
        Layout::new("d:c,w:100%,h:100%"),
        listbox::Flags::ScrollBars | listbox::Flags::CheckBoxes | listbox::Flags::SearchBar,
    );
    for i in 0..100 {
        l.add(&format!("My long {} textual item number {}", i % 11, i));
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
    let mut l = ListBox::new(Layout::new("d:c,w:100%,h:100%"), listbox::Flags::ScrollBars);
    for i in 0..100 {
        l.add(&format!("My long {} textual item number {}", i % 11, i));
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
    let mut l = ListBox::new(
        Layout::new("d:c,w:100%,h:100%"),
        listbox::Flags::ScrollBars | listbox::Flags::CheckBoxes | listbox::Flags::SearchBar,
    );
    for i in 0..100 {
        l.add(&format!("My long {} textual item number {}", i % 11, i));
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
    let mut l = ListBox::new(
        Layout::new("d:c,w:100%,h:100%"),
        listbox::Flags::ScrollBars | listbox::Flags::CheckBoxes | listbox::Flags::SearchBar,
    );
    for i in 0..100 {
        l.add(&format!("My long {} textual item number {}", i % 11, i));
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

#[test]
fn check_search_use_space() {
    let script = "
        Paint.Enable(false)
        Paint('Initial state')
        CheckHash(0x711F311CB5D1D016)
        Key.Pressed(Space)
        Paint('Red is checked')
        CheckHash(0xA2566D97A4924322)
        Key.Pressed(K)
        Paint('Edit mode - dark green is selected (2 matches)')
        CheckHash(0xB7E19FA73A5AFB7A)
        Key.Pressed(Backspace)
        Key.Pressed(Space)
        Paint('Edit mode - dark green is selected (3 matches that contain space)')
        CheckHash(0x716D450967818537)
        Key.Pressed(Down)
        Key.Pressed(Space)
        Paint('Normal mode - light green is checked')
        CheckHash(0x9EC9834D8A0DDBB8)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Test,d:c,w:50,h:11,flags: Sizeable");
    let l = listbox!("d:c,w:100%,h:100%,flags: ScrollBars+CheckBoxes+SearchBar, lsm:2, items:[Red,Green,'dark green', 'light green',White,'Special Greeb',Black,Orange,Yellow,Purple]");
    w.add(l);
    a.add_window(w);
    a.run();
}

#[test]
fn check_events() {
    #[Window(events=ListBoxEvents, internal: true)]
    struct MyWin {
        lbox: Handle<ListBox>,
        log: Handle<ListBox>,
    }
    impl MyWin {
        fn new() -> Self {
            let mut w = Self {
                base: window!("Title:'Colors',d:c,w:60,h:10,flags:Sizeable"),
                lbox: Handle::None,
                log: Handle::None,
            };
            let mut vs = vsplitter!("25%,d:c,w:100%,h:100%");
            w.lbox = vs.add(
                vsplitter::Panel::Left,
                listbox!(
                    "d:c,w:100%,h:100%,flags: ScrollBars+CheckBoxes+SearchBar,items:['Red','Green','Blue','Yellow','Black','White'],tsm:4,lsm:1"
                ),
            );
            let mut p = panel!("caption:'Event logs',d:c,w:100%,h:100%,type: TopBar");
            w.log = p.add(listbox!("d:c,w:100%,h:100%,flags: ScrollBars+SearchBar+AutoScroll, lsm:1"));
            vs.add(vsplitter::Panel::Right, p);
            w.add(vs);
            w
        }
    }
    impl ListBoxEvents for MyWin {
        fn on_current_item_changed(&mut self, handle: Handle<ListBox>, index: usize) -> EventProcessStatus {
            if self.lbox == handle {
                let idx = self.control(self.lbox).map(|l| l.index()).unwrap_or(usize::MAX);
                // idx should be the sae of index
                assert_eq!(idx, index);
                let h = self.log;
                if let Some(log) = self.control_mut(h) {
                    let idx = log.count() + 1;
                    log.add(&format!("{} => Current item changed to index: {}", idx, index));
                }
            }
            EventProcessStatus::Processed
        }

        fn on_item_checked(&mut self, handle: Handle<ListBox>, index: usize, checked: bool) -> EventProcessStatus {
            if self.lbox == handle {
                let correct_checked = self
                    .control(self.lbox)
                    .map(|l| l.item(index).unwrap().is_checked() == checked)
                    .unwrap_or(false);
                assert!(correct_checked);
                let h = self.log;
                if let Some(log) = self.control_mut(h) {
                    log.add(&format!(
                        "Item with index: {} is {}",
                        index,
                        if checked { "checked" } else { "unchecked" }
                    ));
                }
            }
            EventProcessStatus::Processed
        }
    }

    let script = "
        Paint.Enable(false)
        Mouse.Click(10,1,left)
        Paint('Initial state')
        CheckHash(0x869E551D2A656160)
        Key.Pressed(Down)
        Paint('One item in log')
        CheckHash(0x79F6E686682C941E)
        Key.Pressed(Space)
        Paint('two items in log')
        CheckHash(0xD180C851FE95ADD1)
        Key.Pressed(Space)
        Paint('three items in log')
        CheckHash(0x6B1BBAB7CC71CFC9)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

#[test]
fn check_autoscroll() {
    #[Window(events=ButtonEvents, internal: true)]
    struct MyWin {
        log: Handle<ListBox>,
    }
    impl MyWin {
        fn new() -> Self {
            let mut w = Self {
                base: window!("Title:'AutoScroll',d:c,w:60,h:10"),
                log: Handle::None,
            };
            w.log = w.add(listbox!("x:50%,y:0,w:50%,h:100%,flags: ScrollBars+CheckBoxes+SearchBar+AutoScroll"));
            w.add(button!("Inc,x:1,y:1,w:10"));
            w
        }
    }
    impl ButtonEvents for MyWin {
        fn on_pressed(&mut self, _: Handle<Button>) -> EventProcessStatus {
            let h = self.log;
            if let Some(log) = self.control_mut(h) {
                let idx = log.count() + 1;
                log.add(&format!("Item {}", idx));
            }
            EventProcessStatus::Processed
        }
    }

    let script = "
        Paint.Enable(false)
        Paint('Initial state')
        CheckHash(0x50250C606662391B)
        Key.Pressed(Enter)
        Paint('Items: 1')
        CheckHash(0x4F9BCD302EA34064)
        Key.Pressed(Enter,5)
        Paint('Items: 6')
        CheckHash(0xB3C12E90A284F20)
        Key.Pressed(Enter,10)
        Paint('Items: 16 (last item index is 16)')
        CheckHash(0x87CEEDA275074E5F)
        Key.Pressed(Tab)
        Paint('ListBox is focused and has vscroll enabled and position to last char')
        CheckHash(0x2BA891709571BF8D)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

#[test]
fn check_clear() {
    #[Window(events=ButtonEvents, internal: true)]
    struct MyWin {
        log: Handle<ListBox>,
    }
    impl MyWin {
        fn new() -> Self {
            let mut w = Self {
                base: window!("Title:'AutoScroll',d:c,w:60,h:10"),
                log: Handle::None,
            };
            w.log = w.add(listbox!(
                "x:50%,y:0,w:50%,h:100%,flags: ScrollBars+CheckBoxes, items=[1,2,3,4,5,6,7,8,9,10]"
            ));
            w.add(button!("Clear,x:1,y:1,w:10"));
            w
        }
    }
    impl ButtonEvents for MyWin {
        fn on_pressed(&mut self, _: Handle<Button>) -> EventProcessStatus {
            let h = self.log;
            if let Some(log) = self.control_mut(h) {
                log.clear();
                log.sort(); // this wil do nothing, but will force the execution through a different path
                log.sort_by(|a, b| b.text().cmp(a.text())); // this wil do nothing, but will force the execution through a different path
            }
            EventProcessStatus::Processed
        }
    }

    let script = "
        Paint.Enable(false)
        Paint('Initial state')
        CheckHash(0xF6AD8317293168DA)
        Key.Pressed(Tab)
        Paint('Listbox has scrollbar and is focused')
        CheckHash(0x88628681D2D53749)
        Key.Pressed(End)
        Paint('Listbox has scrollbar - last element selected')
        CheckHash(0xDAB730F14A7DCA4E)
        Key.Pressed(Tab)
        Key.Pressed(Enter)
        Paint('Listbox is cleared')
        CheckHash(0xC86821FC74AB4BA)
        Key.Pressed(Tab)
        Paint('Listbox has inactive vertical scrollbar')
        CheckHash(0xB80B6F0694BA7F8B)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

#[test]
fn check_autoscroll_on_create() {
    let script = "
        Paint.Enable(false)
        Paint('Initial state (15 should be selected and on the bottom of the window)')
        CheckHash(0x64D5987E9F212984)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Test,d:c,w:50,h:11,flags: Sizeable");
    let l = listbox!("d:c,w:100%,h:100%,flags: ScrollBars+AutoScroll, lsm:2, items:[1,2,3,4,5,6,7,8,9,10,11,12,13,14,15]");
    w.add(l);
    a.add_window(w);
    a.run();
}

#[test]
fn check_empty_list_message() {
    let script = "
        Paint.Enable(false)
        Mouse.Click(10,3,left)
        Paint('Initial state')
        CheckHash(0x5D6BE4A96323ECD6)
        Key.Pressed(Tab);
        Paint('Width focus')    
        CheckHash(0x3D264DF35A9248A9)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Test,d:c,w:50,h:11,flags: Sizeable");
    let mut p = panel!("Test,l:10,t:1,b:1,r:1");
    let mut l = listbox!("d:c,w:100%,h:100%,flags: ScrollBars+CheckBoxes+SearchBar, lsm:2");
    l.set_empty_message("No items in the list. Insert some items by pressing the 'Add' button.");
    p.add(l);
    w.add(p);
    w.add(button!("Add,x:1,y:1,w:7,type:flat"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_empty_list_message_with_macro() {
    let script = "
        Paint.Enable(false)
        Mouse.Click(10,3,left)
        Paint('Initial state')
        CheckHash(0xA2D72BDE0BE182DC)
        Key.Pressed(Tab);
        Paint('Width focus')    
        CheckHash(0x36162E7E39B5F673)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Test,d:c,w:50,h:11,flags: Sizeable");
    let mut p = panel!("Test,l:10,t:1,b:1,r:1");
    let l = listbox!(
        "d:c,w:100%,h:100%,flags: ScrollBars+CheckBoxes+SearchBar, lsm:2, em='No items in the list. Insert some items by pressing the <Add> button.'"
    );
    p.add(l);
    w.add(p);
    w.add(button!("Add,x:1,y:1,w:7,type:flat"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_sort() {
    #[Window(events=ButtonEvents, internal: true)]
    struct MyWin {
        log: Handle<ListBox>,
    }
    impl MyWin {
        fn new() -> Self {
            let mut w = Self {
                base: window!("Title:'Sort',d:c,w:60,h:10"),
                log: Handle::None,
            };
            w.log = w.add(listbox!(
                "x:50%,y:0,w:50%,h:100%,flags: ScrollBars+CheckBoxes, items=[Red,Green,Blue,Yellow,Black,White,Orange,Purple]"
            ));
            w.add(button!("Sort,x:1,y:1,w:10"));
            w
        }
    }
    impl ButtonEvents for MyWin {
        fn on_pressed(&mut self, _: Handle<Button>) -> EventProcessStatus {
            let h = self.log;
            if let Some(log) = self.control_mut(h) {
                log.sort();
            }
            EventProcessStatus::Processed
        }
    }

    let script = "
        Paint.Enable(false)
        Paint('Initial state')
        CheckHash(0xC02891D8E535215)
        Key.Pressed(Tab)
        Paint('Listbox has scrollbar and is focused')
        CheckHash(0xE86843629485A14D)
        Key.Pressed(Down,3)
        Paint('Yellow is selected')
        CheckHash(0x19199C80DA1101F2)
        Key.Pressed(Tab)
        Key.Pressed(Enter)
        Paint('List is sorted')
        CheckHash(0x1C8CADCF8F9DBA81)
        Key.Pressed(Tab)
        Paint('Listbox is sorted, Yellow is selected')
        CheckHash(0x14EC25C01F6E20A6)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

#[test]
fn check_sort_by() {
    #[Window(events=ButtonEvents, internal: true)]
    struct MyWin {
        log: Handle<ListBox>,
    }
    impl MyWin {
        fn new() -> Self {
            let mut w = Self {
                base: window!("Title:'Sort',d:c,w:60,h:10"),
                log: Handle::None,
            };
            w.log = w.add(listbox!(
                "x:50%,y:0,w:50%,h:100%,flags: ScrollBars+CheckBoxes, items=[Red,Green,Blue,Yellow,Black,White,Orange,Purple]"
            ));
            w.add(button!("Sort,x:1,y:1,w:10"));
            w
        }
    }
    impl ButtonEvents for MyWin {
        fn on_pressed(&mut self, _: Handle<Button>) -> EventProcessStatus {
            let h = self.log;
            if let Some(log) = self.control_mut(h) {
                log.sort_by(|a, b| a.text().len().cmp(&b.text().len()));
            }
            EventProcessStatus::Processed
        }
    }

    let script = "
        Paint.Enable(false)
        Paint('Initial state')
        CheckHash(0xC02891D8E535215)
        Key.Pressed(Tab)
        Paint('Listbox has scrollbar and is focused')
        CheckHash(0xE86843629485A14D)
        Key.Pressed(Down,3)
        Paint('Yellow is selected')
        CheckHash(0x19199C80DA1101F2)
        Key.Pressed(Tab)
        Key.Pressed(Enter)
        Paint('List is sorted (based on length: Red,Blue,Greem,Black,White,Yellow,Orange,Purple)')
        CheckHash(0xEC290E75D5C9A6C9)
        Key.Pressed(Tab)
        Paint('Listbox is sorted, Yellow is selected')
        CheckHash(0x7793BA7F95CF1B06)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

#[test]
fn check_empty_highlight_selected_when_inactive() {
    let script = "
        Paint.Enable(false)
        Mouse.Click(10,3,left)
        Paint('Initial state (1st item highlighed)')
        CheckHash(0x8BF3E970899BE985)
        Key.Pressed(Tab);
        Paint('Width focus (first item)')    
        CheckHash(0xE06F3CA0C8FEC6E9)
        Key.Pressed(Down,3)
        Paint('Width focus (4th item)')    
        CheckHash(0x967F46130D49EB19)
        Key.Pressed(Tab)
        Paint('Button has focus (4th item is highlighet)')    
        CheckHash(0x791D611DAE470635)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Test,d:c,w:50,h:11,flags: Sizeable");
    let mut p = panel!("Test,l:10,t:1,b:1,r:1");
    let l = listbox!("d:c,w:100%,h:100%,flags: ScrollBars+HighlightSelectedItemWhenInactive, lsm:2, items=[1,2,3,4,5,6,7,8,9,10]");
    p.add(l);
    w.add(p);
    w.add(button!("Add,x:1,y:1,w:7,type:flat"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_empty_highlight_selected_when_inactive_2() {
    let script = "
        Paint.Enable(false)
        Mouse.Click(10,3,left)
        Paint('Initial state (1st item highlighed)')
        CheckHash(0xD162FF2B84F7A4D5)
        Key.Pressed(Tab);
        Paint('Width focus (first item)')    
        CheckHash(0x6001DC468E08D2F1)
        Key.Pressed(Down,3)
        Paint('Width focus (4th item)')    
        CheckHash(0x206A093518CCEB39)
        Key.Pressed(Tab)
        Paint('Button has focus (4th item is highlighet)')    
        CheckHash(0xABD626D0EDE05CAD)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Test,d:c,w:50,h:11,flags: Sizeable");
    let mut p = panel!("Test,l:10,t:1,b:1,r:1");
    let l = listbox!("d:c,w:100%,h:100%,flags: ScrollBars+HighlightSelectedItemWhenInactive+CheckBoxes, lsm:2, items=[1,2,3,4,5,6,7,8,9,10]");
    p.add(l);
    w.add(p);
    w.add(button!("Add,x:1,y:1,w:7,type:flat"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_items_checked_with_mouse() {
    #[Window(events=ListBoxEvents, internal: true)]
    struct MyWin {
        lbox: Handle<ListBox>,
        log: Handle<ListBox>,
    }
    impl MyWin {
        fn new() -> Self {
            let mut w = Self {
                base: window!("Title:'Colors',d:c,w:60,h:10,flags:Sizeable"),
                lbox: Handle::None,
                log: Handle::None,
            };
            let mut vs = vsplitter!("25%,d:c,w:100%,h:100%");
            w.lbox = vs.add(
                vsplitter::Panel::Left,
                listbox!(
                    "d:c,w:100%,h:100%,flags: ScrollBars+CheckBoxes+SearchBar,items:['Red','Green','Blue','Yellow','Black','White'],tsm:4,lsm:1"
                ),
            );
            let mut p = panel!("caption:'Event logs',d:c,w:100%,h:100%,type: TopBar");
            w.log = p.add(listbox!("d:c,w:100%,h:100%,flags: ScrollBars+SearchBar+AutoScroll, lsm:1"));
            vs.add(vsplitter::Panel::Right, p);
            w.add(vs);
            w
        }
    }
    impl ListBoxEvents for MyWin {
        fn on_item_checked(&mut self, handle: Handle<ListBox>, index: usize, checked: bool) -> EventProcessStatus {
            if self.lbox == handle {
                let h = self.log;
                let cnt = self.control(self.lbox).map(|l| l.count_checked()).unwrap_or(0);
                let log_cnt = self.control(self.log).map(|l| l.count_checked()).unwrap_or(usize::MAX);
                // log does not have the Checkboxes flag set up --> so count_check() should return 0
                assert_eq!(log_cnt, 0);
                if let Some(log) = self.control_mut(h) {
                    log.add(&format!(
                        "Item with index: {} is {}",
                        index,
                        if checked { "checked" } else { "unchecked" }
                    ));
                    log.add(&format!("Total checked items: {}", cnt));
                }
            }
            EventProcessStatus::Processed
        }
    }

    let script = "
        Paint.Enable(false)
        Mouse.Click(10,1,left)
        Paint('Initial state')
        CheckHash(0x869E551D2A656160)
        Mouse.Click(1,2,left)
        Paint('Checked: Green, ItemsChecked: 1')
        CheckHash(0xAE2962CC9B98DAFB)
        Mouse.Click(1,4,left)
        Paint('Checked: Green,Yellow, ItemsChecked: 2')
        CheckHash(0x9DD60AF5535A49CC)
        Mouse.Click(1,5,left)
        Paint('Checked: Green,Yellow,Black, ItemsChecked: 3')
        CheckHash(0x139815234DC93155)
        Mouse.Click(1,5,left)
        Paint('Checked: Green,Yellow, ItemsChecked: 2')
        CheckHash(0x6855A969D5D764BC)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

#[test]
fn check_scroll_from_scrollbar() {
    let script = "
        Paint.Enable(false)
        Paint('Initial state (White is selected,Red is the first item)')
        CheckHash(0x77D7DB5C1EE81C58)
        Mouse.Click(29,7,left)
        Paint('Scroll: Green -> Yellow')
        CheckHash(0x24ADD0AFDD88A744)
        Mouse.Click(29,7,left)
        Paint('Scroll: Blue -> Purple')
        CheckHash(0xC761A8E1BB84BA1E)
        Mouse.Click(20,5,left)
        Paint('Scroll: Blue -> Purple, selected: Orange')
        CheckHash(0x4718103A7AA79E79)
        Mouse.Click(29,2,left)
        Paint('Scroll: Green -> Yellow')
        CheckHash(0x576DA8ED102C4F24)
        Mouse.Click(29,2,left)
        Paint('Scroll: Red -> Orange')
        CheckHash(0x9C13452BDE1FBA10)
    ";
    let mut a = App::debug(40, 10, script).build().unwrap();
    let mut w = window!("Test,d:c,w:20,h:8,flags: Sizeable");
    let l = listbox!("d:c,w:100%,h:100%,flags: ScrollBars, lsm:2, items:[Red,Green,Blue,White,Black,Orange,Yellow,Purple]");
    w.add(l);
    a.add_window(w);
    a.run();
}

#[test]
fn check_scroll_from_mouse_wheel() {
    let script = "
        Paint.Enable(false)
        Paint('Initial state (White is selected,Red is the first item)')
        CheckHash(0x77D7DB5C1EE81C58)
        Mouse.Wheel(20,5,down,1)
        Paint('Scroll down by wheel: Green->Yellow')
        CheckHash(0x92DB91CD71A7864F)
        Mouse.Wheel(20,5,down,1)
        Paint('Scroll down by wheel: Blue->Purple')
        CheckHash(0xB63121D5EC6EF23D)
        Mouse.Wheel(20,5,down,1)
        Paint('Scroll down by wheel: Blue->Purple')
        CheckHash(0xB63121D5EC6EF23D)
        Mouse.Wheel(20,5,up,1)
        Paint('Scroll down by wheel: Green->Yellow')
        CheckHash(0x92DB91CD71A7864F)
        Mouse.Wheel(20,5,up,100)
        Paint('Back to initial state')
        CheckHash(0x77D7DB5C1EE81C58)
    ";
    let mut a = App::debug(40, 10, script).build().unwrap();
    let mut w = window!("Test,d:c,w:20,h:8,flags: Sizeable");
    let l = listbox!("d:c,w:100%,h:100%,flags: ScrollBars, lsm:2, items:[Red,Green,Blue,White,Black,Orange,Yellow,Purple]");
    w.add(l);
    a.add_window(w);
    a.run();
}
