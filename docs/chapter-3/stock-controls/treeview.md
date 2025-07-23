# TreeView

A TreeView is a templetize (generics based) control that allows you to view a list of objects as a tree.

<img src="img/treeview.png" width=400/>

It can be created using `TreeView::new(...)` and `TreeView::with_capacity(...)` methods or with the `treeview!` macro. 

```rs
let l1: TreeView<T> = TreeView::new(layout!("..."),treeview::Flags::None);
let l2: TreeView<T> = TreeView::with_capacity(10,layout!("..."),treeview::Flags::ScrollBars);
let l3 = treeview!("class: T, flags: Scrollbar, d:c, w:100%, h:100%");
let l4 = treeview!("type: T, flags: Scrollbar, d:c, view:Columns(3)");
let l5 = treeview!("T, d:c, columns:[{Name,10,left},{Age,5,right},{City,20,center}]");
```

where type `T` is the type of the elements that are shown in the tree view and has to implement [ListItem](../object-traits/listitem.md) trait.

A treeview supports all common parameters (as they are described in [Instantiate via Macros](../instantiate_via_macros.md) section). Besides them, the following **named parameters** are also accepted:

| Parameter name                | Type    | Positional parameter                | Purpose                                                                                                                                                                                                                  |
| ----------------------------- | ------- | ----------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `class` or `type`             | String  | **Yes**, first positional parameter | The type of items that are being displayed in the TreeView control.                                                                                                                                                      |
| `flags`                       | String  | **No**                              | TreeView initialization flags                                                                                                                                                                                            |
| `lsm` or `left-scroll-margin` | Numeric | **No**                              | The left margin of the bottom scroll bar in characters. If not provided the default value is 0. This should be a positive number and it only has an effect if the flag `ScrollBars` or `SearchBar` flags were specified. |
| `tsm` or `top-scroll-margin`  | Numeric | **No**                              | The top margin of the right scroll bar in characters. If not provided the default value is 0. This should be a positive number and it only has an effect if the flag `ScrollBars` flags was used to create the control.  |
| `columns`                     | List    | **No**                              | The list of columns for the the TreeView control.                                                                                                                                                                        |

The field `columns` is a list of columns that are displayed in the TreeView control. Each column is a tuple with three elements: the name of the column, the width of the column in characters, and the alignment of the column (`left`, `right`, or `center`). The column field accespts the following parameters:

| Parameter name                | Type    | Positional parameter                 | Purpose                                                                                                                                                                                |
| ----------------------------- | ------- | ------------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `caption` or `name` or `text` | String  | **Yes**, first positional parameter  | The name of the column. If a character in the name is precedeed by the `&` character, that column will have a hot key associated that will allow clicking on a column via `Ctrl`+<key> |
| `width` or `w`                | Numeric | **Yes**, second positional parameter | The width of the column in characters.                                                                                                                                                 |
| `align` or `alignment` or `a` | String  | **Yes**, third positional parameter  | The alignment of the column (`left` or `l`, `right` or `r`, and `center` or `c`).                                                                                                      |

To create a column with the name `Test`, that hast `Ctrl+E` assigned as a hot key, with a width of 10 characters, and aligned to the right, you can use the following formats:
* `{caption: "T&est", width: 10, align: right}`
* `{name: "T&est", w: 10, a: right}`
* `{T&est, 10, right}`
* `{T&est,10,r}`

Similary, to create a treeview with 3 columns (Name, Age, and City) with the widths of 10, 5, and 20 characters, respectively, and aligned to the left, right, and center, you can use the following format:

```rs
let l = treeview!("T, d:c, columns:[{Name,10,left},{Age,5,right},{City,20,center}]");
```

A treeview supports the following initialization flags:
* `treeview::Flags::ScrollBars` or `ScrollBars` (for macro initialization) - this enables a set of scrollbars that can be used to navigate through the list of items. The scrollbars are visible only when the control has focus
* `treeview::Flags::SearchBar` or `SearchBar` (for macro initialization) - this enables a search bar that can be used to filter the list of items. The search bar is visible only when the control has focus
* `treeview::Flags::SmallIcons` or `SmallIcons` (for macro initialization) - this enables the small icons (one character) view mode for the tree view.
* `treeview::Flags::LargeIcons` or `LargeIcons` (for macro initialization) - this enables the large icons (two characters or unicode surrogates) view mode for the tree view.
* `treeview::Flags::CustomFilter` or `CustomFilter` (for macro initialization) - this enables the custom filter that can be used to filter the list of items. The custom filter should be provided by the user in the [ListItem](../object-traits/listitem.md) implementation.
* `treeview::Flags::NoSelection` or `NoSelection` (for macro initialization) - this disables the selection of items from the tree view. This flag is useful when the tree view is used only for displaying information and the selection is not needed (such as a Save or Open file dialog).
* `treeview::Flags::HideHeader` or `HideHeader` (for macro initialization) - this hides the header of the tree view. This flag is useful when the tree view is used only for displaying information and the header is not needed.

## Events

To intercept events from a treeview, the following trait has to be implemented to the Window that processes the event loop:

```rs
pub trait TreeViewEvents<T: ListItem + 'static> {
    // called when the current item is changed
    fn on_current_item_changed(&mut self, 
                               handle: Handle<TreeView<T>>, 
                               item:   Handle<treeview::Item<T>>) -> EventProcessStatus 
    {
        EventProcessStatus::Ignored
    }
    
    // called whenever an item was collapes.
    // the recursive parameter indicates that all children and their children 
    // were collapesed as well
    fn on_item_collapsed(&mut self, 
                         handle:    Handle<TreeView<T>>, 
                         item:      Handle<treeview::Item<T>>, 
                         recursive: bool) -> EventProcessStatus 
    {
        EventProcessStatus::Ignored
    }

    // called whenever an item was expanded.
    // the recursive parameter indicates that all children and their children 
    // were expanded as well
    fn on_item_expanded(&mut self, 
                        handle:    Handle<TreeView<T>>, 
                        item:      Handle<treeview::Item<T>>, 
                        recursive: bool) -> EventProcessStatus 
    {
        EventProcessStatus::Ignored
    }


    // called when the selection is changed
    fn on_selection_changed(&mut self, handle: Handle<TreeView<T>>) -> EventProcessStatus 
    {
        EventProcessStatus::Ignored
    }

    // called when you double click on an item (or press Enter)
    fn on_item_action(&mut self, 
                      handle: Handle<TreeView<T>>, 
                      item:   Handle<treeview::Item<T>>) -> EventProcessStatus 
    {
        EventProcessStatus::Ignored
    }
}
```

## Methods

Besides the [Common methods for all Controls](../common_methods.md) a tree view also has the following aditional methods:

### Adding items

| Method                    | Purpose                                                                                                                                                                                                                                                |
| ------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `add_column(...)`         | Adds a new column to the TreeView control. This method is in particular usefull when you need to create a custom treeview.                                                                                                                             |
| `add(...)`                | Adds a new item to the root of the TreeView control.                                                                                                                                                                                                   |
| `add_to_parent(...)`      | Adds a new item as a child for another item that exists in TreeView control.                                                                                                                                                                           |
| `add_item(...)`           | Adds a new item to the root of the TreeView control. This methods allows you to specify the color, icon and selection state for that item.                                                                                                             |
| `add_item_to_parent(...)` | Adds a new item as a child for another item that exists in TreeView control. This methods allows you to specify the color, icon and selection state for that item.                                                                                     |
| `add_batch(...)`          | Adds multiple items to the treeview. When an item is added to a treeview, it is imediatly filtered based on the current search text. If you want to add multiple items (using various methods) and then filter them, you can use the add_batch method. |
| `items_count()`           | Returns the number of items in the treeview.                                                                                                                                                                                                           |


### Deleting items

| Method                      | Purpose                                                                                    |
| --------------------------- | ------------------------------------------------------------------------------------------ |
| `delete_item(...)`          | Deletes an item from the treeview. If the item has children, they will be deleted as well. |
| `delete_item_children(...)` | Deletes all children of an item from the treeview.                                         |
| `clear()`                   | Clears all items from the treeview                                                         |


### Item access

| Method                  | Purpose                                                                                                                |
| ----------------------- | ---------------------------------------------------------------------------------------------------------------------- |
| `current_item_handle()` | Returns a handle to the current item or None if the treeview is empty                                                  |
| `current_item()`        | Returns a immutable reference to the current item or None if the treeview is empty                                     |
| `current_item_mut()`    | Returns a mutable reference to the current item or None if the treeview is empty                                       |
| `item(...)`             | Returns an immutable reference to an item based on its handle or None if the handle is invalid (e.g. item was deleted) |
| `item_mut(...)`         | Returns a mutable reference to an item based on its handle or None if the handle is invalid (e.g. item was deleted)    |
| `root_items()`          | Returns a list of handles to the root items in the treeview.                                                           |
| `root_item_mut(...)`    | Returns a mutable reference to the root item based on its index or None if the index is invalid                        |
| `root_item(...)`        | Returns an immutable reference to the root item based on its index or None if the index is invalid                     |


### Selection & Folding

| Method                   | Purpose                                                                                                                                        |
| ------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------- |
| `select_item(...)`       | Selects or deselects an item based on its handle.                                                                                              |
| `selected_items_count()` | Returns the number of selected items in the treeview.                                                                                          |
| `collapse_item(...)`     | Collapses an item based on its handle. This methods takes a `recursive` parameter that if **true** will also collapse all of the item children |
| `expand_item(...)`       | Expands an item based on its handle. This methods takes a `recursive` parameter that if **true** will also expand all of the item children     |
| `collapse_all()`         | Collapses all items in the treeview.                                                                                                           |
| `expand_all()`           | Expands all items in the treeview.                                                                                                             |


### Miscellaneous

| Method                    | Purpose                                                                                                                         |
| ------------------------- | ------------------------------------------------------------------------------------------------------------------------------- |
| `set_frozen_columns(...)` | Sets the number of frozen columns. Frozen columns are columns that are not scrolled when the treeview is scrolled horizontally. |
| `sort(...)`               | Sorts the items in the TreeView control based on a column index.                                                                |
| `clear_search()`          | Clears the content of the search box of the treeview.                                                                           |
| `move_cursor_to(...)`     | Moves the cursor to a specific item in the treeview.                                                                            |

## Key association

The following keys are processed by a `TreeView` control if it has focus:

| Key                                                        | Purpose                                                                                                                                                                                                                                         |
| ---------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Up`, `Down`                                               | Changes the current item from the TreeView.                                                                                                                                                                                                     |
| `Left`, `Right`                                            | Scrolls the view to the left or to the right                                                                                                                                                                                                    |
| `PageUp`, `PageDown`                                       | Navigates through the list of items page by page.                                                                                                                                                                                               |
| `Home`                                                     | Moves the current item to the first element in the tree view                                                                                                                                                                                    |
| `End`                                                      | Moves the current item to the last element in the tree view                                                                                                                                                                                     |
| `Shift`+{`Up`, `Down`,`PageUp`, `PageDown`, `Home`, `End`} | Selects multiple items in the tree view.                                                                                                                                                                                                        |
| `Insert`                                                   | Toggle the selection state of the current item. Once the selection is toggled, the cursor will me moved to the next item in the tree view.                                                                                                      |
| `Space`                                                    | Folds or un-foldes an item in the tree view                                                                                                                                                                                                     |
| `Ctrl`+`Alt`+{`Up`, `Down`}                                | Moves the scroll up or down                                                                                                                                                                                                                     |
| `Enter`                                                    | Triggers the `TreeViewEvents::on_item_action` event for the current item                                                                                                                                                                        |
| `Ctrl`+{`A`..`Z`, `0`..`9`}                                | If a column has a hot key associated (by using the `&` character in the column name), this will sort all items bsed on that column. If that column is already selected, this will reverse the order of the sort items (ascendent or descendent) |
| `Ctrl`+{`Left`, `Right`}                                   | Enter in the column resize mode.                                                                                                                                                                                                                |

Aditionally, typing any character will trigger the search bar (if the flag `SearchBar` is present) and will filter the items based on the search text. While the search bar is active, the following keys are processed:
* `Backspace` - removes the last character from the search text
* `Escape` - clears the search text and closes the search bar
* `Enter` - moves to the next match
* Movement keys (such as `Up`, `Down`, `Left`, `Right`, `PageUp`, `PageDown`, `Home`, `End`) - will disable the search bar, but will keep the search text

While in the column resize mode, the following keys are processed:
* `Left`, `Right` - increases or decreases the width of the current column
* `Ctrl`+`Left`, `Ctrl`+`Right` - moves the focus to the previous or next column
* `Escape` or movement keys - exits the column resize mode

## Populating a tree view

To add items to a tree view, you can use the `add` and `add_to_parent` methods. The `add` method adds an item to the root of the tree view, while the `add_to_parent` method adds an item as a child to another item. Both of them return a handle to the newly added item.

The following example shows how to add items to a tree view:

```rs
let mut treeview = TreeView::new(layout!("d:f"),treeview::Flags::ScrollBars);
// add two items to the root of the tree view
let handle_item_1 = treeview.add(...);
let handle_item_2 = treeview.add(...);
// add a child item to the first item
let handle_item_3 = treeview.add_to_parent(...,handle_item_1);
// add a child to the child of the first item
let handle_item_4 = treeview.add_to_parent(...,handle_item_3);
```

Whenever an element is being added to a TreeView, the TreeView will try to filter and sort the item based on its content. These operations are expensive so if you need to add multiple items to a TreeView, you can use the `add_batch` method. This method will add all items to the TreeView and will filter and sort the items only once, after all items were added.

To add an item to a tree view, the item type has to implement the [ListItem](../object-traits/listitem.md) trait. Based on the implementation of this trait, the TreeView will:
* display an item based on a specification
* filter the item based on the search text or a specific filtering algorithm
* sort the items based on a column index
* get a list of columns and their specifications (name, width, alignment)


# Example

The following example shows how to create a tree view with a custom item type that implements the `ListItem` trait:

```rs
use appcui::prelude::*;

#[derive(ListItem)]
struct MyItem {
    #[Column(name="Text", width=100)]
    text: String,
}
impl MyItem {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
        }
    }
}

fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().build()?;
    let mut w = window!("Tree,a:c");
    let mut tv = treeview!("MyItem,a:c,flags: ScrollBars+SearchBar+HideHeader");
    let h1 = tv.add(MyItem::new("Root Item 1"));    
    let h2 = tv.add(MyItem::new("Root Item 2"));
    let h1_1 = tv.add_to_parent(MyItem::new("First Child of Root Item 1"), h1);
    let h1_2 = tv.add_to_parent(MyItem::new("Second Child of Root Item 1"), h1);
    let h1_3 = tv.add_to_parent(MyItem::new("Third Child of Root Item 1"), h1);
    let h1_1_1 = tv.add_to_parent(MyItem::new("First Child of First Child of Root Item 1"), h1_1);
    let h2_1 = tv.add_to_parent(MyItem::new("First Child of Root Item 1"), h2);
    let h2_2 = tv.add_to_parent(MyItem::new("Second Child of Root Item 1"), h2);

    w.add(tv);
    a.add_window(w);
    a.run();
    Ok(())
}
```