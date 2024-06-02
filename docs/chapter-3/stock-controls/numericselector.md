# NumericSelector

The `NumericSelector` control is a simple control that allows the user to select a number from a range of numbers. The control is made up of a text field and two buttons, one to increase the number and one to decrease it. The control can be used to select a number from a range of numbers.

<img src="img/numericselector.png" width=300/>

It can be create using `NumericSelector::new(...)`, `NumericSelector::with_format(...)` or the `numericselector!` macro. Using `NumericSelector::new(...)` can be done in two ways:
1. by specifying the type for a variable:
    ```rs
    let s: NumericSelector<T> = NumericSelector::new(...);
    ```

2. by using turbo-fish notation (usually when you don't want to create a separate variable for the control):
    ```rs
    let s = NumericSelector::<T>::new(...);
    ```
**Remarks**: The type `T` can be one of the following: `i8`, `i16`, `i32`, `i64`, `i128`, `u8`, `u16`, `u32`, `u64`, `u128`, `usize`, `isize`, `f32`, `f64`.

## Examples

Assuming we want to create a NumeicSelector for `i32` type, we can do it as follows:

```rs
let n1: NumericSelector<i32> = NumericSelector::new(Layout::new("..."),numericselector::Flags::None);
let n2: NumericSelector<i32> = NumericSelector::with_format(1,Layout::new("..."),numericselector::Flags::None, numericselector::Format::Percentage);
let n3 = numericselector!("class:i32,value:5,min:0,max:10,step:1,x:1,y:1,w:20");
let n4 = numericselector!("u32,5,0,10,step:1,x:1,y:1,w:20,format:Percentage");
let n5 = numericselector!("i32,5,0,10,step:1,x:1,y:1,w:20,flags:ReadOnly,format:Percentage");
```

A numeric selector supports all common parameters (as they are described in [Instantiate via Macros](../instantiate_via_macros.md) section). Besides them, the following **named parameters** are also accepted:

| Parameter name                      | Type   | Positional parameter                  | Purpose                                                                                                                                          |
| ----------------------------------- | ------ | ------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------ |
| `class` or `type`                   | String | **Yes** (first postional parameter)   | The name of a templetized type to be used when creating the numeric selector                                                                     |
| `value`                             | String | **Yes** (second positional parameter) | The initial value of the numeric selector. If it is not within the bounds (`min` and `max` parameters) it will be adjusted to the closest limit. |
| `min`                               | String | **Yes** (third positional parameter)  | The minimum value that the numeric selector can have. If the initial value is less than this, it will be adjusted to this value.                 |
| `max`                               | String | **Yes** (fourth positional parameter) | The maximum value that the numeric selector can have. If the initial value is greater than this, it will be adjusted to this value.              |
| `step`                              | String | **Yes** (fifth positional parameter)  | The step by which the value of the numeric selector will be increased or decreased.                                                              |
| `flags`                             | String | **No**                                | Numeric selector initialization flags                                                                                                            |
| `format` or `nf` or `numericformat` | String | **No**                                | The format in which the value of the numeric selector will be displayed.                                                                         |

A numeric selector supports the following initialization flags:
* `numericselector::Flags::ReadOnly` or `ReadOnly` (for macro initialization) - this will make the numeric selector read-only   
* `numericselector::Flags::HideButtons` or `HideButtons` (for macro initialization) - this will hide the buttons that allow the user to increase or decrease the value

The format parameter can be one of the following:
* `numericselector::Format::Decimal` or `Decimal` (for macro initialization) - this will display the value as it is
* `numericselector::Format::Percentage` or `Percentage` (for macro initialization) - this will display the value as a percentage
* `numericselector::Format::Hex` or `Hex` (for macro initialization) - this will display the value as a hexadecimal number  
* `numericselector::Format::DigitGrouping` or `DigitGrouping` (for macro initialization) - this will display the value with digit grouping (for example: 1,000,000)
* `numericselector::Format::Size` or `Size` (for macro initialization) - this will display the value as a size (for example: 1.5 KB, 1.5 MB, 1.5 GB, 1.5 TB)

## Events

To intercept events from a numeric selector, the following trait has to be implemented to the Window that processes the event loop:
```rs
pub trait NumericSelectorEvents<T> {
    fn on_value_changed(&mut self, handle: Handle<NumericSelector<T>>, value: T) -> EventProcessStatus {...}
}
```

## Methods

Besides the [Common methods for all Controls](../common_methods.md) a numeric selector also has the following aditional methods:

| Method           | Purpose                                                                                                                                                                                 |
| ---------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `set_value(...)` | Sets the new value associated with the selector. The value will be adjusted to the `min` / `max` parameters. This method will do nothing if the control was created in a read-only mode |
| `value()`        | Returns the current value of the control.                                                                                                                                               |


## Key association

The following keys are processed by a `NumericSelector` control if it has focus:

| Key             | Purpose                                                                                                                                 |
| --------------- | --------------------------------------------------------------------------------------------------------------------------------------- |
| `Enter`         | Either enters the edit mode, or if already in edit mode validates the new value and sets it up                                          |
| `Up`, `Left`    | Decreases the value using the `step` parameter. If the new value is less than the `min` parameter, it will be adjusted to this value    |
| `Down`, `Right` | Increases the value using the `step` parameter. If the new value is greater than the `max` parameter, it will be adjusted to this value |
| `Home`          | Sets the value to the `min` parameter                                                                                                   |
| `End`           | Sets the value to the `max` parameter                                                                                                   |
| `Backspace`     | Deletes the last digit from the value                                                                                                   |   
| `Escape`        | Cancels the edit mode and restores the previous value                                                                                   |   

Besides this using any one of the following keys: `A` to `F` and/or `0` to `9` will move enter the edit mode and will allow the user to enter a new value.  

## Example

The following example shows how to create a simple application that converts a temperature from Celsius to Fahrenheit and vice versa. The application uses two numeric selectors, one for Celsius and one for Fahrenheit. When the value of one of the numeric selectors is changed, the other numeric selector is updated with the converted value.

```rs
use appcui::prelude::*;

#[Window(events = NumericSelectorEvents<f64>)]
struct MyWin {
    celsius: Handle<NumericSelector<f64>>,
    fahrenheit: Handle<NumericSelector<f64>>,
}

impl MyWin {
    fn new() -> Self {
        let mut win = MyWin {
            base: window!("'Convert',d:c,w:40,h:8"),
            celsius: Handle::None,
            fahrenheit: Handle::None,
        };
        win.add(label!("'Celsius:',x:1,y:1,w:12,h:1"));
        win.celsius = win.add(numericselector!("f64,0.0,x:14,y:1,w:25,min:-100.0,max:100.0,step:1.0"));
        win.add(label!("'Fahrenheit:',x:1,y:3,w:12,h:1"));
        win.fahrenheit = win.add(numericselector!("f64,32.0,x:14,y:3,w:25,min:-213.0,max:213.0,step:0.1"));
        win
    }
    fn convert_celsius_to_feherenheit(&mut self) {
        let celsius = self.control(self.celsius).unwrap().value();
        let fahrenheit = celsius * 9.0 / 5.0 + 32.0;
        let h = self.fahrenheit;
        self.control_mut(h).unwrap().set_value(fahrenheit);
    }
    fn convert_fahrenheit_to_celsius(&mut self) {
        let fahrenheit = self.control(self.fahrenheit).unwrap().value();
        let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
        let h = self.celsius;
        self.control_mut(h).unwrap().set_value(celsius);
    }
}

impl NumericSelectorEvents<f64> for MyWin {
    fn on_value_changed(&mut self, handle: Handle<NumericSelector<f64>>, _value: f64) -> EventProcessStatus {
        match () {
            _ if handle == self.celsius => {
                self.convert_celsius_to_feherenheit();
                EventProcessStatus::Processed
            }
            _ if handle == self.fahrenheit => {
                self.convert_fahrenheit_to_celsius();
                EventProcessStatus::Processed
            }
            _ => EventProcessStatus::Ignored,
        }
    }
}

fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().build()?;
    a.add_window(MyWin::new());
    a.run();
    Ok(())
}
```