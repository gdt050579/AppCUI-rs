# Single Window Apps

A single window app is an AppCUI application where you only have one window that ocupies the entire desktop. Usually, when you create a AppCUI app, you can add multiple windows to a desktop object. In this mode you can only add one window, and terminating that window will close the app.

To do this you need to use the `.single_window()` method from the App builder as follows:
```rs
let mut app = App::new().single_window().build()?;
// add one and only one window
app.add_window(...);
// run the application
app.run()
```

## Remarks
* in a `Single Window` mode you can not set a custom desktop as there is only one window and it covers the entire visible size of a desktop. Using a `.desktop(...)` method with a `.single_window()` method will result in a panic:
    ```rs
    // the following code wil panic
    App::new().single_window().desktop(...).build()?
    ```
* Since in a `Single Window` mode there is only one window , you can not use the `.add_window(...)` method twice. Using it wll result in a panic.
    ```rs
    let mut a = App::new().single_window()..build()?;
    a.add_window(...);
    // the following line will panic as there is alreay a window added
    a.add_window(...); // panic
    a.run();
    ```
* Since in a `Single Window` mode the window ocupies the entire visible size of a desktop, you can not resize or move it. As such, window flag attributes like `Sizeable` are not allowed. If used, the code will panic. The layout (regardless on how you set it up) will be changed to make sure that the window ocupies the entire visible desktop space.
    ```rs
    let mut a = App::new().single_window()..build()?;
    // the following line will panic as Sizeable flag is not allow on windows in Single Window mode
    a.add_window(window!("Test,a:c,flags: Sizeable"));
    a.run();
    ```
* In a `Single Window` mode, the event loop will be associated with the single window. As such, not adding a window will result in a panic.
    ```rs
    let mut a = App::new().single_window()..build()?;
    // the following line will panic no window was added
    a.run();
    ```
