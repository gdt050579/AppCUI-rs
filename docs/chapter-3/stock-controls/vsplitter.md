# Vertical Splitter

Renders a vertical splitter that allows the user to resize the two panes it separates.

<img src="img/vsplitter.png" width=300/>

To create a vertical splitter use `VSplitter::new` method or the `vsplitter!` macro.

```rust
let vs_1 = VSplitter::new(0.5,Layout::new("x:1,y:1,w:20,h:10"),vsplitter::ResizeBehavior::PreserveRightPanelSize);
let vs_2 = VSplitter::new(20,Layout::new("x:1,y:1,w:20,h:10"),vsplitter::ResizeBehavior::PreserveRightPanelSize);
```

or

```rust
let vs_3 = vsplitter!("x:1,y:1,w:20,h:10,pos:50%");
let vs_4 = vsplitter!("x:1,y:1,w:20,h:10,pos:20,resize:PreserveRightPanelSize");
```


A vertical splitter supports all common parameters (as they are described in [Instantiate via Macros](../instantiate_via_macros.md) section). Besides them, the following **named parameters** are also accepted:

| Parameter name                                       | Type       | Positional parameter                | Purpose                                                                                                                                      |
| ---------------------------------------------------- | ---------- | ----------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------- |
| `pos`                                                | Coordonate | **Yes** (first postional parameter) | The position of the splitter (can be an abosolute value - like `10` or a percentage like `50%` )                                             |
| `resize` or `resize-behavior` or `on-resize` or `rb` | String     | **No**                              | The resize behavior of the splitter. Can be one of the following: `PreserveLeftPanelSize`, `PreserveRightPanelSize` or `PreserveAspectRatio` |
| `min-left-width` or `minleftwidth` or `mlw`          | Dimension  | **No**                              | The minimum width of the left panel (in characters - e.g. `5`) or as a percentage (e.g. `10%`)                                               |
| `min-right-width` or `minrightwidth` or `mrw`        | Dimension  | **No**                              | The minimum width of the right panel (in characters - e.g. `5`) or as a percentage (e.g. `10%`)                                              |

A vertial splitters supports the following resize modes:
* `vsplitter::ResizeBehavior::PreserveLeftPanelSize` or `PreserveLeftPanelSize` (for macro initialization) - this will keep the size of the left panel constant when resizing the splitter  
* `vsplitter::ResizeBehavior::PreserveRightPanelSize` or `PreserveRightPanelSize` (for macro initialization) - this will keep the size of the right panel constant when resizing the splitter   
* `vsplitter::ResizeBehavior::PreserveAspectRatio` or `PreserveAspectRatio` (for macro initialization) - this will keep the aspect ratio of the two panels constant when resizing the splitter  


## Events

A vertical splitter emits no events.

## Methods

Besides the [Common methods for all Controls](../common_methods.md) a vertical splitter also has the following aditional methods:

| Method               | Purpose                                                                                                                                                                                                                   |
| -------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `add(...)`           | Adds an element to the left or right panel of the splitter.                                                                                                                                                               |
| `set_min_width(...)` | Sets the minimum width of the left or right panel.                                                                                                                                                                        |
| `set_position(...)`  | Sets the position of the splitter.  If an integer value is being used, the position will be considered in characters. If a flotant value (`f32` or `f64`) is being used, the position will be considered as a percentage. |
| `position()`         | Returns the current position of the splitter (in characters).                                                                                                                                                             |

## Key association

The following keys are processed by a VSplitter control if it has focus:

| Key                | Purpose                                       |
| ------------------ | --------------------------------------------- |
| `Ctrl+Left`        | Moves the splitter one character to the left  |
| `Ctrl+Right`       | Moves the splitter one character to the right |
| `Ctrl+Shift+Left`  | Move the splitter to its left most position   |
| `Ctrl+Shift+Right` | Move the splitter to its right most position  |

## Example

The following code creates a window with a vertical splitter that separates two panels. The left panel contains a panel with the text `Left` and the right panel contains a panel with the text `Right`.

```rust
use appcui::prelude::*;

fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().build()?;
    let mut w = window!("'Vertical Splitter',d:c,w:50,h:10,flags: Sizeable");
    let mut vs = vsplitter!("50%,d:c,w:100%,h:100%,resize:PreserveRightPanelSize");
    vs.add(vsplitter::Panel::Left,panel!("Left,l:1,r:1,t:1,b:1"));
    vs.add(vsplitter::Panel::Right,panel!("Right,l:1,r:1,t:1,b:1"));
    a.add_window(w);
    a.run();
    Ok(())
}
```