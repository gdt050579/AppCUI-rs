use appcui::{backend, prelude::*};

#[Window(events = HyperLinkEvents)]
struct MyWin {
    hl1: Handle<HyperLink>,
    hl2: Handle<HyperLink>,
    hl3: Handle<HyperLink>,
    hl4: Handle<HyperLink>,
}

impl MyWin {
    fn new() -> Self {
        let mut win = Self {
            base: window!("'Hyperlink test window',a:c,w:60,h:20,flags:Sizeable"),
            hl1: Handle::None,
            hl2: Handle::None,
            hl3: Handle::None,
            hl4: Handle::None,
        };
        win.hl1 = win.add(hyperlink!("name=AppCUI-rs,url='https://github.com/gdt050579/AppCUI-rs',x:1,y:0,w:10"));
        win.hl2 = win.add(hyperlink!(
            "text=AppCUI-rs,url='https://github.com/gdt050579/AppCUI-rs',tooltip:'A cross-platform TUI framework for Rust',x:1,y:1,w:10"
        ));
        win.hl3 = win.add(hyperlink!("'AppCUI-rs',url='https://github.com/gdt050579/AppCUI-rs',x:1,y:2,w:10"));
        win.hl4 = win.add(hyperlink!(
            "'AppCUI-rs',link='https://github.com/gdt050579/AppCUI-rs',tooltip:'A cross-platform TUI framework for Rust',x:1,y:3,w:10"
        ));
        win
    }
}

impl HyperLinkEvents for MyWin {
    fn on_open(&mut self, handle: Handle<HyperLink>) -> EventProcessStatus {
        if (handle == self.hl1) || (handle == self.hl2) || (handle == self.hl3) || (handle == self.hl4) {
            if let Some(l) = self.control(handle) {
                let s = l.url();
                dialogs::message("Message", s);
            }
            return EventProcessStatus::Processed;
        }
        EventProcessStatus::Ignored
    }
}

fn main() -> Result<(), appcui::system::Error> {
    #[cfg(target_os = "windows")]
    let mut app = App::new().backend(backend::Type::WindowsVT).color_schema(false).run()?;
    #[cfg(not(target_os = "windows"))]
    let mut app = App::new().color_schema(false).build()?;

    app.add_window(MyWin::new());

    app.run();
    Ok(())
}
