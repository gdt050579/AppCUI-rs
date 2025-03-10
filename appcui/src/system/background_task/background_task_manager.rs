use super::task::Task;

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
    pub(crate) fn get<T: Send+'static, R: Send+'static>(&self, index: usize) -> Option<&Box<dyn Task>> {
        self.tasks.get(index).map(|x| x.as_ref().unwrap())
    }
}
