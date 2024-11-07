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

## Methods

Besides the [Common methods for all Controls](../common_methods.md) a button also has the following aditional methods:

| Method              | Purpose                                                                                                                                                                                                                                                |
| ------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `add_group(...)`    | Creates a new group with a specified name and return a group identifier. You can further used the group identified to add an item to a group.                                                                                                          |
| `add_column(...)`   | Adds a new column to the ListView control. This method is in particular usefull when you need to create a custom listview.                                                                                                                             |
| `add(...)`          | Adds a new item to the ListView control.                                                                                                                                                                                                               |
| `add_item(...)`     | Adds a new item to the ListView control. This methods allows you to specify the color, icon, group and selection state for that item.                                                                                                                  |
| `add_items(...)`    | Adds a vector of items to the ListView control.                                                                                                                                                                                                        |
| `add_to_group(...)` | Adds a vector if items to the ListView control and associate all of them to a group                                                                                                                                                                    |
| `add_batch(...)`    | Adds multiple items to the listview. When an item is added to a listview, it is imediatly filtered based on the current search text. If you want to add multiple items (using various methods) and then filter them, you can use the add_batch method. |


## Key association

The following keys are processed by a `ListView` control if it has focus:

| Key                  | Purpose                                                                                                                        |
| -------------------- | ------------------------------------------------------------------------------------------------------------------------------ |
| `Up`, `Down`         | Changes the current item from the ListBox.                                                                                     |
| `Left`, `Right`      | Scrolls the view to the left or to the right (when the view is `Details` or changes the current item if the view is `Columns`) |
| `PageUp`, `PageDown` | Navigates through the list of items page by page.                                                                              |
| `Home`               | Moves the current item to the first element in the list                                                                        |
| `End`                | Moves the current item to the last element in the list                                                                         |



## List view items
- cum se implementeaza ListItem
- rendering methods
- macro listview!
- custom filtering
  
## Populating a litsview
- cum se adauga items eficient intr-un listview (diferente dintre diverse metode)

## View modes
- poze cu diferite view modes

## Example

The following example shows how to create a listview with a custom item type and how to add items to it.
The item used in the example is a `DownloadItem` that has the following fields:
* `name` - the name of the item
* `age` - the age of the item
* `server` - the server from which the item is downloaded
* `stars` - the rating of the item
* `download` - the download status of the item
* `created` - the creation date of the item
* `enabled` - a flag that indicates if the item is enabled or not
  

```rs
use appcui::prelude::*;

#[derive(ListViewItem)]
struct DownloadItem {
    #[Column(name: "&Name", width: 12, align: Left)]
    name: &'static str,
    #[Column(name: "&Age", width: 10, align: Center)]
    age: u32,
    #[Column(name: "&Server")]
    server: &'static str,
    #[Column(name: "&Stars", width: 10, align: Center, render: Rating, format:Stars)]
    stars: u8,
    #[Column(name: "Download", width:15)]
    download: listview::Status,
    #[Column(name: "Created", w: 20, align: Center, render: DateTime, format: Short)]
    created: chrono::NaiveDateTime,
    #[Column(name: "Enabled", align: Center)]
    enabled: bool,
}

fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().build()?;
    let mut w = window!("Download,d:c,w:100%,h:100%,flags: Sizeable");
    let mut l = listview!("DownloadItem,d:c,view:Details,flags: ScrollBars+CheckBoxes");
    l.add(DownloadItem {
        name: "music.mp3",
        age: 21,
        server: "London",
        stars: 4,
        download: listview::Status::Running(0.5),
        created: chrono::NaiveDate::from_ymd_opt(2016, 7, 8).unwrap().and_hms_opt(9, 10, 11).unwrap(),
        enabled: true,
    });
    l.add(DownloadItem {
        name: "picture.png",
        age: 30,
        server: "Bucharest",
        stars: 3,
        download: listview::Status::Paused(0.25),
        created: chrono::NaiveDate::from_ymd_opt(2016, 7, 8).unwrap().and_hms_opt(9, 10, 11).unwrap(),
        enabled: false,
    });
    l.add(DownloadItem {
        name: "game.exe",
        age: 40,
        server: "Bucharest",
        stars: 5,
        download: listview::Status::Completed,
        created: chrono::NaiveDate::from_ymd_opt(2016, 7, 8).unwrap().and_hms_opt(9, 10, 11).unwrap(),
        enabled: true,
    });
    w.add(l);
    a.add_window(w);
    a.run();
    Ok(())
}
```