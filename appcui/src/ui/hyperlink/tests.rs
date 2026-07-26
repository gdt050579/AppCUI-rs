use crate::prelude::*;

#[test]
fn check_hyperlink_control() {
    #[Window(events = HyperLinkEvents, internal = true)]
    struct MyWin {
        info: Handle<Label>,
        hl1: Handle<HyperLink>,
        hl2: Handle<HyperLink>,
        hl3: Handle<HyperLink>,
    }

    impl MyWin {
        fn new() -> Self {
            let mut win = Self {
                base: Window::new("Win-1", layout!("a:c,w:40,h:10"), window::Flags::None),
                info: Handle::None,
                hl1: Handle::None,
                hl2: Handle::None,
                hl3: Handle::None,
            };
            win.hl1 = win.add(HyperLink::new(
                "AppCUI-rs",
                "https://github.com/gdt050579/AppCUI-rs",
                layout!("x:1,y:1,w:10"),
            ));
            win.hl2 = win.add(HyperLink::with_url("https://github.com/gdt050579/AppCUI-rs", layout!("x:0,y:2,w:38")));
            win.hl3 = win.add(HyperLink::with_tooltip(
                "AppCUI-rs",
                "https://github.com/gdt050579/AppCUI-rs",
                "A cross-platform TUI framework for Rust",
                layout!("x:3,y:3,w:10"),
            ));

            win.info = win.add(Label::new("<none>", layout!("x:0,y:0,w:35")));

            win
        }
        fn set_info(&mut self, txt: &str) {
            let h_label = self.info;
            if let Some(label) = self.control_mut(h_label) {
                label.set_caption(txt);
            }
        }
    }

    impl HyperLinkEvents for MyWin {
        fn on_open(&mut self, handle: Handle<HyperLink>) -> EventProcessStatus {
            if self.hl1 == handle {
                self.set_info("HyperLink 1 presed");
                return EventProcessStatus::Processed;
            }
            if self.hl2 == handle {
                self.set_info("HyperLink 2 presed");
                return EventProcessStatus::Processed;
            }
            EventProcessStatus::Ignored
        }
    }

    let script = "
        Paint.Enable(false)
        Paint('1.initial state')
        CheckHash(0xDEDD4F59D13E8A44)
        Mouse.Move(14,4)
        Paint('2.hyperlink 1 is hovered')
        CheckHash(0x150B19159EA52C4D)
        Mouse.Hold(14,4,left)
        Paint('3.hyperlink 1 is pressed')
        CheckHash(0x63283E8E4A32A33D)
        Mouse.Release(14,4,left)
        Paint('4. hyperlink 1 is released')
        CheckHash(0x70E6543ACF1D300A)
        Mouse.Click(14,4,left)
        Paint('5.hyperlink 1 is clicked')
        CheckHash(0x70E6543ACF1D300A)

        Mouse.Move(14,5)
        Paint('6.hyperlink 2 is hovered')
        CheckHash(0x9C02190F657A3F26)
        Mouse.Hold(14,5,left)
        Paint('7.hyperlink 2 is pressed')
        CheckHash(0x22BB237276F70E76)
        Mouse.Release(14,5,left)
        Paint('8. hyperlink 2 is released')
        CheckHash(0x535EA143919C70D)
        Mouse.Click(14,5,left)
        Paint('9.hyperlink 2 is clicked')
        CheckHash(0x535EA143919C70D)

        Mouse.Move(14,6)
        Paint('10.hyperlink 3 is hovered')
        CheckHash(0xD686C99401E08907)
        Mouse.Hold(14,6,left)
        Paint('11.hyperlink 3 is pressed')
        CheckHash(0x408CCC5AE113A7E8)
        Mouse.Release(14,6,left)
        Paint('12. hyperlink 3 is released')
        CheckHash(0x63626CC4102F4EC1)
        Mouse.Click(14,6,left)
        Paint('13.hyperlink 3 is clicked')
        CheckHash(0x63626CC4102F4EC1)
    ";
    let mut a = App::debug(60, 14, script).build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

#[test]
fn check_hyperlink_control_2() {
    #[Window(events = HyperLinkEvents, internal = true)]
    struct MyWin {
        info: Handle<Label>,
        hl1: Handle<HyperLink>,
        hl2: Handle<HyperLink>,
        hl3: Handle<HyperLink>,
    }

    impl MyWin {
        fn new() -> Self {
            let mut win = Self {
                base: Window::new("Win-1", layout!("a:c,w:50,h:10"), window::Flags::None),
                info: Handle::None,
                hl1: Handle::None,
                hl2: Handle::None,
                hl3: Handle::None,
            };
            win.hl1 = win.add(HyperLink::new(
                "AppCUI-rs",
                "https://github.com/gdt050579/AppCUI-rs",
                layout!("x:1,y:1,w:10"),
            ));
            win.hl2 = win.add(HyperLink::with_url("https://github.com/gdt050579/AppCUI-rs", layout!("x:0,y:2,w:38")));
            win.hl3 = win.add(HyperLink::with_tooltip(
                "AppCUI-rs",
                "https://github.com/gdt050579/AppCUI-rs",
                "A cross-platform TUI framework for Rust",
                layout!("x:3,y:3,w:10"),
            ));

            win.info = win.add(Label::new("<none>", layout!("x:0,y:0,w:48")));

            win
        }
        fn set_info(&mut self, txt: &str) {
            let h_label = self.info;
            if let Some(label) = self.control_mut(h_label) {
                label.set_caption(txt);
            }
        }
    }

    impl HyperLinkEvents for MyWin {
        fn on_open(&mut self, handle: Handle<HyperLink>) -> EventProcessStatus {
            if self.hl1 == handle {
                let url = self.control(handle).map(|l| l.url().to_string());
                if let Some(url) = url {
                    self.set_info(&url);
                }
                if let Some(link) = self.control_mut(handle) {
                    link.set_url("Ok");
                }
                return EventProcessStatus::Processed;
            }
            if self.hl2 == handle {
                let name = self.control(handle).map(|l| l.name().to_string());
                if let Some(name) = name {
                    self.set_info(&name);
                }
                if let Some(link) = self.control_mut(handle) {
                    link.set_name("Ok");
                }
                return EventProcessStatus::Processed;
            }
            if self.hl3 == handle {
                let tooltip = self.control(handle).map(|l| l.tooltip().to_string());
                if let Some(tooltip) = tooltip {
                    self.set_info(&tooltip);
                }
                if let Some(link) = self.control_mut(handle) {
                    link.set_tooltip("Ok");
                }
                return EventProcessStatus::Processed;
            }
            EventProcessStatus::Ignored
        }
    }

    let script = "
        Paint.Enable(false)
        Paint('1.initial state')
        CheckHash(0x2E5D5E402BE230C8)
        Mouse.Move(14,4)
        Mouse.Click(14,4,left)
        Paint('2.hyperlink 1 is clicked')
        CheckHash(0x7E66E0493FBB509C)
        Mouse.Click(14,4,left)
        Paint('3.hyperlink 1 is clicked')
        CheckHash(0x31B7A6C97715C638)

        Mouse.Move(14,5)
        Mouse.Click(14,5,left)
        Paint('4.hyperlink 2 is clicked')
        CheckHash(0x344D0D527284367C)
        Mouse.Click(14,5,left)
        Paint('5.hyperlink 2 is clicked')
        CheckHash(0x39374F5179765778)

        Mouse.Move(14,6)
        Mouse.Click(14,6,left)
        Paint('6.hyperlink 3 is clicked')
        CheckHash(0xBE98CE10D38856C9)
        Key.Pressed(Enter)
        Paint('7.hyperlink 3 is clicked')
        CheckHash(0x19324713879CF4AC)
    ";
    let mut a = App::debug(60, 14, script).build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

#[test]
fn check_hyperlink_control_with_macro() {
    let script = "
        Paint.Enable(false)
        Paint('1.initial state')
        CheckHash(0x33C839A5D930A167)
        Mouse.Move(12,2)
        Mouse.Hold(12,2,left)
        Paint('2.button pressed over')
        CheckHash(0x1F8A1799D9614CF7)
        Mouse.Release(12,2,left)
        Paint('3.after release')
        CheckHash(0x33C839A5D930A167)
        Key.Pressed(Space)
        Paint('4.space is ignored (nothing happens)')
        CheckHash(0x33C839A5D930A167)
        ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = Window::new("Macro Test", layout!("a:c,w:40,h:10"), window::Flags::None);
    w.add(hyperlink!("'Wiki',url:'www.wikipedia.com',x:1,y:1,w:9"));
    a.add_window(w);
    a.run();
}
