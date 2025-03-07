use std::{any::Any, thread};
use crate::system::Handle;
use super::BackgroundTaskNotifier;


pub(crate) trait Task {
    fn read_data(&mut self) -> Option<&dyn Any>;
    fn update_control_handle(&mut self, control_handle: Handle<()>);
}
pub(crate) struct InnerTask<T: Sized> {
    pub(crate) control: Handle<()>,
    pub(crate) receiver: std::sync::mpsc::Receiver<T>,
    pub(crate) sender: std::sync::mpsc::Sender<T>,
    data: Option<T>,
}

impl<T: Sized+'static> InnerTask<T> {
    pub(crate) fn new() -> InnerTask<T> {
        let (sender, receiver) = std::sync::mpsc::channel();
        InnerTask {
            control: Handle::None,
            receiver,
            sender,
            data: None
        }
    }
    fn run(&self, task: fn(notifier: &BackgroundTaskNotifier<T>)) {
        let notifier = BackgroundTaskNotifier::new(0, std::sync::mpsc::Sender::clone(&self.sender));
        thread::spawn(move || {
            notifier.notify_start();
            task(&notifier);
            notifier.notify_end();
        });
    }
}
impl<T: Sized+'static> Task for InnerTask<T> {
    fn read_data(&mut self) -> Option<&dyn Any> {
        if let Ok(data) = self.receiver.try_recv() {
            self.data = Some(data);
            Some(&self.data)
        } else {
            None
        }
        //self.receiver.try_recv().ok()
    }
    fn update_control_handle(&mut self, control_handle: Handle<()>) {
        self.control = control_handle;
    }
}