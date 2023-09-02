use std::sync::Mutex;
use std::thread::Builder;

use super::Error;
use super::ErrorKind;
use super::Handle;
use super::HandleSupport;
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
    pub(super) fn create(builder: crate::system::Builder) -> Result<Self, Error> {
        let mut app_created = APP_CREATED_MUTEX.lock().unwrap();
        if *app_created {
            return Err(Error::new(
                ErrorKind::InitializationFailure,
                format!("App has already been created ! There can only be one instance of an Application at one time. If you have more, make sure that you have only one !"),
            ));
        }
        RuntimeManager::create(builder)?;
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
    pub fn debug(width: u16, height: u16, script: &str) -> crate::system::Builder {
        let mut builder = crate::system::Builder::new();
        builder.size = Some(Size::new(width as u32, height as u32));
        builder.debug_script = Some(String::from(script));
        builder
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
