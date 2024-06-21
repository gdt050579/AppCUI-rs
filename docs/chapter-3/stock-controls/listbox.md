# ListBox

A listbox is a control that displays a list of items. 

<img src="img/listbox.png" width=300/>

It can be created using `ListBox::new(...)` and `ListBox::with_capacity(...)` methods or with the `listbox!` macro. 

```rs
let l1 = ListBox::new(Layout::new("..."),listbox::Flags::None);
let l2 = ListBox::with_capacity(10,Layout::new("..."),listbox::Flags::ScrollBars);
let l3 = listbox!("x:1,y:1,w:20,h:10,items=['Red','Greem','Blue']");
let l4 = listbox!("x:1,y:1,w:20,h:10,items=['Red','Greem','Blue'],index:2");
let l5 = listbox!("x:1,y:1,w:20,h:10,items=['Red','Greem','Blue'],index:2, flags: ScrollBars+SearchBar");
```

A listbox supports all common parameters (as they are described in [Instantiate via Macros](../instantiate_via_macros.md) section). Besides them, the following **named parameters** are also accepted:

| Parameter name                | Type    | Positional parameter | Purpose                                                                                                                                                                                                                  |
| ----------------------------- | ------- | -------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `flags`                       | String  | **No**               | Listbox initialization flags                                                                                                                                                                                             |
| `items`                       | List    | **No**               | A list of string items to populate the listbox with.                                                                                                                                                                     |
| `index` or `selected_index`   | Numeric | **No**               | The index of the selected item (it should be a value between 0 and number if items - 1)                                                                                                                                  |
| `lsm` or `left-scroll-margin` | Numeric | **No**               | The left margin of the bottom scroll bar in characters. If not provided the default value is 0. This should be a positive number and it only has an effect if the flag `ScrollBars` or `SearchBar` flags were specified. |
| `tsm` or `top-scroll-margin`  | Numeric | **No**               | The top margin of the right scroll bar in characters. If not provided the default value is 0. This should be a positive number and it only has an effect if the flag `ScrollBars` flags was used to create the control.  |


A listbox supports the following initialization flags:
* `listbox::Flags::ScrollBars` or `ScrollBars` (for macro initialization) - this enables a set of scrollbars that can be used to navigate through the list of items. The scrollbars are visible only when the control has focus
* `listbox::Flags::SearchBar` or `SearchBar` (for macro initialization) - this enables a search bar that can be used to filter the list of items. The search bar is visible only when the control has focus
* `listbox::Flags::CheckBoxes` or `CheckBoxes` (for macro initialization) - this enable a set of checkboxes that can be used to select multiple items from the list.
* `listbox::Flags::AutoScroll` or `AutoScroll` (for macro initialization) - this will automatically scroll the listbox to the last item whenever a new item is being added. This flag is usefull for scenarios where the listbox is used as a log/event viewer. 

## Events

To intercept events from a listbox, the following trait has to be implemented to the Window that processes the event loop:

```rs
pub trait ListBoxEvents {
    fn on_current_item_changed(&mut self, handle: Handle<ListBox>, index: usize) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    fn on_item_checked(&mut self, handle: Handle<ListBox>, index: usize, checked: bool) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
```

## Methods

Besides the [Common methods for all Controls](../common_methods.md) a listbox also has the following aditional methods:

| Method            | Purpose                                                                                                                                                    |
| ----------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `add(...)`        | Adds a new string to the list of items in the ListBox                                                                                                      |
| `add_item(...)`   | Adds a new item (text and check status) to the list of items in the ListBox                                                                                |
| `clear()`         | Clears the list of items available in the ListBox                                                                                                          |
| `index()`         | Returns the selected index from the ListBox list                                                                                                           |
| `item(...)`       | Returns the item from a specified index from the ListBox. If the index is invalid, `None` will be returned                                                 |
| `set_index(...)`  | Selects a new element from the ListBox based on its index                                                                                                  |
| `count()`         | Returns the number of items from the ListBox                                                                                                               |
| `count_checked()` | Returns the number of checked items from the ListBox. This method will always return `0` if the flags `CheckBoxes` was **NOT** set when creating a ListBox |


An item from the ListBox is represented by the following structure:

```rs
pub struct Item { ...}

impl Item {
    pub fn new(text: &str, checked: bool) -> Self {...}
    pub fn text(&self) -> &str {...}
    pub fn is_checked(&self) -> bool {...}
}

```


## Key association

The following keys are processed by a `ListBox` control if it has focus:

| Key                            | Purpose                                                                                                                                    |
| ------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------ |
| `Space` or `Enter`             | Expands or packs (collapses) the ListBox control.                                                                                          |
| `Up`, `Down`, `Left`,  `Right` | Changes the current selected color from the ListBox.                                                                                       |
| `PageUp`, `PageDown`           | Navigates through the list of variants page by page. If the control is not expanded, their behavior is similar to the keys `Up` and `Down` |
| `Home`                         | Move the selection to the first variant                                                                                                    |
| `End`                          | Move the selection to the last variant or to `None` if `AllowNoneVariant` flag was set upon initialization                                 |
| `Ctrl`+`Up`   | Scroll the view to top. If the new view can not show the current selection, move the selection to the previous value so that it would be visible                                |
| `Ctrl`+`Down` | Scroll the view to bottom. If the new view can not show the current selection, move the selection to the next value so that it would be visible                                 |
| `Escape`      | Collapses the control. If the ListBox is already colapsed, this key will not be captured (meaning that one of the ListBox ancestors will be responsable with treating this key) |

Besides this using any one of the following keys ascii keys will start a search in the listbox


## Example

The following example creates a Window with a ListBox that was populated with various animals and their speed. Selecting one animal from the list changes the title of the window to the name of that animal.

```rs
use appcui::prelude::*;


fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().build()?;
    a.add_window(MyWin::new());
    a.run();
    Ok(())
}
```