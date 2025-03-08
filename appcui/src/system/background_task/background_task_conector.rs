use std::sync::{Arc, Condvar, Mutex};

use crate::terminals::SystemEvent;

pub struct BackgroundTaskConector<T: Sized, R: Sized> {
    id: u32,
    sysevent_sender: std::sync::mpsc::Sender<SystemEvent>,
    sender: std::sync::mpsc::Sender<T>,
    receiver: std::sync::mpsc::Receiver<R>,
    state: Arc<(Mutex<bool>, Condvar)>,
}

impl<T, R> BackgroundTaskConector<T, R> {
    pub(super) fn new(
        id: u32,
        sysevent_sender: std::sync::mpsc::Sender<SystemEvent>,
        sender: std::sync::mpsc::Sender<T>,
        receiver: std::sync::mpsc::Receiver<R>,
        state: Arc<(Mutex<bool>, Condvar)>,
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
    
}
