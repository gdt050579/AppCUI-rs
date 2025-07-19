# ProgressBar

Represent a progress bar that can be used to show the progress of a task.

<img src="img/progressbar.png" width=300/>

To create a label use `ProgressBar::new` method (with 2 parameters: a caption and a layout).
```rs
let pg1 = ProgressBar::new(1000, Layout::new("x:10,y:5,w:15"), progressbar::Flags::None);
```
or the macro `progressbar!`
```rs
let pg1 = progressbar!("total: 1000, x:10,y:5,w:15");
let pg2 = progressbar!("count: 125 ,x:10,y:5,w:15, text: 'Copying ...'");
```

A progress supports all common parameters (as they are described in [Instantiate via Macros](../instantiate_via_macros.md) section). Besides them, the following **named parameters** are also accepted:

| Parameter name               | Type    | Positional parameter | Purpose                                                                                                   |
| ---------------------------- | ------- | -------------------- | --------------------------------------------------------------------------------------------------------- |
| `text` or `caption`          | String  | **No**               | The caption (text) written on the progress bar                                                            |
| `total` or `count` or `c`    | Integer | **No**               | The total number of steps that the progress bar will show                                                 |
| `progress` or `value` or `v` | Integer | **No**               | The current value of the progress bar                                                                     |
| `pause` or `paused`          | Boolean | **No**               | If `true`, the progress bar will be paused                                                                |
| `flags`                      | Flags   | **No**               | Additional flags for the progress bar that can be used to control how the progress bar is being displayed |

A progress bar supports the following initialization types:
* `progressbar::Type::HidePercentage` or `HidePercentage` (for macro initialization) - thils will hide the percentage displayed on the progress bar.


## Events
A progress bar emits no events.

## Methods

Besides the [Common methods for all Controls](../common_methods.md) a label also has the following aditional methods:

| Method                 | Purpose                                                                                                                                                                                                     |
| ---------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `update_text(...)`     | Updates the text display on the progress bar                                                                                                                                                                |
| `update_progress(...)` | Updates the progress of the progress bar. If the progress bar is paused this method also resume its activity.                                                                                               |
| `processed()`          | Returns the current progress of the progress bar                                                                                                                                                            |
| `count()`              | Returns the total number of steps of the progress bar                                                                                                                                                       |
| `pause()`              | Pauses the progress bar                                                                                                                                                                                     |
| `resume()`             | Resumes the progress bar                                                                                                                                                                                    |
| `is_paused()`          | Returns `true` if the progress bar is paused and `false` otherwise                                                                                                                                          |
| `reset(...)`           | Resets the progress bar to its initial state. This method should be use to also set up a new count (total items) value for the progress bar. This method is often used when a progress bar is being reused. |

## Key association

A progress bar does not receive any input and as such it has no key associated with it.

## Example

The following code creates a window with a progress bar that shows the progress of a task that copies 100 files from one location to another.
```rs
use appcui::prelude::*;

fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().build()?;
    let mut w = window!("Test,a:c");
    let mut p = ProgressBar::new(100,Layout::new("x:1,y:1,w:30,h:2"), progressbar::Flags::None);
    p.update_text("Copying ...");
    w.add(p);
    a.add_window(w);
    a.run();
    Ok(())
}
```
