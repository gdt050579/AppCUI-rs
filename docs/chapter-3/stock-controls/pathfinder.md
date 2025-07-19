# PathFinder

Represents a control where you can navigate through the file system and select a path:

<img src="img/pathfinder.png" width=300/>

To create a path finder control use the `PathFinder::new` method with the 3 parameteres: a starting file path, a layout and initialization flags:
```rs
let mut control = PathFinder::new("C:\\Program Files", Layout::new("x:1 , y:1 , width:40"), pathfinder::Flags::CaseSensitive)
```
or the macro `pathfinder!`
```rs
let mut control = pathfinder!(" x: 1, y:1,  path: 'C:\\Program Files', w:40"));
```

A pathfinder supports all common parameters (as they are described in [Instantiate via Macros](../instantiate_via_macros.md) section). Besides them, the following **named parameters** are also accepted:

| Parameter name      | Type   | Positional parameter                | Purpose                                                                                                              |
| ------------------- | ------ | ----------------------------------- | -------------------------------------------------------------------------------------------------------------------- |
| `path` | String | **Yes** (first postional parameter) | The file path used as a starting point when navigating through the file system. |
| `flags`             | List   | **No**                              | PathFinder initialization flags that control if the path finder is case-sensitive, readonly, etc                  |

A pathfinder supports the following initialization flags:
* `pathfinder::Type::Readonly` or `Readonly` (for macro initialization) - thils will allow you to view or copy the text but not to modify it
* `pathfinder::Type::CaseSensitive` or `CaseSensitive` (for macro initialization) - by default the control is case insensitive, set this if you want it to be case sensitive.
Some examples that use these parameters:
```rs
let cb = pathfinder!(" x: 1, y:1,  path: 'C:\\Program Files', w:40, flags:ReadOnly|CaseSensitive");
let cb = pathfinder!(" x: 1, y:1,  path: 'C:\\Program Files', w:40, enabled: false);
```

## Events
To intercept events from a pathfinder, the following trait has to be implemented to the Window that processes the event loop:

```rs
pub trait PathFinderEvents {
    fn on_path_updated(&mut self, handle: Handle<PathFinder>) -> EventProcessStatus {...}
}
```

## Methods

Besides the [Common methods for all Controls](../common_methods.md) a pathfinder also has the following aditional methods:

| Method          | Purpose                                                                                                                  |
| --------------- | ------------------------------------------------------------------------------------------------------------------------ |
| `set_path(...)` | Set the path for a pathfinder.                                                                                        |
| `path()`        | Returns the current path from a pathfinder.                                                                                |

## Key association

The following keys are processed by a PathFinder control if it has focus:

| Key                                  | Purpose                                                                                                                                   |
| ------------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------- |
| `Left`, `Right`        | Navigate through the text from the pathfinder                                                                                              |
| `Shift`+{`Left`,`Right`} | Selects part of the text pathfinder                                                                                                        |
| `Home`                               | Move to the begining of the text                                                                                                          |
| `Shift`+`Home`                       | Selects the text from the beging of the text until the current position                                                                   |
| `End`                                | Moves to the end of the text                                                                                                              |
| `Shift` + `End`                      | Selects the text from current position until the end of the text                                                                          |
| `Delete`                             | Deletes the current character. If a selection exists, it deletes it first                                                                 |
| `Backspace`                          | Deletes the previous charactr. If a selection exists, it deletes it first                                                                 |
| `Ctrl`+`A`                           | Selects the entire text               |
| `Ctrl`+`C` or `Ctrl`+`Insert`        | Copy the current selection to clipboard                                                                                                   |
| `Ctrl`+`V` or `Shift`+`Insert`       | Paste the text from the clipboard (if any) to current position                                                                            |
| `Ctrl`+`X` or `Shift`+`Delete`       | If a selection is present, it copies it into the clipboard and then delets it (acts like a `Cut` command)                                 |

Aditionally, all printable characters can be used to insert / modify or edit the current text.

## Mouse actions

Mouse cursor can be used to select the text. Aditionally, a double click over the control will select all the text.

## Example

The following code creates multiple path finders with both unicode and regular text.

```rs
use appcui::prelude::*;

fn main() -> Result<(), appcui::system::Error> {
    let mut app = App::new().build()?;
    let mut w = Window::new("Title", Layout::new("a:c,w:40,h:11"), window::Flags::None);
    w.add(pathfinder!("path: 'C:\\Program Files',x:1,y:1,w:36,h:1"));
    w.add(pathfinder!("'C:\\Program Files',x:1,y:3,w:36,h:1, flags: ReadOnly"));
    w.add(pathfinder!("path:'C:\\Program Files\\Țambal.exe',x:1,y:5,w:36,h:1,enable: false"));
    a.add_window(w);
    a.run();
    Ok(())
}
```