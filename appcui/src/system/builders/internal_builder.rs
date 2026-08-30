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
        }
    }
    #[inline(always)]
    pub(crate) fn build(self) -> Result<App, Error> {
        App::create(self)
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
}

macro_rules! impl_internal_builder_methods {
    () => {
        /// Sets the size of the terminal.
        #[inline(always)]
        pub fn size(mut self, terminal_size: crate::graphics::Size) -> Self {
            self.builder.size(terminal_size);
            self
        }

        /// Sets the title of the application.
        #[inline(always)]
        pub fn title(mut self, title: &str) -> Self {
            self.builder.title(title);
            self
        }

        /// Sets the log file where logs will be displayed. This option is used only in debug mode.
        #[inline(always)]
        pub fn log_file(mut self, name: &str, append: bool) -> Self {
            self.builder.log_file(name, append);
            self
        }

        /// Enables or disables the use of the terminal color schema.
        #[inline(always)]
        pub fn color_schema(mut self, enabled: bool) -> Self {
            self.builder.color_schema(enabled);
            self
        }

        /// If enabled the backend will attempt to restore the original screen content and cursor position when the application ends.
        /// If disabled, when the application ends the screen will be cleared.
        /// By default this option is set.
        ///
        /// **Remarks:** Not all backends have the support to restore the original screen (for those that do not have this support, the screen will always be cleared when application ends).
        #[inline(always)]
        pub fn restore_screen(mut self, enable: bool) -> Self {
            self.builder.restore_screen(enable);
            self
        }

        /// Sets the backend to use.
        #[inline(always)]
        pub fn backend(mut self, backend: crate::backend::Type) -> Self {
            self.builder.backend(backend);
            self
        }

        /// Enables the single window mode.
        #[inline(always)]
        pub fn single_window(mut self) -> Self {
            self.builder.single_window();
            self
        }

        /// Builds the application using the current settings.
        #[inline(always)]
        pub fn build(self) -> Result<crate::system::App, crate::system::Error> {
            self.builder.build()
        }

        /// Sets the debug script to use.
        /// Creates a builder designed for unit testing.
        /// The provided parameters indicated:
        /// * `width` and `height` : the size of the simulated terminal
        /// * `script` : a script with multiple commands (one command per line) that will be executed one after another simulating events that could be send to the AppCUI. Once all commands are being executed, the application will end.
        ///
        /// ## Debug commands
        /// The following list of commands are supported for the script:
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
        #[inline(always)]
        pub fn debug_script(mut self, script: &str) -> Self {
            self.builder.debug_script(script);
            self
        }
    };
}

pub(crate) use impl_internal_builder_methods;
