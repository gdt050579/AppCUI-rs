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
    pub fn run(task: fn(conector: &BackgroundTaskConector<T, R>), receiver: Handle<Window>) -> Handle<BackgroundTask<T, R>> {
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
        let btm = RuntimeManager::get().get_background_task_manager();
        if let Some(t) = btm.get_mut::<T, R>(self.id as usize) {
            t.task_to_main.read()
        } else {
            None
        }
    }
    pub fn pause(&self) {
        let btm = RuntimeManager::get().get_background_task_manager();
        if let Some(t) = btm.get_mut::<T, R>(self.id as usize) {
            t.pause();
        }
    }
    pub fn resume(&self) {
        let btm = RuntimeManager::get().get_background_task_manager();
        if let Some(t) = btm.get_mut::<T, R>(self.id as usize) {
            t.resume();
        }
    }
    pub fn stop(&self) {
        let btm = RuntimeManager::get().get_background_task_manager();
        if let Some(t) = btm.get_mut::<T, R>(self.id as usize) {
            t.stop();
        }
    }
}
