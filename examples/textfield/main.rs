use appcui::prelude::*;

#[Window(events = TextFieldEvents)]
struct MyWin {}

impl MyWin {
    fn new() -> Self {
        let mut win = MyWin {
            base: Window::new("TextField example", Layout::new("d:c,w:60,h:18"), window::Flags::None),
        };
        win.add(label!("'A normal text that can be edited',x:1,y:1,w:100%"));
        win.add(textfield!("l:1,t:2,r:1,h:1"));
        win.add(label!("'A read only text',x:1,y:4,w:100%"));
        win.add(textfield!("'read only text',l:1,t:5,r:1,h:1, flags: ReadOnly"));
        win.add(label!("'Multi-line string',x:1,y:7,w:100%"));
        win.add(textfield!("'This is a very large string that spreads over multiple lines',l:1,t:8,r:1,h:3"));
        win.add(label!("'Type 1234 and ENTER to validate',x:1,y:12,w:100%"));
        win.add(textfield!("l:1,t:13,r:1,h:1, flags: ProcessEnter"));
        win
    }
}
impl TextFieldEvents for MyWin {
    fn on_validate(&mut self, _handle: Handle<TextField>, text: &str) -> EventProcessStatus {
        if text == "1234" {
            dialogs::message("OK", "You enter the correct number !");
        } else {
            dialogs::error("Error", "Incorect - type 1234 and hit Enter");
        }
        EventProcessStatus::Ignored
    }
}

fn main() -> Result<(), appcui::system::Error> {
    let mut app = App::new().build()?;
    app.add_window(MyWin::new());
    app.run();
    Ok(())
}
