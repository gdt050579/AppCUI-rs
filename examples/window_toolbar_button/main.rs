use appcui::prelude::*;
 
 #[Window(events = ToolBarEvents)]
 struct CounterWindow {
     increase_button: Handle<toolbar::Button>,
     decrease_button: Handle<toolbar::Button>,
     counter_label: Handle<Label>,
     counter: i32,
 }

 impl CounterWindow {
     fn new() -> Self {
         let mut win = CounterWindow {
             base: window!("'Counter',a:c,w:40,h:6"),
             increase_button: Handle::None,
             decrease_button: Handle::None,
             counter_label: Handle::None,
             counter: 0,
         };
         
         // Create a toolbar group at the bottom right of the window
         let group = win.toolbar().create_group(toolbar::GroupPosition::BottomRight);
         
         // Add buttons to the toolbar group
         let mut btn_minus = toolbar::Button::new("-"); 
         btn_minus.set_tooltip("Decrease counter");
         win.decrease_button = win.toolbar().add(group, btn_minus);
         let mut btn_plus = toolbar::Button::new("+");
         btn_plus.set_tooltip("Increase counter");
         win.increase_button = win.toolbar().add(group, btn_plus);
                
         // Add a label to display the counter value
         win.counter_label = win.add(label!("0,a:c,w:10,h:1"));
         
         win
     }
     
     // Update the counter label
     fn update_counter(&mut self) {
         let h = self.counter_label;
         let text = format!("{}", self.counter);
         if let Some(label) = self.control_mut(h) {
             label.set_caption(&text);
         }
     }
 }

 impl ToolBarEvents for CounterWindow {
     fn on_button_clicked(&mut self, handle: Handle<toolbar::Button>) -> EventProcessStatus {
         if handle == self.increase_button {
             self.counter += 1;
             self.update_counter();
             EventProcessStatus::Processed
         } else if handle == self.decrease_button {
             self.counter -= 1;
             self.update_counter();
             EventProcessStatus::Processed
         } else {
             EventProcessStatus::Ignored
         }
     }
 }

 fn main() -> Result<(), appcui::system::Error> {
     let mut app = App::new().build()?;
     app.add_window(CounterWindow::new());
     app.run();
     Ok(())
 }