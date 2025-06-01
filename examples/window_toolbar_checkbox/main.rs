 use appcui::prelude::*;
 
 #[Window(events = ToolBarEvents)]
 struct CheckboxWindow {
     checkbox_one: Handle<toolbar::CheckBox>,
     checkbox_two: Handle<toolbar::CheckBox>,
     status_label: Handle<Label>,
 }

 impl CheckboxWindow {
     fn new() -> Self {
         let mut win = CheckboxWindow {
             base: window!("'Checkbox Demo',d:c,w:40,h:6"),
             checkbox_one: Handle::None,
             checkbox_two: Handle::None,
             status_label: Handle::None,
         };
         
         // Create a toolbar group at the bottom right of the window
         let group = win.toolbar().create_group(toolbar::GroupPosition::BottomLeft);
         
         // Add checkboxes to the toolbar group
         let mut cb1 = toolbar::CheckBox::new("Option 1", false); 
         cb1.set_tooltip("First option");
         win.checkbox_one = win.toolbar().add(group, cb1);
         
         let mut cb2 = toolbar::CheckBox::new("Option 2", false);
         cb2.set_tooltip("Second option");
         win.checkbox_two = win.toolbar().add(group, cb2);
         
         // Add a label to display the checkbox states
         win.status_label = win.add(label!("'Select an option',d:c,w:30,h:1"));
         
         win
     }
 }

 impl ToolBarEvents for CheckboxWindow {
     fn on_checkbox_clicked(&mut self, handle: Handle<toolbar::CheckBox>, checked: bool) -> EventProcessStatus {
         let message = if handle == self.checkbox_one {
             format!("Option 1 is {}", if checked { "checked" } else { "unchecked" })
         } else if handle == self.checkbox_two {
             format!("Option 2 is {}", if checked { "checked" } else { "unchecked" })
         } else {
             return EventProcessStatus::Ignored;
         };
         
         let h = self.status_label;
         if let Some(label) = self.control_mut(h) {
             label.set_caption(&message);
         }
         EventProcessStatus::Processed
     }
 }

 fn main() -> Result<(), appcui::system::Error> {
     let mut app = App::new().build()?;
     app.add_window(CheckboxWindow::new());
     app.run();
     Ok(())
 }
