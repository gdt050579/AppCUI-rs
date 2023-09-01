use std::sync::Mutex;
use std::thread::Builder;

use super::Error;
use super::Handle;
use super::InitializationData;
use super::InitializationFlags;
use super::RuntimeManager;
use crate::graphics::Size;
use crate::terminals::TerminalType;
use crate::ui::common::traits::*;
use crate::ui::common::*;

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
    pub fn new() -> crate::system::Builder {
        crate::system::Builder::new()
    }
    pub fn with_terminal(terminal: TerminalType) -> crate::system::Builder {
        let mut builder = crate::system::Builder::new();
        builder.terminal = Some(terminal);
        builder
    }
    pub fn default() -> Result<Self, Error> {
        App::create(InitializationData::default())
    }
    pub fn debug<T>(width: u16, height: u16, flags: InitializationFlags, desktop: T, script: &str) -> Result<Self, Error>
    where
        T: Control + DesktopControl + 'static,
    {
        let i = InitializationData {
            flags,
            size: Some(Size {
                width: width as u32,
                height: height as u32,
            }),
            desktop_manager: ControlManager::new(desktop),
            terminal: TerminalType::Debug,
            debug_script: String::from(script),
        };
        App::create(i)
    }
    pub fn run(self) {
        // must pe self so that after a run a second call will not be possible
        RuntimeManager::get().run();
        // clear the mutex so that other apps can be created after this step
        RuntimeManager::destroy();
        let mut app_created = APP_CREATED_MUTEX.lock().unwrap();
        *app_created = false;
    }

    pub fn add_window<T>(&mut self, window: T) -> Handle<T>
    where
        T: Control + WindowControl + 'static,
    {
        return RuntimeManager::get().add_window(window);
    }
}
