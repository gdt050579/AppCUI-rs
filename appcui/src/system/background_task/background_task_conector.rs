use super::{BackgroundTask, StatusUpdateRequest};
use crate::{system::Handle, terminals::SystemEvent};
use std::sync::{Arc, Condvar, Mutex};

pub struct BackgroundTaskConector<T: Send + 'static, R: Send + 'static> {
    handle:          Handle<BackgroundTask<T, R>>,
    sysevent_sender: std::sync::mpsc::Sender<SystemEvent>,
    sender:          std::sync::mpsc::Sender<T>,
    receiver:        std::sync::mpsc::Receiver<R>,
    state:           Arc<(Mutex<StatusUpdateRequest>, Condvar)>,
}

impl<T, R> BackgroundTaskConector<T, R>
where
    T: Send + 'static,
    R: Send + 'static,
{
    pub(super) fn new(
        handle: Handle<BackgroundTask<T, R>>,
        sysevent_sender: std::sync::mpsc::Sender<SystemEvent>,
        sender: std::sync::mpsc::Sender<T>,
        receiver: std::sync::mpsc::Receiver<R>,
        state: Arc<(Mutex<StatusUpdateRequest>, Condvar)>,
    ) -> Self {
        Self {
            handle,
            sysevent_sender,
            sender,
            receiver,
            state,
        }
    }
    /// Notify the main thread with some data. Returns true if the data was sent successfully.
    /// This method works asynchronously.
    pub fn notify(&self, data: T) -> bool {
        if self.sender.send(data).is_err() {
            return false;
        }
        self.sysevent_sender.send(SystemEvent::BackgroundTaskNotify(self.handle.cast())).is_ok()
    }
    pub(super) fn notify_start(&self) {
        let _ = self.sysevent_sender.send(SystemEvent::BackgroundTaskStart(self.handle.cast()));
    }
    pub(super) fn notify_end(&self) {
        let _ = self.sysevent_sender.send(SystemEvent::BackgroundTaskEnd(self.handle.cast()));
    }
    /// Query the main thread with some data. Returns the result of the query.
    /// This method works synchronously.
    pub fn query(&self, data: T) -> Option<R> {
        if self.sender.send(data).is_err() {
            return None;
        }
        if self.sysevent_sender.send(SystemEvent::BackgroundTaskQuery(self.handle.cast())).is_err() {
            return None;
        }

        self.receiver.recv().ok()
    }

    /// If there is a task request to pause the task from the main thread, it pauses the task and waits until the task is resumed.
    /// If there is a task request to close the task from the main thread, it returns true. Otherwise, it returns false.
    /// This method should be used in the background task to validate that the task should continue.
    ///
    /// # Example
    /// ```rust, no_compile
    /// use appcui::prelude::*;
    ///
    /// fn run_background_task_function<T,R>(conector: &BackgroundTaskConector<T, R>)
    /// where
    ///     T:Send+'static,
    ///     R:Send+'static
    /// {
    ///     loop {
    ///        if !conector.should_stop() { break; }
    ///        // do some tasks
    ///     }
    /// }
    /// ```
    pub fn should_stop(&self) -> bool {
        let (lock, cvar) = &*self.state;
        let mut status = lock.lock().unwrap();
        match *status {
            StatusUpdateRequest::None => false,
            StatusUpdateRequest::Pause => {
                status = cvar.wait_while(status, |s| *s == StatusUpdateRequest::Pause).unwrap();
                *status == StatusUpdateRequest::Close
            }
            StatusUpdateRequest::Close => true,
        }
    }
}
