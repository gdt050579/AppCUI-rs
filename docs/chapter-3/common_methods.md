# Common methods for all Controls

All controls (including Window and Desktop) have a set of common methods obtained via the `Deref` trait over a common base object.


## Status related methods

| Method                | Purpose                                                                                                          |
| --------------------- | ---------------------------------------------------------------------------------------------------------------- |
| `set_visible(...)`    | Shows or hides a control (has no effect on a Desktop)                                                   |
| `is_visible()`        | Returns **true** if the current control is visible, or **false** otherwise                                       |
| `set_enable(...)`     | Changes the enable/disable status of a control (has no effect on a Window or a Desktop control)                   |
| `is_enabled()`        | Returns **true** if the current control is enabled, or **false** otherwise                                       |
| `has_focus()`         | Returns **true** if the current control has focus, or **false** otherwise                                        |
| `can_receive_input()` | Returns **true** if the current control could receive mouse or keyboard events if focused or **false** otherwise |
| `is_mouse_over()`     | Returns **true** if the mouse cursor is over the current control                                                 |

## Layout related methods

| Method                                | Purpose                                                                                                                                                                                                                                                                                                                                                                                   |
| ------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `size()`                              | Returns the size (**width**x**height**) for the current control                                                                                                                                                                                                                                                                                                                           |
| `client_size()`                       | Returns the client size (the size minus the margins) for the current control                                                                                                                                                                                                                                                                                                              |
| `set_size(...)`                       | Sets the new size for a control (to a specified size given by parameters `width` and `height`). Keep in mind that this method will change the existing layout to a layout based on the top-left corner (given by the control's `x` and `y` coordinates) and the newly provided size. Any dock or alignment properties will be removed.<br>This method has no effect on a Desktop control.         |
| `position()`                          | Returns the relative position (x,y) of the current control to its parent.                                                                                                                                                                                                                                                                                                                 |
| `set_position(...)`                   | Sets the new position for a control (to specified coordinates given by parameters `x` and `y`). Keep in mind that this method will change the existing layout to a layout based on the top-left corner (given by coordinates `x` and `y`) and the control's current width and height. Any dock or alignment properties will be removed.<br>This method has no effect on a Desktop control. |
| `set_components_toolbar_margins(...)` | Sets the left and top components margins - for scrollbars, filters, etc                                                                                                                                                                                                                                                                                                                   |
| `update_layout(...)`                  | Updates the layout of a control                                                                                                                                                                                                                                                                                                                                                           |



## Hotkey related methods

| Method         | Purpose                                                                                                   |
| -------------- | --------------------------------------------------------------------------------------------------------- |
| `hotkey()`     | Returns the hotkey associated with a control, or `Key::None` otherwise                                      |
| `set_hotkey()` | Sets the hotkey for a control. To clear the hotkey call this function like this: `.set_hotkey(Key::None)` |

## Update methods

| Method             | Purpose                                                                                                                 |
| ------------------ | ----------------------------------------------------------------------------------------------------------------------- |
| `request_focus()`  | Request the framework to assign the focus to the current control                                                        |
| `request_update()` | Request the framework to update itself. This action will update the command bar, menus, and the positions of the controls. |

## Menu related methods

| Method               | Purpose                                                                                                         |
| -------------------- | --------------------------------------------------------------------------------------------------------------- |
| `register_menu(...)` | Registers a menu in the `AppCUI` framework and returns a handle for it                                             |
| `show_menu(...)`     | Show a popup menu that was registered by the current control                                                    |
| `menuitem(...)`      | Returns an immutable reference to a menu item based on two handles: one for the menu, and one for the menu item |
| `menuitem_mut(...)`  | Returns a mutable reference to a menu item based on two handles: one for the menu, and one for the menu item   |

## Theme related methods

| Method    | Purpose                                                                                                                                                            |
| --------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `theme()` | Returns a reference to the theme object that is being used by the application. This is the same reference an object receives when the `OnPaint` method is called. |

## Application Bar related methods

| Method     | Purpose                                                                                                                                                                                                          |
| ---------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `appbar()` | Returns a reference to the application bar object that is being used by the application. This method will panic if the application bar is not enabled (via the `app_bar()` method when building the application) |
