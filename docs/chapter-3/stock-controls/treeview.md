# TreeView

A TreeView is a templetize (generics based) control that allows you to view a list of objects as a tree.

<img src="img/treeview.png" width=400/>

It can be created using `TreeView::new(...)` and `TreeView::with_capacity(...)` methods or with the `treeview!` macro. 

```rs
let l1: TreeView<T> = TreeView::new(Layout::new("..."),treeview::Flags::None);
let l2: TreeView<T> = TreeView::with_capacity(10,Layout::new("..."),treeview::Flags::ScrollBars);
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
