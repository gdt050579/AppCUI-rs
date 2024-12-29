use crate::system::RuntimeManager;

use super::Command;
use super::{super::Handle, thread_logic::ThreadLogic};
use std::{
    sync::{Arc, Condvar, Mutex},
    thread,
};

#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
enum TimerState {
    RequiresControlHandle,
    Ready,
    Running,
    Paused,
    Terminate,
}

pub struct Timer {
    synk: Arc<(Mutex<Command>, Condvar)>,
    control_handle: Handle<()>,
    handle: Handle<Timer>,
    requested_command: Command,
    state: TimerState,
}
impl Timer {
    pub(super) fn new(control_handle: Handle<()>, handle: Handle<Timer>) -> Self {
        Self {
            synk: Arc::new((Mutex::new(Command::Stop), Condvar::new())),
            control_handle,
            handle,
            requested_command: Command::None,
            state: if control_handle.is_none() {
                TimerState::RequiresControlHandle
            } else {
                TimerState::Ready
            },
        }
    }
    #[inline(always)]
    pub(super) fn handle(&self) -> Handle<Timer> {
        self.handle
    }
    pub(super) fn update_control_handle(&mut self, control_handle: Handle<()>) {
        if (self.state == TimerState::RequiresControlHandle) && (!control_handle.is_none()) {
            self.control_handle = control_handle;
            self.state = TimerState::Ready;
        }
    }
    pub(super) fn start_thread(&mut self) {
        let mut thread_logic = ThreadLogic::new(self.control_handle, self.requested_command);
        let synk = self.synk.clone();
        thread::spawn(move || {
            thread_logic.run(synk);
        });
    }
    fn send_command(&mut self, command: Command) {
        match self.state {
            TimerState::RequiresControlHandle => {
                self.requested_command = command;
            }
            TimerState::Ready => {
                self.requested_command = command;
                RuntimeManager::get().request_timer_threads_update();
            }
            TimerState::Running | TimerState::Paused => {
                let mut guard = self.synk.0.lock().unwrap();
                *guard = command;
                self.synk.1.notify_one();
            }
            TimerState::Terminate => {
                // do nothing (wait for the thread to finish)
                RuntimeManager::get().request_timer_threads_update();
            }
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
