use crate::backend;
use crate::graphics::*;
use crate::system::*;
use crate::ui::common::traits::*;
use crate::ui::common::*;

pub struct InternalBuilder {
    pub(crate) size: Option<Size>,
    pub(crate) backend: Option<backend::Type>,
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
    /// Builds the application using the current settings.
    #[inline(always)]
    pub(crate) fn build(self) -> Result<App, Error> {
        App::create(self)
    }
    /// Sets the size of the terminal.
    #[inline(always)]
    pub(crate) fn size(mut self, terminal_size: Size) -> Self {
        self.size = Some(terminal_size);
        self
    }
    /// Sets the title of the application.
    #[inline(always)]
    pub(crate) fn title(mut self, title: &str) -> Self {
        self.title = Some(String::from(title));
        self
    }
    /// Enables the Application bar.
    #[inline(always)]
    pub(crate) fn app_bar(mut self) -> Self {
        self.has_app_bar = true;
        self
    }
    /// Enables the command bar.
    #[inline(always)]
    pub(crate) fn command_bar(mut self) -> Self {
        self.has_command_bar = true;
        self
    }
    /// Enables the single window mode.
    #[inline(always)]
    pub(crate) fn single_window(mut self) -> Self {
        self.single_window = true;
        self
    }
    /// Sets the desktop manager.
    #[inline(always)]
    pub(crate) fn desktop<T>(mut self, desktop: T) -> Self
    where
        T: Control + DesktopControl + 'static,
    {
        self.desktop_manager = Some(ControlManager::new(desktop));
        self
    }
    /// Sets the theme of the application. If not specified, the default theme will be used.
    #[inline(always)]
    pub(crate) fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
    /// Sets the number of timers that can be used in the application.
    #[inline(always)]
    pub(crate) fn timers_count(mut self, count: u8) -> Self {
        self.max_timer_count = count.max(1); // at least one timer
        self
    }
    /// Sets the log file where logs will be displayed. This option is used only in debug mode.
    #[inline(always)]
    pub(crate) fn log_file(mut self, name: &str, append: bool) -> Self {
        self.log_file = Some(String::from(name));
        self.log_append = append;
        self
    }
    /// Enables or disables the use of the terminal color schema.
    #[inline(always)]
    pub(crate) fn color_schema(mut self, enabled: bool) -> Self {
        self.use_color_schema = enabled;
        self
    }

    /// If enabled the backend will attempt to restore the original screen content and cursor position when the application ends.
    /// If disabled, when the application ends the screen will be cleared.
    /// By default this option is set.
    /// 
    /// **Remarks:** Not all backends have the support to restore the original screen (for those that do not have this support, the screen will always be cleared when application ends).
    #[inline(always)]
    pub(crate) fn restore_screen(mut self, enable: bool) -> Self {
        self.restore_screen = enable;
        self
    }
}

/// Forwards all [`InternalBuilder`] methods onto a wrapper that stores
/// an `InternalBuilder` in a field named `builder`.
///
/// # Example
/// ```ignore
/// impl MultiWindowAppBuilder {
///     impl_internal_builder_methods!();
/// }
/// ```
macro_rules! impl_internal_builder_methods {
    () => {
        /// Builds the application using the current settings.
        #[inline(always)]
        pub fn build(self) -> Result<crate::system::App, crate::system::Error> {
            self.builder.build()
        }

        /// Sets the size of the terminal.
        #[inline(always)]
        pub fn size(mut self, terminal_size: crate::graphics::Size) -> Self {
            self.builder = self.builder.size(terminal_size);
            self
        }

        /// Sets the title of the application.
        #[inline(always)]
        pub fn title(mut self, title: &str) -> Self {
            self.builder = self.builder.title(title);
            self
        }

        /// Enables the Application bar.
        #[inline(always)]
        pub fn app_bar(mut self) -> Self {
            self.builder = self.builder.app_bar();
            self
        }

        /// Enables the command bar.
        #[inline(always)]
        pub fn command_bar(mut self) -> Self {
            self.builder = self.builder.command_bar();
            self
        }

        /// Enables the single window mode.
        #[inline(always)]
        pub fn single_window(mut self) -> Self {
            self.builder = self.builder.single_window();
            self
        }

        /// Sets the desktop manager.
        #[inline(always)]
        pub fn desktop<T>(mut self, desktop: T) -> Self
        where
            T: crate::ui::common::traits::Control + crate::ui::common::traits::DesktopControl + 'static,
        {
            self.builder = self.builder.desktop(desktop);
            self
        }

        /// Sets the theme of the application. If not specified, the default theme will be used.
        #[inline(always)]
        pub fn theme(mut self, theme: crate::system::Theme) -> Self {
            self.builder = self.builder.theme(theme);
            self
        }

        /// Sets the number of timers that can be used in the application.
        #[inline(always)]
        pub fn timers_count(mut self, count: u8) -> Self {
            self.builder = self.builder.timers_count(count);
            self
        }

        /// Sets the log file where logs will be displayed. This option is used only in debug mode.
        #[inline(always)]
        pub fn log_file(mut self, name: &str, append: bool) -> Self {
            self.builder = self.builder.log_file(name, append);
            self
        }

        /// Enables or disables the use of the terminal color schema.
        #[inline(always)]
        pub fn color_schema(mut self, enabled: bool) -> Self {
            self.builder = self.builder.color_schema(enabled);
            self
        }

        /// If enabled the backend will attempt to restore the original screen content and cursor position when the application ends.
        /// If disabled, when the application ends the screen will be cleared.
        /// By default this option is set.
        ///
        /// **Remarks:** Not all backends have the support to restore the original screen (for those that do not have this support, the screen will always be cleared when application ends).
        #[inline(always)]
        pub fn restore_screen(mut self, enable: bool) -> Self {
            self.builder = self.builder.restore_screen(enable);
            self
        }
    };
}

pub(crate) use impl_internal_builder_methods;

