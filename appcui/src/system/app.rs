use super::InitializationData;
use super::InitializationFlags;
use super::RuntimeManager;
use super::Error;
use crate::controls::events::Control;
use crate::controls::menu::Menu;
use crate::graphics::Size;
use crate::terminal::TerminalType;
use crate::utils::Caption;


pub struct App {
    _phantom: (),
}

impl App {
    fn create(data: InitializationData)->Result<Self,Error> {
        RuntimeManager::create(data)?;
        Ok(App { _phantom: () })
    }
    pub fn new() -> Result<Self,Error> {
        App::create(InitializationData::new())
    }
    pub fn debug() -> Result<Self,Error> {
        let i = InitializationData {
            flags: InitializationFlags::None,
            size: Some(Size{width: 80, height: 25}),
            terminal: TerminalType::Debug
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
