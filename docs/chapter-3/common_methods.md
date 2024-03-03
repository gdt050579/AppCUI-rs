# Common methods for all Controls

All controls (including Window and Desktop) have a set of common methods obtaing via Deref trait over a common base object.


## Status related methods

| Method                | Purpose                                                                                                          |
| --------------------- | ---------------------------------------------------------------------------------------------------------------- |
| `set_visible(...)`    | Shows or hides a controls (will have no change over a Desktop)                                                   |
| `is_visible()`        | Returns **true** if the current control is visible, or **false** otherwise                                       |
| `set_enable(...)`     | Changes the enable/disable status of a control (has no effect on a Window or a Desktop control                   |
| `is_enabled()`        | Returns **true** if the current control is enabled, or **false** otherwise                                       |
| `has_focus()`         | Returns **true** if the current control has focus, or **false** otherwise                                        |
| `can_receive_input()` | Returns **true** if the current control could receive mouse or keyboard events if focused or **false** otherwise |
| `is_mouse_over()`     | Returns **true** if the mouse cursor is over the current control                                                 |

## Layout related methods

| Method                                | Purpose                                                                                                                                                                                                                                                                                                                                                                                    |
| ------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `get_size()`                          | Returns the size (**width**x**height**) for the current control                                                                                                                                                                                                                                                                                                                            |
| `get_client_size()`                   | Returns the client size (the size minus the margins) for the current control                                                                                                                                                                                                                                                                                                               |
| `set_size(...)`                       | Sets the new size for a control (to a specified size given by parameters `width` and `height`). Keep in mind that this method will change the existing layout to an a layout based on top-left corner (given by controls `x` and `y` coordonates) and the new provided size. Any dock or alignament properties will be removed.<br>This method has no effect on a Desktop control.         |
| `get_position()`                      | Returns the relatove position (x,y) of the current control to its parent.                                                                                                                                                                                                                                                                                                                  |
| `set_position(...)`                   | Sets the new position for a control (to a specified coordonate given by parameters `x` and `y`). Keep in mind that this method will change the existing layout to an a layout based on top-left corner (given by coordonates `x` and `y`) and the controls current width and height. Any dock or alignament properties will be removed.<br>This method has no effect on a Desktop control. |
| `set_components_toolbar_margins(...)` | Sets the left and top components margins - for scrollbars, filters, etc                                                                                                                                                                                                                                                                                                                    |



## Hotkey related methods

| Method         | Purpose                                                                                                   |
| -------------- | --------------------------------------------------------------------------------------------------------- |
| `get_hotkey()` | Returns the hotkey associated witha control or `Key::None` otherwise                                      |
| `set_hotkey()` | Sets the hotkey for a control. To clear the hotkey call this function like this: `.set_hotkey(Key::None)` |

## Update methods

| Method             | Purpose                                                                                                                 |
| ------------------ | ----------------------------------------------------------------------------------------------------------------------- |
| `request_focus()`  | Request the framework to assign the focus to the current control                                                        |
| `request_update()` | Request the framework to update itself. This actian will update the commandbar, menus and the position of the controls. |

## Menu related methods

| Method                  | Purpose                                                                                                         |
| ----------------------- | --------------------------------------------------------------------------------------------------------------- |
| `register_menu(...)`    | Register a menu into `AppCUI` framework and returns a Handle for it                                             |
| `show_menu(...)`        | Show a popup menu that was registered by the current control                                                    |
| `get_menuitem(...)`     | Returns an immutable reference to a menu item based on two handles: one for the menu, and one for the menu item |
| `get_menuitem_mut(...)` | Returns an mutable reference to a menu item based on two handles: one for the menu, and one for the menu item   |
