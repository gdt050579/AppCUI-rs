use super::SystemEvent;
use std::{sync::mpsc::Sender, thread};

pub(crate) trait SystemEventReader {
    fn read(&mut self) -> Option<SystemEvent>;
    fn start(mut self, sender: Sender<SystemEvent>)
    where
        Self: Sized + Send + 'static,
    {
        thread::spawn(move || {
            let mut should_close = false;
            while !should_close {
                if let Some(ev) = self.read() {
                    should_close = ev.should_close();
                    match sender.send(ev) {
                        Err(_) => should_close = true,
                        _ => {}
                    }
                }
            }
        });
    }
}
