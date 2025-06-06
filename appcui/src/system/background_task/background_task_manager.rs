use super::task::{InnerTask, Task};
use crate::system::Handle;

pub(crate) struct BackgroundTaskManager {
    tasks: Vec<Option<Box<dyn Task>>>,
}
impl BackgroundTaskManager {
    pub(crate) fn new() -> Self {
        Self { tasks: Vec::new() }
    }
    pub(crate) fn add_task<T: Task + 'static>(&mut self, task: T) -> usize {
        // find the index of first None
        let index = self.tasks.iter().position(|x| x.is_none());
        if let Some(index) = index {
            self.tasks[index] = Some(Box::new(task));
            index
        } else {
            self.tasks.push(Some(Box::new(task)));
            self.tasks.len() - 1
        }
    }
    pub(crate) fn get<T: Send + 'static, R: Send + 'static>(&self, index: usize) -> Option<&InnerTask<T, R>> {
        if index >= self.tasks.len() {
            return None;
        }
        if let Some(interface) = &self.tasks[index] {
            if let Some(task) = interface.as_any().downcast_ref::<InnerTask<T, R>>() {
                Some(task)
            } else {
                None
            }
        } else {
            None
        }
    }
    pub(crate) fn get_mut<T: Send + 'static, R: Send + 'static>(&mut self, index: usize) -> Option<&mut InnerTask<T, R>> {
        if index >= self.tasks.len() {
            return None;
        }
        if let Some(interface) = &mut self.tasks[index] {
            if let Some(task) = interface.as_any_mut().downcast_mut::<InnerTask<T, R>>() {
                Some(task)
            } else {
                None
            }
        } else {
            None
        }
    }
    #[inline(always)]
    pub(crate) fn receiver_control_handle(&self, index: usize) -> Option<Handle<()>> {
        if index >= self.tasks.len() {
            return None;
        }
        self.tasks[index].as_ref().map(|interface| interface.receiver_control_handle())
    }
    pub(crate) fn remove_task(&mut self, handle: Handle<()>) {
        let index = handle.index();
        if index < self.tasks.len() {
            self.tasks[index] = None;
        }
    }
}
