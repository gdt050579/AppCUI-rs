# Debug scenarios

When using AppCUI and needing to test the interface, it is recommended to write the unit tests using `App::debug(...)` method. This method allows one to write a succesion of system events (mouse clicks, keys being pressed, etc) and validate if the output is the expected one. This succesion of command is considered an event script - form out of multiple commands, each command written on a line. A command can have parameters. You can also use `//` to comment a command.

**General format for a script**
```rs
Command-1(param1,param2,param3)
Command-2()
// comment
```

**Remarks**: `App::debug(...)` will panic if the script with differnt commands is incorect (a command is not valid, the number of parameters is incorect, etc).

     
## Mouse related commands

Mouse related commands are a set of commands that simulate various mouse events

| Command                          | Purpose |
|----------------------------------|---------|
|`Mouse.Hold(x,y,button)`          |simulates an event where the mouse button is being pressed while the mouse is located at a specific position on screen. The parameters `x` and `y` are a screen position, while the parameter `button` is one of `left`, `right` or `center`|
|`Mouse.Release(x,y)`              |simulates the release of all mouse buttons while the mouse is located at a specific screen position.|
|`Mouse.Click(x,y,button)`         |simulates a click (hold and release). It is equivalent to<br> - Mouse.Hold(x,y,button)<br> - Mouse.Release(x,y) |
|`Mouse.Move(x,y)`                 |simulates the movement of a mouse to coordonates (x,y). No mouse button are being pressed.|
|`Mouse.Drag(x1,y1,x2,y2)`         |simulates the movement of a mouse from (x1,y1) to (x2,y2) while the `left` button is being pressed |
|`Mouse.Wheel(x,y,direction,times)`|simulates the wheel mouse being rotated into a direction (one of `top`, `bottom`, `left`, `right`) for a number of times. The `times` parameter must be biggen than 0. |
     
## Keyboard related commands

| Command          | Purpose |
|------------------|---------|
|`Key.Pressed(key)`| where `key` parameter can be a key name or any combination of control key and a regular key such as<br>- `Z` (for pressin the `Z` key)<br>- `Enter` (for pressing the `Enter` key)<br>-`Alt+T` (`Alt` + `T` combination)<br>-`Ctrl+Alt+F1` (`Ctrl`+`Alt`+`F1` keys)|

Usually the key parameter can have several forms:
* `key`
* `modifier-1`+`key`
* `modifier-1`+`modifier-2`+`key`
* `modifier-1`+`modifier-2`+`modifier-3`+`key`

where the list of all keys supported by this command is:
* F-commands (`F1` to `F12`)
* Letters (`A` to `Z`) - with upper case
* Numbers (`0` to `9`)
* Arrows (`Up`, `Down`, `Left`, `Right`)
* Navigation keys (`PageUp`, `PageDown`, `Home`, `End`)
* Deletion and Insertions (`Delete` , `Backspace`, `Insert`)
* White-spaces (`Space`, `Tab`)
* Other (`Enter`, `Escape`)

and the list of modifiers consists in `Shift`, `Ctrl` and `Alt`.
     
## Paint related commands

| Command                              | Purpose |
|--------------------------------------|---------|     
|`Paint(staet_name)`<br>or<br>`Paint()`| paints the current virtual screen into the current screen using ANSI codes and colors. This command also computes a hash over the current virtual screen and prints it. The `state_name` is a name can be used to reflect the current execution state. This is useful if multipl `Paint` command are being used and you need to differentiate between them.  |
|`Paint.Enable(value)`                 | enables or disables painting. `value` is a boolean value (**true** or **false**). If set to **false** all subsequent calls to command `Paint` will be ignored. By default, all paints are enabled. |

## System events

| Command                | Purpose |
|------------------------|---------|  
| `Resize(width,height)` | simulates a resize of the virtual terminal to the size represented by `width` and `height` parameters |
     
## Validation commands
| Command            | Purpose |
|--------------------|---------|  
| `CheckHash(hash)`  | checks if the hash computer over the current virtual screen is as expected. If not it will panic. This is useful for unit testing. |