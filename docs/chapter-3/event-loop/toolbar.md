# Toolbar

A toolbar is a generic concept of an area over the margins of a window (top or bottom margins) where several controls (or toolbar items) reside. A toolbar items are simplier controls that convey their events directly to the window via a spcial trait: `ToolBarEvents`. 

All toolbar items are organized in groups. A group is similar to a flow (all toolbar items are aligned one after another, depending on the direction of the flow). It is important to notice that a toolbar item does not have a layout of itself (its position is decided by the group). The following direction are supported by a toolbar group (via enum `toolbar::GroupPosition`):

```rs
pub enum GroupPosition {
    TopLeft,
    BottomLeft,
    TopRight,
    BottomRight,
}
```

## Constructing a toolbar item

To add a toolbar item into a window, you need to perform the following steps:
1. create a group
2. create a toolbar item
3. add the previously created toolbar item to the group created on step 1.

Typically, a toolbar initialization mechanims is done when creating a window, in a similar manner as with the next snippet:
```rs
#[Window(...)]
struct MyWin {
    // data members
}
impl MyWin {
    fn new() -> Self {
        let mut me = Self {
            base: window!("..."),
        };
        let a_group = me.get_toolbar().create_group(toolbar::GroupPosition::<Value>);
        let item_handle = me.get_toolbar().add(a_group, toolbar::<Type>::new("..."));
        // other initializations
        me
    }
}
```

To create a toolbar item, there are two options:
* use `toolbar::<Item>::new(...)` method
* use `toolbaritem!` macro

Curenly AppCUI supports the following toolbar item types:
* [Label](toolbar-items/label.md)
* [CheckBox](toolbar-items/checkbox.md)
* [SingleChoice](toolbar-items/singlechoice.md)
* [Button](toolbar-items/button.md)