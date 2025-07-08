use crate::graphics::*;
use crate::system::*;
use crate::ui::common::traits::*;
use crate::ui::common::*;
use crate::backend;
 
pub struct Builder {
    pub(crate) size: Option<Size>,
    pub(crate) backend: Option<backend::Type>,
    pub(crate) debug_script: Option<String>,
    pub(crate) title: Option<String>,
    pub(crate) desktop_manager: Option<ControlManager>,
    pub(crate) has_menu_bar: bool,
    pub(crate) has_command_bar: bool,
    pub(crate) single_window: bool,
    pub(crate) theme: Theme,
    pub(crate) max_timer_count: u8,
    pub(crate) log_file: Option<String>,
    pub(crate) log_append: bool,
    pub(crate) use_color_schema: bool,
}
impl Builder {
    pub(crate) fn new() -> Self {
        Self {
            size: None,
            title: None,
            backend: None,
            debug_script: None,
            desktop_manager: None,
            has_menu_bar: false,
            has_command_bar: false,
            single_window: false,
            max_timer_count: 4,
            theme: Theme::new(Themes::Default),
            log_file: None,
            log_append: false,
            use_color_schema: true,
        }
    }
    /// Builds the application using the current settings.
    #[inline(always)]
    pub fn build(self) -> Result<App, Error> {
        App::create(self)
    }
    /// Sets the size of the terminal.
    #[inline(always)]
    pub fn size(mut self, terminal_size: Size) -> Self {
        self.size = Some(terminal_size);
        self
    }
    /// Sets the title of the application.
    #[inline(always)]
    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(String::from(title));
        self
    }
    /// Enables the menu bar.
    #[inline(always)]
    pub fn menu_bar(mut self) -> Self {
        self.has_menu_bar = true;
        self
    }
    /// Enables the command bar.
    #[inline(always)]
    pub fn command_bar(mut self) -> Self {
        self.has_command_bar = true;
        self
    }
    /// Enables the single window mode.
    #[inline(always)]
    pub fn single_window(mut self) -> Self {
        self.single_window = true;
        self
    }
    /// Sets the desktop manager.
    #[inline(always)]
    pub fn desktop<T>(mut self, desktop: T) -> Self
    where
        T: Control + DesktopControl + 'static,
    {
        self.desktop_manager = Some(ControlManager::new(desktop));
        self
    }
    /// Sets the theme of the application. If not specified, the default theme will be used.
    #[inline(always)]
    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
    /// Sets the number of timers that can be used in the application.
    #[inline(always)]
    pub fn timers_count(mut self, count: u8) -> Self {
        self.max_timer_count = count.max(1); // at least one timer
        self
    }
    /// Sets the log file where logs will be displayed. This option is used only in debug mode.
    #[inline(always)]
    pub fn log_file(mut self, name: &str, append: bool) -> Self {
        self.log_file = Some(String::from(name));
        self.log_append = append;
        self
    }
    /// Enables or disables the use of the terminal color schema.
    #[inline(always)]
    pub fn color_schema(mut self, enabled: bool) -> Self {
        self.use_color_schema = enabled;
        self
    }
}
