use crate::terminals::*;

use super::Command;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Condvar, Mutex};
use std::time::Duration;

#[derive(Copy, Clone)]
pub(crate) struct ThreadLogic {
    tick: u64,
    interval: u32,
    id: u8,
    paused: bool,
}

impl ThreadLogic {
    pub(super) fn new(id: u8, interval: u32) -> Self {
        Self {
            tick: 0,
            interval,
            id,
            paused: true,
        }
    }
    pub(crate) fn run(&mut self, sync: Arc<(Mutex<Command>, Condvar)>, sender: Sender<SystemEvent>) {
        let (mutex, cvar) = &*sync;
        let mut guard = mutex.lock().unwrap();
        if self.update_status(*guard, &sender) {
            return; // exit the thread
        }
        let mut time_to_wait = self.wait_time();
        loop {
            let (new_guard, timeout_status) = cvar.wait_timeout(guard, time_to_wait).unwrap();
            guard = new_guard;
            if timeout_status.timed_out() {
                if sender
                    .send(SystemEvent::TimerTickUpdate(TimerTickUpdateEvent {
                        id: self.id,
                        tick: self.tick.into(),
                    }))
                    .is_err()
                {
                    self.update_status(Command::Stop, &sender);
                    return;
                }
                self.tick += 1;
            } else {
                if self.update_status(*guard, &sender) {
                    return;
                }
                time_to_wait = self.wait_time();
            }
        }
    }
    /// true means thread should finish, false keep alive
    #[inline(always)]
    fn update_status(&mut self, command: Command, sender: &Sender<SystemEvent>) -> bool {
        match command {
            Command::None => false,
            Command::Start(interval) => {
                self.interval = interval;
                self.tick = 0;
                self.paused = false;
                if sender
                    .send(SystemEvent::TimerStart(TimerStartEvent { id: self.id, tick: 0.into() }))
                    .is_err()
                {
                    self.paused = true;
                    return true;
                }
                false
            }
            Command::Stop => {
                self.paused = true;
                true
            }
            Command::Resume => {
                if sender
                    .send(SystemEvent::TimerStart(TimerStartEvent {
                        id: self.id,
                        tick: self.tick.into(),
                    }))
                    .is_err()
                {
                    self.paused = true;
                    true
                } else {
                    self.paused = false;
                    false
                }
            }
            Command::SetInterval(interval) => {
                self.interval = interval;
                false
            }
            Command::Pause => {
                self.paused = true;
                sender
                    .send(SystemEvent::TimerPaused(TimerPausedEvent {
                        id: self.id,
                        tick: self.tick.into(),
                    }))
                    .is_err()
            }
        }
    }
    #[inline(always)]
    fn wait_time(&self) -> Duration {
        if self.paused {
            Duration::MAX
        } else {
            Duration::from_millis(self.interval as u64)
        }
    }
}
