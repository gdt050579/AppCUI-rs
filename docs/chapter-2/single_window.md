# Single Window Mode

A single window mode is ...

## Aditional constructor methods

Besides the methods described in [Builder](application.md#builder) the following methods are available for a multi-window mode setup:
* `.desktop(custom_desktop)` if you want to use a custom desktop instead of the default one (not available in single-window mode)
* `.app_bar()` to enable the application top app bar
* `.command_bar()` to enable the application command bar
* `.theme(custom_theme)` to set up a custom theme or another predefined theme. Read more on themes in the [Themes](chapter-6/themes.md) section.
* `.timers_count(count)` to set up the number of timers that can be used in the application (if not specified the default value is 4)