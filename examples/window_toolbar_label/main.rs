use appcui::prelude::*;

#[Window(events = ButtonEvents+CheckBoxEvents)]
struct NumberFormatsWindow {
    increase_button: Handle<Button>,
    decimal_label: Handle<toolbar::Label>,
    hex_label: Handle<toolbar::Label>,
    binary_label: Handle<toolbar::Label>,
    show_decimal: Handle<CheckBox>,
    show_hex: Handle<CheckBox>,
    show_binary: Handle<CheckBox>,
    number: u32,
}

impl NumberFormatsWindow {
    fn new() -> Self {
        let mut win = NumberFormatsWindow {
            base: window!("'Number Formats',d:c,w:40,h:10"),
            increase_button: Handle::None,
            decimal_label: Handle::None,
            hex_label: Handle::None,
            binary_label: Handle::None,
            show_decimal: Handle::None,
            show_hex: Handle::None,
            show_binary: Handle::None,
            number: 42,
        };

        // Add the increase button
        win.increase_button = win.add(button!("'Increase',w:15,d:l"));

        // Add checkboxes to control visibility
        win.show_decimal = win.add(checkbox!("'Show decimal',x:20,y:2,w:16,checked:true"));
        win.show_hex = win.add(checkbox!("'Show hex',x:20,y:4,w:16,checked:true"));
        win.show_binary = win.add(checkbox!("'Show binary',x:20,y:6,w:16,checked:true"));

        // Create toolbar groups
        let bottom_group = win.toolbar().create_group(toolbar::GroupPosition::BottomLeft);
        let top_group = win.toolbar().create_group(toolbar::GroupPosition::TopRight);

        // Add toolbar labels
        win.decimal_label = win.toolbar().add(bottom_group, toolbar::Label::new("Dec:42"));
        win.hex_label = win.toolbar().add(bottom_group, toolbar::Label::new("Hex:2A"));
        win.binary_label = win.toolbar().add(top_group, toolbar::Label::new("Bin:101010"));

        win
    }

    fn update_labels(&mut self) {
        // Update all labels with the current number in different formats
        let h = self.decimal_label;
        let number = self.number;
        if let Some(label) = self.toolbar().get_mut(h) {
            label.set_content(&format!("Dec:{number}"));
        }
        let h = self.hex_label;
        if let Some(label) = self.toolbar().get_mut(h) {
            label.set_content(&format!("Hex:{number:X}"));
        }
        let h = self.binary_label;
        if let Some(label) = self.toolbar().get_mut(h) {
            label.set_content(&format!("Bin:{number:b}"));
        }
    }
}

impl ButtonEvents for NumberFormatsWindow {
    fn on_pressed(&mut self, _handle: Handle<Button>) -> EventProcessStatus {
        self.number += 1;
        self.update_labels();
        EventProcessStatus::Processed
    }
}

impl CheckBoxEvents for NumberFormatsWindow {
    fn on_status_changed(&mut self, handle: Handle<CheckBox>, checked: bool) -> EventProcessStatus {
        if let Some(label) = self.toolbar().get_mut(handle) {
            label.set_visible(checked);
        }
        EventProcessStatus::Processed
    }
}


fn main() -> Result<(), appcui::system::Error> {
    let mut app = App::new().build()?;
    app.add_window(NumberFormatsWindow::new());
    app.run();
    Ok(())
}