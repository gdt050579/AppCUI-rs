# DropDownList

A drop down list is a templetize (generics based) control that allows you to select a variant of a list of variants of type `T`. 

<img src="img/dropdownlist.png" width=300/>

It can be create using `DropDownList::new(...)` , `DropDownList::with_symbol(...)` or the `dropdownlist!` macro. Using `DropDownList::new(...)` and `DropDownList::with_symbol(...)` can be done in two ways:
1. by specifying the type for a variable:
    ```rs
    let s: DropDownList<T> = DropDownList::new(...);
    ```

2. by using turbo-fish notation (usually when you don't want to create a separate variable for the control):
    ```rs
    let s = DropDownList::<T>::with_symbol(...);
    ```
**Remarks**: It is important to notice that the `T` type must implement a special trait `DropDownListType` that is defined as follows:

```rs
pub trait DropDownListType {
    fn name(&self) -> &str;
    fn description(&self) -> &str {
        ""
    }
    fn symbol(&self) -> &str {
        ""
    }
}
```

where:
* `name()` is a method that provides a string representation (name) for a specific variant
* `description()` is a method that provides a detailed description for a specific variant
* `symbol()` is a method that returns a suggested symbol for a specific variant


## Examples

Assuming we have the following struct: `MyData` thet implements the required traits as follows:

```rs
struct MyData { ... }
impl DropDownListType for MyData { ... }
```

then we can create a dropdown list object based on this type as follows:

```rs
let d1: DropDownList<MyData> = DropDownList::new(layout!("..."),dropdownlist::Flags::None);

let d2: DropDownList<MyData> = DropDownList::with_symbol(1,layout!("..."),dropdownlist::Flags::AllowNoneSelection);

let d3 = dropdownlist!("class:MyData,x:1,y:1,w:20");

let d4 = dropdownlist!("class:MyData,x:1,y:1,w:20, flags: AllowNoneSelection, symbolsize:1");

let d5 = dropdownlist!("MyData,x:1,y:1,w:20");

```

A dropdown list supports all common parameters (as they are described in [Instantiate via Macros](../instantiate_via_macros.md) section). Besides them, the following **named parameters** are also accepted:

| Parameter name    | Type    | Positional parameter                | Purpose                                                                                                                                    |
| ----------------- | ------- | ----------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------ |
| `class` or `type` | String  | **Yes** (first postional parameter) | The name of a templetized type to be used when creating the dropdown list                                                                  |
| `flags`           | String  | **No**                              | DropDownList initialization flags                                                                                                          |
| `symbolsize`      | Numeric | **No**                              | The size (width) of the symbol in characters. It can be one of the following `0` (symbol will not be displayed), `1`, `2` or `3`           |
| `none`            | String  | **No**                              | The display name for the `None` variant that will be displayed in the dropdown list. If not specified, the `None` variant will not be used |

A dropdown list supports the following initialization flags:
* `dropdownlist::Flags::AllowNoneSelection` or `AllowNoneSelection` (for macro initialization) - thils will allow a dropdown list to hold a `None` value (meaning that the user can select no variant) 
* `dropdownlist::Flags::ShowDescription` or `ShowDescription` (for macro initialization) - this will show the description of the selected variant in the dropdown list  

## Events

To intercept events from a dropdown list, the following trait has to be implemented to the Window that processes the event loop:
```rs
pub trait DropDownListEvents<T> {
    fn on_selection_changed(&mut self, handle: Handle<DropDownList<T>>) -> EventProcessStatus {...}
}
```

## Methods

Besides the [Common methods for all Controls](../common_methods.md) a dropdown list also has the following aditional methods:

| Method                   | Purpose                                                                                                                  |
| ------------------------ | ------------------------------------------------------------------------------------------------------------------------ |
| `index()`                | Returns the selected index from the DropDownList list                                                                    |
| `set_index(...)`         | Selects a new element from the DropDownList based on its index                                                           |
| `add(...)`               | Adss a new string to the list of items in the DropDownList                                                               |
| `clear()`                | Clears the list of items available in the DropDownList                                                                   |
| `selected_item(...)`     | Provides immutable (read-only) access to the selected item from the DropDownList                                         |
| `selected_item_mut(...)` | Provides mutable access to the selected item from the DropDownList                                                       |
| `item(...)`              | Returns a immutable reference to the item at the specified index. If the index is invalid, the code will return **None** |
| `item_mut(...)`          | Returns a mutable reference to the item at the specified index. If the index is invalid, the code will return **None**   |
| `has_selection()`        | **True** if an item is selected, **False** otherwise                                                                     |
| `count()`                | Returns the number of items available in the combo box                                                                   |
| `set_none_string(...)`   | Sets the display name for the `None` variant that will be displayed in the dropdown list                                 |




## Key association

The following keys are processed by a `DropDownList` control if it has focus:

| Key                            | Purpose                                                                                                                                    |
| ------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------ |
| `Space` or `Enter`             | Expands or packs (collapses) the DropDownList control.                                                                                     |
| `Up`, `Down`, `Left`,  `Right` | Changes the current selected color from the DropDownList.                                                                                  |
| `PageUp`, `PageDown`           | Navigates through the list of variants page by page. If the control is not expanded, their behavior is similar to the keys `Up` and `Down` |
| `Home`                         | Move the selection to the first variant                                                                                                    |
| `End`                          | Move the selection to the last variant or to `None` if `AllowNoneSelection` flag was set upon initialization                               |

Besides this using any one of the following keys: `A` to `Z` and/or `0` to `9` will move the selection to the fist variant that starts with that letter (case is ignored). The search starts from the next variant after the current one. This means that if you have multiple variants that starts with letter `G`, pressing `G` multiple times will efectively switch between all of the variants that starts with letter `G`.

When the dropdown list is expanded the following additional keys can be used:

| Key           | Purpose                                                                                                                                                                                   |
| ------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Ctrl`+`Up`   | Scroll the view to top. If the new view can not show the current selection, move the selection to the previous value so that it would be visible                                          |
| `Ctrl`+`Down` | Scroll the view to bottom. If the new view can not show the current selection, move the selection to the next value so that it would be visible                                           |
| `Escape`      | Collapses the control. If the DropDownList is already colapsed, this key will not be captured (meaning that one of the DropDownList ancestors will be responsable with treating this key) |

## Example

The following example creates a Window with a DropDownList from where we can select a symbol: `♥` or `♠`.
```rs
use appcui::prelude::*;

struct MyObject {
    name: String,
    description: String,
    symbol: String,
}

impl MyObject {
    fn new(name: &str, description: &str, symbol: &str) -> MyObject {
        MyObject {
            name: name.to_string(),
            description: description.to_string(),
            symbol: symbol.to_string(),
        }
    }
}

impl DropDownListType for MyObject {
    fn name(&self) -> &str {
        &self.name
    }
    fn description(&self) -> &str {
        &self.description
    }
    fn symbol(&self) -> &str {
        &self.symbol
    }
}

fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().build()?;
    let mut w = window!("x:1,y:1,w:60,h:20,title:Win");
    let mut db = DropDownList::<MyObject>::with_symbol(1, layout!("x:1,y:1,w:56"), dropdownlist::Flags::ShowDescription);
    db.add(MyObject::new("Heart", "(symbol of love)", "♥"));
    db.add(MyObject::new("Spade", "(used in a deck of cards)", "♠"));
    w.add(db);
    a.add_window(w);
    a.run();
    Ok(())
}
```