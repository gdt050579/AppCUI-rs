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

