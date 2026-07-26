use appcui::{backend, prelude::*};

#[Window(events = HyperLinkEvents)]
struct MyWin {}

impl MyWin {
    fn new() -> Self {
        let mut obj = Self {
            base: window!("'Hyperlink test window',a:c,w:60,h:20,flags:Sizeable"),
        };
        let link = hyperlink!("
            'HyperLink',
            url:'https://github.com/gdt050579/AppCUI-rs',
            tooltip:'A cross-platform TUI framework for Rust',
            x:1,y:0,w:10"
        );
        obj.add(link);
        let h1 = HyperLink::new("AppCUI-rs", "https://github.com/gdt050579/AppCUI-rs", layout!("x:1,y:1,w:10"));
        let h2 = HyperLink::with_url("https://github.com/gdt050579/AppCUI-rs", layout!("x:1,y:2,w:40"));
        let h3 = HyperLink::with_tooltip("AppCUI-rs", "https://github.com/gdt050579/AppCUI-rs", "A cross-platform TUI framework for Rust", layout!("x:1,y:3,w:10"));
        let h4 = hyperlink!("name=AppCUI-rs,url='https://github.com/gdt050579/AppCUI-rs',x:1,y:4,w:10");
        let h5 = hyperlink!("'AppCUI-rs',url='https://github.com/gdt050579/AppCUI-rs',x:1,y:5,w:10");
        obj.add(h1);
        obj.add(h2);
        obj.add(h3);
        obj.add(h4);
        obj.add(h5);
        let disabled_link = hyperlink!("'AppCUI-rs',url='https://github.com/gdt050579/AppCUI-rs',x:1,y:6,w:10,enabled=false");
        let hidden_link = hyperlink!("text='AppCUI-rs',url='https://github.com/gdt050579/AppCUI-rs',x:1,y:7,w:10,visible=false");
        let link_with_tooltip = hyperlink!("'AppCUI-rs',link='https://github.com/gdt050579/AppCUI-rs',tooltip='A cross-platform TUI framework for Rust',x:1,y:8,w:10");
        obj.add(disabled_link);
        obj.add(hidden_link);
        obj.add(link_with_tooltip);
        obj
    }
}

impl HyperLinkEvents for MyWin {
    fn on_open(&mut self, handle: Handle<HyperLink>) -> EventProcessStatus {
        if let Some(l) = self.control(handle) {
            let s = l.url();
            dialogs::message("Ok", s);
        }
        EventProcessStatus::Processed
    }
}

fn main() -> Result<(), appcui::system::Error> {
    let mut app = App::with_backend(backend::Type::WindowsVT).color_schema(false).build()?;

    app.add_window(MyWin::new());

    app.run();
    Ok(())
}