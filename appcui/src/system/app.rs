use std::sync::Mutex;

use super::Error;
use super::InitializationData;
use super::InitializationFlags;
use super::RuntimeManager;
use crate::controls::events::Control;
use crate::controls::ControlManager;
use crate::controls::events::DesktopControl;
use crate::controls::events::WindowControl;
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
    pub fn debug<T>(
        width: u16,
        height: u16,
        flags: InitializationFlags,
        desktop: T,
        script: &str,
    ) -> Result<Self, Error>
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

    pub fn add_window<T>(&mut self, window: T)
    where
        T: Control + WindowControl + 'static,
    {
        RuntimeManager::get().add(window);
    }
}
