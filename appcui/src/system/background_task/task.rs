use crate::system::Handle;

pub(crate) trait Task<T: Sized> {
    fn read_data(&self) -> Option<T>;
    fn update_control_handle(&mut self, control_handle: Handle<()>);
}
pub(crate) struct InnerTask<T: Sized> {
    pub(crate) control: Handle<()>,
    pub(crate) receiver: std::sync::mpsc::Receiver<T>,
    pub(crate) sender: std::sync::mpsc::Sender<T>,
}

impl<T: Sized> InnerTask<T> {
    pub(crate) fn new() -> InnerTask<T> {
        let (sender, receiver) = std::sync::mpsc::channel();
        InnerTask {
            control: Handle::None,
            receiver,
            sender,
        }
    }
    // fn run(&self, task: Fn()) {
    //     task();
    // }
}
impl<T: Sized> Task<T> for InnerTask<T> {
    fn read_data(&self) -> Option<T> {
        self.receiver.try_recv().ok()
    }
    fn update_control_handle(&mut self, control_handle: Handle<()>) {
        self.control = control_handle;
    }
}