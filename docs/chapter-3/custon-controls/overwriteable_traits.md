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