use super::super::Handle;
use super::Command;
use std::sync::{Arc, Condvar, Mutex};
use std::time::Duration;

#[derive(Copy, Clone)]
pub(crate) struct ThreadLogic {
    tick: u64,
    interval: u32,
    handle: Handle<()>,
    paused: bool,
}

impl ThreadLogic {
    pub(super) fn new(handle: Handle<()>, command: Command) -> Self {
        Self {
            tick: 0,
            interval: command.iterval().unwrap_or(1000).max(1),
            handle,
            paused: true,
        }
    }
    pub(crate) fn run(&mut self, sync: Arc<(Mutex<Command>, Condvar)>) {
        let (mutex, cvar) = &*sync;
        let mut guard = mutex.lock().unwrap();
        if self.update_status(*guard) {
            return; // exit the thread
        }
        let mut time_to_wait = self.wait_time();
        loop {
            let (new_guard, timeout_status) = cvar.wait_timeout(guard, time_to_wait).unwrap();
            guard = new_guard;
            if timeout_status.timed_out() {
                // send timer event
                self.tick += 1;
            } else {
                if self.update_status(*guard) {
                    return;
                }
                time_to_wait = self.wait_time();
            }
        }
    }
    /// true means thread should finish, false keep alive
    fn update_status(&mut self, command: Command) -> bool {
        match command {
            Command::None => false,
            Command::Start(interval) => {
                self.interval = interval;
                self.tick = 0;
                self.paused = false;
                false
            }
            Command::Stop => {
                self.paused = true;
                true
            }
            Command::Resume => {
                self.paused = false;
                false
            }
            Command::SetInterval(interval) => {
                self.interval = interval;
                false
            }
            Command::Pause => {
                self.paused = true;
                false
            }
        }
    }
    #[inline(always)]
    fn wait_time(&self) -> Duration {
        if self.paused {
            Duration::MAX
        } else {
            Duration::from_micros(self.interval as u64)
        }
    }
}
