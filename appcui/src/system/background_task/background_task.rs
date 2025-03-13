use crate::{system::{Handle, RuntimeManager}, ui::Window};

use super::{task::InnerTask, BackgroundTaskConector};

pub struct BackgroundTask<T: Send + 'static, R: Send + 'static> {
    id: u32,
    _phantom: std::marker::PhantomData<(T, R)>,
}

impl<T: Send, R: Send> BackgroundTask<T, R> {
    const INVALID: u32 = u32::MAX;
    pub fn new() -> BackgroundTask<T, R> {
        // add task to runtime manager
        BackgroundTask {
            id: Self::INVALID,
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn run(self, task: fn(conector: &BackgroundTaskConector<T, R>), receiver: Handle<Window>) -> Handle<BackgroundTask<T, R>> {
        if self.id == Self::INVALID {
            return Handle::None;
        }
        let btm = RuntimeManager::get().get_background_task_manager();
        let id = btm.add_task(InnerTask::<T, R>::new(receiver.cast()));
        if let Some(t) = btm.get_mut::<T, R>(id) {
            t.run(task, id as u32);
            Handle::new(id as u32)
        } else {
            Handle::None
        }
    }
}
