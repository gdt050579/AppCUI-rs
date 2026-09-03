# Single Window Apps

A single window app is an AppCUI application where you only have one window that ocupies the entire desktop. Usually, when you create a AppCUI app, you can add multiple windows to a desktop object. In this mode you can only add one window, and terminating that window will close the app.

To do this you need to use `App::single_window(...)` as follows:
```rs
App::single_window(|| {
    // construct the one and only window
    window!("Demo,d:f")
}).run()
```

## Remarks
* in a `Single Window` mode you can not set a custom desktop as there is only one window and it covers the entire visible size of a desktop. Using a `.desktop(...)` method with `App::single_window(...)` will result in a panic:
    ```rs
    // the following code wil panic
    App::single_window(|| window!("Demo,d:f"))
        .desktop(...)
        .run()
    ```
* Since in a `Single Window` mode there is only one window, the factory passed to `App::single_window(...)` is invoked once and must return that window. You can not register a second window.
* Since in a `Single Window` mode the window ocupies the entire visible size of a desktop, you can not resize or move it. As such, window flag attributes like `Sizeable` are not allowed. If used, the code will panic. The layout (regardless on how you set it up) will be changed to make sure that the window ocupies the entire visible desktop space.
    ```rs
    // the following line will panic as Sizeable flag is not allow on windows in Single Window mode
    App::single_window(|| window!("Test,a:c,flags: Sizeable")).run()
    ```
