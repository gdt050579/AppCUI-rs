use super::{BackgroundTask, BackgroundTaskConector, SingleChannel, StatusUpdateRequest};
use crate::system::{Handle, RuntimeManager};
use std::{
    any::{Any, TypeId},
    sync::{Arc, Condvar, Mutex},
};

#[cfg(not(target_arch = "wasm32"))]
use std::thread;

#[cfg(target_arch = "wasm32")]
use rayon;

pub(crate) trait Task {
    fn update_control_handle(&mut self, control_handle: Handle<()>);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn validate(&self, t: TypeId, r: TypeId) -> bool;
    fn receiver_control_handle(&self) -> Handle<()>;
}

pub(crate) struct InnerTask<T: Send, R: Send> {
    pub(crate) control: Handle<()>,
    pub(crate) main_to_task: SingleChannel<R>,
    pub(crate) task_to_main: SingleChannel<T>,
    state: Arc<(Mutex<StatusUpdateRequest>, Condvar)>,
}

impl<T: Send + 'static, R: Send + 'static> InnerTask<T, R> {
    pub(crate) fn new(control_handle: Handle<()>) -> InnerTask<T, R> {
        InnerTask {
            control: control_handle,
            main_to_task: SingleChannel::new(),
            task_to_main: SingleChannel::new(),
            state: Arc::new((Mutex::new(StatusUpdateRequest::None), Condvar::new())),
        }
    }

    pub(super) fn run(&mut self, task: fn(conector: &BackgroundTaskConector<T, R>), handle: Handle<BackgroundTask<T, R>>) {
        let conector = BackgroundTaskConector::new(
            handle,
            RuntimeManager::get().get_system_event_sender(),
            self.task_to_main.take_ownership_for_sender().unwrap(),
            self.main_to_task.take_ownership_for_receiver().unwrap(),
            self.state.clone(),
        );

        #[cfg(not(target_arch = "wasm32"))]
        {
            thread::spawn(move || {
                conector.notify_start();
                task(&conector);
                conector.notify_end();
            });
        }

        #[cfg(target_arch = "wasm32")]
        {
            rayon::spawn(move || {
                conector.notify_start();
                task(&conector);
                conector.notify_end();
            });
        }
    }

    pub(super) fn pause(&mut self) {
        let (lock, cvar) = &*self.state;
        let mut status = lock.lock().unwrap();
        if (*status) != StatusUpdateRequest::None {
            return;
        }
        *status = StatusUpdateRequest::Pause;
        cvar.notify_one();
    }

    pub(super) fn resume(&mut self) {
        let (lock, cvar) = &*self.state;
        let mut status = lock.lock().unwrap();
        if (*status) != StatusUpdateRequest::Pause {
            return;
        }
        *status = StatusUpdateRequest::None;
        cvar.notify_one();
    }

    pub(super) fn stop(&mut self) {
        let (lock, cvar) = &*self.state;
        let mut status = lock.lock().unwrap();
        if (*status) == StatusUpdateRequest::Close {
            return;
        }
        *status = StatusUpdateRequest::Close;
        cvar.notify_one();
    }
}

impl<T: Send + 'static, R: Send + 'static> Task for InnerTask<T, R> {
    fn update_control_handle(&mut self, control_handle: Handle<()>) {
        self.control = control_handle;
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    #[inline(always)]
    fn validate(&self, t: TypeId, r: TypeId) -> bool {
        TypeId::of::<T>() == t && TypeId::of::<R>() == r
    }
    #[inline(always)]
    fn receiver_control_handle(&self) -> Handle<()> {
        self.control
    }
}
