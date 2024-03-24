use appcui::prelude::*;

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
    b_cancel: Handle<Button>,
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
            b_cancel: Handle::None
        };
        win.rb_error = win.add(radiobox!("'&Error popup dialog',x:1,y:1,w:38,h:1,select:true"));
        win.rb_retry = win.add(radiobox!("'Error with &retry popup',x:1,y:2,w:38,h:1"));
        win.rb_alert = win.add(radiobox!("'&Alert popup dialog',x:1,y:3,w:38,h:1"));
        win.rb_proceed = win.add(radiobox!("'&Proceed alert with Yes/No options',x:1,y:4,w:38,h:1"));
        win.rb_message = win.add(radiobox!("'Simple &message popup',x:1,y:5,w:38,h:1"));
        win.rb_validate = win.add(radiobox!("'&Validate message with Yes/No options',x:1,y:6,w:39,h:1"));
        win.rb_validate = win.add(radiobox!("'&Try validate message with Yes, No and Cancel options',x:1,y:7,w:38,h:2"));
        win.b_show = win.add(button!("&Show,x:5,y:10,w:15"));
        win.b_cancel = win.add(button!("E&xit,x:22,y:10,w:15"));
        win
    }
}

impl ButtonEvents for MyWin {

}
impl RadioBoxEvents for MyWin {
    
}

fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().build()?;
    a.add_window(MyWin::new());
    a.run();
    Ok(())
}
