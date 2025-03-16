use crate::{
    system::{Handle, RuntimeManager},
    ui::Window,
};

use super::{
    task::{InnerTask, Task},
    BackgroundTaskConector,
};

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
    pub(crate) fn from_handle(handle: Handle<BackgroundTask<T, R>>) -> Option<BackgroundTask<T, R>> {
        let id = handle.index();
        let btm = RuntimeManager::get().get_background_task_manager();
        if let Some(t) = btm.get::<T, R>(id) {
            if t.validate(std::any::TypeId::of::<T>(), std::any::TypeId::of::<R>()) {
                Some(Self {
                    id: id as u32,
                    _phantom: std::marker::PhantomData,
                })
            } else {
                None
            }
        } else {
            None
        }
    }
    pub fn run(self, task: fn(conector: &BackgroundTaskConector<T, R>), receiver: Handle<Window>) -> Handle<BackgroundTask<T, R>> {
        // if it was already started
        if self.id != Self::INVALID {
            return Handle::None;
        }
        let btm = RuntimeManager::get().get_background_task_manager();
        let id = btm.add_task(InnerTask::<T, R>::new(receiver.cast()));
        if let Some(t) = btm.get_mut::<T, R>(id) {
            let h = Handle::new(id as u32);
            t.run(task, h);
            h
        } else {
            Handle::None
        }
    }
    pub fn read(&self) -> Option<T> {
        if self.id == Self::INVALID {
            return None;
        }
        let btm = RuntimeManager::get().get_background_task_manager();
        if let Some(t) = btm.get_mut::<T, R>(self.id as usize) {
            t.task_to_main.read()
        } else {
            None
        }
    }
}
