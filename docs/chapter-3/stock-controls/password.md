# Password

Represent a clickable password control:

<img src="img/password.png" width=300/>

To create a password use `Password::new` method (with one parameter - the layout).
```rs
let p = Password::new(Layout::new("x:10,y:5,w:15"));
```
or use the macro `password!`
```rs
let p1 = password!("pass=1234,x:10,y:5,w:15");
let p2 = password!("password='MyP@ssw0rd',x:10,y:5,w:15");
```

A password control supports all common parameters (as they are described in [Instantiate via Macros](../instantiate_via_macros.md) section). Besides them, the following **named parameters** are also accepted:

| Parameter name       | Type   | Positional parameter | Purpose               |
| -------------------- | ------ | -------------------- | --------------------- |
| `pass` or `password` | String | **No**               | A password to be used |


Some examples that uses these paramateres:
```rs
let disabled_password = password!("x:10,y:5,w:15,enable=false");
let hidden_password = password!("pass='admin',x=9,y:1,align:center,w:9,visible=false");
```

## Events
To intercept events from a password, the following trait has to be implemented to the Window that processes the event loop:
```rs
pub trait PasswordEvents {
    fn on_accept(&mut self, handle: Handle<Password>) -> EventProcessStatus {
        // called when you hit the ENTER key (to accept a passowrd)
    }
    fn on_cancel(&mut self, handle: Handle<Password>) -> EventProcessStatus {
        // called when you hit the ESCAPE key
    }
}
```

## Methods

Besides the [Common methods for all Controls](../common_methods.md) a password also has the following aditional methods:

| Method              | Purpose                                                                           |
| ------------------- | --------------------------------------------------------------------------------- |
| `set_password(...)` | Programatically sets a new password .<br>Example: `password.set_password("1234")` |
| `password()`        | Returns the current password                                                      |

## Key association

The following keys are processed by a password control if it has focus:

| Key      | Purpose                                                                                                                     |
| -------- | --------------------------------------------------------------------------------------------------------------------------- |
| `Enter`  | Attempts to accept password by emitting `passwordEvents::on_accept(...)` event.  This is where a password can be validated. |
| `Escape` | Cancel the password validation and emits `passwordEvents::on_cancel(...)`.                                                  |


## Example

The following code creates a login window where you need to type the password **admin** to continue.

```rs
use appcui::prelude::*;

#[Window(events = ButtonEvents+PasswordEvents)]
struct MyWin {
    p: Handle<Password>,
    b_ok: Handle<Button>,
    b_cancel: Handle<Button>,
}

impl MyWin {
    fn new() -> Self {
        let mut win = MyWin {
            base: window!("'Login',d:c,w:40,h:8"),
            p: Handle::None,
            b_ok: Handle::None,
            b_cancel: Handle::None
        };
        win.add(label!("'Enter the password:',x:1,y:1,w:36,h:1"));
        win.b_ok = win.add(button!("&Ok,x:5,y:4,w:11"));
        win.b_cancel = win.add(button!("&Cancel,x:22,y:4,w:11"));
        win.p = win.add(password!("x:1,y:2,w:36"));

        win
    }
    fn check_password(&mut self) {
        let p = self.p;
        if let Some(pass) = self.control(p) {
            if pass.password() == "admin" {
                dialogs::message("Login", "Correct password. Let's start !");
            } else {
                if !dialogs::retry("Login", "Invalid password. Try again ?") {
                    self.close();
                }
            }
        }
    }
}

impl ButtonEvents for MyWin {
    fn on_pressed(&mut self, handle: Handle<Button>) -> EventProcessStatus {
        match () {
            _ if handle == self.b_cancel => {
                self.close();
                EventProcessStatus::Processed
            }
            _ if handle == self.b_ok => {
                self.check_password();
                EventProcessStatus::Processed
            }
            _ => { EventProcessStatus::Ignored }
        }
    }
}
impl PasswordEvents for MyWin {
    fn on_accept(&mut self, _: Handle<Password>) -> EventProcessStatus {
        self.check_password();
        EventProcessStatus::Processed
    }

    fn on_cancel(&mut self, _: Handle<Password>) -> EventProcessStatus {
        self.close();
        EventProcessStatus::Processed
    }
}
fn main() -> Result<(), appcui::system::Error> {
    let mut app = App::new().build()?;
    app.add_window(MyWin::new());
    app.run();
    Ok(())
}
```