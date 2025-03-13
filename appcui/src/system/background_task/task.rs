use super::{BackgroundTaskConector, SingleChannel, StatusUpdateRequest};
use crate::system::{Handle, RuntimeManager};
use std::{any::Any, sync::{Arc, Condvar, Mutex}, thread};

pub(crate) trait Task {
    fn read_data(&mut self) -> Option<&dyn Any>;
    fn update_control_handle(&mut self, control_handle: Handle<()>);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
pub(crate) struct InnerTask<T: Send, R: Send> {
    pub(crate) control: Handle<()>,
    pub(crate) main_to_task: SingleChannel<R>,
    pub(crate) task_to_main: SingleChannel<T>,
    state: Arc<(Mutex<StatusUpdateRequest>, Condvar)>,
    data: Option<T>,
}

impl<T: Send + 'static, R: Send + 'static> InnerTask<T, R> {
    pub(crate) fn new(control_handle: Handle<()>) -> InnerTask<T, R> {
        InnerTask {
            control: control_handle,
            main_to_task: SingleChannel::new(),
            task_to_main: SingleChannel::new(),
            state: Arc::new((Mutex::new(StatusUpdateRequest::None), Condvar::new())),
            data: None,
        }
    }
    pub(super) fn run(&mut self, task: fn(conector: &BackgroundTaskConector<T, R>), id: u32) {
        let conector = BackgroundTaskConector::new(
            id,
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
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
