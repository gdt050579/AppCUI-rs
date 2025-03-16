use std::sync::mpsc::{Receiver, Sender};

pub(crate) struct SingleChannel<T> {
    sender: Option<Sender<T>>,
    receiver: Option<Receiver<T>>,
}

impl<T> SingleChannel<T> {
    pub(super) fn new() -> SingleChannel<T> {
        let (sender, receiver) = std::sync::mpsc::channel();
        Self {
            sender: Some(sender),
            receiver: Some(receiver),
        }
    }
    pub(super) fn take_ownership_for_sender(&mut self) -> Option<Sender<T>> {
        self.sender.take()
    }
    pub(super) fn take_ownership_for_receiver(&mut self) -> Option<Receiver<T>> {
        self.receiver.take()
    }
    pub(super) fn read(&self) -> Option<T> {
        if let Some(receiver) = &self.receiver {
            receiver.try_recv().ok()
        } else {
            None 
        }
    }
    pub(super) fn send(&self, value: T)  {
        if let Some(sender) = &self.sender {
            let _ = sender.send(value);
        } 
    }

}