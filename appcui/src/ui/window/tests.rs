use crate::prelude::*;

use super::{
    toolbar::{self, GroupPosition},
    Window,
};

#[test]
fn check_window_just_title() {
    let script = "
        Paint.Enable(false)
        // expect: ╔═════ 123456 ═════╗
        Paint('123456 centered')
        CheckHash(0x87AACF295BE859E6)
    ";
    let mut a = App::debug(20, 10, script).build().unwrap();
    a.add_window(Window::new("123456", layout!("a:c,w:20,h:10"), window::Flags::NoCloseButton));
    a.run();
}

#[test]
fn check_window_pos() {
    let script = "
        Paint.Enable(false)
        Paint('Centered')
        CheckHash(0x361C1A2F695D024B)
    ";
    let mut a = App::debug(60, 14, script).build().unwrap();
    a.add_window(Window::new("Centered", layout!("a:c,w:30,h:8"), window::Flags::NoCloseButton));
    a.run();
}

#[test]
fn check_window_pos_using_layout_builder() {
    let script = "
        Paint.Enable(false)
        Paint('Centered')
        CheckHash(0x361C1A2F695D024B)
    ";
    let mut a = App::debug(60, 14, script).build().unwrap();
    a.add_window(Window::new(
        "Centered",
        LayoutBuilder::new().width(30).height(8).alignament(Alignament::Center).build(),
        window::Flags::NoCloseButton,
    ));
    a.run();
}

#[test]
fn check_window_just_large_title() {
    let script = "
        Paint.Enable(false)
        //expect: ╔═ 0123456789ABCD ═╗
        Paint('0123456789ABCD centered')
        CheckHash(0x21339488E2980718)
    ";
    let mut a = App::debug(20, 10, script).build().unwrap();
    a.add_window(Window::new("0123456789ABCD", layout!("a:c,w:20,h:10"), window::Flags::NoCloseButton));
    a.run();
}
#[test]
fn check_window_just_oversized_title() {
    let script = "
        Paint.Enable(false)
        //expect: ╔═ 01234...CDEFGH ═╗
        Paint('012345...BCDEFGH')
        CheckHash(0x8AD5C306676ACF04)
    ";
    let mut a = App::debug(20, 10, script).build().unwrap();
    a.add_window(Window::new(
        "0123456789ABCDEFGH",
        layout!("a:c,w:20,h:10"),
        window::Flags::NoCloseButton,
    ));
    a.run();
}
#[test]
fn check_window_title() {
    let script = "
        Paint.Enable(false)
        //expect:  ╔════ Title ════[x]╗
        Paint('title')
        CheckHash(0xA0CFD68A45B1786C)
    ";
    let mut a = App::debug(20, 10, script).build().unwrap();
    a.add_window(Window::new("Title", layout!("a:c,w:20,h:10"), window::Flags::None));
    a.run();
}
#[test]
fn check_window_full_title_1() {
    let script = "
        Paint.Enable(false)
        // expect:  ╔═ 1234567890A ═[x]╗
        Paint('full title')
        CheckHash(0xF410B9650F4ADF18)
    ";
    let mut a = App::debug(20, 10, script).build().unwrap();
    a.add_window(Window::new("1234567890A", layout!("a:c,w:20,h:10"), window::Flags::None));
    a.run();
}
#[test]
fn check_window_full_title_2() {
    let script = "
        Paint.Enable(false)
        //expect: ╔════ Title ════[x]╗
        Paint('full title')
        CheckHash(0xA0CFD68A45B1786C)
    ";
    let mut a = App::debug(20, 10, script).build().unwrap();
    a.add_window(Window::new("Title", layout!("a:c,w:20,h:10"), window::Flags::None));
    a.run();
}
#[test]
fn check_window_full_title_3() {
    let script = "
        Paint.Enable(false)
        //expect: ╔═ ABC ═[x]╗
        Paint('full title')
        CheckHash(0xEEBF652BB26E9C4C)
    ";
    let mut a = App::debug(20, 10, script).build().unwrap();
    a.add_window(Window::new("ABC", layout!("a:c,w:12,h:8"), window::Flags::None));
    a.run();
}
#[test]
fn check_window_title_close_button_and_minimize_button() {
    let script = "
        Paint.Enable(false)
        //expect: ╔[↑]══ 123456 ══[x]╗
        Paint('full title')
        CheckHash(0xEF7A5C3AFD21BD32)
    ";
    let mut a = App::debug(20, 10, script).build().unwrap();
    a.add_window(Window::new("123456", layout!("a:c,w:20,h:8"), window::Flags::Sizeable));
    a.run();
}
#[test]
fn check_window_minimize_title_1() {
    let script = "
        Paint.Enable(false)
        //expect: ╔═ ABCD...IJKL ═[x]╗
        Paint('Title = ABCD...IJKL')
        CheckHash(0x671DB3CA4AD392AE)
    ";
    let mut a = App::debug(20, 10, script).build().unwrap();
    a.add_window(Window::new("ABCDEFGHIJKL", layout!("a:c,w:20,h:10"), window::Flags::None));
    a.run();
}
#[test]
fn check_window_minimize_title_2() {
    let script = "
        Paint.Enable(false)
        //expect: ╔═ AB...KL ═[x]╗
        Paint('Title = AB...KL')
        CheckHash(0x7F7F1F564130F50E)
    ";
    let mut a = App::debug(20, 10, script).build().unwrap();
    a.add_window(Window::new("ABCDEFGHIJKL", layout!("a:c,w:16,h:8"), window::Flags::None));
    a.run();
}
#[test]
fn check_window_minimize_title_3() {
    let script = "
        Paint.Enable(false)
        //expect: ╔═ A...L ═[x]
        Paint('Title = A...L')
        CheckHash(0x6CB0EAB5DDA0E087)
    ";
    let mut a = App::debug(20, 10, script).build().unwrap();
    a.add_window(Window::new("ABCDEFGHIJKL", layout!("a:c,w:14,h:6"), window::Flags::None));
    a.run();
}
#[test]
fn check_window_minimize_title_4() {
    let script = "
        Paint.Enable(false)
        //expect: ╔═ A…L ═[x]╗
        Paint('Short title')
        CheckHash(0x3A1C142AE9968A2F)
    ";
    let mut a = App::debug(20, 10, script).build().unwrap();
    a.add_window(Window::new("ABCDEFGHIJKL", layout!("a:c,w:12,h:6"), window::Flags::None));
    a.run();
}

#[test]
fn check_multiple_items_top_bar() {
    let script = "
    Paint.Enable(false)
    //expect on top   : ╔[↑]═[F1]═[ABC]═[Lb-1|Lb-2]═[Single]═══ Title ═══[AB]═[Lb-2|Lb-1]═[x]╗
    //expect on bottom: ╚[Lb-1|Lb-2]═[Single]══════════════════════════════[AB]═[Lb-2|Lb-1]═─┘
    Paint('Multiple items on top and bottom bar')
    //CheckHash(0x3F0441B8433D629B)
    ";
    let mut a = App::debug(80, 10, script).build().unwrap();
    let mut w = Window::new("Title", layout!("a:c,w:70,h:10"), window::Flags::Sizeable);
    w.set_tag("ABC");
    w.set_hotkey(key!("F1"));
    let g = w.toolbar().create_group(GroupPosition::TopLeft);
    w.toolbar().add(g, toolbar::Label::new("Lb-1"));
    w.toolbar().add(g, toolbar::Label::new("Lb-2"));
    let g = w.toolbar().create_group(GroupPosition::TopLeft);
    w.toolbar().add(g, toolbar::Label::new("Single"));
    let g = w.toolbar().create_group(GroupPosition::TopRight);
    w.toolbar().add(g, toolbar::Label::new("Lb-1"));
    w.toolbar().add(g, toolbar::Label::new("Lb-2"));
    let g = w.toolbar().create_group(GroupPosition::TopRight);
    w.toolbar().add(g, toolbar::Label::new("AB"));
    let g = w.toolbar().create_group(GroupPosition::BottomLeft);
    w.toolbar().add(g, toolbar::Label::new("Lb-1"));
    w.toolbar().add(g, toolbar::Label::new("Lb-2"));
    let g = w.toolbar().create_group(GroupPosition::BottomLeft);
    w.toolbar().add(g, toolbar::Label::new("Single"));
    let g = w.toolbar().create_group(GroupPosition::BottomRight);
    w.toolbar().add(g, toolbar::Label::new("Lb-1"));
    w.toolbar().add(g, toolbar::Label::new("Lb-2"));
    let g = w.toolbar().create_group(GroupPosition::BottomRight);
    w.toolbar().add(g, toolbar::Label::new("AB"));

    a.add_window(w);
    a.run();
}

#[test]
fn check_window_tag_1() {
    let script = "
        Paint.Enable(false)
        //expect: ╔[AB]══ Title ══[x]╗
        Paint('Tag=AB + Title')
        CheckHash(0x3A2846E4BAE2A1A1)
    ";
    let mut a = App::debug(20, 10, script).build().unwrap();
    let mut w = Window::new("Title", layout!("a:c,w:20,h:10"), window::Flags::None);
    w.set_tag("AB");
    a.add_window(w);
    a.run();
}
#[test]
fn check_window_tag_2() {
    let script = "
        Paint.Enable(false)
        //expect: ╔[ABCD]═ Title ═[x]╗
        Paint('title should be visible')
        CheckHash(0xE2CB87CCC6FD9E4A)
    ";
    let mut a = App::debug(20, 10, script).build().unwrap();
    let mut w = Window::new("Title", layout!("a:c,w:20,h:10"), window::Flags::None);
    w.set_tag("ABCD");
    a.add_window(w);
    a.run();
}
#[test]
fn check_window_tag_and_split_title_1() {
    let script = "
        Paint.Enable(false)
        // expect: ╔[ABCDE]═ T…le ═[x]╗
        Paint('title split with 3 special chars')
        CheckHash(0x34902E0B6D58F035)
    ";
    let mut a = App::debug(20, 10, script).build().unwrap();
    let mut w = Window::new("Title", layout!("a:c,w:20,h:10"), window::Flags::None);
    w.set_tag("ABCDE");
    a.add_window(w);
    a.run();
}
#[test]
fn check_window_tag_and_split_title_2() {
    let script = "
        Paint.Enable(false)
        //expect: ╔[ABCDEF]═ T…e ═[x]╗
        Paint('╔[ABCDEF]═ T…e ═[x]╗')
        CheckHash(0xA52995587B045766)
    ";
    let mut a = App::debug(20, 10, script).build().unwrap();
    let mut w = Window::new("Title", layout!("a:c,w:20,h:10"), window::Flags::None);
    w.set_tag("ABCDEF");
    a.add_window(w);
    a.run();
}
#[test]
fn check_window_tag_and_title_first_letter() {
    let script = "
        Paint.Enable(false)
        //expect: ╔[ABCDEFG]═ T… ═[x]╗
        Paint('title first letter and special char with 3 points')
        CheckHash(0x6F914F802B3B7B5D)
    ";
    let mut a = App::debug(20, 10, script).build().unwrap();
    let mut w = Window::new("Title", layout!("a:c,w:20,h:10"), window::Flags::None);
    w.set_tag("ABCDEFG");
    a.add_window(w);
    a.run();
}
#[test]
fn check_window_tag_and_title_not_visible() {
    let script = "
        Paint.Enable(false)
        //expect: ╔[ABCDEFGH]═════[x]╗
        Paint('title not visible')
        CheckHash(0xA2C91CB6A1484009)
    ";
    let mut a = App::debug(20, 10, script).build().unwrap();
    let mut w = Window::new("Title", layout!("a:c,w:20,h:10"), window::Flags::None);
    w.set_tag("ABCDEFGH");
    a.add_window(w);
    a.run();
}

#[test]
fn check_window_toolbar_title_pos_recompute() {
    #[Window(events = ToolBarEvents,internal = true)]
    struct MyWin {
        info: Handle<toolbar::Label>,
        change_info: Handle<toolbar::Button>,
    }
    impl MyWin {
        fn new() -> Self {
            let mut me = Self {
                base: Window::new("12345678", layout!("a:c,w:30,h:8"), window::Flags::None),
                info: Handle::None,
                change_info: Handle::None,
            };
            let g = me.toolbar().create_group(GroupPosition::TopLeft);
            me.info = me.toolbar().add(g, toolbar::Label::new("?"));
            let g = me.toolbar().create_group(GroupPosition::BottomLeft);
            me.change_info = me.toolbar().add(g, toolbar::Button::new("&Change Info Size"));
            me
        }
    }
    impl ToolBarEvents for MyWin {
        fn on_button_clicked(&mut self, handle: Handle<toolbar::Button>) -> EventProcessStatus {
            if handle == self.change_info {
                let h = self.info;
                self.toolbar().get_mut(h).unwrap().set_content("ABCDEFGHI");
                return EventProcessStatus::Processed;
            }
            EventProcessStatus::Ignored
        }
    }

    let script = "
        Paint.Enable(false)
        // expect on top:    ╔[?]══════ 12345678 ══════[x]╗
        // expect on bottom: ╚[Change Info Size]══════════╝ 
        Paint('initial state')
        CheckHash(0xB64C88B7BC1DE1B2)
        Mouse.Click(20,8,left)
        // expect on top: ╔[ABCDEFGHI]══ 12345678 ══[x]╗
        Paint('after click on button --> title move to right (MOUSE is OVER)')
        CheckHash(0x2524C7C64CAF6368)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

#[test]
fn check_window_hotkey_1() {
    let script = "
        Paint.Enable(false)
        //expect: ╔[F1]══ Title ══[x]╗
        Paint('hotkey')
        CheckHash(0x4454159FD9AA73E9)
    ";
    let mut a = App::debug(20, 10, script).build().unwrap();
    let mut w = Window::new("Title", layout!("a:c,w:20,h:10"), window::Flags::None);
    w.set_hotkey(key!("Alt+F1"));
    a.add_window(w);
    a.run();
}
#[test]
fn check_window_hotkey_2() {
    let script = "
        Paint.Enable(false)
        //expect: ╔[Enter]═ T…le ═[x]╗
        Paint('hotkey')
        CheckHash(0xC9D2F0E450475385)
    ";
    let mut a = App::debug(20, 10, script).build().unwrap();
    let mut w = Window::new("Title", layout!("a:c,w:20,h:10"), window::Flags::None);
    w.set_hotkey(KeyCode::Enter);
    a.add_window(w);
    a.run();
}

#[test]
fn check_window_hotkey_and_tag() {
    let script = "
        Paint.Enable(false)
        //expect: ╔[1]═[XYZ]═ T… ═[x]╗
        Paint('hotkey & tag')
        CheckHash(0x8F6D9DF3500A2D7A)
    ";
    let mut a = App::debug(20, 10, script).build().unwrap();
    let mut w = Window::new("Title", layout!("a:c,w:20,h:10"), window::Flags::None);
    w.set_hotkey(key!("Alt+1"));
    w.set_tag("XYZ");
    a.add_window(w);
    a.run();
}

#[test]
fn check_window_resize() {
    let script = "
        Paint.Enable(false)
        Paint('initial state')
        CheckHash(0x4BA8D83661491642)
        Mouse.Move(39,7)
        Paint('over the resize handler')
        CheckHash(0x4104D1AD861DF6F6)
        Mouse.Hold(39,7,left)
        Paint('click on resize handler')
        CheckHash(0x20EFD3353E8B662E)
        Mouse.Move(41,8)
        Paint('resized')
        CheckHash(0xF03C109AE54A7CC2)
        Mouse.Move(51,8)
        Paint('even bigger')
        CheckHash(0x33208E261BC024C6)
        Mouse.Move(47,6)
        Paint('smaller')
        CheckHash(0x1DEAE376D9FCDF6)
        Mouse.Release(47,6,left)
        Paint('after release of left button')
        // since mouse is still over the window corner, the window corner should be selected after the left button is released
        CheckHash(0x24D38D8CA6584432)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let w = Window::new("Title", layout!("a:c,w:20,h:5"), window::Flags::Sizeable);
    a.add_window(w);
    a.run();
}

#[test]
fn check_window_move() {
    let script = "
        Paint.Enable(false)
        Mouse.Move(30,3)
        Paint('over the title')
        CheckHash(0xB13AACDBDB5FD18C)
        Mouse.Hold(30,3,left)
        Paint('press on window title')
        CheckHash(0xA1C4D2F2B8B883A8)
        Mouse.Move(15,1)
        Paint('window moved')
        CheckHash(0x67671A5381A27E24)
        Mouse.Move(1,1)
        Paint('window moved outside')
        CheckHash(0x31A5BA52AA70667B)
        Mouse.Release(1,1,left)
        Paint('after release of left mouse button')
        CheckHash(0x22B329EC1888AB5E)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let w = Window::new("Title", layout!("a:c,w:21,h:5"), window::Flags::Sizeable);
    a.add_window(w);
    a.run();
}

#[test]
fn check_window_move_2() {
    let script = "
        Paint.Enable(false)
        Paint('initial state')
        CheckHash(0xB1471A30B30F5C6C)
        Mouse.Drag(30,3,35,5)
        Paint('window was moved')
        CheckHash(0x419533D4BBEFE538)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let w = Window::new("Title", layout!("a:c,w:20,h:5"), window::Flags::None);
    a.add_window(w);
    a.run();
}

#[test]
fn check_window_on_layout_changed() {
    #[Window(events = WindowEvents,internal = true)]
    struct MyWin {
        info: Handle<Label>,
    }
    impl MyWin {
        fn new() -> Self {
            let mut me = Self {
                base: Window::new("Win", layout!("a:c,w:40,h:7"), window::Flags::Sizeable),
                info: Handle::None,
            };
            me.info = me.add(Label::new("", layout!("x:1,y:1,w:36,h:3")));
            me
        }
    }
    impl WindowEvents for MyWin {
        fn on_layout_changed(&mut self, old_layout: Rect, new_layout: Rect) {
            let label_handle = self.info;
            let size = self.size();
            let client_size = self.client_size();
            if let Some(label) = self.control_mut(label_handle) {
                label.set_caption(
                    format!(
                        "Previous rect : {},{} - {}x{}\nNew rect      : {},{} - {}x{}\nClient size   : {}x{}",
                        old_layout.left(),
                        old_layout.top(),
                        old_layout.width(),
                        old_layout.height(),
                        new_layout.left(),
                        new_layout.top(),
                        new_layout.width(),
                        new_layout.height(),
                        client_size.width,
                        client_size.height
                    )
                    .as_str(),
                );
            }
            assert!(size.width == new_layout.width());
            assert!(size.height == new_layout.height());
        }
    }

    let script = "
        Paint.Enable(false)
        Paint('initial (pos=10,2 with 40x7)')
        CheckHash(0x303678F286D0D033)
        Mouse.Move(49,8)
        Mouse.Hold(49,8,left)
        Mouse.Move(51,9)
        Mouse.Release(51,9,left)
        Paint('Resize to 51x9')
        CheckHash(0xEA3976FE4B283B6E)
        Mouse.Move(28,2)
        Mouse.Hold(28,2,left)
        Mouse.Move(26,1)
        Mouse.Release(26,1,left)
        Paint('Move to 8,1 with 42x8 size')
        CheckHash(0xA79AF7FBE4808C6A)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

#[test]
fn check_window_on_activate_deactivate() {
    #[Window(events = WindowEvents,internal = true)]
    struct MyWin {
        info: Handle<Label>,
    }
    impl MyWin {
        fn new(layout: &str) -> Self {
            let mut me = Self {
                base: Window::new("Win", layout!(layout), window::Flags::None),
                info: Handle::None,
            };
            me.info = me.add(Label::new("<no-state>", layout!("x:1,y:1,w:16")));
            me
        }
    }
    impl WindowEvents for MyWin {
        fn on_activate(&mut self) {
            let h = self.info;
            if let Some(label) = self.control_mut(h) {
                label.set_caption("Activated");
            }
        }

        fn on_deactivate(&mut self) {
            let h = self.info;
            if let Some(label) = self.control_mut(h) {
                label.set_caption("Deactivated");
            }
        }
    }

    let script = "
        Paint.Enable(false)
        Paint('initial state (left=no-state,right=activated)')
        CheckHash(0x28340BCAB3DD9C4E)
        Mouse.Click(10,1,left)
        Paint('left=activated, right=deactivated')
        CheckHash(0x9D3DA7A748B1F238)
        Mouse.Click(40,1,left)
        Paint('left=deactivated, right=activated')
        CheckHash(0xBFA2B90246E3753C)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    a.add_window(MyWin::new("x:1,y:1,w:25,h:6"));
    a.add_window(MyWin::new("x:30,y:1,w:25,h:6"));
    a.run();
}

#[test]
fn check_window_toolbar_label() {
    let script = "
        Paint.Enable(false)
        //expect on bottom: ╚[Label 1|Label 2]════════════[Label 3]╝
        Paint('multiple label')
        CheckHash(0xDF61C8FA80CF037F)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = Window::new("Title", layout!("a:c,w:40,h:8"), window::Flags::None);
    let g = w.toolbar().create_group(GroupPosition::BottomLeft);
    let l = toolbar::Label::new("Label 1");
    assert_eq!(l.caption(), "Label 1");
    w.toolbar().add(g, l);
    w.toolbar().add(g, toolbar::Label::new("Label 2"));
    let g = w.toolbar().create_group(GroupPosition::BottomRight);
    w.toolbar().add(g, toolbar::Label::new("Label 3"));

    a.add_window(w);
    a.run();
}

#[test]
fn check_window_toolbar_label_tooltip() {
    let script = "
        Paint.Enable(false)
        Mouse.Move(12,8)
        Paint('Tooltip shown for the first label')
        CheckHash(0xAA0FA9F4B624EF51)
        Mouse.Move(22,8)
        Paint('Tooltip shown for the second label')
        CheckHash(0x1983A71F1BB817BD)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = Window::new("Title", layout!("a:c,w:40,h:8"), window::Flags::None);
    let g = w.toolbar().create_group(GroupPosition::BottomLeft);
    let mut l = toolbaritem!("first,type:label");
    l.set_tooltip("A toolbar label");
    w.toolbar().add(g, l);
    w.toolbar().add(g, toolbaritem!("second,type:label,tooltip:'second tooltip'"));
    w.toolbar().add(g, toolbaritem!("caption:hidden,type:label,visible:false"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_window_toolbar_button() {
    let script = "
        Paint.Enable(false)
        // expect on bottom: ╚[Start|Stop]════════════════════[Exit]╝
        Paint('buttons')
        CheckHash(0xDF332807DB1EA7F9)
        Mouse.Move(16,8)
        Paint('Mouse on button start')
        CheckHash(0x393CEF7541C6CAC3)
        Mouse.Move(21,8)
        Paint('Mouse on button stop')
        CheckHash(0x79F3F04E3DCFA549)
        Mouse.Hold(21,8,left)
        Paint('Mouse press over stop button')
        CheckHash(0x1128DAD41B3E99B9)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = Window::new("Title", layout!("a:c,w:40,h:8"), window::Flags::None);
    let g = w.toolbar().create_group(GroupPosition::BottomLeft);
    w.toolbar().add(g, toolbar::Button::new("Start"));
    w.toolbar().add(g, toolbar::Button::new("Stop"));
    let g = w.toolbar().create_group(GroupPosition::BottomRight);
    w.toolbar().add(g, toolbar::Button::new("Exit"));

    a.add_window(w);
    a.run();
}

#[test]
fn check_window_toolbar_button_click() {
    #[Window(events = ToolBarEvents, internal = true)]
    struct MyWin {
        increase_button: Handle<toolbar::Button>,
        decrease_button: Handle<toolbar::Button>,
        text: Handle<Label>,
        number: u32,
    }
    impl MyWin {
        fn new() -> Self {
            let mut win = MyWin {
                base: window!("'My Win',a:c,w:40,h:6"),
                increase_button: Handle::None,
                decrease_button: Handle::None,
                text: Handle::None,
                number: 10,
            };
            // create a group
            let g = win.toolbar().create_group(toolbar::GroupPosition::BottomRight);
            // add buttons
            win.increase_button = win.toolbar().add(g, toolbaritem!("'+',type:button,tooltip:'Click to increase'"));
            win.decrease_button = win.toolbar().add(g, toolbar::Button::new("-"));
            // add a label
            win.text = win.add(label!("10,a:c,w:2,h:1"));
            win
        }
    }
    impl ToolBarEvents for MyWin {
        fn on_button_clicked(&mut self, handle: Handle<toolbar::Button>) -> EventProcessStatus {
            match () {
                _ if handle == self.increase_button => self.number += 1,
                _ if handle == self.decrease_button => self.number -= 1,
                _ => {}
            }
            let h = self.text;
            let n = self.number;
            if let Some(label) = self.control_mut(h) {
                label.set_caption(format!("{n}").as_str());
            }
            EventProcessStatus::Processed
        }
    }

    let script = "
        Paint.Enable(false)
        Paint('value = 10')
        CheckHash(0x93866D3CDC262038)
        Mouse.Move(47,7)
        Paint('Mouse over increase button')
        CheckHash(0x578BA3778B17CB87)
        Mouse.Click(47,7,left)
        Mouse.Click(47,7,left)
        Paint('value = 12')
        CheckHash(0x532FBC16FB8C9B08)
        Mouse.Click(45,7,left)
        Paint('value = 11')
        CheckHash(0xD5FB8878CC6A03E3)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

#[test]
fn check_window_toolbar_button_events() {
    #[Window(events = ToolBarEvents,internal = true)]
    struct MyWin {
        info: Handle<Label>,
        run_button: Handle<toolbar::Button>,
        stop_button: Handle<toolbar::Button>,
        exit_button: Handle<toolbar::Button>,
    }
    impl MyWin {
        fn new() -> Self {
            let mut me = Self {
                base: Window::new("Win", layout!("a:c,w:40,h:8"), window::Flags::None),
                info: Handle::None,
                run_button: Handle::None,
                stop_button: Handle::None,
                exit_button: Handle::None,
            };
            me.info = me.add(Label::new("<no-state>", layout!("x:1,y:1,w:16")));
            let g = me.toolbar().create_group(GroupPosition::BottomLeft);
            me.run_button = me.toolbar().add(g, toolbar::Button::new("&Run"));
            me.stop_button = me.toolbar().add(g, toolbar::Button::new("&Stop"));
            me.exit_button = me.toolbar().add(g, toolbar::Button::new("E&xit"));
            me
        }
        fn set_info(&mut self, info: &str) {
            let h = self.info;
            if let Some(label) = self.control_mut(h) {
                label.set_caption(info);
            }
        }
    }
    impl ToolBarEvents for MyWin {
        fn on_button_clicked(&mut self, handle: Handle<toolbar::Button>) -> EventProcessStatus {
            if handle == self.run_button {
                self.set_info("Run pressed");
                return EventProcessStatus::Processed;
            }
            if handle == self.stop_button {
                self.set_info("Stop pressed");
                return EventProcessStatus::Processed;
            }
            if handle == self.exit_button {
                self.set_info("Exit pressed");
                return EventProcessStatus::Processed;
            }

            EventProcessStatus::Ignored
        }
    }

    let script = "
        Paint.Enable(false)
        // expect on bottom: ╚[Run|Stop|Exit]═══════════════════════╝
        Paint('initial state (no button pressed)')
        CheckHash(0xBA8D94B2CED24515)
        Mouse.Click(14,8,left)
        Paint('Run button pressed')
        CheckHash(0x635616EB62F477C2)
        Mouse.Click(18,8,left)
        Paint('Stop button pressed')
        CheckHash(0x9D7A9FFD20295CDE)
        Mouse.Click(22,8,left)
        Paint('Exit button pressed')
        CheckHash(0x3573E5B4967E93C6)
        Key.Pressed(Alt+R)
        Paint('Run button pressed (with hotkey) - mouse over Exit button')
        CheckHash(0x3FE5987F9A9BDD40)
        Mouse.Move(22,9)
        Key.Pressed(Alt+S)
        Paint('Stop button pressed (with hotkey) - mouse inside window')
        CheckHash(0x6A94FBA39DD0582)
        Mouse.Move(22,8)
        Paint('Mouse over exit button')
        CheckHash(0xC09C0468269358E)
        Mouse.Move(0,0)
        Key.Pressed(Alt+R)
        Paint('Run button pressed (with hotkey) - mouse outside window')
        CheckHash(0xAED6AA2070247D44)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

#[test]
fn check_window_toolbar_checkbox() {
    let script = "
        Paint.Enable(false)
        //expect: ╚[√ Task 1|  Task 2]═════[  Enable All]╝
        Paint('Task1 is checkes, Task2 + Enable all are not')
        CheckHash(0xF3733435595ED5E4)
        Mouse.Move(16,8)
        Paint('Mouse over Task 1')
        CheckHash(0x9262823526914F49)
        Mouse.Move(23,8)
        Paint('Mouse over Task 2')
        CheckHash(0xD7B160520844D260)
        Mouse.Click(23,8,left)
        Paint('Task 2 checked')
        //CheckHash(0x11C054FAECF9D51F)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = Window::new("Title", layout!("a:c,w:40,h:8"), window::Flags::None);
    let g = w.toolbar().create_group(GroupPosition::BottomLeft);
    w.toolbar().add(g, toolbar::CheckBox::new("Task &1", true));
    w.toolbar().add(g, toolbar::CheckBox::new("Task &2", false));
    let g = w.toolbar().create_group(GroupPosition::BottomRight);
    w.toolbar().add(g, toolbar::CheckBox::new("Enable &All", false));

    a.add_window(w);
    a.run();
}

#[test]
fn check_window_toolbar_checkbox_events() {
    #[Window(events = ToolBarEvents,internal = true)]
    struct MyWin {
        cb: Handle<toolbar::CheckBox>,
    }
    impl MyWin {
        fn new() -> Self {
            let mut me = Self {
                base: Window::new("Win", layout!("a:c,w:50,h:6"), window::Flags::None),
                cb: Handle::None,
            };
            let g = me.toolbar().create_group(GroupPosition::TopLeft);
            me.cb = me.toolbar().add(g, toolbar::CheckBox::new("No State", false));
            me
        }
    }
    impl ToolBarEvents for MyWin {
        fn on_checkbox_clicked(&mut self, handle: Handle<toolbar::CheckBox>, checked: bool) -> EventProcessStatus {
            if handle == self.cb {
                let h = self.cb;
                if let Some(checkbox) = self.toolbar().get_mut(h) {
                    if checked {
                        checkbox.set_content("&Checked");
                    } else {
                        checkbox.set_content("&Not checked");
                    }
                    return EventProcessStatus::Processed;
                }
            }
            EventProcessStatus::Ignored
        }
    }

    let script = "
        Paint.Enable(false)
        //expect: ╔[  No State]══════════════ Win ══════════════[x]╗
        Paint('checbox = <No State>, NOT hovered')
        CheckHash(0x20443FB3C98F025C)
        Mouse.Click(12,2,left)
        //expect: ╔[√ Checked]═══════════════ Win ══════════════[x]╗
        Paint('checkbox = Checked (Mouse is OVER)')
        CheckHash(0x30D2AC3FDE47AB74)
        Mouse.Click(12,2,left)
        //expect: ╔[  Not checked]═════════════ Win ════════════[x]╗
        Paint('checkbox = Not checked (Mouse is OVER)')
        CheckHash(0x3E2001B74B24EE81)
        Key.Pressed(Alt+N)
        //expect: ╔[√ Checked]═══════════════ Win ══════════════[x]╗
        Paint('Checkbox is checked again (via Alt+N) (Mouse is OVER)')
        CheckHash(0x30D2AC3FDE47AB74)
        Mouse.Move(0,0)
        Key.Pressed(Alt+N)
        //expect: ╔[  Not checked]═════════════ Win ════════════[x]╗
        Paint('checkbox = Not checked (via Alt+N), NOT hovered')
        CheckHash(0x4BEB73AE53479ABF)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

#[test]
fn check_window_toolbar_single_choice() {
    let script = "
        Paint.Enable(false)
        //expect on bottom: ╚[Opt 1|Opt 2|Opt 3]══════════════════[Opt C|Opt B|Opt A]╝
        Paint('2 groupse of options (opt 1,2,3) and (opt A,B,C)')
        CheckHash(0x9A4788CCD67E31CC)
        Mouse.Move(5,8)
        Paint('Mouse over Opt 1')
        CheckHash(0x68D92FF00A40FDEE)
        Mouse.Move(40,8)
        Paint('Mouse over Opt C')
        CheckHash(0xCEE59E5E54543176)
        Mouse.Move(11,8)
        Paint('Mouse over Opt 2')
        CheckHash(0x30F92AB21E82C25E)
        Mouse.Click(11,8,left)
        Paint('Opt 2 selected')
        CheckHash(0x94FB8F617FAAD206)
        Mouse.Click(6,8,left)
        Paint('Opt 1 selected')
        CheckHash(0x55454B312AB9A1A6)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = Window::new("Title", layout!("a:c,w:58,h:8"), window::Flags::None);
    let g = w.toolbar().create_group(GroupPosition::BottomLeft);
    w.toolbar().add(g, toolbar::SingleChoice::new("Opt &1"));
    w.toolbar().add(g, toolbar::SingleChoice::new("Opt &2"));
    w.toolbar().add(g, toolbar::SingleChoice::new("Opt &3"));
    let g = w.toolbar().create_group(GroupPosition::BottomRight);
    w.toolbar().add(g, toolbar::SingleChoice::new("Opt &A"));
    w.toolbar().add(g, toolbar::SingleChoice::new("Opt &B"));
    w.toolbar().add(g, toolbar::SingleChoice::new("Opt &C"));

    a.add_window(w);
    a.run();
}

#[test]
fn check_window_toolbar_singlechoice_events() {
    #[Window(events = ToolBarEvents,internal = true)]
    struct MyWin {
        opt1: Handle<toolbar::SingleChoice>,
        opt2: Handle<toolbar::SingleChoice>,
        opt3: Handle<toolbar::SingleChoice>,
        optA: Handle<toolbar::SingleChoice>,
        optB: Handle<toolbar::SingleChoice>,
        optC: Handle<toolbar::SingleChoice>,
    }
    impl MyWin {
        fn new() -> Self {
            let mut me = Self {
                base: Window::new("Win", layout!("a:c,w:58,h:8"), window::Flags::None),
                opt1: Handle::None,
                opt2: Handle::None,
                opt3: Handle::None,
                optA: Handle::None,
                optB: Handle::None,
                optC: Handle::None,
            };
            let g = me.toolbar().create_group(GroupPosition::BottomLeft);
            me.opt1 = me.toolbar().add(g, toolbar::SingleChoice::new("Opt &1"));
            me.opt2 = me.toolbar().add(g, toolbar::SingleChoice::new("Opt &2"));
            me.opt3 = me.toolbar().add(g, toolbar::SingleChoice::new("Opt &3"));
            let g = me.toolbar().create_group(GroupPosition::BottomRight);
            me.optA = me.toolbar().add(g, toolbar::SingleChoice::new("Opt &A"));
            me.optB = me.toolbar().add(g, toolbar::SingleChoice::new("Opt &B"));
            me.optC = me.toolbar().add(g, toolbar::SingleChoice::new("Opt &C"));
            me
        }
    }
    impl ToolBarEvents for MyWin {
        fn on_choice_selected(&mut self, handle: Handle<toolbar::SingleChoice>) -> EventProcessStatus {
            if handle == self.opt1 {
                let h = self.optA;
                if let Some(opt_a) = self.toolbar().get_mut(h) {
                    opt_a.select();
                }
                return EventProcessStatus::Processed;
            }
            if handle == self.opt2 {
                let h = self.optB;
                if let Some(opt_b) = self.toolbar().get_mut(h) {
                    opt_b.select();
                }
                return EventProcessStatus::Processed;
            }
            EventProcessStatus::Ignored
        }
    }

    let script = "
        Paint.Enable(false)
        // expect: ╚[Opt 1|Opt 2|Opt 3]══════════════════[Opt C|Opt B|Opt A]╝
        Paint('original status - nothing set up')
        CheckHash(0x5922FFDD9B712DD4)
        Mouse.Move(5,8)
        Paint('Mouse over Opt 1')
        CheckHash(0x29A6F76CEBDB98B6)
        Mouse.Click(5,8,left)
        Paint('Opt 1 selected => Opt A selected as a result')
        CheckHash(0x8C79B1ADB4DAA4EC)
        Mouse.Click(10,8,left)
        Paint('Opt 2 selected => Opt B selected as a result')
        CheckHash(0xDD22A81F2349BD90)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

#[test]
fn check_window_toolbar_maximize_restore() {
    let script = "
        Paint.Enable(false)
        //expect on top: ╔[↑]═════════════ Title ════════════[x]╗▒
        Paint('initial state')
        CheckHash(0x85955D7DF379551A)
        Mouse.Click(12,1,left)
        Paint('Now it should be maximized')
        CheckHash(0x5EFE0CAA67490E16)
        Mouse.Move(2,0)
        Paint('Mouse over Minimize Restore button')
        // expect: ╔[↕]═══════════════════════ Title ══════════════════════[x]╗
        // expect: ║ ↑                                                        ║
        // expect:  Press to maximize or restore                              ║
        CheckHash(0xBAA8B69A66EBB8ED)
        Mouse.Click(2,0,left)
        Paint('Now it should be restored')
        CheckHash(0x85955D7DF379551A)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let w = Window::new("Title", layout!("a:c,w:40,h:8"), window::Flags::Sizeable);
    a.add_window(w);
    a.run();
}

#[test]
fn check_window_toolbar_item_visibility() {
    #[Window(events = ButtonEvents+CheckBoxEvents, internal=true)]
    struct MyWin {
        increase_button: Handle<Button>,
        dec: Handle<toolbar::Label>,
        hex: Handle<toolbar::Label>,
        bin: Handle<toolbar::Label>,
        show_dec: Handle<CheckBox>,
        show_hex: Handle<CheckBox>,
        show_bin: Handle<CheckBox>,
        number: u32,
    }

    impl MyWin {
        fn new() -> Self {
            let mut win = MyWin {
                base: window!("'My Win',a:c,w:40,h:6"),
                increase_button: Handle::None,
                dec: Handle::None,
                hex: Handle::None,
                bin: Handle::None,
                show_dec: Handle::None,
                show_hex: Handle::None,
                show_bin: Handle::None,
                number: 24,
            };
            // add the increasebutton
            win.increase_button = win.add(button!("Increase,w:15,a:l"));
            // add checkboxes
            win.show_dec = win.add(checkbox!("'Show decimal',x:20,y:1,w:16,checked:true"));
            win.show_hex = win.add(checkbox!("'Show hex',x:20,y:2,w:16,checked:true"));
            win.show_bin = win.add(checkbox!("'Show binary',x:20,y:3,w:16,checked:true"));
            // add toolbar labels
            let first_group = win.toolbar().create_group(toolbar::GroupPosition::BottomLeft);
            let second_group = win.toolbar().create_group(toolbar::GroupPosition::TopRight);
            win.dec = win.toolbar().add(first_group, toolbar::Label::new(""));
            win.hex = win.toolbar().add(first_group, toolbar::Label::new(""));
            win.bin = win.toolbar().add(second_group, toolbar::Label::new(""));
            win.update_toolbar_labels();
            win
        }
        fn update_toolbale_label(&mut self, handle: Handle<toolbar::Label>, text: String) {
            if let Some(label) = self.toolbar().get_mut(handle) {
                label.set_content(text.as_str());
            }
        }
        fn update_visibility_status_for_label(&mut self, handle: Handle<toolbar::Label>, visible: bool) {
            if let Some(label) = self.toolbar().get_mut(handle) {
                label.set_visible(visible);
            }
        }
        fn update_toolbar_labels(&mut self) {
            self.update_toolbale_label(self.dec, format!("Dec:{}", self.number));
            self.update_toolbale_label(self.hex, format!("Hex:{:X}", self.number));
            self.update_toolbale_label(self.bin, format!("Bin:{:b}", self.number));
        }
    }

    impl ButtonEvents for MyWin {
        fn on_pressed(&mut self, _handle: Handle<Button>) -> EventProcessStatus {
            self.number += 1;
            self.update_toolbar_labels();
            EventProcessStatus::Processed
        }
    }
    impl CheckBoxEvents for MyWin {
        fn on_status_changed(&mut self, handle: Handle<CheckBox>, checked: bool) -> EventProcessStatus {
            match () {
                _ if handle == self.show_bin => self.update_visibility_status_for_label(self.bin, checked),
                _ if handle == self.show_hex => self.update_visibility_status_for_label(self.hex, checked),
                _ if handle == self.show_dec => self.update_visibility_status_for_label(self.dec, checked),
                _ => {}
            }
            EventProcessStatus::Processed
        }
    }

    let script = "
        Paint.Enable(false)
        //expect on top    : ╔════════ My Win ═══════[Bin:11000]═[x]╗
        //expect on bottom : ╚[Dec:24|Hex:18]═══════════════════════╝
        Paint('initial state')
        CheckHash(0x26CBACC61695381B)
        Mouse.Click(17,4,left)
        Mouse.Click(17,4,left)
        //expect on top    : ╔════════ My Win ═══════[Bin:11010]═[x]╗
        //expect on bottom : ╚[Dec:26|Hex:1A]═══════════════════════╝
        Paint('Number is 26')
        CheckHash(0x5093F71409CDB401)
        Mouse.Click(36,4,left)
        //expect on top    : ╔════════ My Win ═══════[Bin:11010]═[x]╗
        //expect on bottom : ╚[Hex:1A]══════════════════════════════╝
        Paint('Decimal is missing')
        CheckHash(0x6E330F6FF7C5A9E9)
        Mouse.Click(36,6,left)
        //expect on top    : ╔══════════════ My Win ═════════════[x]╗
        //expect on bottom : ╚[Hex:1A]══════════════════════════════╝
        Paint('Bin & Dec are missing')
        CheckHash(0x553722D16D8569A8)
        Mouse.Click(17,4,left)
        //expect on top    : ╔══════════════ My Win ═════════════[x]╗
        //expect on bottom : ╚[Hex:1B]══════════════════════════════╝
        Paint('Number if nou 27')
        CheckHash(0x5DF422977AF47E13)
        Mouse.Click(36,5,left)
        //expect on top    : ╔══════════════ My Win ═════════════[x]╗
        //expect on bottom : ╚══════════════════════════════════════╝
        Paint('All are hidden')
        CheckHash(0x7E23A72AB78B3DA3)
        Mouse.Click(17,4,left)
        Mouse.Click(36,4,left)
        //expect on top    : ╔══════════════ My Win ═════════════[x]╗
        //expect on bottom : ╚[Dec:28]══════════════════════════════╝
        Paint('Dec is visible, number is 28')
        CheckHash(0x89AB3F6209F291DD)
        Mouse.Click(36,5,left)
        Mouse.Click(36,6,left)
        //expect on top    : ╔════════ My Win ═══════[Bin:11100]═[x]╗
        //expect on bottom : ╚[Dec:28|Hex:1C]═══════════════════════╝
        Paint('All are visible')
        CheckHash(0x38A45512A58B0DE5)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

#[test]
fn check_window_move_and_resize_via_keys() {
    let script = "
        Paint.Enable(false)
        Paint('initial state')
        CheckHash(0x85955D7DF379551A)
        Key.Pressed(Ctrl+Alt+M)
        Paint('moving state')
        CheckHash(0xDC4029B447A8AA86)
        Key.Pressed(Left)
        Paint('Moved to left')
        CheckHash(0xF7D7B25914F7C796)
        Key.Pressed(Down)
        Paint('Moved down')
        CheckHash(0x30767E399BB306F6)
        Key.Pressed(Ctrl+Right,3)
        Paint('Increase with by 3')
        CheckHash(0x923F6BE5676D333E)
        Key.Pressed(Escape)
        Paint('Exit resize mode')
        CheckHash(0x477535899976906A)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let w = Window::new("Title", layout!("a:c,w:40,h:8"), window::Flags::Sizeable);
    a.add_window(w);
    a.run();
}

#[test]
fn check_modal_window() {
    #[ModalWindow(events = ButtonEvents+WindowEvents,response=i32,internal=true)]
    struct MyWin {
        b1: Handle<Button>,
        b2: Handle<Button>,
        b3: Handle<Button>,
        lb: Handle<Label>,
        counter: i32,
    }

    impl MyWin {
        fn new(title: &str, counter: i32) -> Self {
            let mut win = MyWin {
                base: ModalWindow::new(title, layout!("a:c,w:40,h:9"), window::Flags::None),
                b1: Handle::None,
                b2: Handle::None,
                b3: Handle::None,
                lb: Handle::None,
                counter,
            };
            win.b1 = win.add(button!("'Show modal &window',x:50%,y:2,p:c,w:30"));
            win.b2 = win.add(Button::new(
                format!("Counter = {counter}").as_str(),
                layout!("x:50%,y:4,p:c,w:30"),
                button::Type::Normal,
            ));
            win.b3 = win.add(button!("E&xit,x:50%,y:6,p:c,w:30"));
            win.lb = win.add(Label::new("", layout!("x:0,y:0,w:100%")));
            win
        }
        fn update_counter(&mut self) {
            let handle = self.b2;
            let counter = self.counter;
            if let Some(b2) = self.control_mut(handle) {
                b2.set_caption(format!("Counter = {counter}").as_str());
            }
        }
    }

    impl WindowEvents for MyWin {
        fn on_accept(&mut self) {
            self.exit_with(self.counter * 3);
        }
    }
    impl ButtonEvents for MyWin {
        fn on_pressed(&mut self, button_handle: Handle<Button>) -> EventProcessStatus {
            if button_handle == self.b1 {
                let response = MyWin::new(format!("{}", self.counter + 1).as_str(), self.counter + 1).show();
                let handle = self.lb;
                if let (Some(r), Some(lb)) = (response, self.control_mut(handle)) {
                    lb.set_caption(format!("Reponse from modal window: {r}").as_str());
                } else if response.is_none() {
                    if let Some(lb) = self.control_mut(handle) {
                        lb.set_caption("Exit with None from modal window !");
                    }
                }
                return EventProcessStatus::Processed;
            }
            if button_handle == self.b2 {
                self.counter += 1;
                self.update_counter();
                return EventProcessStatus::Processed;
            }
            if button_handle == self.b3 {
                self.exit_with(self.counter * 2);
                return EventProcessStatus::Processed;
            }
            EventProcessStatus::Ignored
        }
    }

    #[Desktop(events=CommandBarEvents,internal=true, commands=CreateModal)]
    struct MyDesktop {}
    impl MyDesktop {
        fn new() -> Self {
            Self { base: Desktop::new() }
        }
    }
    impl CommandBarEvents for MyDesktop {
        fn on_update_commandbar(&self, commandbar: &mut CommandBar) {
            commandbar.set(key!("F1"), "Create a modal window", mydesktop::Commands::CreateModal);
        }

        fn on_event(&mut self, command_id: mydesktop::Commands) {
            if command_id == mydesktop::Commands::CreateModal {
                let _response = MyWin::new("1", 1).show();
            }
        }
    }

    let script = "
    Paint.Enable(false)
    Paint('initial state (no window)')
    CheckHash(0x172AA26FB2F2488C)
    Key.Pressed(F1);    
    // we should see a window with Title '1' and Exit Button seleected
    Paint('One Window, nothing on command bar')
    CheckHash(0x7240CD633AEAD74C)
    Mouse.Click(30,10,left)
    Paint('Counter = 2')
    CheckHash(0xF657C3966674A472)
    Mouse.Click(30,8,left)
    Mouse.Drag(30,6,20,1)
    // we have two windows, second with title = 3
    Paint('2 windows')
    CheckHash(0xBDB15B3066946AFE)       
    Mouse.Click(20,5,left)
    Mouse.Click(20,3,left)
    Mouse.Drag(30,6,40,0)
    // we have the 3rd window created with Counter=5
    Paint('New window created')
    CheckHash(0x8EAEAD312FC57319)
    Key.Pressed(Tab,2)
    Key.Pressed(Enter,2)
    Paint('Press enter on 1st button')
    // now we should have window 5 with counter = 7 and focus button on Counter button
    CheckHash(0xD0AD1B5F86BCD962)
    Key.Pressed(Tab)
    Key.Pressed(Enter)
    // the 3rd window is closed, the message received on the second window should be:
    // Reponse from modal window: 14 
    Paint('3rd window closed')
    CheckHash(0xD17830D3177DAA48)
    Key.Pressed(Escape)
    Paint('2rd window closed')
    // the 2nd window is closed, the message received on the first window should be None:
    // Exit with None from modal window !
    CheckHash(0x10E7726F51F0AA03)
    Mouse.Click(30,13,left)
    // we should expect the same desktop as with the initial state
    Paint('1rd window closed')
    CheckHash(0x172AA26FB2F2488C)
    ";
    let app = App::debug(60, 20, script).desktop(MyDesktop::new()).command_bar().build().unwrap();
    app.run();
}

#[test]
fn check_window_fixed_pos() {
    let script = "
        Paint.Enable(false)
        Paint('centered window')
        CheckHash(0x47625E4DA1A7B38F)
        Mouse.Drag(12,1,10,0)
        Paint('Moveable window moved')
        CheckHash(0xC3DDD1ED3F648A7)
        Mouse.Drag(40,1,30,2)
        Paint('Non-Moveable window not-moved, just focused')
        CheckHash(0x9A7FCF55DCA77E9F)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    a.add_window(Window::new("Moveable", layout!("x:5,y:1,w:20,h:6"), window::Flags::None));
    a.add_window(Window::new(
        "Non-Moveable",
        layout!("x:30,y:1,w:25,h:6"),
        window::Flags::FixedPosition,
    ));
    a.run();
}

#[test]
fn check_window_macro() {
    let script = "
        Paint.Enable(false)
        Paint('two windows')
        CheckHash(0x88E8AE83D20D73A7)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    a.add_window(window!("First,x:5,y:2,w:20,h:6"));
    a.add_window(window!("title='Test me',x:30,y:1,w:25,h:6,flags=[Sizeable,FixedPosition]"));
    a.run();
}

#[test]
fn check_window_resize_mode_keys() {
    let script = "
        Paint.Enable(false)
        Paint('centered window')
        CheckHash(0x7062D4CC2F7219DA)
        Key.Pressed(Ctrl+Alt+M)
        Paint('Resize mode')
        CheckHash(0x609DD7B9237DC346)
        Key.Pressed(Left,4)
        Key.Pressed(Down,2)
        Paint('After moving')
        CheckHash(0x2BB68415887137A6)
        Key.Pressed(Ctrl+Right,5)
        Key.Pressed(Ctrl+Up,1)
        Paint('After Resizing')
        CheckHash(0x7B2E82058CE2E560)
        Key.Pressed(M)
        Paint('Maximizing')
        CheckHash(0x34CC388C72DC662)
        Key.Pressed(R)
        Paint('Restored')
        CheckHash(0x7B2E82058CE2E560)
        Key.Pressed(Alt+Up)
        Key.Pressed(Alt+Right)
        Paint('Align to TopRight')
        CheckHash(0x4DECC8DF7632F5EC)
        Key.Pressed(Alt+Left)
        Key.Pressed(Alt+Down)
        Paint('Align to BottomRight')
        CheckHash(0x9166F8D445C80E10)
        Key.Pressed(Escape)
        Paint('Exit from resize mode')
        CheckHash(0x1D6AFE13EB1B0934)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    a.add_window(Window::new("Title", layout!("a:c,w:20,h:6"), window::Flags::Sizeable));
    a.run();
}

#[test]
fn check_window_background() {
    let script = "
        Paint.Enable(false)
        Mouse.Click(10,2,left)
        Paint('Normal window enabled')
        CheckHash(0xCB2510920C66FDB)
        Mouse.Click(40,2,left)
        Paint('Error window enabled')
        CheckHash(0x1E3C6A9A4F7E2ED7)
        Mouse.Click(10,7,left)
        Paint('Notify window enabled')
        CheckHash(0x6A9C490B34C3AA7B)
        Mouse.Click(40,7,left)
        Paint('Warning window enabled')
        CheckHash(0x8786A6E5A142320F)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    a.add_window(window!("Normal,x:0,y:0,w:30,h:5"));
    a.add_window(window!("Error,x:30,y:0,w:30,h:5,type=Error"));
    a.add_window(window!("Notify,x:0,y:5,w:30,h:5,type=Notification"));
    a.add_window(window!("Warning,x:30,y:5,w:30,h:5,type=Warning"));
    a.run();
}

#[test]
fn check_window_on_cancel_callback() {
    #[Window(events=WindowEvents, internal = true)]
    struct MyWin {
        count: u32,
        txt: Handle<Label>,
    }
    impl MyWin {
        fn new() -> Self {
            let mut win = MyWin {
                base: window!("Test,a:c,w:30,h:6"),
                txt: Handle::None,
                count: 4,
            };
            win.txt = win.add(label!("'',x:1,y:1,w:10"));
            win.update_count();
            win
        }
        fn update_count(&mut self) {
            let h = self.txt;
            let c = self.count;
            if let Some(label) = self.control_mut(h) {
                label.set_caption(format!("tries: {c}").as_str());
            }
        }
    }
    impl WindowEvents for MyWin {
        fn on_cancel(&mut self) -> ActionRequest {
            self.count -= 1;
            self.update_count();
            if self.count == 0 {
                ActionRequest::Allow
            } else {
                ActionRequest::Deny
            }
        }
    }
    let script = "
        Paint.Enable(false)
        Paint('tries: 4')
        CheckHash(0x37BC6BC2CBE19B06)
        Mouse.Click(42,2,left)
        Paint('tries: 3')
        CheckHash(0x2E1FABC21167ED70)
        Key.Pressed(Escape,2)
        Paint('tries: 1')
        CheckHash(0xC745411D9CC128DE)
        Mouse.Click(42,2,left)
        Paint('Windows is closed')
        CheckHash(0x734FECAF52FDE955)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

#[test]
fn check_window_enter_resize_mode() {
    #[Window(events=ButtonEvents, internal = true)]
    struct MyWin {}
    impl MyWin {
        fn new() -> Self {
            let mut win = MyWin {
                base: window!("Test,a:c,w:30,h:6"),
            };
            win.add(button!("ResizeMe,a:c,w:16"));
            win
        }
    }
    impl ButtonEvents for MyWin {
        fn on_pressed(&mut self, _: Handle<Button>) -> EventProcessStatus {
            self.enter_resize_mode();
            EventProcessStatus::Processed
        }
    }
    let script = "
        Paint.Enable(false)
        Paint('initial state')
        CheckHash(0x4CAC2CA20D555971)
        Key.Pressed(Enter)
        Paint('Windows in resize mode')
        CheckHash(0xA09BD0005E903DE4)
        Key.Pressed(Left,4)
        Key.Pressed(Up,2)
        Paint('Windows move')
        CheckHash(0x2A18B451961FA2A4)
        Key.Pressed(Escape)
        Paint('Normal mode again')
        CheckHash(0xC394FEBC8D729121)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

#[test]
fn check_window_keybeforechildren() {
    #[CustomControl(overwrite=OnPaint+OnKeyPressed,internal=true)]
    struct MyCustomControl {
        text: String,
    }
    impl MyCustomControl {
        fn new() -> Self {
            MyCustomControl {
                base: ControlBase::with_status_flags(
                    layout!("a:c,w:100%,h:1"),
                    StatusFlags::AcceptInput | StatusFlags::Enabled | StatusFlags::Visible,
                ),
                text: String::new(),
            }
        }
    }
    impl OnPaint for MyCustomControl {
        fn on_paint(&self, surface: &mut Surface, _theme: &Theme) {
            surface.clear(Character::new(' ', Color::White, Color::Black, CharFlags::None));
            surface.write_string(1, 0, self.text.as_str(), CharAttribute::with_color(Color::White, Color::Black), false);
        }
    }
    impl OnKeyPressed for MyCustomControl {
        fn on_key_pressed(&mut self, key: Key, _character: char) -> EventProcessStatus {
            self.text = format!("Key = {:?}", key.code);
            if key.code == KeyCode::Enter {
                // this is not a safe practice as a custom control can not emmit a ButtonEvent
                // not it is possible when using the crate.
                // However, sa this is an internal implementation, we can access this type of
                // data and we can emit such am event.
                self.raise_event(ControlEvent {
                    emitter: self.handle,
                    receiver: self.event_processor,
                    data: ControlEventData::Button(button::events::EventData {}),
                });
            }
            // all keys are processed
            EventProcessStatus::Processed
        }
    }
    #[Window(events=ButtonEvents, internal = true)]
    struct MyWin {}
    impl MyWin {
        fn new() -> Self {
            let mut win = MyWin {
                base: window!("Test,a:c,w:40,h:6"),
            };
            win.add(MyCustomControl::new());
            win
        }
    }
    impl ButtonEvents for MyWin {
        fn on_pressed(&mut self, _handle: Handle<Button>) -> EventProcessStatus {
            // the _handle is in fact the handle of the MyCustomControl
            // as such we will not check it here nor we will try to convert it into a Button
            // as this will result in an undefine behavior (most likely a crash)
            // However, as we just want to test if the event reaches this point, we can use it safely.
            self.enter_resize_mode();
            EventProcessStatus::Processed
        }
    }
    let script = "
        Paint.Enable(false)
        Paint('initial state')
        CheckHash(0xB78356C77DB102B1)
        Key.Pressed(Left)
        Paint('Key = Left')
        CheckHash(0xF64D6BBF7331F0F0)
        Key.Pressed(Enter)
        Paint('Key = Enter, Go to resize mode')
        CheckHash(0x55A350BD025BC27A)
        Key.Pressed(Left,4)
        Paint('Key = Enter, Window is moved')
        CheckHash(0x5E72A50367CD6A1A)
        Key.Pressed(Escape)
        Paint('Window is normal now (Key = Enter)')
        CheckHash(0x4713697F6B3E4713)
        // starting from this point, the keys are process by child and not by the window
        Key.Pressed(Left)
        Paint('Key = Left')
        CheckHash(0x8A6D6218FD2656D0)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

#[test]
fn check_window_on_close_default() {
    let script = "
        Paint.Enable(false)
        Paint('Two windows')
        CheckHash(0x782B90B5A3044A22)
        Mouse.Click(57,0,left)
        Paint('Win-2 is closed (only Win-1 exists)')
        // CheckHash(0x1E3C6A9A4F7E2ED7)
        // Mouse.Click(10,7,left)
        // Paint('Notify window enabled')
        // CheckHash(0x6A9C490B34C3AA7B)
        // Mouse.Click(40,7,left)
        // Paint('Warning window enabled')
        // CheckHash(0x8786A6E5A142320F)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    a.add_window(window!("Win-1,x:0,y:0,w:30,h:5"));
    a.add_window(window!("Win-2,x:30,y:0,w:30,h:5"));
    a.run();
}

#[test]
fn check_window_close() {
    #[Window(events = ButtonEvents,internal = true)]
    struct MyWin {}
    impl MyWin {
        fn new() -> Self {
            let mut me = Self {
                base: window!("Test,a:c,w:30,h:8"),
            };
            me.add(button!("Close,a:c,w:14,h:1"));
            me
        }
    }
    impl ButtonEvents for MyWin {
        fn on_pressed(&mut self, _: Handle<Button>) -> EventProcessStatus {
            self.close();
            EventProcessStatus::Processed
        }
    }

    let script = "
        Paint.Enable(false)
        Paint('initial state')
        CheckHash(0x9E8F35BB0431A941)
        Mouse.Click(30,4,left)
        Paint('window is closed - empty desktop')
        CheckHash(0x734FECAF52FDE955)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

#[test]
fn check_modal_window_close() {
    #[ModalWindow(events = ButtonEvents, response: i32, internal=true)]
    struct MyModalWin {
        value: i32,
    }
    impl MyModalWin {
        fn new(value: i32) -> Self {
            let mut me = Self {
                base: ModalWindow::with_type("Modal", layout!("a:c,w:50,h:6"), window::Flags::None, window::Type::Notification),
                value,
            };
            if value % 2 == 0 {
                me.add(button!("Close,a:c,w:24,h:1"));
            } else {
                me.add(button!("Exit,a:c,w:24,h:1"));
            }
            me
        }
    }
    impl ButtonEvents for MyModalWin {
        fn on_pressed(&mut self, _: Handle<Button>) -> EventProcessStatus {
            if self.value % 2 == 0 {
                self.close();
            } else {
                self.exit();
            }
            EventProcessStatus::Processed
        }
    }

    #[Window(events = ButtonEvents,internal = true)]
    struct MyWin {
        value: i32,
    }
    impl MyWin {
        fn new() -> Self {
            let mut me = Self {
                base: window!("Test,a:c,w:30,h:8"),
                value: 0,
            };
            me.add(button!("Run,a:c,w:14,h:1"));
            me
        }
    }
    impl ButtonEvents for MyWin {
        fn on_pressed(&mut self, _: Handle<Button>) -> EventProcessStatus {
            MyModalWin::new(self.value).show();
            self.value += 1;
            EventProcessStatus::Processed
        }
    }

    let script = "
        Paint.Enable(false)
        Paint('initial state')
        CheckHash(0xF724A64E6A51AC9E)
        Mouse.Click(30,4,left)
        Paint('Modal with close button')
        CheckHash(0xC1F4302009EB0D63)
        Key.Pressed(Enter)
        Paint('back initial state')
        CheckHash(0xF724A64E6A51AC9E)
        Key.Pressed(Enter)
        Paint('Modal with exit button')
        CheckHash(0xD2F95351BFDDC8A1)
        Key.Pressed(Enter)
        Paint('back initial state')
        CheckHash(0xF724A64E6A51AC9E)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

#[test]
fn check_window_close_with_commandbar_and_menu() {
    #[Window(events = CommandBarEvents+MenuEvents, commands: A, internal = true)]
    struct MyWin {
        h_menu: Handle<Menu>,
    }
    impl MyWin {
        fn new() -> Self {
            let mut w = Self {
                base: window!("Test,a:c,w:30,h:8"),
                h_menu: Handle::None,
            };
            w.h_menu = w.register_menu(menu!("File,class:MyWin,items=[{New,cmd:A}]"));
            w
        }
    }
    impl CommandBarEvents for MyWin {
        fn on_update_commandbar(&self, commandbar: &mut CommandBar) {
            commandbar.set(key!("F1"), "Do something", mywin::Commands::A);
        }

        fn on_event(&mut self, _: mywin::Commands) { /* do nothing */
        }
    }
    impl MenuEvents for MyWin {
        fn on_update_menubar(&self, menubar: &mut MenuBar) {
            menubar.add(self.h_menu, 0);
        }
    }
    let script = "
        Paint.Enable(false)
        Paint('initial state (F1 Do something is visible, File menu is visible)')
        CheckHash(0x3BC0083FFBDD10CF)
        Key.Pressed(Escape)
        Paint('window is closed - empty desktop & empty command bar, no menus')
        // we should NOT see any command visible on the command bar
        // we should NOT see any menu in the menu bar
        CheckHash(0x75E8571FB3005265)
    ";
    let mut a = App::debug(60, 10, script).command_bar().menu_bar().build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

#[test]
fn check_procmacro_tag() {
    let script = "
        Paint.Enable(false)
        // ╔[myWin]══════ Test ══════[x]╗
        Paint('has myWin tag')
        CheckHash(0x68F0A3E0FB3C4F91)
    ";
    let mut a = App::debug(40, 10, script).build().unwrap();
    a.add_window(window!("Test,a:c,w:30,h:8,tag:'myWin'"));
    a.run();
}

#[test]
fn check_procmacro_hotkey() {
    let script = "
        Paint.Enable(false)
        // ┌[3]──────── Test ────────[x]┐╔[F10]═══════ Test ═══════[x]╗
        Paint('hot keys')
        CheckHash(0xD9C42D40B16A6F46)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    a.add_window(window!("Test,x:0,y:1,w:30,h:8,hotkey:Ctrl+3"));
    a.add_window(window!("Test,x:30,y:1,w:30,h:8,hotkey:Ctrl+Alt+F10"));
    a.run();
}

#[test]
fn check_procmacro_auto_hotkey() {
    let script = "
        Paint.Enable(false)
        // ┌[1]─── Test ───[x]┐┌[2]─── Test ───[x]┐┌[3]─── Test ───[x]┐╔[4]═══ Test ═══[x]╗
        Paint('hot keys (1 to 4) - 4th window selected')
        CheckHash(0x293B1966B54D22E6)
        Key.Pressed(Alt+2)
        Paint('hot keys (1 to 4) - 2nd window selected')
        CheckHash(0x2FB120200251AD9E)
    ";
    let mut a = App::debug(80, 10, script).build().unwrap();
    a.add_window(window!("Test,x:0,y:1,w:20,h:8,hotkey:auto"));
    a.add_window(window!("Test,x:20,y:1,w:20,h:8,hotkey:auto"));
    a.add_window(window!("Test,x:40,y:1,w:20,h:8,hotkey:auto"));
    a.add_window(window!("Test,x:60,y:1,w:20,h:8,hotkey:auto"));
    a.run();
}

#[test]
fn check_single_window() {
    let script = "
        Paint.Enable(false)
        Paint('initial state (full-screen)')
        CheckHash(0x8B0177987DEEAB59)
    ";
    let mut a = App::debug(40, 10, script).single_window().build().unwrap();
    a.add_window(window!("Test,x:0,y:1,w:10,h:8,hotkey:auto"));
    a.run();
}

#[test]
fn check_single_window_with_commandbar() {
    let script = "
        Paint.Enable(false)
        Paint('initial state (full-screen,command bar visible)')
        CheckHash(0x1250605ADBD5CC99)
    ";
    let mut a = App::debug(40, 10, script).single_window().command_bar().build().unwrap();
    a.add_window(window!("Test,x:0,y:1,w:10,h:8,hotkey:auto"));
    a.run();
}

#[test]
fn check_single_window_with_menubar() {
    let script = "
        Paint.Enable(false)
        Paint('initial state (full-screen,menu bar visible)')
        CheckHash(0x4E4AA2CCB6734C99)
    ";
    let mut a = App::debug(40, 10, script).single_window().menu_bar().build().unwrap();
    a.add_window(window!("Test,x:0,y:1,w:10,h:8,hotkey:auto"));
    a.run();
}

#[test]
fn check_single_window_with_menu_and_command_bar() {
    let script = "
        Paint.Enable(false)
        Paint('initial state (full-screen,menu and command bar visible)')
        CheckHash(0xA95AE2CA1B885CD9)
    ";
    let mut a = App::debug(40, 10, script).single_window().menu_bar().command_bar().build().unwrap();
    a.add_window(window!("Test,x:0,y:1,w:10,h:8,hotkey:auto"));
    a.run();
}

#[test]
#[should_panic(
    expected = "When `single_window(...)` is being used to initialized an application, you can only use add_window(...) method once (to add the first and single window) !"
)]
fn check_single_window_panic_on_multiple_add_window() {
    let script = "
        Paint.Enable(false)
        Paint('initial state (full-screen)')
        // this code will not be reached
        CheckHash(0x0)
    ";
    let mut a = App::debug(40, 10, script).single_window().build().unwrap();
    a.add_window(window!("Test,x:0,y:1,w:10,h:8,hotkey:auto"));
    a.add_window(window!("Test,x:0,y:1,w:10,h:8,hotkey:auto"));
    a.run();
}

#[test]
#[should_panic(
    expected = "A window used in a single window mode (via App::build().single_window()) can not be sizeable as it will always have the same size as the desktop. Remove the Sizeable flag and try again !"
)]
fn check_single_window_panic_on_sizeable_flags() {
    let script = "
        Paint.Enable(false)
        Paint('initial state (full-screen)')
        // this code will not be reached
        CheckHash(0x0)
    ";
    let mut a = App::debug(40, 10, script).single_window().build().unwrap();
    a.add_window(window!("Test,x:0,y:1,w:10,h:8,flags:sizeable"));
    a.run();
}

#[test]
#[should_panic(
    expected = "You can not run a single window app and not add a window to the app. Have you forget to add an '.add_window(...)' call before the .run() call ?"
)]
fn check_single_window_panic_no_window() {
    let script = "
        Paint.Enable(false)
        Paint('initial state (full-screen)')
        // this code will not be reached
        CheckHash(0x0)
    ";
    let a = App::debug(40, 10, script).single_window().build().unwrap();
    a.run();
}

#[test]
#[should_panic(
    expected = "When `single_window(...)` is being used to initialized an application, you can not use `.desktop(...)` command to provide a custom desktop !"
)]
fn check_single_window_panic_no_custom_desktop() {
    #[Desktop(overwrite = OnPaint, internal = true)]
    struct MyDesktop {}
    impl MyDesktop {
        fn new() -> Self {
            Self { base: Desktop::new() }
        }
    }
    impl OnPaint for MyDesktop {
        fn on_paint(&self, surface: &mut Surface, _theme: &Theme) {
            surface.clear(Character::new('x', Color::Red, Color::Green, CharFlags::None));
        }
    }
    let script = "
        Paint.Enable(false)
        Paint('desktop with red and green')
        CheckHash(0x0)
    ";
    let a = App::debug(60, 10, script).desktop(MyDesktop::new()).single_window().build().unwrap();
    a.run();
}

#[test]
fn check_multiple_windows_focus() {
    let script = "
        Paint.Enable(false)
        Paint('initial state')
        CheckHash(0x97D61849202889DF)
        Mouse.Click(18,4,left)
        Paint('Win2 on top & focus')
        CheckHash(0xFB56BEF5838FD80C)
        Mouse.Click(10,1,left)
        Paint('Win1 on top & focus')
        CheckHash(0x53405223F149E9D)
        Mouse.Click(35,6,left)
        Paint('Win3 on top & focus')
        CheckHash(0x97D61849202889DF)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    a.add_window(window!("Win1,x:1,y:1,w:30,h:5"));
    a.add_window(window!("Win2,x:11,y:3,w:30,h:5"));
    a.add_window(window!("Win3,x:21,y:5,w:30,h:5"));
    a.run();
}

#[test]
fn check_window_toolbar_single_choice_caption() {
    let script = "
        Paint.Enable(false)
        //expect on bottom: ╚[Option 1|Option 2]═══════════════════════╝
        Paint('Initial state - Option 1 selected')
        CheckHash(0x6A43F6EA0169CF5B)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = Window::new("Title", layout!("a:c,w:40,h:8"), window::Flags::None);

    // Create toolbar with single choice items
    let g = w.toolbar().create_group(GroupPosition::BottomLeft);
    // Add items to toolbar
    let h1 = w.toolbar().add(g, toolbar::SingleChoice::new("Option &1"));
    let h2 = w.toolbar().add(g, toolbar::SingleChoice::new("Option &2"));

    // Verify initial state
    assert_eq!(w.toolbar().get(h1).unwrap().caption(), "Option 1");
    assert_eq!(w.toolbar().get(h2).unwrap().caption(), "Option 2");
    assert!(!w.toolbar().get(h1).unwrap().is_selected());
    assert!(!w.toolbar().get(h2).unwrap().is_selected());

    a.add_window(w);
    a.run();
}

#[test]
fn check_resize_mode_keys() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xFB90C0EC876B3F5)
        Mouse.Click(12,3,left)
        Paint('2. Maximized')
        CheckHash(0xA4905A9581B29981)
        Mouse.Click(3,0,left)
        Paint('3. Restore initial size')
        CheckHash(0xFB90C0EC876B3F5)
        Key.Pressed(Ctrl+Alt+R)
        Paint('4. Enter in resize mode')
        CheckHash(0x9D898E4DE3BD8DE8)
        Key.Pressed(Right,10)
        Paint('5. Move to Right 10 characters')
        CheckHash(0x7EE9041F0A3176B8)
        Key.Pressed(C)
        Paint('6. Center the window')
        CheckHash(0x9D898E4DE3BD8DE8)
        Key.Pressed(Alt+F1)
        Paint('7. Nothing happens')
        CheckHash(0x9D898E4DE3BD8DE8)      
        Key.Pressed(Ctrl+Down)
        Paint('8. Increase height')
        CheckHash(0x8344EF1CD0DB1C08)   
        Key.Pressed(Ctrl+Left)
        Paint('9. Decrease Width')
        CheckHash(0xE0A5BFC25FE43E2E)   
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Title,a:c,w:40,h:8,flags: Sizeable");
    w.set_tag("XYZ");
    assert_eq!(w.tag(), Some("XYZ"));
    assert_eq!(w.title(), "Title");
    a.add_window(w);
    a.run();
}
