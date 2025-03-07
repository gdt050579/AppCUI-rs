use std::sync::{Arc, Condvar, Mutex};

use crate::terminals::SystemEvent;

pub struct BackgroundTaskNotifier<T: Sized> {
    id: u32,
    sysevent_sender: std::sync::mpsc::Sender<SystemEvent>,
    sender: std::sync::mpsc::Sender<T>,
    receiver: std::sync::mpsc::Receiver<T>,
    state: Arc<(Mutex<bool>, Condvar)>,
}

impl<T> BackgroundTaskNotifier<T> {
    pub(super) fn new(
        id: u32,
        sysevent_sender: std::sync::mpsc::Sender<SystemEvent>,
        sender: std::sync::mpsc::Sender<T>,
        receiver: std::sync::mpsc::Receiver<T>,
        state: Arc<(Mutex<bool>, Condvar)>,
    ) -> BackgroundTaskNotifier<T> {
        Self { id, sysevent_sender, sender, receiver, state }
    }
    pub fn send(&self, data: T) -> bool {
        if self.sender.send(data).is_err() {
            return false;
        }
        self.sysevent_sender.send(SystemEvent::BackgroundTaskUpdate(self.id)).is_ok()
    }
    pub(super) fn notify_start(&self) {
        self.sysevent_sender.send(SystemEvent::BackgroundTaskStart(self.id));
    }
    pub(super) fn notify_end(&self) {
        self.sysevent_sender.send(SystemEvent::BackgroundTaskEnd(self.id));
    }
    // pub fn wait(&self) -> T {
    //     let (lock, cvar) = &*self.state;
    //     let mut started = lock.lock().unwrap();
    //     while !*started {
    //         started = cvar.wait(started).unwrap();
    //     }
    //     self.receiver.recv().unwrap()
    // }
}
