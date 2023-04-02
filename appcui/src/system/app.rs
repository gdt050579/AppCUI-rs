use super::Error;
use super::InitializationData;
use super::InitializationFlags;
use super::RuntimeManager;
use crate::controls::events::Control;
use crate::controls::menu::Menu;
use crate::graphics::Size;
use crate::terminal::TerminalType;
use crate::utils::Caption;

pub struct App {
    _phantom: (),
}

impl App {
    fn create(data: InitializationData) -> Result<Self, Error> {
        RuntimeManager::create(data)?;
        Ok(App { _phantom: () })
    }
    pub fn new(
        terminal: TerminalType,
        size: Option<Size>,
        flags: InitializationFlags,
    ) -> Result<Self, Error> {
        App::create(InitializationData::new(terminal, size, flags))
    }
    pub fn default() -> Result<Self, Error> {
        App::create(InitializationData::default())
    }
    pub fn debug(width: u16, height: u16) -> Result<Self, Error> {
        let i = InitializationData {
            flags: InitializationFlags::None,
            size: Some(Size {
                width: width as u32,
                height: height as u32,
            }),
            terminal: TerminalType::Debug,
        };
        App::create(i)
    }
    pub fn run(self) {
        // must pe self so that after a run a second call will not be possible
        RuntimeManager::get().run();
    }

    pub fn add<T>(&mut self, window: T)
    where
        T: Control + 'static,
    {
        RuntimeManager::get().add(window);
    }
    pub fn add_menu(&mut self, menu: Menu, name: &str) {
        RuntimeManager::get().add_menu(menu, Caption::new(name, true));
    }
}
