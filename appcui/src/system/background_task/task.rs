use super::{BackgroundTaskConector, SingleChannel};
use crate::system::{Handle, RuntimeManager};
use std::{any::Any, sync::{Arc, Condvar, Mutex}, thread};

pub(crate) trait Task {
    fn read_data(&mut self) -> Option<&dyn Any>;
    fn update_control_handle(&mut self, control_handle: Handle<()>);
}
pub(crate) struct InnerTask<T: Send, R: Send> {
    pub(crate) control: Handle<()>,
    pub(crate) main_to_task: SingleChannel<R>,
    pub(crate) task_to_main: SingleChannel<T>,
    state: Arc<(Mutex<bool>, Condvar)>,
    data: Option<T>,
}

impl<T: Send + 'static, R: Send + 'static> InnerTask<T, R> {
    pub(crate) fn new() -> InnerTask<T, R> {
        InnerTask {
            control: Handle::None,
            main_to_task: SingleChannel::new(),
            task_to_main: SingleChannel::new(),
            state: Arc::new((Mutex::new(false), Condvar::new())),
            data: None,
        }
    }
    fn run(&mut self, task: fn(conector: &BackgroundTaskConector<T, R>)) {
        let conector = BackgroundTaskConector::new(
            0,
            RuntimeManager::get().get_system_event_sender(),
            self.task_to_main.to_own_sender().unwrap(),
            self.main_to_task.to_own_receiver().unwrap(),
            self.state.clone(),
        );
        thread::spawn(move || {
            conector.notify_start();
            task(&conector);
            conector.notify_end();
        });
    }
}
impl<T: Send + 'static, R: Send + 'static> Task for InnerTask<T, R> {
    fn read_data(&mut self) -> Option<&dyn Any> {
        if let Some(data) = self.task_to_main.read() {
            self.data = Some(data);
            Some(&self.data)
        } else {
            None
        }
    }
    fn update_control_handle(&mut self, control_handle: Handle<()>) {
        self.control = control_handle;
    }
}
