use super::RuntimeManager;
use crate::controls::events::Control;


pub struct App {
    _phantom: (),
}

impl App {
    pub fn new() -> Self {
        // before creating an App --> initialize Runtime Manager
        RuntimeManager::create();
        App { _phantom: () }
    }
    pub fn run(mut self) {
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
