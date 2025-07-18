use appcui::prelude::*;

#[Window(events = ButtonEvents)]
struct MyWin {
    b_normal: Handle<Button>,
    b_default: Handle<Button>,
    b_validation: Handle<Button>,
    info: Handle<Label>,
}

impl MyWin {
    fn new() -> Self {
        let mut win = MyWin {
            base: window!("'InputDialog Example',d:c,w:50,h:11"),
            b_normal: Handle::None,
            b_default: Handle::None,
            b_validation: Handle::None,
            info: Handle::None,
        };
        
        win.info = win.add(label!("'',x:1,y:1,w:46,h:1"));
        win.b_normal = win.add(button!("'Get Number',x:1,y:3,w:46"));
        win.b_default = win.add(button!("'Get Number (Default: 42)',x:1,y:5,w:46"));
        win.b_validation = win.add(button!("'Get Number (1-100)',x:1,y:7,w:46"));
        
        win
    }
    
    fn update_result(&mut self, text: &str) {
        let handle = self.info;
        if let Some(label) = self.control_mut(handle) {
            label.set_caption(text);
        }
    }
    
    fn show_simple_input(&mut self) {
        if let Some(number) = dialogs::input::<i32>("Enter Number", "Please enter a number:", None, None) {
            self.update_result(&format!("Value : {number}"));
        } else {
            self.update_result("Input was cancelled");
        }
    }
    
    fn show_input_with_default(&mut self) {
        if let Some(number) = dialogs::input::<i32>("Enter Number", "Please enter a number:", Some(42), None) {
            self.update_result(&format!("Value : {number}"));
        } else {
            self.update_result("Input was cancelled");
        }
    }
    
    fn show_input_with_validation(&mut self) {
        // Validation function that ensures the number is between 1 and 100
        let validation = |value: &i32| {
            if *value < 1 || *value > 100 {
                Err("Number must be between 1 and 100".to_string())
            } else {
                Ok(())
            }
        };
        
        if let Some(number) = dialogs::input::<i32>("Enter Number", "Please enter a number between 1 and 100:", None, Some(validation)) {
            self.update_result(&format!("Value : {number} (validated)"));
        } else {
            self.update_result("Input was cancelled");
        }
    }
}

impl ButtonEvents for MyWin {
    fn on_pressed(&mut self, handle: Handle<Button>) -> EventProcessStatus {
        if handle == self.b_normal {
            self.show_simple_input();
            return EventProcessStatus::Processed;
        }
        if handle == self.b_default {
            self.show_input_with_default();
            return EventProcessStatus::Processed;
        }
        if handle == self.b_validation {
            self.show_input_with_validation();
            return EventProcessStatus::Processed;
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