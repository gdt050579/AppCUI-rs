use crate::system::RuntimeManager;

use super::{task::InnerTask, BackgroundTaskConector};


pub struct BackgroundTask<T: Send+'static, R: Send+'static> {
    id: u32,
    _phantom: std::marker::PhantomData<(T, R)>,
}

impl<T: Send, R: Send> BackgroundTask<T, R> {
    pub fn new() -> BackgroundTask<T, R> {
        let task = InnerTask::<T, R>::new();
        // add task to runtime manager
        BackgroundTask {
            id: 0,
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn run(self, task: fn(conector: &BackgroundTaskConector<T, R>)) {
        let m = RuntimeManager::get().get_background_task_manager();
        if let Some(task) = m.get(self.id as usize) {
            
        }
    }
}
