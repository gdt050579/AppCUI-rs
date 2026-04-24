# Events

Normally, the theme of the application is set when the application is created. However, you can change the theme at any time while the application is running. While stock controls automatically update their appearance when the theme changes, custom controls may need to be notified about the change. This is particularly the case when a double buffer is used to draw the control (for example, when your control has an inner `Surface` that is updated by separate logic and is then used in `on_paint`).

In these scenarios, the control must implement the `OnThemeChanged` trait in order to receive a notification when the theme is updated.

```rust
impl OnThemeChanged for MyControl {
    fn on_theme_changed(&mut self, theme: &Theme) {
        // update your inner state using the current theme
    }
}
```

**Remarks:** The `on_theme_changed` method is called only when the theme is changed, and **only** after the application has been started (i.e., after the `run` method has been called). This means you will never be notified about the initial theme applied when the application is created.

## Example

Let's create a simple custom control that uses an attribute from the theme to draw itself. Such a control must be notified when the theme changes so it can update its fields.

```rust
#[CustomControl(overwrite : OnThemeChanged+OnPaint)]
struct MyControl {
    attr: CharAttribute,
}
impl MyControl {
    fn new() -> Self {
        let mut obj = Self {
            base: ControlBase::new(layout!("l:1,r:1,t:1,b:1"), true),
            attr: CharAttribute::default(),
        };
        // set up the attribute based on the current theme
        obj.attr = obj.theme().window.normal;
        obj
    }
}
impl OnPaint for MyControl {
    fn on_paint(&self, surface: &mut crate::prelude::Surface, _theme: &Theme) {
        // this is where we use the self.attr to draw something
        surface.clear(Character::with_attributes('X', self.attr));
    }
}
impl OnThemeChanged for MyControl {
    fn on_theme_changed(&mut self, theme: &Theme) {
        // this is where we update the attribute to match the new theme
        self.attr = theme.window.normal;
    }
}
```