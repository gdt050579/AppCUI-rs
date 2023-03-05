use super::RuntimeManager;
use super::Error;
use crate::controls::events::Control;


pub struct App {
    _phantom: (),
}

impl App {
    pub fn new() -> Result<Self,Error> {
        // before creating an App --> initialize Runtime Manager
        RuntimeManager::create();
        Ok(App { _phantom: () })
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
