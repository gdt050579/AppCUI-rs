# DatePicker

Represent a control from where you can choose a color:

<img src="img/datepicker.png" width=300/>

To create a color picker use `DatePicker::new` method (with 2 parameters: a date and a layout), or it can be created with `DatePicker::with_date` method (with 2 parameters: a NaiveDate object and a layout).

```rs
let d = DatePicker::new("2024-06-13", Layout::new("d:c,w:19"));
let d = DatePicker::with_date(NaiveDate::from_ymd_opt(2000, 10, 1).unwrap(), Layout::new("d:c,w:19"));
```

or the macro `datepicker!`

```rs
let d1 = datepicker!("2024-06-13,x:1,y:1,w:19");
let d2 = datepicker!("date:2024-06-13,x:1,y:1,w:19");
```

A DatePicker control supports all common parameters (as they are described in [Instantiate via Macros](../instantiate_via_macros.md) section). Besides them, the following **named parameters** are also accepted:

| Parameter name | Type   | Positional parameter                | Purpose                                                                                                              |
| -------------- | ------ | ----------------------------------- | -------------------------------------------------------------------------------------------------------------------- |
| `date`         | String | **Yes** (first postional parameter) | The initial date of the DatePicker in `YYYY-MM-DD` format or any other format supported by NaiveDate in chrono crate |

## Events

To intercept events from a DatePicker control, the following trait has to be implemented to the Window that processes the event loop:

```rs
pub trait DatePickerEvents {
    fn on_date_change(&mut self, _handle: Handle<DatePicker>, date: chrono::prelude::NaiveDate) -> EventProcessStatus {...}
}
```

## Methods

Besides the [Common methods for all Controls](../common_methods.md) a DatePicker control also has the following aditional methods:

| Method              | Purpose                                                                                    |
| ------------------- | ------------------------------------------------------------------------------------------ |
| `set_date(...)`     | Manually sets the date of the DatePicker control. It receives an object of type NaiveDate. |
| `set_date_str(...)` | Manually sets the date of the DatePicker control. It receives an string slice.             |
| `date()`            | Returns the current date selected in the DatePicker control                                |

## Key association

The following keys are processed by a DatePicker control if it has focus:

On unexpanded calendar:

| Key                                | Purpose                                  |
| ---------------------------------- | ---------------------------------------- |
| `Space` or `Enter`                 | Extends(unpacks) the DatePicker control. |
| `Up`, `Down`                       | Changes the date's day with 1 day.       |
| `Shift+Up`, `Shift+Down`           | Changes the date's month by 1.           |
| `Ctrl+Up`, `Ctrl+Down`             | Changes the date's year by 1.            |
| `Ctrl+Shift+Up`, `Ctrl+Shift+Down` | Changes the date's year by 10.           |

On expanded calendar:

| Key                                   | Purpose                                                                                                                                                                                                              |
| ------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Enter`                               | Packs (collapses) the DatePicker control, saving the date and triggering a call to `DatePickerEvents::on_date_change(...)`.                                                                                          |
| `Escape`                              | It collapses the control without saving the new date. If the DatePicker is already colapsed, this key will not be captured (meaning that one of the DatePicker ancestors will be responsable with treating this key) |
| `Up`, `Down`, `Left`, `Right`         | Changes the date's day with 1 (left, right) or 7(up, down) days.                                                                                                                                                     |
| `Shift+Left`, `Shift+Right`           | Changes the date's month by 1.                                                                                                                                                                                       |
| `Ctrl+Left`, `Ctrl+Right`             | Changes the date's year by 1.                                                                                                                                                                                        |
| `Ctrl+Shift+Left`, `Ctrl+Shift+Right` | Changes the date's year by 10.                                                                                                                                                                                       |

On both calendar types:

| Key                    | Purpose                                                                                                                                                        |
| ---------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Letter (ex. `D`)       | Changes the date's month to the next month starting with that letter.                                                                                          |
| Shift+Letter (ex. `D`) | Changes the date's month to the previous month starting with that letter. (Working for letters for which there are multiple months starting with it (ex. `A`)) |

## Example

The following example creates a Window with a DatePicker. The window implements the `DatePickerEvents` to intercept DatePicker events.

```rust,no_run
use appcui::prelude::*;

use appcui::prelude::*;

#[Window(events=DatePickerEvents)]
struct MyWin {
    dp: Handle<DatePicker>,
}

impl MyWin{
    fn new() -> Self{
        let mut win = MyWin{
            base: window!("Dates,a:c,w:25,h:6"),
            dp: Handle::None,
        };
        win.dp = win.add(datepicker!("2024-06-13,x:1,y:1,w:19"));
        win
    }

}

impl DatePickerEvents for MyWin{
    fn on_date_change(&mut self, _handle: Handle<DatePicker>, date: chrono::prelude::NaiveDate) -> EventProcessStatus {
        self.base.set_title(&format!("Date: {}", date));
        EventProcessStatus::Processed                                                                        
    }
}

fn main(){
    let mut a =  App::new().build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

```