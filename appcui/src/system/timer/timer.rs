use crate::system::runtime_manager_traits::TimerMethods;
use crate::system::RuntimeManager;
use crate::system::SystemEvent;

use super::Command;
use super::{super::Handle, thread_logic::ThreadLogic};
use std::sync::mpsc::Sender;

#[cfg(not(target_arch = "wasm32"))]
use std::thread;
#[cfg(not(target_arch = "wasm32"))]
use std::time::Duration;
#[cfg(target_arch = "wasm32")]
use web_time::Duration;

use std::sync::{Arc, Condvar, Mutex};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(u8)]
enum TimerState {
    RequiresControlHandle,
    Ready,
    Running,
    Paused,
    Terminate,
}

#[derive(Debug)]
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
    #[inline(always)]
    pub(super) fn control_handle(&self) -> Handle<()> {
        self.control_handle
    }
    pub(super) fn update_control_handle(&mut self, control_handle: Handle<()>) {
        if (self.state == TimerState::RequiresControlHandle) && (!control_handle.is_none()) {
            self.control_handle = control_handle;
            self.state = TimerState::Ready;
        }
    }
    #[inline(always)]
    pub(super) fn is_ready(&self) -> bool {
        self.state == TimerState::Ready
    }
    #[inline(always)]
    pub(super) fn is_closed(&self) -> bool {
        self.state == TimerState::Terminate
    }
    pub(super) fn start_thread(&mut self, sender: Sender<SystemEvent>) {
        let mut thread_logic = ThreadLogic::new(self.handle.index() as u8, self.requested_command.iterval().unwrap_or(1000).max(1));
        if let Ok(mut guard) = self.synk.0.lock() {
            *guard = self.requested_command;
        }
        let synk = self.synk.clone();
        self.state = match self.requested_command {
            Command::Start(_) | Command::Resume => TimerState::Running,
            _ => TimerState::Paused,
        };

        #[cfg(not(target_arch = "wasm32"))]
        thread::spawn(move || {
            thread_logic.run(synk, sender);
        });

        #[cfg(target_arch = "wasm32")]
        rayon::spawn(move || {
            thread_logic.run(synk, sender);
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
    /// Pause the timer
    pub fn pause(&mut self) {
        self.send_command(Command::Pause);
    }
    /// Resume the timer. The internal tick counter will not be modified.
    pub fn resume(&mut self) {
        self.send_command(Command::Resume);
    }
    /// Set the interval of the timer. If the timer is running, the interval will be changed imediatelly. If the timer is paused, the interval will be changed when the timer is resumed.
    /// The interval (**duration** parameter) will be clamped between 1 and 0xFFFFFFFE miliseconds.
    pub fn set_interval(&mut self, duration: Duration) {
        self.send_command(Command::SetInterval(Timer::duration_to_miliseconds(duration)));
    }
    /// Start the timer with the specified duration
    /// If the timer already has an associated thread it will be used. Otherwise, a new thread will be started.
    /// The internal tick counter will be reset to 0.   
    /// The interval (**duration** parameter) will be clamped between 1 and 0xFFFFFFFE miliseconds.
    pub fn start(&mut self, duration: Duration) {
        self.send_command(Command::Start(Timer::duration_to_miliseconds(duration)));
    }
    /// Stop the timer. closes the associated thread and releases the internal timer slot so that other control can use it.
    pub fn stop(&mut self) {
        match self.state {
            TimerState::Running | TimerState::Paused => {
                self.send_command(Command::Stop);
            }
            _ => {}
        }
        self.state = TimerState::Terminate;
        RuntimeManager::get().request_timer_threads_update();
    }
    /// Returns true if the timer is paused or false otherwise (running or stopped)
    pub fn is_paused(&self) -> bool {
        self.state == TimerState::Paused
    }
    /// Returns true if the timer is running or false otherwise (paused or stopped)
    pub fn is_running(&self) -> bool {
        self.state == TimerState::Running
    }
    pub(in super::super) fn set_pause_state(&mut self) {
        if self.state == TimerState::Running {
            self.state = TimerState::Paused
        }
    }
    pub(in super::super) fn set_running_state(&mut self) {
        if self.state == TimerState::Paused {
            self.state = TimerState::Running
        }
    }
    #[inline(always)]
    fn duration_to_miliseconds(dur: Duration) -> u32 {
        dur.as_millis().clamp(1, 0xFFFFFFFE) as u32
    }
}
