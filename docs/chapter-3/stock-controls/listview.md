# ListView

A ListView is a templetize (generics based) control that allows you to view a list of objects.

<img src="img/listview.png" width=300/>

It can be created using `ListView::new(...)` and `ListView::with_capacity(...)` methods or with the `listview!` macro. 

```rs
let l1: T = ListView::new(Layout::new("..."),listview::Flags::None);
let l2: T = ListView::with_capacity(10,Layout::new("..."),listview::Flags::ScrollBars);
let l3 = listview!("class: T, flags: Scrollbar, d:c, w:100%, h:100%");
let l4 = listview!("type: T, flags: Scrollbar, d:c, view:Columns(3)");
let l5 = listview!("T, d:c, view:Details, columns:[{Name,10,left},{Age,5,right},{City,20,center}]");
```

where type `T` is the type of the elements that are shown in the list view and has to implement `listview::ListItem` trait.

A listview supports all common parameters (as they are described in [Instantiate via Macros](../instantiate_via_macros.md) section). Besides them, the following **named parameters** are also accepted:

| Parameter name                | Type    | Positional parameter                | Purpose                                                                                                                                                                                                                  |
| ----------------------------- | ------- | ----------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `class` or `type`             | String  | **Yes**, first positional parameter | The type of items that are being displayed in the ListView control.                                                                                                                                                      |
| `flags`                       | String  | **No**                              | ListView initialization flags                                                                                                                                                                                            |
| `lsm` or `left-scroll-margin` | Numeric | **No**                              | The left margin of the bottom scroll bar in characters. If not provided the default value is 0. This should be a positive number and it only has an effect if the flag `ScrollBars` or `SearchBar` flags were specified. |
| `tsm` or `top-scroll-margin`  | Numeric | **No**                              | The top margin of the right scroll bar in characters. If not provided the default value is 0. This should be a positive number and it only has an effect if the flag `ScrollBars` flags was used to create the control.  |
| `view` or `viewmode` or `vm`  | String  | **No**                              | The view mode of the ListView control (`Details` or `Columns`).                                                                                                                                                          |
| `columns`                     | List    | **No**                              | The list of columns for the the ListView control.                                                                                                                                                                        |

The field `columns` is a list of columns that are displayed in the ListView control. Each column is a tuple with three elements: the name of the column, the width of the column in characters, and the alignment of the column (`left`, `right`, or `center`). The column field accespts the following parameters:

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

Similary, to create a listview with 3 columns (Name, Age, and City) with the widths of 10, 5, and 20 characters, respectively, and aligned to the left, right, and center, you can use the following format:

```rs
let l = listview!("T, d:c, view:Details, columns:[{Name,10,left},{Age,5,right},{City,20,center}]");
```

A listview supports the following initialization flags:
* `listview::Flags::ScrollBars` or `ScrollBars` (for macro initialization) - this enables a set of scrollbars that can be used to navigate through the list of items. The scrollbars are visible only when the control has focus
* `listview::Flags::SearchBar` or `SearchBar` (for macro initialization) - this enables a search bar that can be used to filter the list of items. The search bar is visible only when the control has focus
* `listview::Flags::CheckBoxes` or `CheckBoxes` (for macro initialization) - this enable a set of checkboxes that can be used to select multiple items from the list.
* `listview::Flags::ShowGroups` or `ShowGroups` (for macro initialization) - this enables the grouping of items in the list view. 
* `listview::Flags::SmallIcons` or `SmallIcons` (for macro initialization) - this enables the small icons (one character) view mode for the list view.
* `listview::Flags::LargeIcons` or `LargeIcons` (for macro initialization) - this enables the large icons (two characters or unicode surrogates) view mode for the list view.
* `listview::Flags::CustomFilter` or `CustomFilter` (for macro initialization) - this enables the custom filter that can be used to filter the list of items. The custom filter should be provided by the user in the `listview::ListItem` implementation.


## Events

To intercept events from a listview, the following trait has to be implemented to the Window that processes the event loop:

```rs
pub trait ListViewEvents<T: listview::ListItem + 'static> {
    // called when the current item is changed
    fn on_current_item_changed(&mut self, handle: Handle<ListView<T>>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    
    // called when a group (if groups are enabled) is collapsed
    fn on_group_collapsed(&mut self, handle: Handle<ListView<T>>, group: listview::Group) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }

    // called when a group (if groups are enabled) is expanded
    fn on_group_expanded(&mut self, handle: Handle<ListView<T>>, group: listview::Group) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }

    // called when the selection is changed
    fn on_selection_changed(&mut self, handle: Handle<ListView<T>>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }

    // called when you double click on an item (or press Enter)
    fn on_item_action(&mut self, handle: Handle<ListView<T>>, item_index: usize) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
```
