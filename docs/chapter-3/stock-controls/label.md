# Label

Represent a label (a text):

<img src="img/label.png" width=300/>

To create a label use `Label::new` method (with 2 parameters: a caption and a layout).
```rs
let b = Label::new("My label", Layout::new("x:10,y:5,w:15"));
```
or the macro `label!`
```rs
let l1 = label!("caption='a caption for the label',x:10,y:5,w:15");
let l2 = label!("MyLabel,x:10,y:5,w:15");
```

The caption of a label may contain the special character `&` that indicates that the next character is a hot-key. 

By default, a label has a height of one character. If the height is specified in the layout, and it is bigger than 1, then you cam use a label object to print multi-line texts as well.

A label supports all common parameters (as they are described in [Instantiate via Macros](../instantiate_via_macros.md) section). Besides them, the following **named parameters** are also accepted:

| Parameter name | Type | Positional parameter  | Purpose |
|----------------|------|-----------------------|---------|
| `name` or `text` or `caption` | String | **Yes** (first postional parameter) | The caption (text) written on the label |



## Events
A label emits no events.

## Methods

Besides the [Common methods for all Controls](../common_methods.md) a button also has the following aditional methods:

| Method             | Purpose                                                                             |
|--------------------|-------------------------------------------------------------------------------------|
| `set_caption(...)` | Set the new caption for a button. If the string provided contains the special character `&`, this method also sets the hotkey associated with a control. If the string provided does not contain the `&` character, this method will clear the current hotkey (if any).<br>Example: `button.set_caption("&Start")` - this will set the caption of the button cu `Start` and the hotket to `Alt+S` |
| `get_caption()`    | Returns the current caption of a button |

## Key association

A label does not receive any input and as such it has no key associated with it.

## Example

The following code creates a window with a label that contains the text `Hello world !`.
```rs
use appcui::prelude::*;

fn main() -> Result<(), appcui::system::Error> {
    let mut app = App::new().build()?;
    let mut w = Window::new("Title", Layout::new("d:c,w:40,h:9"), window::Flags::None);
    w.add(Label::new("Hello world !", Layout::new("d:c,w:14")));
    a.add_window(w);
    a.run();
    Ok(())
}
```