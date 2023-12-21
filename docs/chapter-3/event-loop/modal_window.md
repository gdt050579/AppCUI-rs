# Modal Window

A modal window is a window that captures the entire focus for the duration of its existance. In other word, when a modal window is started, it will be on top of everything else and the entire input (mouse and keyboard will be treated by it). 

When a modal window is opened the rest of the windows or other modal windows will be disabled:
<img src="img/modal_window.png"/>

A modal window is in fact just like a regular window (you can add other controls, you can resize and move it) and you can intercept events just like in a regular window case. However, since a modal window can not lose the focus, it has another property (**response**) that implies that it will provide a response once its execution ends. The **response** can be any kind of type (including a void type).

To create a modal window that will handle events from its children, use `#[ModalWindow(...)]` method:

```rs
#[ModalWindow(events=..., response=...)]
struct MyWindow {
    // specific fields
}
```