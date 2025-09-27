# TimePicker

Represent a control from where you can choose/modify a time:

<img src="img/timepicker.png" width=200/>

To create a time picker use `TimePicker::new` method (with 3 parameters: a time string, a layout, and flags), or it can be created with `TimePicker::with_time` method (with 3 parameters: a NaiveTime object, a layout, and flags).

```rs
let t = TimePicker::new("12:34:56", layout!("a:c,w:10"), timepicker::Flags::Seconds);
let t = TimePicker::with_time(NaiveTime::from_hms_opt(12, 34, 56).unwrap(), layout!("a:c,w:10"), timepicker::Flags::Seconds);
```

or the macro `timepicker!`

```rs
let t1 = timepicker!("'12:34:56',x:1,y:1,w:10");
let t2 = timepicker!("time:'12:34:56',x:1,y:1,w:10,flags:Seconds");
```

A TimePicker control supports all common parameters (as they are described in [Instantiate via Macros](../instantiate_via_macros.md) section). Besides them, the following **named parameters** are also accepted:

| Parameter name | Type   | Positional parameter                 | Purpose                                                                                                            |
| -------------- | ------ | ------------------------------------ | ------------------------------------------------------------------------------------------------------------------ |
| `time`         | String | **Yes** (first positional parameter) | The initial time of the TimePicker in `HH:MM:SS` format or any other format supported by NaiveTime in chrono crate |
| `flags`        | Flags  | No                                   | Additional flags that control the appearance and behavior of the TimePicker                                        |

## Flags

The TimePicker control supports the following flags:

| Flag name | Purpose                                                    |
| --------- | ---------------------------------------------------------- |
| `Seconds` | Shows seconds in the time picker (format becomes HH:MM:SS) |
| `AMPM`    | Shows AM/PM indicator and uses 12-hour format              |

Flags can be combined using the `|` operator (e.g., `timepicker::Flags::Seconds | timepicker::Flags::AMPM`).

## Events

To intercept events from a TimePicker control, the following trait has to be implemented to the Window that processes the event loop:

```rs
pub trait TimePickerEvents {
    fn on_time_changed(&mut self, handle: Handle<TimePicker>, 
                                  time: chrono::prelude::NaiveTime) -> EventProcessStatus 
    {...}
}
```

## Methods

Besides the [Common methods for all Controls](../common_methods.md) a TimePicker control also has the following additional methods:

| Method          | Purpose                                                                                    |
| --------------- | ------------------------------------------------------------------------------------------ |
| `set_time(...)` | Manually sets the time of the TimePicker control. It receives an object of type NaiveTime. |
| `time()`        | Returns the current time selected in the TimePicker control                                |

## Key association

The following keys are processed by a TimePicker control if it has focus:

| Key             | Purpose                                                                                                                        |
| --------------- | ------------------------------------------------------------------------------------------------------------------------------ |
| `Up`, `Down`    | Increases/decreases the value of the currently selected component (hour, minute, second, or AM/PM)                             |
| `Left`, `Right` | Moves between time components (hour → minute → second → AM/PM → hour)                                                          |
| `0`-`9`         | Enters a digit for the currently selected component. Automatically moves to the next component after entering a complete value |
| `Backspace`     | Resets the digit editing position to the first digit of the current component                                                  |

## Mouse interaction

The TimePicker control supports the following mouse interactions:

| Mouse Action             | Purpose                                                                             |
| ------------------------ | ----------------------------------------------------------------------------------- |
| `Click`                  | Selects the time component (hour, minute, second, or AM/PM) at the clicked position |
| `Hover`                  | Highlights the time component under the mouse cursor                                |
| `Mouse Wheel Up/Down`    | Increases/decreases the value of the currently selected component                   |
| `Mouse Wheel Left/Right` | Moves between time components                                                       |

## Example

The following example creates a Window with a TimePicker. The window implements the `TimePickerEvents` to intercept TimePicker events.

```rust
use appcui::prelude::*;

#[Window(events=TimePickerEvents)]
struct MyWin {
    tp: Handle<TimePicker>,
}

impl MyWin {
    fn new() -> Self {
        let mut win = MyWin {
            base: window!("Time,a:c,w:25,h:6"),
            tp: Handle::None,
        };
        win.tp = win.add(timepicker!("'12:34:56',x:1,y:1,w:10,flags:Seconds"));
        win
    }
}

impl TimePickerEvents for MyWin {
    fn on_time_changed(&mut self, 
                       handle: Handle<TimePicker>, 
                       time: chrono::prelude::NaiveTime) -> EventProcessStatus 
    {
        self.set_title(&format!("Time: {time}"));
        EventProcessStatus::Processed                                                                        
    }
}

fn main() {
    let mut a = App::new().build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}
```