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

    /// Starts a new background task. The task will run in a separate thread.
    /// The task will receive a conector that can be used to send and receive data from the main thread.
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

    /// Reads the data sent by the background task. If there is no data, it returns None.
    /// This method is not meant to be used directly (it will be used by the generated code).
    pub fn read(&self) -> Option<T> {
        let btm = RuntimeManager::get().get_background_task_manager();
        if let Some(t) = btm.get_mut::<T, R>(self.id as usize) {
            t.task_to_main.read()
        } else {
            None
        }
    }

    /// Sends data to the background task.
    /// This method is not meant to be used directly (it will be used by the generated code).
    pub fn send(&self, value: R) {
        let btm = RuntimeManager::get().get_background_task_manager();
        if let Some(t) = btm.get_mut::<T, R>(self.id as usize) {
            t.main_to_task.send(value);
        }
    }

    /// Request the background task to pause. For this method to work, the background task must use the `should_stop()` method
    pub fn pause(&self) {
        let btm = RuntimeManager::get().get_background_task_manager();
        if let Some(t) = btm.get_mut::<T, R>(self.id as usize) {
            t.pause();
        }
    }

    /// Request the background task to resume. For this method to work, the background task must use the `should_stop()` method
    pub fn resume(&self) {
        let btm = RuntimeManager::get().get_background_task_manager();
        if let Some(t) = btm.get_mut::<T, R>(self.id as usize) {
            t.resume();
        }
    }

    /// Request the background task to stop. For this method to work, the background task must use the `should_stop()` method
    pub fn stop(&self) {
        let btm = RuntimeManager::get().get_background_task_manager();
        if let Some(t) = btm.get_mut::<T, R>(self.id as usize) {
            t.stop();
        }
    }

    /// Updates the control handle of the background task for scenarios when the current window that receives the data is replaced.
    pub fn update_control_handle(&self, control_handle: Handle<()>) {
        let btm = RuntimeManager::get().get_background_task_manager();
        if let Some(t) = btm.get_mut::<T, R>(self.id as usize) {
            t.update_control_handle(control_handle);
        }
    }
}
