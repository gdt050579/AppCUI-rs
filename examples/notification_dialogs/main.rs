use appcui::prelude::*;

#[derive(Copy, Clone, PartialEq, Eq)]
enum NotificationType {
    None,
    Error,
    Retry,
    Alert,
    Proceed,
    Message,
    Validate,
    TryValidate,
}

#[Window(events = ButtonEvents+RadioBoxEvents)]
struct MyWin {
    rb_error: Handle<RadioBox>,
    rb_retry: Handle<RadioBox>,
    rb_alert: Handle<RadioBox>,
    rb_proceed: Handle<RadioBox>,
    rb_message: Handle<RadioBox>,
    rb_validate: Handle<RadioBox>,
    rb_try_validate: Handle<RadioBox>,
    b_show: Handle<Button>,
    b_exit: Handle<Button>,
    ntype: NotificationType,
}

impl MyWin {
    fn new() -> Self {
        let mut win = MyWin {
            base: window!("'Select',d:c,w:43,h:14"),
            rb_error: Handle::None,
            rb_retry: Handle::None,
            rb_alert: Handle::None,
            rb_proceed: Handle::None,
            rb_message: Handle::None,
            rb_validate: Handle::None,
            rb_try_validate: Handle::None,
            b_show: Handle::None,
            b_exit: Handle::None,
            ntype: NotificationType::Error,
        };
        win.rb_error = win.add(radiobox!("'&Error popup dialog',x:1,y:1,w:38,h:1,select:true"));
        win.rb_retry = win.add(radiobox!("'Error with &retry popup',x:1,y:2,w:38,h:1"));
        win.rb_alert = win.add(radiobox!("'&Alert popup dialog',x:1,y:3,w:38,h:1"));
        win.rb_proceed = win.add(radiobox!("'&Proceed alert with Yes/No options',x:1,y:4,w:38,h:1"));
        win.rb_message = win.add(radiobox!("'Simple &message popup',x:1,y:5,w:38,h:1"));
        win.rb_validate = win.add(radiobox!("'&Validate message with Yes/No options',x:1,y:6,w:39,h:1"));
        win.rb_try_validate = win.add(radiobox!("'&Try validate message with Yes, No and Cancel options',x:1,y:7,w:38,h:2"));
        win.b_show = win.add(button!("&Show,x:5,y:10,w:15"));
        win.b_exit = win.add(button!("E&xit,x:22,y:10,w:15"));
        win
    }
    fn show_notification(&self) {
        match self.ntype {
            NotificationType::None => {}
            NotificationType::Error => {
                dialogs::error("Error", "An error has occured while performin the last operation");
            }
            NotificationType::Retry => {
                if dialogs::retry("Error", "An error has occured while performin the last operation. Do you want to retry ?") {
                    // retry option was selected
                    dialogs::message("Result", "We will retry the last operation");
                } else {
                    // cancel button was pressed
                    dialogs::message("Result", "We will cancel");
                }
            }
            NotificationType::Alert => {
                dialogs::alert("Alert", "Some of the operations that were performed did not finish coretly !");
            }
            NotificationType::Proceed => {
                if dialogs::proceed(
                    "Alert",
                    "Some of the operations that were performed did not finish coretly !\nDo you wish to continue ?",
                ) {
                    // proceed with the action
                    dialogs::message("Result", "We will continue");
                } else {
                    // do not process with the action
                    dialogs::message("Result", "We will stop");
                }
            }
            NotificationType::Message => {
                dialogs::message("Message", "The last operation has been completed succesifully.");
            }
            NotificationType::Validate => {
                if dialogs::validate("Delete", "Are you sure that you want to delete this folder ?") {
                    // request was validated -> continue
                    dialogs::message("Result", "We will delete the folder");
                } else {
                    // request was not validated.
                    dialogs::message("Result", "We will not delete the folder");
                }
            }
            NotificationType::TryValidate => {
                if let Some(save_files_and_exit) = dialogs::try_validate("Exit", "Some of the files were modified. Do you want to save them ?") {
                    // we got a result
                    if save_files_and_exit {
                        // YES button was pressed
                        dialogs::message("Result", "We will exit and save the files.");
                    } else {
                        // NO button was pressed
                        dialogs::message("Result", "We will exit but we will not save the modified files.");
                    }
                } else {
                    // cancel button was pressed
                    dialogs::message("Result", "We will not exit the app.");
                }
            }
        }
    }
}

impl ButtonEvents for MyWin {
    fn on_pressed(&mut self, handle: Handle<Button>) -> EventProcessStatus {
        if handle == self.b_show {
            self.show_notification();
            return EventProcessStatus::Processed;
        }
        if handle == self.b_exit {
            self.close();
            return EventProcessStatus::Processed;
        }
        EventProcessStatus::Ignored
    }
}
impl RadioBoxEvents for MyWin {
    fn on_selected(&mut self, handle: Handle<RadioBox>) -> EventProcessStatus {
        self.ntype = match () {
            _ if handle == self.rb_error => NotificationType::Error,
            _ if handle == self.rb_retry => NotificationType::Retry,
            _ if handle == self.rb_alert => NotificationType::Alert,
            _ if handle == self.rb_proceed => NotificationType::Proceed,
            _ if handle == self.rb_message => NotificationType::Message,
            _ if handle == self.rb_validate => NotificationType::Validate,
            _ if handle == self.rb_try_validate => NotificationType::TryValidate,
            _ => NotificationType::None,
        };
        EventProcessStatus::Processed
    }
}

fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().build()?;
    a.add_window(MyWin::new());
    a.run();
    Ok(())
}
