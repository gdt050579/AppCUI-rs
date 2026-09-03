use crate::backend;
use crate::graphics::*;
use crate::system::*;
use crate::ui::common::traits::*;
use crate::ui::common::*;

pub struct InternalBuilder {
    pub(crate) size: Option<Size>,
    pub(crate) backend: Option<crate::backend::Type>,
    pub(crate) debug_script: Option<String>,
    pub(crate) title: Option<String>,
    pub(crate) desktop_manager: Option<ControlManager>,
    pub(crate) has_app_bar: bool,
    pub(crate) has_command_bar: bool,
    pub(crate) single_window: bool,
    pub(crate) theme: Theme,
    pub(crate) max_timer_count: u8,
    pub(crate) log_file: Option<String>,
    pub(crate) log_append: bool,
    pub(crate) use_color_schema: bool,
    pub(crate) restore_screen: bool,
    #[cfg(test)]
    pub(crate) runtime_check: Option<fn()>,
}
impl InternalBuilder {
    pub(crate) fn new() -> Self {
        Self {
            size: None,
            title: None,
            backend: None,
            debug_script: None,
            desktop_manager: None,
            has_app_bar: false,
            has_command_bar: false,
            single_window: false,
            max_timer_count: 4,
            theme: Theme::new(Themes::Default),
            log_file: None,
            log_append: false,
            use_color_schema: true,
            restore_screen: true,
            #[cfg(test)]
            runtime_check: None,
        }
    }
    #[inline(always)]
    pub(crate) fn build(self) -> Result<App, Error> {
        #[cfg(test)]
        {
            let check = self.runtime_check.clone();
            let app = App::create(self)?;
            if let Some(check) = check {
                check();
            }
            return Ok(app);
        }
        #[cfg(not(test))]
        {
            App::create(self)
        }
    }
    #[inline(always)]
    pub(crate) fn size(&mut self, terminal_size: Size) {
        self.size = Some(terminal_size);
    }
    #[inline(always)]
    pub(crate) fn title(&mut self, title: &str) {
        self.title = Some(String::from(title));
    }
    #[inline(always)]
    pub(crate) fn app_bar(&mut self) {
        self.has_app_bar = true;
    }
    #[inline(always)]
    pub(crate) fn command_bar(&mut self) {
        self.has_command_bar = true;
    }
    #[inline(always)]
    pub(crate) fn single_window(&mut self) {
        self.single_window = true;
    }
    #[inline(always)]
    pub(crate) fn desktop<T>(&mut self, desktop: T)
    where
        T: Control + DesktopControl + 'static,
    {
        self.desktop_manager = Some(ControlManager::new(desktop));
    }
    #[inline(always)]
    pub(crate) fn theme(&mut self, theme: Theme) {
        self.theme = theme;
    }
    #[inline(always)]
    pub(crate) fn timers_count(&mut self, count: u8) {
        self.max_timer_count = count.max(1);
    }
    #[inline(always)]
    pub(crate) fn log_file(&mut self, name: &str, append: bool) {
        self.log_file = Some(String::from(name));
        self.log_append = append;
    }
    #[inline(always)]
    pub(crate) fn color_schema(&mut self, enabled: bool) {
        self.use_color_schema = enabled;
    }
    #[inline(always)]
    pub(crate) fn restore_screen(&mut self, enable: bool) {
        self.restore_screen = enable;
    }
    #[inline(always)]
    pub(crate) fn backend(&mut self, backend: backend::Type) {
        self.backend = Some(backend);
    }
    #[inline(always)]
    pub(crate) fn debug_script(&mut self, script: &str) {
        self.debug_script = Some(String::from(script));
    }
    #[inline(always)]
    #[cfg(test)]
    pub(crate) fn runtime_check(&mut self, check: fn()) {
        self.runtime_check = Some(check);
    }
}

macro_rules! impl_terminal_builder_methods {
    () => {
        /// Sets the size of the terminal in character cells.
        ///
        /// If not specified, the backend uses the current terminal size (or a
        /// backend-specific default).
        ///
        /// # Parameters
        /// * `terminal_size` - Width and height of the terminal, in character cells.
        ///
        /// # Examples
        ///
        /// ```rust, no_run
        /// use appcui::prelude::*;
        ///
        /// App::new().size(Size::new(80, 25));
        /// ```
        #[inline(always)]
        pub fn size(mut self, terminal_size: crate::graphics::Size) -> Self {
            self.builder.size(terminal_size);
            self
        }

        /// Sets the title shown by the terminal window or browser tab.
        ///
        /// Support depends on the selected backend. If not specified, a backend
        /// default title is used.
        ///
        /// # Parameters
        /// * `title` - The application title.
        ///
        /// # Examples
        ///
        /// ```rust, no_run
        /// use appcui::prelude::*;
        ///
        /// App::new().title("My Application");
        /// ```
        #[inline(always)]
        pub fn title(mut self, title: &str) -> Self {
            self.builder.title(title);
            self
        }

        /// Sets the log file used when the crate is compiled in debug mode.
        ///
        /// This option has no effect in release builds.
        ///
        /// # Parameters
        /// * `name` - Path of the log file.
        /// * `append` - If `true`, new logs are appended; if `false`, the file is overwritten.
        ///
        /// # Examples
        ///
        /// ```rust, no_run
        /// use appcui::prelude::*;
        ///
        /// App::new().log_file("appcui.log", false);
        /// ```
        #[inline(always)]
        pub fn log_file(mut self, name: &str, append: bool) -> Self {
            self.builder.log_file(name, append);
            self
        }

        /// Enables or disables the terminal color schema.
        ///
        /// When enabled (the default), the backend may map AppCUI colors through
        /// the terminal's color schema. Disable this to keep the exact colors
        /// defined by the theme.
        ///
        /// # Parameters
        /// * `enabled` - `true` to use the terminal color schema, `false` to disable it.
        ///
        /// # Examples
        ///
        /// ```rust, no_run
        /// use appcui::prelude::*;
        ///
        /// App::new().color_schema(false);
        /// ```
        #[inline(always)]
        pub fn color_schema(mut self, enabled: bool) -> Self {
            self.builder.color_schema(enabled);
            self
        }

        /// Controls whether the original screen is restored when the application ends.
        ///
        /// When enabled (the default), the backend attempts to restore the original
        /// screen content and cursor position. When disabled, the screen is cleared
        /// on exit.
        ///
        /// # Parameters
        /// * `enable` - `true` to restore the original screen, `false` to clear it.
        ///
        /// # Remarks
        /// Not all backends can restore the original screen. Backends without this
        /// support always clear the screen when the application ends.
        ///
        /// # Examples
        ///
        /// ```rust, no_run
        /// use appcui::prelude::*;
        ///
        /// App::new().restore_screen(false);
        /// ```
        #[inline(always)]
        pub fn restore_screen(mut self, enable: bool) -> Self {
            self.builder.restore_screen(enable);
            self
        }

        /// Selects the terminal backend used to render the application.
        ///
        /// If not specified, AppCUI picks a backend appropriate for the current
        /// platform and enabled crate features.
        ///
        /// # Parameters
        /// * `backend` - The [`crate::backend::Type`] to use.
        ///
        /// # Examples
        ///
        /// ```rust, no_run
        /// use appcui::prelude::*;
        ///
        /// #[cfg(target_os = "windows")]
        /// let _builder = App::new().backend(appcui::backend::Type::WindowsVT);
        /// #[cfg(not(target_os = "windows"))]
        /// let _builder = App::new();
        /// ```
        #[inline(always)]
        pub fn backend(mut self, backend: crate::backend::Type) -> Self {
            self.builder.backend(backend);
            self
        }

        /// Configures a debug script that simulates input for unit tests.
        ///
        /// Each line of `script` is a command executed in order against a virtual
        /// terminal. After the last command the application ends.
        ///
        /// Combine this method with [`size`](Self::size) to set the simulated
        /// terminal dimensions (`width` and `height`).
        ///
        /// # Parameters
        /// * `script` - Commands to execute, one per line.
        ///
        /// # Debug commands
        ///
        /// **Mouse related commands**
        /// * `Mouse.Hold(x,y,button)` simulates an event where the mouse button is being pressed while the mouse is located at a specific position on screen. The parameters `x` and `y` are a screen position, while the parameter `button` is one of `left`, `right` or `center`
        /// * `Mouse.Release(x,y)` simulates the release of all mouse buttons while the mouse is located at a specific screen position.
        /// * `Mouse.Click(x,y,button)` simulates a click (hold an release)
        /// * `Mouse.Move(x,y)` simulates the movement of a mouse to coordonates (x,y). No mouse button are being pressed.
        /// * `Mouse.Drag(x1,y1,x2,y2)` simulates the movement of a mouse from (x1,y1) to (x2,y2) while the left button is being pressed
        /// * `Mouse.Wheel(x,y,direction,times)` simulates the wheel mouse being rotated into a direction (one of `top`, `bottom`, `left`, `right`) for a number of times. The `times` parameter must be biggen than 0.
        ///
        /// **Key related commands**
        /// * `Key.Pressed(key)` where key can be any combination of keys
        ///
        /// **Paint related commands**
        /// * `Paint(name)` paints the current virtual screen into the current screen using ANSI codes.
        /// * `Paint.Enable(value)` enables or disables painting. `value` is a boolean value (**true** or **false**). If set to **false** all subsequent calls to command `Paint` will be ignored.
        ///
        /// **System events**
        /// * `Resize(width,height)` simulates a resize of the virtual terminal to the size represented by `width` and `height` parameters
        ///
        /// **Validation commands**
        /// * `CheckHash(hash)` checks if the hash computer over the current virtual screen is as expected. If not it will panic. This is useful for unit testing.
        ///
        /// # Examples
        ///
        /// ```rust, no_run
        /// use appcui::prelude::*;
        ///
        /// let script = "
        ///     Paint(initial)
        ///     Key.Pressed(Escape)
        /// ";
        /// App::new()
        ///     .size(Size::new(60, 10))
        ///     .debug_script(script)
        ///     .window(|| window!("'Test',a:c,w:20,h:6"));
        /// ```
        #[inline(always)]
        pub fn debug_script(mut self, script: &str) -> Self {
            self.builder.debug_script(script);
            self
        }

        #[inline(always)]
        #[cfg(test)]
        pub(crate) fn runtime_check(mut self, check: fn()) -> Self {
            self.builder.runtime_check(check);
            self
        }
    };
}

macro_rules! impl_ui_builder_methods {
    () => {
        /// Enables the application bar at the top of the desktop.
        ///
        /// The app bar hosts menus and other application-wide commands. It is
        /// disabled by default.
        ///
        /// # Examples
        ///
        /// ```rust, no_run
        /// use appcui::prelude::*;
        ///
        /// App::new().app_bar();
        /// ```
        #[inline(always)]
        pub fn app_bar(mut self) -> Self {
            self.builder.app_bar();
            self
        }

        /// Enables the command bar at the bottom of the desktop.
        ///
        /// The command bar shows keyboard shortcuts associated with the focused
        /// control. It is disabled by default.
        ///
        /// # Examples
        ///
        /// ```rust, no_run
        /// use appcui::prelude::*;
        ///
        /// App::new().command_bar();
        /// ```
        #[inline(always)]
        pub fn command_bar(mut self) -> Self {
            self.builder.command_bar();
            self
        }

        /// Sets the theme used by the desktop and all controls.
        ///
        /// If not specified, the default theme is used. To change the theme after
        /// the application is running, call [`App::set_theme`](crate::system::App::set_theme).
        ///
        /// # Parameters
        /// * `theme` - The [`crate::system::Theme`] to apply at startup.
        ///
        /// # Examples
        ///
        /// ```rust, no_run
        /// use appcui::prelude::*;
        ///
        /// App::new().theme(Theme::new(Themes::DarkGray));
        /// ```
        #[inline(always)]
        pub fn theme(mut self, theme: crate::system::Theme) -> Self {
            self.builder.theme(theme);
            self
        }

        /// Sets the maximum number of timers the application can use.
        ///
        /// The value is clamped to at least `1`. The default is `4`.
        ///
        /// # Parameters
        /// * `count` - Maximum number of simultaneous timers.
        ///
        /// # Examples
        ///
        /// ```rust, no_run
        /// use appcui::prelude::*;
        ///
        /// App::new().timers_count(8);
        /// ```
        #[inline(always)]
        pub fn timers_count(mut self, count: u8) -> Self {
            self.builder.timers_count(count);
            self
        }

        /// Replaces the default desktop with a custom desktop control.
        ///
        /// Use this to implement your own desktop background, window management,
        /// or desktop-level event handling.
        ///
        /// # Parameters
        /// * `desktop` - A control that implements [`crate::ui::common::traits::DesktopControl`].
        ///
        /// # Type Constraints
        /// * `T` must implement [`crate::ui::common::traits::Control`] and
        ///   [`crate::ui::common::traits::DesktopControl`].
        ///
        /// # Examples
        ///
        /// ```rust, no_run
        /// use appcui::prelude::*;
        ///
        /// App::new().desktop(Desktop::new());
        /// ```
        #[inline(always)]
        pub fn desktop<T>(mut self, desktop: T) -> Self
        where
            T: crate::ui::common::traits::Control + crate::ui::common::traits::DesktopControl + 'static,
        {
            self.builder.desktop(desktop);
            self
        }
    };
}

pub(crate) use impl_terminal_builder_methods;
pub(crate) use impl_ui_builder_methods;
