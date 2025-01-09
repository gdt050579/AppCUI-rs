# Events

Normally, the theme of the application is set up when the application is created. However, you can change the theme at any time during the execution of the application. While, the stock controls will automatically update their appearance when the theme is changed, custom controls may need to be notified about the change. This is in particular the case when a double buffer is used to draw the control (e.g. you control has an inner `Surface` object that is updated based on a different logic and that will further be used in the `on_paint` method).

In these scenarios, the control must implement `OnThemeChanged`  trait in order to receive notification when the theme is updated.

```rust
impl OnThemeChanged for MyControl {
    fn on_theme_changed(&mut self, theme: &Theme) {
        // update your inner state using the current theme
    }
}
```

**Remarks**: The `on_theme_changed` method is called only when the theme is changed and **ONLY** after the Application has been started (i.e. after the `run` method has been called). This means that you will never get notified about the theme change for the initial theme set up when the application is created.

## Example

Let's create a simple custom control that uses an attribute from the theme to draw itself. Such a control will requre to be notified when the theme is changed in order to update its data members.

```rust
#[CustomControl(overwrite : OnThemeChanged+OnPaint)]
struct MyControl {
    attr: CharAttribute,
}
impl MyControl {
    fn new() -> Self {
        let mut obj = Self {
            base: ControlBase::new(Layout::new("l:1,r:1,t:1,b:1"), true),
            attr: CharAttribute::default(),
        };
        // we set up  the attribute based on the current theme
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