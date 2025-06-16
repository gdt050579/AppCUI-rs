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
        }
    }
    #[inline(always)]
    pub fn build(self) -> Result<App, Error> {
        App::create(self)
    }
    #[inline(always)]
    pub fn size(mut self, terminal_size: Size) -> Self {
        self.size = Some(terminal_size);
        self
    }
    #[inline(always)]
    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(String::from(title));
        self
    }
    #[inline(always)]
    pub fn menu_bar(mut self) -> Self {
        self.has_menu_bar = true;
        self
    }
    #[inline(always)]
    pub fn command_bar(mut self) -> Self {
        self.has_command_bar = true;
        self
    }
    #[inline(always)]
    pub fn single_window(mut self) -> Self {
        self.single_window = true;
        self
    }
    #[inline(always)]
    pub fn desktop<T>(mut self, desktop: T) -> Self
    where
        T: Control + DesktopControl + 'static,
    {
        self.desktop_manager = Some(ControlManager::new(desktop));
        self
    }
    #[inline(always)]
    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
    #[inline(always)]
    pub fn timers_count(mut self, count: u8) -> Self {
        self.max_timer_count = count.max(1); // at least one timer
        self
    }
    #[inline(always)]
    pub fn log_file(mut self, name: &str, append: bool) -> Self {
        self.log_file = Some(String::from(name));
        self.log_append = append;
        self
    }
}
