# HNumericSlider

The `HNumericSlider` is a horizontal slider control that allows users to select a numeric value from a defined range. Unlike traditional input fields or stepper buttons, this control offers an interactive and visual method to navigate between values, making it ideal for UI scenarios where intuitive numeric selection is essential.

It supports any primitive numeric type and is configured via either the `HNumericSlider::new(...)` constructor or the `hnumericslider!` macro. The slider adapts to the type, bounds, and step provided by the user.

**Supported types:**
`i8`, `i16`, `i32`, `i64`, `i128`, `u8`, `u16`, `u32`, `u64`, `u128`, `f32`, `f64`

## Constructor Parameters

The `HNumericSlider::new(...)` function takes the following arguments:

```rs
fn new(
    value: T,
    min: T,
    max: T,
    step: T,
    layout: Layout,
    flags: Flags,
    format: Format,
) -> HNumericSlider<T>
```

| Parameter | Type   | Description                                                         |
| --------- | ------ | ------------------------------------------------------------------- |
| `value`   | `T`    | Initial value. Clamped to `[min, max]` if outside the bounds.       |
| `min`     | `T`    | Minimum allowed value. Must be less than `max`.                     |
| `max`     | `T`    | Maximum allowed value. Must be greater than `min`.                  |
| `step`    | `T`    | Value increment. Must not be zero.                                  |
| `layout`  | Layout | Specifies the position and size of the control. Height must be ≥ 3. |
| `flags`   | Flags  | Optional control flags, such as `OnTop`.                            |
| `format`  | Format | Display format for the values (e.g., Decimal, Hex, Percentage).     |

## Examples

Creating a `HNumericSlider` for `i32` values:

```rs
let slider = HNumericSlider::new(
    6,                             // initial value
    0,                             // min
    10,                            // max
    2,                             // step
    Layout::new("x:1,y:2,w:100%,h:3"), // layout (height must be >= 3)
    hnumericslider::Flags::OnTop,  // flags
    common::number::Format::Decimal // format
);
```

Using the macro:

```rs
let slider = hnumericslider!("i32, min:0, max:10, step:2, value:6, x:0, y:0, w:100%, h:3");
```

## Macro Usage

The `hnumericslider!` macro supports both positional and named parameters.

- **Positional parameters** are specified in order.
- **Named parameters** use the format `name:value` and may appear in any order.

### Positional Parameters (in order):

| Index | Name    | Type   | Description                                |
| ----- | ------- | ------ | ------------------------------------------ |
| 1     | `class` | String | The numeric type used (e.g., `i32`, `f64`) |
| 2     | `value` | String | The initial value                          |
| 3     | `min`   | String | Minimum allowed value                      |
| 4     | `max`   | String | Maximum allowed value                      |
| 5     | `step`  | String | Step size between values                   |

### Named Parameters (case-insensitive aliases supported):

| Name     | Alias(es)             | Type   | Description                    |
| -------- | --------------------- | ------ | ------------------------------ |
| `class`  | `type`                | String | Numeric type                   |
| `value`  | `v`                   | String | Initial value                  |
| `min`    |                       | String | Minimum bound                  |
| `max`    |                       | String | Maximum bound                  |
| `step`   | `s`                   | String | Step size                      |
| `flags`  |                       | Flags  | Control behavior customization |
| `format` | `numericformat`, `nf` | String | Display format for numbers     |

### Example with mixed parameters:

```rs
let slider = hnumericslider!("i32, 6, 0, 10, 2, x:1, y:1, format:Percentage, flags:OnTop");
```

## Layout Requirements

The `HNumericSlider` requires a height of **at least 3 units** to render correctly. This ensures sufficient space for the slider line, tick marks, and numeric labels.

Make sure your layout definition includes `h:3` or greater:

```rs
Layout::new("x:1,y:2,w:100%,h:3")
```

## Safety & Validation

If the specified width is **not sufficient to fit all the values** in the slider range with the provided step, the control will attempt to **dynamically adjust the step size** to fit within the available space. This ensures the control remains usable and visually coherent even in constrained layouts.

To ensure predictable behavior and avoid runtime issues, the following constraints must be respected:

- **`step`**\*\* must not be 0\*\* — if `step == 0`, the program will panic.
- **`min`**\*\* must be strictly less than \*\***`max`** — if `min >= max`, the program will panic.

These validations are enforced at runtime in both the constructor and macro forms.

### Supported Flags

- `OnTop`: Displays ticks and values above the slider line, reversing the triangle indicator direction.

### Supported Formats

Defined by the `Format` enum:

```rs
pub enum Format {
    Decimal,
    Percentage,
    DigitGrouping,
    Hex,
    Size,
}
```

- `Decimal` (default): Plain number formatting.
- `Percentage`: Displays values as percentages.
- `DigitGrouping`: Adds digit grouping (e.g., `1,000,000`).
- `Hex`: Displays value as hexadecimal.
- `Size`: Displays size units (e.g., `1.5 KB`, `2.0 MB`).

## Events

To respond to value changes, implement the `HNumericSliderEvents<T>` trait for your window:

```rs
pub trait HNumericSliderEvents<T> {
    fn on_value_changed(&mut self, handle: Handle<HNumericSlider<T>>) -> EventProcessStatus;
}
```

An event is emitted every time the value changes (either via keyboard or mouse).

## Methods

| Method                 | Purpose                                                              |
| ---------------------- | -------------------------------------------------------------------- |
| `get_selected_value()` | Returns the currently selected value                                 |
| `set_selected_value()` | Sets the current value and **automatically clamps** it within bounds |

## Keyboard & Mouse Interaction

If the slider control is focused, the following keyboard and mouse interactions are supported:

### Keyboard Controls

| Key            | Action                         |
| -------------- | ------------------------------ |
| `Left`, `Down` | Decrease the value by one step |
| `Right`, `Up`  | Increase the value by one step |
| `Home`         | Set value to minimum (`min`)   |
| `End`          | Set value to maximum (`max`)   |

### Mouse Controls

- **Left-click** on the slider line selects the closest value.
- **Dragging** the mouse across the slider updates the value dynamically.

### Example

A small code example that initializes a HNumericSlider and also intercepts the events generated by it.

```rs
use appcui::prelude::*;

#[Window(events:HNumericSliderEvents<i32>)]
struct MyWindow {
    h: Handle<HNumericSlider<i32>>,
}

impl MyWindow {
    fn new() -> Self {
        let mut w = Self {
            base: window!("Test, d:c, w:40, h:10, flags:Sizeable"),
            h: Handle::None,
        };
        w.h = w.add(hnumericslider!("i32, min:0, max:10, step:2, value:6, x:0, y:0, w:100%, h:3"));
        w
    }
}

impl HNumericSliderEvents<i32> for MyWindow {
    fn on_value_changed(&mut self, handle: Handle<HNumericSlider<i32>>) -> EventProcessStatus {
        if handle == self.h {
            if let Some(hn) = self.control(handle) {
                let txt = format!("New value:{}", hn.get_selected_value());
                self.set_title(&txt);
                return EventProcessStatus::Processed;
            }
        }
        EventProcessStatus::Ignored
    }
}

fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().log_file("debug.log",false).build()?;
    a.add_window(MyWindow::new());
    a.run();
    Ok(())
}
```