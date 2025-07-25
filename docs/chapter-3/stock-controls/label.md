# Label

Represent a label (a text):

<img src="img/label.png" width=300/>

To create a label use `Label::new` method (with 2 parameters: a caption and a layout).
```rs
let b = Label::new("My label", layout!("x:10,y:5,w:15"));
```
or the macro `label!`
```rs
let l1 = label!("caption='a caption for the label',x:10,y:5,w:15");
let l2 = label!("MyLabel,x:10,y:5,w:15");
```

The caption of a label may contain the special character `&` that indicates that the next character is a hot-key. However, as a label can not receive any input, the hotkey is meant to be for display only.

A label supports all common parameters (as they are described in [Instantiate via Macros](../instantiate_via_macros.md) section). Besides them, the following **named parameters** are also accepted:

| Parameter name                | Type   | Positional parameter                | Purpose                                 |
| ----------------------------- | ------ | ----------------------------------- | --------------------------------------- |
| `name` or `text` or `caption` | String | **Yes** (first postional parameter) | The caption (text) written on the label |



## Events
A label emits no events.

## Methods

Besides the [Common methods for all Controls](../common_methods.md) a label also has the following aditional methods:

| Method             | Purpose                                                                                                                                                                                                                                                                                                |
| ------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `set_caption(...)` | Set the new caption for a label. If the string provided contains the special character `&`, this method will highlight the next character just like a hotkey does. <br>Example: `label.set_caption("&Start")` - this will set the caption of the label to `Start` and highlight the first letter (`S`) |
| `caption()`        | Returns the current caption of a label                                                                                                                                                                                                                                                                 |

## Key association

A label does not receive any input and as such it has no key associated with it.

## Example

The following code creates a window with a label that contains the text `Hello world !`.
```rs
use appcui::prelude::*;

fn main() -> Result<(), appcui::system::Error> {
    let mut app = App::new().build()?;
    let mut w = Window::new("Title", layout!("a:c,w:40,h:9"), window::Flags::None);
    w.add(Label::new("Hello world !", layout!("a:c,w:14,h:1")));
    app.add_window(w);
    app.run();
    Ok(())
}
```