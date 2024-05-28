# ComboBox

A combobox is a drop down list control that allows you to select a variant from a list os strings. 

<img src="img/combobox.png" width=300/>

It can be created using `ComboBox::new(...)` or the `combobox!` macro. 

```rs
let c1 = ComboBox::new(Layout::new("..."),combobox::Flags::None);
let c2 = ComboBox::new(Layout::new("..."),combobox::Flags::ShowDescription);
let c3 = combobox!("x:1,y:1,w:20,items=['Red','Greem','Blue']");
let c3 = combobox!("x:1,y:1,w:20,items=['Red','Greem','Blue'],index:2");
```

A combobox supports all common parameters (as they are described in [Instantiate via Macros](../instantiate_via_macros.md) section). Besides them, the following **named parameters** are also accepted:

| Parameter name              | Type    | Positional parameter | Purpose                                                                                 |
| --------------------------- | ------- | -------------------- | --------------------------------------------------------------------------------------- |
| `flags`                     | String  | **No**               | ComboBox initialization flags                                                           |
| `items`                     | List    | **No**               | A list of string items to populate the combobox with.                                   |
| `index` or `selected_index` | Numeric | **No**               | The index of the selected item (it should be a value between 0 and number if items - 1) |

A combobox supports the following initialization flags:
* `combobox::Flags::ShowDescription` or `ShowDescription` (for macro initialization) - thils will allow a combobox show the description of each item (if exists) when expanded 

## Events

To intercept events from a combobox, the following trait has to be implemented to the Window that processes the event loop:
```rs
pub trait ComboBoxEvents {
    fn on_selection_changed(&mut self, handle: Handle<ComboBox>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
```

## Methods

Besides the [Common methods for all Controls](../common_methods.md) a combobox also has the following aditional methods:

| Method                   | Purpose                                                                                       |
| ------------------------ | --------------------------------------------------------------------------------------------- |
| `value()`                | Returns the selected value of the ComboBox. If the current value is `None` a panic will occur |
| `try_value()`            | Returns an `Option<&str>` containint the current selected of the ComboBox.                    |
| `index()`                | Returns the selected index from the ComboBox list                                             |
| `set_index(...)`         | Selects a new element from the ComboBox based on its index                                    |
| `add(...)`               | Adss a new string to the list of items in the ComboBox                                        |
| `add_item(...)`          | Adds a new item (value and descrition) to the list of items in the ComboBox                   |
| `clear()`                | Clears the list of items available in the ComboBox                                            |
| `selected_item(...)`     | Provides immutable (read-only) access to the selected item from the ComboBox                  |
| `selected_item_mut(...)` | Provides mutable access to the selected item from the ComboBox                                |
| `has_selection()`        | **True** if an item is selected, **False** otherwise                                          |
| `count()`                | Returns the number of items available in the combo box                                        |

**Remarks**: Methods `selected_item` and `seletec_item_mut` return an Option over the type `combobox::Item` that is defined as follows:

```rs
pub struct Item { ...}

impl Item {
    pub fn new(value: &str, description: &str) -> Self {...}
    pub fn from_string(value: String, description: String) -> Self {...}
    pub fn set_value(&mut self, value: &str) {...}
    pub fn value(&self) -> &str {...}
    pub fn set_description(&mut self, description: &str) {...}
    pub fn description(&self) -> &str {...}
}

```


## Key association

The following keys are processed by a `ComboBox` control if it has focus:

| Key                            | Purpose                                                                                                                                    |
| ------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------ |
| `Space` or `Enter`             | Expands or packs (collapses) the ComboBox control.                                                                                         |
| `Up`, `Down`, `Left`,  `Right` | Changes the current selected color from the ComboBox.                                                                                      |
| `PageUp`, `PageDown`           | Navigates through the list of variants page by page. If the control is not expanded, their behavior is similar to the keys `Up` and `Down` |
| `Home`                         | Move the selection to the first variant                                                                                                    |
| `End`                          | Move the selection to the last variant or to `None` if `AllowNoneVariant` flag was set upon initialization                                 |

Besides this using any one of the following keys: `A` to `Z` and/or `0` to `9` will move the selection to the fist variant that starts with that letter (case is ignored). The search starts from the next variant after the current one. This means that if you have multiple variants that starts with letter `G`, pressing `G` multiple times will efectively switch between all of the variants that starts with letter `G`.

When the combobox is expanded the following additional keys can be used:

| Key           | Purpose                                                                                                                                                                           |
| ------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Ctrl`+`Up`   | Scroll the view to top. If the new view can not show the current selection, move the selection to the previous value so that it would be visible                                  |
| `Ctrl`+`Down` | Scroll the view to bottom. If the new view can not show the current selection, move the selection to the next value so that it would be visible                                   |
| `Escape`      | Collapses the control. If the ComboBox is already colapsed, this key will not be captured (meaning that one of the ComboBox ancestors will be responsable with treating this key) |

## Example

The following example creates a Window with a ComboBox that was populated with various animals and their speed. Selecting one animal from the list changes the title of the window to the name of that animal.

```rs
use appcui::prelude::*;

#[Window(events = ComboBoxEvents)]
struct MyWin {}
impl MyWin {
    fn new() -> Self {
        let mut w = Self {
            base: window!("x:1,y:1,w:34,h:6,caption:Win"),
        };
        w.add(label!("'Select animal',x:1,y:1,w:30"));
        let mut c = ComboBox::new(Layout::new("x:1,y:2,w:30"), combobox::Flags::ShowDescription);
        // data from https://en.wikipedia.org/wiki/Fastest_animals
        c.add_item(combobox::Item::new("Cheetah","(120 km/h)"));
        c.add_item(combobox::Item::new("Swordfish","(97 km/h)"));
        c.add_item(combobox::Item::new("Iguana","(35 km/h)"));
        c.add_item(combobox::Item::new("Gazelle","(81 km/h)"));
        c.add_item(combobox::Item::new("Lion","(80 km/h)"));
        c.add_item(combobox::Item::new("Dog","(60 km/h)"));
        c.add_item(combobox::Item::new("Zebra","(56 km/h)"));
        w.add(c);
        w
    }
}
impl ComboBoxEvents for MyWin {
    fn on_selection_changed(&mut self, handle: Handle<ComboBox>) -> EventProcessStatus {
        let title = if let Some(cb) = self.control_mut(handle) {
            if let Some(item) = cb.selected_item() {
                item.value().to_string()
            } else {
                String::from("[None]")
            }
        } else {
            String::from("?")
        };
        self.set_title(&title);
        EventProcessStatus::Processed
    }
}


fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().build()?;
    a.add_window(MyWin::new());
    a.run();
    Ok(())
}
```