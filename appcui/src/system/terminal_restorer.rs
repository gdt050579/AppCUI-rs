use crate::backend::{RestoreReason, TerminalRestorer};
use std::sync::{Mutex, Once, TryLockError};

static RESTORER: Mutex<Option<Box<dyn TerminalRestorer>>> = Mutex::new(None);
static HOOK: Once = Once::new();

pub(crate) fn init(restorer: Box<dyn TerminalRestorer>) {
    *RESTORER.lock().unwrap_or_else(|e| e.into_inner()) = Some(restorer);
    HOOK.call_once(|| {
        let prev = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            restore(RestoreReason::Panic); 
            prev(info);                   
        }));
    });
}

pub(crate) fn restore(reason: RestoreReason) {
    let mut guard = match RESTORER.try_lock() {
        Ok(g) => g,
        Err(TryLockError::Poisoned(p)) => p.into_inner(),
        Err(TryLockError::WouldBlock) => return,
    };
    if let Some(restorer) = guard.take() {
        restorer.restore(reason);
    }
}