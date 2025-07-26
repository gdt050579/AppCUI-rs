# ToggleButton

A toggle button is a button that can be toggled on (selected) and off (unselected). 

<img src="img/togglebutton.png" width=200/>

A toggle button is created using the `ToggleButton::new` and `ToggleButotn::with_single_selection` methods:

```rs
let tb1 = ToggleButton::new( 
        "Aa",                         // caption
        "Enable case sensitive",      // tooltip
        layout!(...),             // layout
        false,                        // initial state (on/off)
        togglebutton::Flags::Normal); // type

let tb2 = ToggleButton::with_single_selection( 
        "Aa",                         // caption
        "Enable case sensitive",      // tooltip
        layout!(...),             // layout
        false,                        // initial state (on/off)
        togglebutton::Flags::Normal); // type        
```

or the macro `togglebutton!`:

```rs
let tb1 = togglebutton!("Aa,'Enable case sensitive',x:10,y:5,w:15");
let tb2 = togglebutton!("Aa,'Enable case sensitive',
                         x:10,y:5,w:15,selected: false, group: true");                         
```

A toggle button supports all common parameters (as they are described in [Instantiate via Macros](../instantiate_via_macros.md) section). Besides them, the following **named parameters** are also accepted:

| Parameter name                       | Type   | Positional parameter                  | Purpose                                                                                                                                                                                                                                        |
| ------------------------------------ | ------ | ------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name` or `text` or `caption`        | String | **Yes** (first postional parameter)   | The caption (text) written on a button                                                                                                                                                                                                         |
| `tooltip` or `desc` or `description` | String | **Yes** (second positional parameter) | The tool tip that will be showed if the mouse is hovered over the control. Since the text within this control is usually small (2-3 character - such as a pictogram), this is the way to convey more information on the purpose of the control |
| `type`                               | String | **No**                                | The type of the toggle button                                                                                                                                                                                                                  |
| `state` or `selected` or `select`    | bool   | **No**                                | The initial state of the toggle button (on/off)                                                                                                                                                                                                |
| `group` or `single_selection`        | bool   | **No**                                | If `true` the toggle button will be part of a group of toggle buttons. Only one button in the group can be selected at a time.                                                                                                                 |

A toggle button supports the following types:
* `button::Type::Normal` or `Normal` (for macro initialization) - this is the default type of a toggle button.
* `button::Type::Underlined` or `Underlined` (for macro initialization) - this will underline the caption of the button is it is selected.

## Events

To intercept events from a toggle button, the following trait has to be implemented to the Window that processes the event loop:

```rs
pub trait ToggleButtonEvents {
    fn on_selection_changed(&mut self, handle: Handle<ToggleButton>, selected: bool) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
```

## Methods

Besides the [Common methods for all Controls](../common_methods.md) a button also has the following aditional methods:

| Method              | Purpose                                                          |
| ------------------- | ---------------------------------------------------------------- |
| `set_caption(...)`  | Set the new caption for a toggle  button.                        |
| `caption()`         | Returns the current caption of a toggle button                   |
| `set_selected(...)` | Set the state of the toggle button (on or off)                   |
| `is_selected()`     | Returns the current state of the toggle button (selected or not) |

## Key association

The following keys are processed by a Button control if it has focus:

| Key     | Purpose                                                                                                                                                       |
| ------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Space` | Clicks / pushes the button and emits `ToggleButtonEvents::on_selection_changed(...)` event. It has the same action clicking the toggle button with the mouse. |
| `Enter` | Clicks / pushes the button and emits `ToggleButtonEvents::on_selection_changed(...)` event. It has the same action clicking the toggle button with the mouse. |

## Grouping

If a toggle button is created with the `group` parameter set to `true` or via the api `ToggleButton::with_single_selection` it will be part of a group of toggle buttons. Only one button in the group can be selected at a time.

To create multiple groups, one need to create panels and add toggle buttons as their children, like in the following example:
```rs
// group 1
let mut panel_1 = Panel::new(...);
panel_1.add(ToggleButton::with_single_selection(...));
panel_1.add(ToggleButton::with_single_selection(...));
panel_1.add(ToggleButton::with_single_selection(...));

// group 2
let mut panel_2 = Panel::new(...);
panel_2.add(togglebutton!("....,group:true"));
panel_2.add(togglebutton!("....,group:true"));
panel_2.add(togglebutton!("....,group:true"));
```

## Example


The following code creates a window with three toggle buttons (`Case sensitive`, `Match whole word` and `RegExp search`). 
```rs
use appcui::prelude::*;

fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().build()?;
    let mut w = window!("Test,a:c,w:60,h:10");
    let tg1 = ToggleButton::new(
        "Aa", 
        "Case sensitive", 
        layout!("x:1,y:1,w:2,h:1"), 
        true, 
        togglebutton::Type::Underlined);
    let tg2 = ToggleButton::new(
        "..",
        "Match whole word",
        layout!("x:4,y:1,w:2,h:1"),
        false,
        togglebutton::Type::Underlined,
    );
    let tg3 = ToggleButton::new(
        ".*", 
        "RegExp search", 
        layout!("x:7,y:1,w:2,h:1"), 
        true, 
        togglebutton::Type::Underlined);
    w.add(tg1);
    w.add(tg2);
    w.add(tg3);
    a.add_window(w);
    a.run();
    Ok(())
}
```