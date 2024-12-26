# Themes

Themes are a way to change the look and feel of the application. A theme is a collection of colors, fonts, and other settings that are used to draw the controls. AppCUI comes with a set of predefined themes that can be used out of the box. You can also create your own themes by modifying the predefined ones or by creating a new one from scratch.

To create a new theme, you need to create a new instance of the `Theme` structure and set the desired colors, fonts, and other settings. You can then apply the theme to the application by using the `.theme(...)` method on the application builder.


```rust
let mut my_theme = Theme::new(Themes::Default);
// modify my_theme (colors, characters, etc)
let mut app = App::new().theme(my_theme).build().expect("Fail to create an AppCUI application");
// add aditional windows
app.run();
```

or by calling the `set_theme` method on the App structure later on during the execution.

```rust
let mut my_theme = Theme::new(Themes::Default);
// modify my_theme (colors, characters, etc)
App::set_theme(my_theme);
```

If not specified, the default theme (**Themes::Default**) will be used. 