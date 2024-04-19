use appcui::prelude::*;

// #[Window(events = ButtonEvents)]
// struct MyWin {
//     p: Handle<Password>
// }

// impl MyWin {
//     fn new() -> Self {
//         let mut win = MyWin {
//             base: window!("'My Win',d:c,w:39,h:14"),
//             p: Handle::None,
//         };
//         win.p = win.add(Password::new(Layout::new("x:1,y:1,w:10")));
//         win.add(button!("test,x:1,y:4,w:11"));
//         win
//     }
// }

// impl ButtonEvents for MyWin {
//     fn on_pressed(&mut self, _handle: Handle<Button>) -> EventProcessStatus {
//         //dialogs::error("Error", "Unable to load some stuff as there is an internal error to some of our components. Check:\n1. Program scope\n2. Content\n3. Unit test results");
//         //dialogs::alert("Error","An error has occured during the last operation");
//         // if dialogs::retry("Error", "An error occured. Retry ?") {
//         //     dialogs::message("Response", "We should retry.")
//         // } else {
//         //     dialogs::message("Response", "Stop the action.")
//         // }
//         // if dialogs::proceed("Alert","An error occured while performn a copy operation.\nContinue anyway ?") {
//         //     // retry the operation
//         // }
//         //dialogs::message("Success","All files have been copied");
//         // if let Some(save_files) = dialogs::try_validate("Exit","Do you want to save your files ?") {
//         //     if save_files {
//         //         // save files and then exist application
//         //     } else {
//         //         // exit the application directly
//         //     }
//         // } else {
//         //     // don't exit the application
//         // }
//         EventProcessStatus::Processed
//     }
// }

fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().size(Size::new(60,15)).build()?;
    let mut w = window!("test,d:c,w:40,h:14");
    w.add(TextField::new("Hello world",Layout::new("x:1,y:1,w:36,h:1"),textfield::Flags::None));
    a.add_window(w);
    a.run();
    Ok(())
}
