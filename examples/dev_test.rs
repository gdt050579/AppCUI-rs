use appcui::prelude::*;

#[Window(events = ComboBoxEvents+CommandBarEvents, commands: Add+Clear)]
struct MyWin {
    h: Handle<ComboBox>,
}
impl MyWin {
    fn new() -> Self {
        let mut w = Self {
            base: window!("x:1,y:1,w:30,h:15,caption:Win"),
            h: Handle::None,
        };
        w.h = w.add(ComboBox::new(Layout::new("x:1,y:1,w:26"), combobox::Flags::None));
        w
    }
}
impl ComboBoxEvents for MyWin {
    fn on_selection_changed(&mut self, handle: Handle<ComboBox>) -> EventProcessStatus {
        let title = if let Some(cb) = self.control_mut(handle) {
            if let Some(item) = cb.selected_item() {
                item.value().to_string()
            } else {
                String::from("[None]")
            }
        } else {
            String::from("?")
        };
        self.set_title(&title);
        EventProcessStatus::Processed
    }
}
impl CommandBarEvents for MyWin {
    fn on_update_commandbar(&self, commandbar: &mut CommandBar) {
        commandbar.set(key!("F1"), "Add", mywin::Commands::Add);
        commandbar.set(key!("F2"), "Clear", mywin::Commands::Clear);
    }

    fn on_event(&mut self, command_id: mywin::Commands) {
        let handle = self.h;
        if let Some(cb) = self.control_mut(handle) {
            match command_id {
                mywin::Commands::Add => {
                    let s = format!("Option {}", cb.count() + 1);
                    cb.add(&s);
                }
                mywin::Commands::Clear => {
                    cb.clear();
                }
            }
        }
    }
}

fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().command_bar().build()?;
    a.add_window(MyWin::new());
    a.run();
    Ok(())
}
