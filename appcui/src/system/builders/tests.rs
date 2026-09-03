use crate::prelude::*;

#[test]
fn check_multi_window_app() {
    let script = "
        Paint.Enable(false)
        Paint('1. Two windows (Right is focused)')
        CheckHash(0xEE2D4597125B108A)
        Mouse.Click(5,3,left)
        Paint('2. Left window focused')
        CheckHash(0x5D488BE0D9FF0282)
    ";
    App::new()
        .size(Size::new(60, 12))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Left,x:1,y:1,w:26,h:7");
            w.add(label!("'Multi-window',a:c,w:14,h:1"));
            w
        })
        .window(|| {
            let mut w = window!("Right,x:30,y:2,w:26,h:7");
            w.add(label!("'mode',a:c,w:6,h:1"));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_single_window_app() {
    #[Window(events = CommandBarEvents, internal: true, commands: Ping)]
    struct DemoWindow {
        info: Handle<Label>,
    }
    impl DemoWindow {
        fn new() -> Self {
            let mut w = Self {
                base: window!("title:'Single',d:f"),
                info: Handle::None,
            };
            w.info = w.add(label!("'Press F1',a:c,w:16,h:1"));
            w
        }
    }
    impl CommandBarEvents for DemoWindow {
        fn on_update_commandbar(&self, commandbar: &mut CommandBar) {
            commandbar.set(key!("F1"), "Ping", demowindow::Commands::Ping);
        }

        fn on_event(&mut self, command_id: demowindow::Commands) {
            if command_id == demowindow::Commands::Ping {
                let h = self.info;
                if let Some(label) = self.control_mut(h) {
                    label.set_caption("Ping received");
                }
            }
        }
    }

    let script = "
        Paint.Enable(false)
        Paint('1. Single window fills the desktop')
        CheckHash(0xD4BFFA61E71E1976)
        Key.Pressed(F1)
        Paint('2. Command bar action updated the label')
        CheckHash(0x1ECCBC719BBB3D3E)
    ";
    App::single_window(|| DemoWindow::new())
        .size(Size::new(40, 10))
        .debug_script(script)
        .command_bar()
        .run()
        .unwrap();
}

#[test]
fn check_frame_app() {
    struct Demo {
        message: &'static str,
    }
    impl FrameApp for Demo {
        fn on_key_event(&mut self, key: Key, _ch: char) {
            if key.value() == key!("Space") {
                self.message = "Space pressed";
            }
        }

        fn on_paint(&self, surface: &mut Surface) {
            surface.write_string(1, 1, "Frame app", charattr!("yellow,black"), false);
            surface.write_string(1, 3, self.message, charattr!("white,black"), false);
        }
    }

    let script = "
        Paint.Enable(false)
        Paint('1. Initial frame')
        CheckHash(0xBFFBEFDBE38A038D)
        Key.Pressed(Space)
        Paint('2. After Space')
        CheckHash(0x5B85D6B71128F54A)
    ";
    App::frame_app(Demo { message: "Waiting..." })
        .size(Size::new(40, 10))
        .debug_script(script)
        .title("Frame")
        .auto_close(false)
        .run()
        .unwrap();
}

#[test]
fn check_input_app() {
    struct Demo {
        message: String,
    }
    impl InputApp for Demo {
        fn on_key_event(&mut self, key: Key, _ch: char) -> EventProcessStatus {
            if key.value() == key!("Enter") {
                self.message = "Enter pressed".to_string();
                EventProcessStatus::Processed
            } else {
                EventProcessStatus::Ignored
            }
        }

        fn on_mouse_event(&mut self, ev: &MouseEvent) -> EventProcessStatus {
            if let MouseEvent::Pressed(data) = ev {
                self.message = format!("Click {},{}", data.x, data.y);
                EventProcessStatus::Processed
            } else {
                EventProcessStatus::Ignored
            }
        }

        fn on_paint(&self, surface: &mut Surface) {
            surface.write_string(1, 1, "Input app", charattr!("aqua,black"), false);
            surface.write_string(1, 3, &self.message, charattr!("white,black"), false);
        }
    }

    let script = "
        Paint.Enable(false)
        Paint('1. Initial input surface')
        CheckHash(0x584EAE4CD6A78BB3)
        Key.Pressed(Enter)
        Paint('2. After Enter')
        CheckHash(0xAB25ECE3B88FEB18)
        Mouse.Click(8,6,left)
        Paint('3. After mouse click')
        CheckHash(0x93C5ABD2F47381A)
    ";
    App::input_app(Demo {
        message: "Waiting...".to_string(),
    })
    .size(Size::new(40, 10))
    .debug_script(script)
    .title("Input")
    .auto_close(false)
    .run()
    .unwrap();
}
