# DatePicker

Represent a control from where you can choose a color:

<img src="img/datepicker.png" width=300/>

To create a color picker use `DatePicker::new` method (with 2 parameters: a color and a layout).
```rs
let d = DatePicker::new("2024-06-13", Layout::new("d:c,w:19"));
```
or the macro `datepicker!`
```rs
let d1 = datepicker!("2024-06-13,x:1,y:1,w:19");
let d2 = datepicker!("date:2024-06-13,x:1,y:1,w:19");
```

A DatePicker control supports all common parameters (as they are described in [Instantiate via Macros](../instantiate_via_macros.md) section). Besides them, the following **named parameters** are also accepted:

| Parameter name | Type   | Positional parameter                | Purpose                                                                                                                                                                                                                                               |
| -------------- | ------ | ----------------------------------- | -------------------------------------------------------------------------------------------------------------------- |
| `date`         | String | **Yes** (first postional parameter) | The initial date of the DatePicker in `YYYY-MM-DD` format or any other format supported by NaiveDate in chrono crate |

## Events
No events implemented yet.
<!-- To intercept events from a DatePicker control, the following trait has to be implemented to the Window that processes the event loop:
```rs
pub trait DatePickerEvents {
    fn on_color_changed(&mut self, handle: Handle<DatePicker>, color: Color) -> EventProcessStatus {...}
}
``` -->

## Methods

Besides the [Common methods for all Controls](../common_methods.md) a DatePicker control also has the following aditional methods:

| Method              | Purpose                                                                                    |
| ------------------- | ------------------------------------------------------------------------------------------ |
| `set_date(...)`     | Manually sets the date of the DatePicker control. It receives an object of type NaiveDate. |
| `set_date_str(...)` | Manually sets the date of the DatePicker control. It receives an string slice.             |
| `date()`            | Returns the current date selected in the DatePicker control                                |


## Key association

No key association implemented yet.

<!-- The following keys are processed by a DatePicker control if it has focus:

| Key                           | Purpose                                                                                                                                                                                                                           |
| ----------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Space` or `Enter`            | Expands or packs (collapses) the DatePicker control.                                                                                                                                                                             |
| `Up`, `Down`, `Left`, `Right` | Changes the current selected color from the DatePicker. Using this keys will trigger a call to `DatePickerEvents::on_color_changed(...)`                                                                                        |
| `Escape`                      | Only when the DatePicker is expanded, it collapses the control. If the DatePicker is already colapsed, this key will not be captured (meaning that one of the DatePicker ancestors will be responsable with treating this key) | -->

## Example

The following example creates a Window with a DatePicker.

```rust,no_run
use appcui::prelude::*;

fn main(){
    let mut a =  App::new().build().unwrap();
    let mut w = window!("Dates,d:c,w:25,h:6");
    w.add(datepicker!("2024-06-13,x:1,y:1,w:19"));
    a.add_window(w);
    a.run();
}

```