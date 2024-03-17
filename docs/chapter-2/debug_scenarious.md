# Debug scenarios

When using AppCUI and needing to test the interface, it is recommended to write the unit tests using `App::debug(...)` method. This method allows one to write a succesion of system events (mouse clicks, keys being pressed, etc) and validate if the output is the expected one. This succesion of command is considered an event script - form out of multiple commands, each command written on a line. A command can have parameters. You can also use `//` to comment a command.

**General format for a script**
```rs
Command-1(param1,param2,param3)
Command-2()
// comment
```

**Remarks**: 
* `App::debug(...)` will panic if the script is incorect (a command is not valid, the number of parameters is incorect, etc).
* `AppCUI` allows only one instance at one time (this is done via a mutex object). If you have multiple unit test and you try to run them with `cargo test` command, you might get an error as **cargo** might try to use multiple threads to do this and it is likely that one thread might try to start an `AppCUI` application while another one is already running on another thread. The solution in this case is to run the tests using a single thread:
```
cargo test -- --test-threads=1
```

     
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
| `Key.TypeText(text)` | where `text` parameter is a text that is being typed                                                                                                                                                                                                                |


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
| Command               | Purpose |
|-----------------------|---------|  
| `CheckHash(hash)`     | checks if the hash computer over the current virtual screen is as expected. If not it will panic. This is useful for unit testing. |
| `CheckCursor(x,y)`    | checks if the cursor (caret) is at a specify position |
| `CheckCursor(hidden)` | checks is the cursor (caret) is hidden (not visible). You cal also check this by using `false` instead of `hidded` |

## Example

Let's consider a scenario where we want to test if moving a window with a mouse works as expected. For this we will create a test function, with the following code:

```rs
#[test]
fn check_if_window_can_be_moved() {
    let script = "
        Paint('initial state')
        CheckHash(0xB1471A30B30F5C6C)
        Mouse.Drag(30,3,35,5)
        Paint('window was moved')
        CheckHash(0x419533D4BBEFE538)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let w = Window::new("Title", Layout::new("d:c,w:20,h:5"), window::Flags::None);
    a.add_window(w);
    a.run();
}
```

Let's break the event script in pieces and see exactly what is supposed to happen:
1. `Paint('initial state')` - this will print the virtual screen. It should look like the following (but with colors):
```
    +===================================================================+
    | Name: initial state                                               |
    | Hash: 0xB1471A30B30F5C6C                                          |
    |-------------------------------------------------------------------|
    |    |           11111111112222222222333333333344444444445555555555 |
    |    | 012345678901234567890123456789012345678901234567890123456789 |
    |-------------------------------------------------------------------|
    |  0 | ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒ |
    |  1 | ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒ |
    |  2 | ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒ |
    |  3 | ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒╔════ Title ════[x]╗▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒ |
    |  4 | ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒║                  ║▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒ |
    |  5 | ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒║                  ║▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒ |
    |  6 | ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒║                  ║▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒ |
    |  7 | ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒╚══════════════════╝▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒ |
    |  8 | ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒ |
    |  9 | ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒ |
    |-------------------------------------------------------------------|
```
We can inspect inspect if the position of the window is correct. We can also notice the hash compited for the entire virtual screen: `0xB1471A30B30F5C6C` (this could help us do further checks).

2. `CheckHash(0xB1471A30B30F5C6C)` - this compute the hash for the entire virtual screen and then check it againts the expected one. The usual scenario here is that we firs apply a `Paint` command, validate it, and them write the `CheckHash` command with the hash obtained from the `Paint` command. This way, if something changes to the logic/code of the program, the new hash will be different. **If the hash for the virtual screen is not as expected the application will panic**. If used in a test, this behavior will fail the test.

3. `Mouse.Drag(30,3,35,5)` this command does the following:
    - moves the mouse to the **(30,3)** coordonate (over the title of the window)
    - click and hold the left mouse button
    - moves the mouse to a new position **(35,5)** (since we hold the mouse button, we expect the window to move as well)
    - releases the left mouse button

4. `Paint('window was moved')` now we should see something like the following. Notice that indeed, the window was moved to a new position. We also have a new hash for the virtual screen: `0x419533D4BBEFE538`

```
    +===================================================================+
    | Name: window was moved                                            |
    | Hash: 0x419533D4BBEFE538                                          |
    |-------------------------------------------------------------------|
    |    |           11111111112222222222333333333344444444445555555555 |
    |    | 012345678901234567890123456789012345678901234567890123456789 |
    |-------------------------------------------------------------------|
    |  0 | ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒ |
    |  1 | ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒ |
    |  2 | ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒ |
    |  3 | ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒ |
    |  4 | ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒ |
    |  5 | ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒╔════ Title ════[x]╗▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒ |
    |  6 | ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒║                  ║▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒ |
    |  7 | ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒║                  ║▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒ |
    |  8 | ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒║                  ║▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒ |
    |  9 | ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒╚══════════════════╝▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒ |
    |-------------------------------------------------------------------|
```

25. `CheckHash(0x419533D4BBEFE538)` - finally we check the new hash to see if it maches the one we expect.

**Remarsk**: using unit tests (while it works with the `Paint` command activated) might look strange on the actual screen (especially if all you need is to validate an example). As such, it is best that after one example such as the previous one was validated, to add another command at the begining of the script: `Paint.Enable(false)`. This will not change the logic of the script, instead it will not print anything on the screen. As such, the final test function should look like this:

```rs
#[test]
fn check_if_window_can_be_moved() {
    let script = "
        Paint.Enable(false)
        Paint('initial state')
        CheckHash(0xB1471A30B30F5C6C)
        Mouse.Drag(30,3,35,5)
        Paint('window was moved')
        CheckHash(0x419533D4BBEFE538)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let w = Window::new("Title", Layout::new("d:c,w:20,h:5"), window::Flags::None);
    a.add_window(w);
    a.run();
}
```

and its execution should produce an output similar to the next one:

```
running 1 test
test check_if_window_can_be_moved ... ok
```