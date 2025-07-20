use appcui::prelude::*;

#[Window(events = ComboBoxEvents)]
struct MyWin {}
impl MyWin {
    fn new() -> Self {
        let mut w = Self {
            base: window!("x:1,y:1,w:34,h:6,caption:Win"),
        };
        w.add(label!("'Select animal',x:1,y:1,w:30"));
        let mut c = ComboBox::new(layout!("x:1,y:2,w:30"), combobox::Flags::ShowDescription);
        // data from https://en.wikipedia.org/wiki/Fastest_animals
        c.add_item(combobox::Item::new("Cheetah","(120 km/h)"));
        c.add_item(combobox::Item::new("Swordfish","(97 km/h)"));
        c.add_item(combobox::Item::new("Iguana","(35 km/h)"));
        c.add_item(combobox::Item::new("Gazelle","(81 km/h)"));
        c.add_item(combobox::Item::new("Lion","(80 km/h)"));
        c.add_item(combobox::Item::new("Dog","(60 km/h)"));
        c.add_item(combobox::Item::new("Zebra","(56 km/h)"));
        w.add(c);
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


fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().build()?;
    a.add_window(MyWin::new());
    a.run();
    Ok(())
}
