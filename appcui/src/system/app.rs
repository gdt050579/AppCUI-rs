use std::sync::Mutex;

use super::Error;
use super::InitializationData;
use super::InitializationFlags;
use super::RuntimeManager;
use crate::controls::events::Control;
use crate::graphics::Size;
use crate::terminals::TerminalType;

static APP_CREATED_MUTEX: Mutex<bool> = Mutex::new(false);

pub struct App {
    _phantom: (),
}

impl App {
    fn create(data: InitializationData) -> Result<Self, Error> {
        let mut app_created = APP_CREATED_MUTEX.lock().unwrap();
        if *app_created {
            return Err(Error::AppAlreadyStarted);
        }
        RuntimeManager::create(data)?;
        *app_created = true;
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
    pub fn debug(
        width: u16,
        height: u16,
        flags: InitializationFlags,
        script: &str,
    ) -> Result<Self, Error> {
        let i = InitializationData {
            flags,
            size: Some(Size {
                width: width as u32,
                height: height as u32,
            }),
            terminal: TerminalType::Debug,
            debug_script: String::from(script),
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
}
