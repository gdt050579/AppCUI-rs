# Overwriteable traits

The following traits can be overwritten in a custom control:
* OnPaint
* OnResize
* OnFocus
* OnExpand
* OnDefaultAction
* OnKeyPressed
* OnMouseEvent

## OnPaint

**OnPaint** trait methods are called whenever a control is being painted:

```rs
pub trait OnPaint {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {

    }
}
```

The `surface` object will be clipped to the visible space ocupied by the control and the coordonates will be translated to corespond to the top-left corner of the control(this means that `surface.write_char(0,0,...)` will draw a character to the top-left corner of the control).

## OnResize

**OnResize** trait methods are called whenever the control is being resized:

```rs
pub trait OnResize {
    fn on_resize(&mut self, old_size: Size, new_size: Size) {

    }
}
```

if `old_size` parameter has the size of **(0x0)** then this is the first time this method is being called.

## OnFocus

**OnFocus** trait methods are called whenever the control either receives the focus or it loses it:

```rs
pub trait OnFocus {
    fn on_focus(&mut self) {}
    fn on_lose_focus(&mut self) {}
}
```

## OnExpand

**OnExpand** methods are called whenever a control is being expanded or packed. An expanded control is a control that increases its size when it has the focus amd packs back to its original size when the control loses its focus. One such example is the [ColorPicker](../../chapter-3/stock-controls/colorpicker.md).

```rs
pub trait OnExpand {
    fn on_expand(&mut self, direction: ExpandedDirection) { }
    fn on_pack(&mut self) { }
}
```

## OnDefaultAction

**OnDefaultAction** methods a default action is balled for a control. The default action is different from one control to another (for example in case of a button - the default action is similar to clicking the button, for a checkbox is similar to checking or unchecking the control, etc).

This method is also associated with the control hot key. Assuming we have a hot key associated with a control, pressing that hot key is equivalent to:
1. changing the focus of the control (if it does not have the focus)
2. calling `OnDefaultAction::on_default_action()` for that control.
   
For example, if a button has a hot key, pressinhg that hot-key is similar to clicking the button.

```rs
pub trait OnDefaultAction {
    fn on_default_action(&mut self) {        
    }
}
```

## OnKeyPressed

**OnKeyPressed** methods are called whenever a key is pressed. The control must have the focus at that point.

```rs
pub trait OnKeyPressed {
    fn on_key_pressed(&mut self, key: Key, character: char) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
```

if `OnKeyPressed::on_key_pressed(...)` returns **EventProcessStatus::Ignored** the key is being send to the parent of the current control. If the method returns **EventProcessStatus::Processed** the cotrol will ne redrawn and the event will not be passed to its parent anymore.

The following custom control uses arrow keys to move a rectangle within the control:

```rs
use appcui::prelude::*;

#[CustomControl(overwrite = OnPaint+OnKeyPressed)]
struct MyControl {
    p: Point,
}
impl MyControl {
    fn new(layout: Layout) -> Self {
        Self {
            base: ControlBase::new(layout, true),
            p: Point::ORIGIN,
        }
    }
}

impl OnPaint for MyControl {
    fn on_paint(&self, surface: &mut Surface, _theme: &Theme) {
        surface.clear(char!("' ',black,black"));
        surface.draw_rect(
            Rect::with_point_and_size(self.p, Size::new(2, 2)),
            LineType::Double,
            CharAttribute::with_fore_color(Color::White),
        );
    }
}
impl OnKeyPressed for MyControl {
    fn on_key_pressed(&mut self, key: Key, _character: char) -> EventProcessStatus {
        match key.value() {
            key!("Left")  => { self.p.x -= 1; EventProcessStatus::Processed }
            key!("Right") => { self.p.x += 1; EventProcessStatus::Processed }
            key!("Up")    => { self.p.y -= 1; EventProcessStatus::Processed }
            key!("Down")  => { self.p.y += 1; EventProcessStatus::Processed }
            _             => EventProcessStatus::Ignored
        }        
    }
}

fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().build()?;
    let mut w = window!("caption:'Custom Control',a:c,w:30,h:10");
    w.add(MyControl::new(Layout::new("l:1,t:1,r:1,b:1")));
    a.add_window(w);
    a.run();
    Ok(())
}
```

## OnMouseEvent

**OnMouseEvent** trait methods can be use to react to mouse events such as clicks, drag, wheel movement, etc.

```rs
pub trait OnMouseEvent {
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
```

if `OnMouseEvent::on_mouse_event(...)` returns **EventProcessStatus::Processed** the control is going to be repainted, otherwise nothing happens.

A tipical implementation for this trait looks like the following one:

```rs
impl OnMouseEvent for /* control name */ {
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        match event {
            MouseEvent::Enter => todo!(),
            MouseEvent::Leave => todo!(),
            MouseEvent::Over(_) => todo!(),
            MouseEvent::Pressed(_) => todo!(),
            MouseEvent::Released(_) => todo!(),
            MouseEvent::DoubleClick(_) => todo!(),
            MouseEvent::Drag(_) => todo!(),
            MouseEvent::Wheel(_) => todo!(),
        }
    }
}
```

The following example intercepts the mouse movement while the mouse is over the control and prints it.

```rs
use std::fmt::Write;
use appcui::prelude::*;

#[CustomControl(overwrite = OnPaint+OnMouseEvent)]
struct MyControl {
    text: String
}
impl MyControl {
    fn new(layout: Layout) -> Self {
        Self {
            base: ControlBase::new(layout, true),
            text: String::new()
        }
    }
}

impl OnPaint for MyControl {
    fn on_paint(&self, surface: &mut Surface, _theme: &Theme) {
        surface.clear(char!("' ',black,black"));
        surface.write_string(0, 0, &self.text, CharAttribute::with_fore_color(Color::White), false);
    }
}

impl OnMouseEvent for MyControl {
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        match event {
            MouseEvent::Enter | MouseEvent::Leave => EventProcessStatus::Processed,
            MouseEvent::Over(data) => {
                self.text.clear();
                write!(&mut self.text,"Mouse at: ({}x{})", data.x, data.y).unwrap();
                EventProcessStatus::Processed
            },
            _ => EventProcessStatus::Ignored
        }
    }
}

fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().build()?;
    let mut w = window!("caption:'Custom Control',a:c,w:30,h:10");
    w.add(MyControl::new(Layout::new("l:1,t:1,r:1,b:1")));
    a.add_window(w);
    a.run();
    Ok(())
}
```