# Common methods for all Controls

All controls (including Window and Desktop) have a set of common methods obtaing via Deref trait over a common base object.


## Status related methods

| Method                | Purpose                                                            |
|-----------------------|--------------------------------------------------------------------|
| `set_visible(...)`    | Shows or hides a controls (will have no change over a Desktop)     |
| `is_visible()`        | Returns **true** if the current control is visible, or **false** otherwise |
| `set_enable(...)`     | Changes the enable/disable status of a control (has no effect on a Window or a Desktop control |
| `is_enabled()`        | Returns **true** if the current control is enabled, or **false** otherwise |
| `has_focus()`         | Returns **true** if the current control has focus, or **false** otherwise |
| `can_receive_input()` | Returns **true** if the current control could receive mouse or keyboard events if focused or **false** otherwise |
| `is_mouse_over()`     | Returns **true** if the mouse cursor is over the current control |

## Layout related methods

| Method                | Purpose                                                                      |
|-----------------------|------------------------------------------------------------------------------|
| `get_size()`          | Returns the size (**width**x**height**) for the current control              |
| `get_client_size()`   | Returns the client size (the size minus the margins) for the current control |


## Hotkey related methods

| Method                | Purpose                                                                      |
|-----------------------|------------------------------------------------------------------------------|
| `get_hotkey()`        | Returns the hotkey associated witha control or `Key::None` otherwise         |
| `set_hotkey()`        | Sets the hotkey for a control. To clear the hotkey call this function like this: `.set_hotkey(Key::None)` |