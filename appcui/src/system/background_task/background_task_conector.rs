use std::sync::{Arc, Condvar, Mutex};
use super::StatusUpdateRequest;
use crate::terminals::SystemEvent;


pub struct BackgroundTaskConector<T: Sized, R: Sized> {
    id: u32,
    sysevent_sender: std::sync::mpsc::Sender<SystemEvent>,
    sender: std::sync::mpsc::Sender<T>,
    receiver: std::sync::mpsc::Receiver<R>,
    state: Arc<(Mutex<StatusUpdateRequest>, Condvar)>,
}

impl<T, R> BackgroundTaskConector<T, R> {
    pub(super) fn new(
        id: u32,
        sysevent_sender: std::sync::mpsc::Sender<SystemEvent>,
        sender: std::sync::mpsc::Sender<T>,
        receiver: std::sync::mpsc::Receiver<R>,
        state: Arc<(Mutex<StatusUpdateRequest>, Condvar)>,
    ) -> Self {
        Self {
            id,
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
        self.sysevent_sender.send(SystemEvent::BackgroundTaskNotify(self.id)).is_ok()
    }
    pub(super) fn notify_start(&self) {
        self.sysevent_sender.send(SystemEvent::BackgroundTaskStart(self.id));
    }
    pub(super) fn notify_end(&self) {
        self.sysevent_sender.send(SystemEvent::BackgroundTaskEnd(self.id));
    }
    /// Query the main thread with some data. Returns the result of the query.
    /// This method works synchronously.
    pub fn query(&self, data: T) -> Option<R> {
        if self.sender.send(data).is_err() {
            return None;
        }
        if self.sysevent_sender.send(SystemEvent::BackgroundTaskQuery(self.id)).is_err() {
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
    /// fn run_background_task_function(conector: &BackgroundTaskConector<T, R>) {
    ///     while (/* still things to be done */) {
    ///        if !conector.should_stop() { break; }
    ///       // do something
    ///     }
    /// }
    /// ```
    pub fn should_stop(&self) -> bool {
        let (lock, cvar) = &*self.state;
        let mut status = lock.lock().unwrap();
        match *status {
            StatusUpdateRequest::None => return false,
            StatusUpdateRequest::Pause => {
                status = cvar.wait_while(status, |s| *s == StatusUpdateRequest::Pause).unwrap();
                return *status == StatusUpdateRequest::Close;
            }
            StatusUpdateRequest::Close => return true,
        }
    }
    
}
