use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::window;

use crate::terminals::SystemEvent;
use crate::terminals::SystemEventReader;

impl SystemEventReader for super::WebTerminal {
    fn read(&mut self) -> Option<SystemEvent> {
        let mut queue = self.event_queue.lock().unwrap();
        if !queue.is_empty() {
            Some(queue.remove(0))
        } else {
            None
        }
    }

    fn start(self, sender: Sender<SystemEvent>)
    where
        Self: Sized + Send + 'static,
    {
        let terminal = Arc::new(Mutex::new(self));
        let terminal_clone = terminal.clone();

        let closure = Closure::wrap(Box::new(move || {
            let mut term = terminal_clone.lock().unwrap();
            while let Some(ev) = term.read() {
                if sender.send(ev).is_err() {
                    break;
                }
            }
        }) as Box<dyn FnMut()>);

        window()
            .expect("should have a window")
            .set_interval_with_callback_and_timeout_and_arguments_0(closure.as_ref().unchecked_ref(), 16)
            .expect("Failed to set interval");
        closure.forget();
    }
}
