# Horizontal Splitter

Renders a horizontal splitter that allows the user to resize the two panels it separates.

<img src="img/hsplitter.png" width=300/>

To create a horizontal splitter use `HSplitter::new` method or the `hsplitter!` macro.

```rust
let vs_1 = HSplitter::new(0.5,Layout::new("x:1,y:1,w:20,h:10"),hsplitter::ResizeBehavior::PreserveBottomPanelSize);
let vs_2 = HSplitter::new(20,Layout::new("x:1,y:1,w:20,h:10"),hsplitter::ResizeBehavior::PreserveBottomPanelSize);
```

or

```rust
let vs_3 = hsplitter!("x:1,y:1,w:20,h:10,pos:50%");
let vs_4 = hsplitter!("x:1,y:1,w:20,h:10,pos:20,resize:PreserveBottomPanelSize");
```


A horizontal splitter supports all common parameters (as they are described in [Instantiate via Macros](../instantiate_via_macros.md) section). Besides them, the following **named parameters** are also accepted:

| Parameter name                                       | Type       | Positional parameter                | Purpose                                                                                                                                      |
| ---------------------------------------------------- | ---------- | ----------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------- |
| `pos`                                                | Coordonate | **Yes** (first postional parameter) | The position of the splitter (can be an abosolute value - like `10` or a percentage like `50%` )                                             |
| `resize` or `resize-behavior` or `on-resize` or `rb` | String     | **No**                              | The resize behavior of the splitter. Can be one of the following: `PreserveTopPanelSize`, `PreserveBottomPanelSize` or `PreserveAspectRatio` |
| `min-top-height` or `mintopheight` or `mth`          | Dimension  | **No**                              | The minimum height of the top panel (in characters - e.g. `5`) or as a percentage (e.g. `10%`)                                               |
| `min-bottom-height` or `minbottomheight` or `mbh`    | Dimension  | **No**                              | The minimum height of the bottom panel (in characters - e.g. `5`) or as a percentage (e.g. `10%`)                                            |

A vertial splitters supports the following resize modes:
* `hsplitter::ResizeBehavior::PreserveTopPanelSize` or `PreserveTopPanelSize` (for macro initialization) - this will keep the size of the top panel constant when resizing the splitter  
* `hsplitter::ResizeBehavior::PreserveBottomPanelSize` or `PreserveBottomPanelSize` (for macro initialization) - this will keep the size of the bottom panel constant when resizing the splitter   
* `hsplitter::ResizeBehavior::PreserveAspectRatio` or `PreserveAspectRatio` (for macro initialization) - this will keep the aspect ratio of the two panels constant when resizing the splitter  


## Events

A horizontal splitter emits no events.

## Methods

Besides the [Common methods for all Controls](../common_methods.md) a horizontal splitter also has the following aditional methods:

| Method                | Purpose                                                                                                                                                                                                                   |
| --------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `add(...)`            | Adds an element to the top or bottom panel of the splitter.                                                                                                                                                               |
| `set_min_height(...)` | Sets the minimum height of the top or bottom panel.                                                                                                                                                                       |
| `set_position(...)`   | Sets the position of the splitter.  If an integer value is being used, the position will be considered in characters. If a flotant value (`f32` or `f64`) is being used, the position will be considered as a percentage. |
| `position()`          | Returns the current position of the splitter (in characters).                                                                                                                                                             |

## Key association

The following keys are processed by a HSplitter control if it has focus:

| Key               | Purpose                                       |
| ----------------- | --------------------------------------------- |
| `Ctrl+Up`         | Moves the splitter one character up           |
| `Ctrl+Down`       | Moves the splitter one character down         |
| `Ctrl+Shift+Up`   | Move the splitter to its top most position    |
| `Ctrl+Shift+Down` | Move the splitter to its bottom most position |

## Example

The following code creates a window with a horizontal splitter that separates two panels. The upper panel contains a panel with the text `Top` and the bottom panel contains a panel with the text `Bottom`.

```rust
use appcui::prelude::*;

fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().build()?;
    let mut w = window!("'Horizontal Splitter',a:c,w:50,h:11,flags: Sizeable");
    let mut hs = hsplitter!("50%,a:c,w:100%,h:100%,resize:PreserveBottomPanelSize");
    hs.add(hsplitter::Panel::Top,panel!("Top,l:1,r:1,t:1,b:1"));
    hs.add(hsplitter::Panel::Bottom,panel!("Bottom,l:1,r:1,t:1,b:1"));
    w.add(hs);
    a.add_window(w);
    a.run();
    Ok(())
}
```