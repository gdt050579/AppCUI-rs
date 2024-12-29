use super::{super::Handle, thread_logic::ThreadLogic};
use super::Command;
use std::{sync::{Arc, Condvar, Mutex}, thread};

#[derive(Copy, Clone)]
#[repr(u8)]
enum TimerState {
    Running,
    Paused,
    Stopped,
}

pub struct Timer {
    synk: Arc<(Mutex<Command>, Condvar)>,
    control_handle: Handle<()>,
    handle: Handle<Timer>,
    requested_command: Command,
}
impl Timer {
    pub(super) fn new(control_handle: Handle<()>, handle: Handle<Timer>) -> Self {
        Self {
            synk: Arc::new((Mutex::new(Command::Stop), Condvar::new())),
            control_handle,
            handle,
            requested_command: Command::None,
        }
    }
    #[inline(always)]
    pub(super) fn handle(&self) -> Handle<Timer> {
        self.handle
    }
    pub(super) fn start_thread(&mut self) {
        let mut thread_logic = ThreadLogic::new(self.control_handle, self.requested_command);
        let synk = self.synk.clone();
        thread::spawn(move || {
            thread_logic.run(synk);
        });
    }
    fn send_command(&mut self, command: Command) {
        if self.control_handle.is_none() {
            self.requested_command = command;
        } else {
            let mut guard = self.synk.0.lock().unwrap();
            *guard = command;
            self.synk.1.notify_one();
        }
    }
    pub fn pause(&mut self) {
        self.send_command(Command::Pause);
    }
    pub fn resume(&mut self) {
        self.send_command(Command::Resume);
    }
    pub fn set_interval(&mut self, new_interval: u32) {
        self.send_command(Command::SetInterval(new_interval.max(1)));
    }
    pub fn start(&mut self, interval: u64) {
        self.send_command(Command::Start((interval as u32).max(1)));
    }
    pub fn stop(&mut self) {}
    pub fn is_paused(&self) -> bool {
        false
    }

    
}
