use crate::graphics::*;
use crate::system::*;
use crate::terminals::*;
use crate::ui::common::traits::*;
use crate::ui::common::*;

pub struct Builder {
    pub(crate) size: Option<Size>,
    pub(crate) terminal: Option<TerminalType>,
    pub(crate) debug_script: Option<String>,
    pub(crate) title: Option<String>,
    pub(crate) desktop_manager: Option<ControlManager>,
    pub(crate) has_menu: bool,
    pub(crate) has_command_bar: bool,
}
impl Builder {
    pub(crate) fn new() -> Self {
        Self {
            size: None,
            title: None,
            terminal: None,
            debug_script: None,
            desktop_manager: None,
            has_menu: false,
            has_command_bar: false,
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
    pub fn menu(mut self) -> Self {
        self.has_menu = true;
        self
    }
    #[inline(always)]
    pub fn command_bar(mut self) -> Self {
        self.has_command_bar = true;
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
}
