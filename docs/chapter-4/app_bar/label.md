# Label

A label is a text that is displayed in the app bar.

<img src="img/label.png" width=400/>

To create a label, use `appbar::Label::new(...)` method.

```rs
let label = appbar::Label::new("Label", 0, appbar::Side::Left);
```

## Events

There are no events associated with a label.

## Methods

The following methods are available for a label:

| Method             | Purpose                                                                              |
| ------------------ | ------------------------------------------------------------------------------------ |
| `set_caption(...)` | Set the new caption for a label. Currently the caption is limited to 128 characters. |
| `caption()`        | Returns the current caption of a label.                                              |
| `set_tooltip(...)` | Set the tooltip for a label.                                                         |
| `tooltip()`        | Returns the current tooltip of a label.                                              |

## Example

The following example creates a label with the caption `Label` and the tooltip `This is a label`.

```rs
use appcui::prelude::*;

#[Window(events = AppBarEvents)]
pub(crate) struct Win {
    h_label: Handle<appbar::Label>,
}
impl Win {
    pub(crate) fn new() -> Self {
        let mut w = Win {
            base: window!("'Label',a:c,w:40,h:8,Flags: Sizeable"),
            h_label: Handle::None,
        };

        let mut label = appbar::Label::new(" Label ", 0, appbar::Side::Left);
        label.set_tooltip("This is a label");
        w.h_label = w.appbar().add(label);
        
        w
    }
}
impl AppBarEvents for Win {
    fn on_update(&self, appbar: &mut AppBar) {
        appbar.show(self.h_label);
    }
}

fn main() -> Result<(), appcui::system::Error> {
    let mut app = App::new().app_bar().build()?;
    app.add_window(Win::new());
    app.run();
    Ok(())
}
```