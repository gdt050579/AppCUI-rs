# RadioBox

Represent a group of controls where only one can be selected at one point:

<img src="img/radiobox.png" width=300/>

To create a radiobox use `RadioBox::new` method (with 3 parameters: a caption, a layout and selected status (**true** or **false**)).
```rs
let b = RadioBox::new("A radiobox", Layout::new("x:10,y:5,w:15"),true);
```
or the macro `radiobox!`
```rs
let c1 = radiobox!("caption='Some option',x:10,y:5,w:15,h:1");
let c2 = radiobox!("'Another &option',x:10,y:5,w:15,h:1,checked:true");
let c3 = radiobox!("'&Multi-line option\nthis a hot-key',x:10,y:5,w:15,h:3,checked:false");
```

The caption of a radiobox may contain the special character `&` that indicates that the next character is a hot-key. For example, constructing a radiobox with the following caption `&Option number 1` will set up the text of the radiobox to `Option number 1` and will set up character `O` as the hot key for that radiobox (pressing `Alt+O` will be equivalent to changing the status for that radiobox from checked to unchecked or vice-versa).

A radiobox can contain a multi-line text but you will have to set the height parameter large enough to a larger value (bigger than 1).

A radiobox supports all common parameters (as they are described in [Instantiate via Macros](../instantiate_via_macros.md) section). Besides them, the following **named parameters** are also accepted:

| Parameter name         | Type   | Positional parameter                | Purpose                                           |
| ---------------------- | ------ | ----------------------------------- | ------------------------------------------------- |
| `text` or `caption`    | String | **Yes** (first postional parameter) | The caption (text) written on a radiobox          |
| `select` or `selected` | Bool   | **No**                              | Radiobox selection status: **true** for **false** |


Some examples that uses these paramateres:
```rs
let disabled_radiobox = radiobox!("caption=&Disabled,x:10,y:5,w:15,enable=false");
let hidden_radiobox = radiobox!("text='&Hidden',x=9,y:1,align:center,w:9,visible=false");
let multi_line_radiobox = radiobox!("'&Multi line\nLine2\nLine3',x:1,y:1,w:10,h:3");
```

## Events
To intercept events from a radiobox, the following trait has to be implemented to the Window that processes the event loop:
```rs
pub trait RadioBoxEvents {
    fn on_selected(&mut self, handle: Handle<RadioBox>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
```

## Methods

Besides the [Common methods for all Controls](../common_methods.md) a radiobox also has the following aditional methods:

| Method             | Purpose                                                                                                                                                                                                                                                                                                                                                                                                   |
| ------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `set_caption(...)` | Set the new caption for a radiobox. If the string provided contains the special character `&`, this method also sets the hotkey associated with a control. If the string provided does not contain the `&` character, this method will clear the current hotkey (if any).<br>Example: `radiobox.set_caption("&Option")` - this will set the caption of the radiobox cu `Option` and the hotkey to `Alt+O` |
| `caption()`        | Returns the current caption of a radiobox                                                                                                                                                                                                                                                                                                                                                                  |
| `is_selected()`    | **true** if the radiobox is selected, false otherwise                                                                                                                                                                                                                                                                                                                                                     |
| `set_checked()`    | Sets the new checked status for the radiobox                                                                                                                                                                                                                                                                                                                                                              |

## Key association

The following keys are processed by a Radiobox control if it has focus:

| Key                | Purpose                                                                                                                                                                                                          |
| ------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Space` or `Enter` | Selects a new radiobox. It also emits  `RadioBoxEvents::on_selected(...)` event with the `handle` parameter the current radiobox that was selected. It has the same action clicking the radiobox with the mouse. |

Aditionally, `Alt`+**letter or number** will have the same action (even if the radiobox does not have a focus) if that letter or nunber was set as a hot-key for a radiobox via its caption. 

## Grouping

Implicetelly, **all** radiboxes withing a control (that have the same parent) are considered as part of one group. This means that when you select one radiobox, all other radioboxes from the same group will be unselected.

To create multiple groups, one need to create panels and add radioboxes as their children, like in the following example:
```rs
// group 1
let mut panel_1 = Panel::new(...);
panel_1.add(RadioBox::new(...));
panel_1.add(RadioBox::new(...));
panel_1.add(RadioBox::new(...));

// group 2
let mut panel_2 = Panel::new(...);
panel_2.add(RadioBox::new(...));
panel_2.add(RadioBox::new(...));
panel_2.add(RadioBox::new(...));
```

## Example

The following code creates a window with two groups (panels), each group containing 3 radioboxes. When a radiobox is selected, its content will display on a label.
```rs
use appcui::prelude::*;

#[Window(events = RadioBoxEvents)]
struct MyWin {
    g1_r1: Handle<RadioBox>,
    g1_r2: Handle<RadioBox>,
    g1_r3: Handle<RadioBox>,
    g2_r1: Handle<RadioBox>,
    g2_r2: Handle<RadioBox>,
    g2_r3: Handle<RadioBox>,
    l: Handle<Label>,
}

impl MyWin {
    fn new() -> Self {
        let mut win = MyWin {
            base: window!("'My Win',d:c,w:60,h:14"),
            g1_r1: Handle::None,
            g1_r2: Handle::None,
            g1_r3: Handle::None,
            g2_r1: Handle::None,
            g2_r2: Handle::None,
            g2_r3: Handle::None,
            l: Handle::None,
        };
        win.l = win.add(label!("'<no status>',l:1,r:1,t:1"));
        let mut group_1 = panel!("'Group 1',x:1,y:3,w:26,h:7");
        win.g1_r1 = group_1.add(radiobox!("Meters,x:1,y:1,w:20,select:true"));
        win.g1_r2 = group_1.add(radiobox!("Centimeters,x:1,y:2,w:20"));
        win.g1_r3 = group_1.add(radiobox!("Kilometers,x:1,y:3,w:20"));
        
        let mut group_2 = panel!("'Group 2',x:30,y:3,w:26,h:7");
        win.g2_r1 = group_2.add(radiobox!("Red,x:1,y:1,w:20,select:true"));
        win.g2_r2 = group_2.add(radiobox!("Green,x:1,y:2,w:20"));
        win.g2_r3 = group_2.add(radiobox!("Blue,x:1,y:3,w:20"));

        win.add(group_1);
        win.add(group_2);
        win
    }
}

impl RadioBoxEvents for MyWin {
    fn on_selected(&mut self, handle: Handle<RadioBox>) -> EventProcessStatus {
        let mut s = String::new();
        if let Some(r) = self.control(handle) {
            s += r.caption();
        }
        if s.len()>0 {
            let h = self.l;
            if let Some(l) = self.control_mut(h) {
                l.set_caption(&s);
            }
        }
        EventProcessStatus::Ignored
    }
}

fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().build()?;
    a.add_window(MyWin::new());
    a.run();
    Ok(())
}
```