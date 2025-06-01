use crate::system::{timer::TimerManager, Handle, Timer};

use super::Command;

#[test]
fn check_command_interval() {
    assert_eq!(Command::None.iterval(), None);
    assert_eq!(Command::Start(100).iterval(), Some(100));
    assert_eq!(Command::SetInterval(200).iterval(), Some(200));
    assert_eq!(Command::Stop.iterval(), None);
    assert_eq!(Command::Pause.iterval(), None);
    assert_eq!(Command::Resume.iterval(), None);
}

#[test]
fn check_timer_manager_allocate_for() {
    let mut timer_manager = TimerManager::new(4);
    let h = timer_manager.allocate_for(Handle::new(100));
    assert_eq!(h.index(), 0);
    let h = timer_manager.allocate_for(Handle::new(100));
    assert_eq!(h.index(), 1);
    let h = timer_manager.allocate_for(Handle::new(100));
    assert_eq!(h.index(), 2);
    let h = timer_manager.allocate_for(Handle::new(100));
    assert_eq!(h.index(), 3);
    let h = timer_manager.allocate_for(Handle::new(100));
    assert!(h.is_none());
    // timer_manager.terminate_thread(2);
    // let h = timer_manager.allocate_for(Handle::new(100));
    // assert_eq!(h.index(), 2);
}

#[test]
fn check_timer_manager_control_handle() {
    let mut timer_manager = TimerManager::new(4);
    let ch = Handle::<()>::new(100);
    let _ = timer_manager.allocate_for(ch);
    assert_eq!(timer_manager.control_handle(0), ch);
    assert!(timer_manager.control_handle(1).is_none());
    assert!(timer_manager.control_handle(100).is_none());
}

#[test]
fn check_timer_manager_index_mut() {
    let mut timer_manager = TimerManager::new(4);
    let ch = Handle::<()>::new(100);
    let _ = timer_manager.allocate_for(ch);
    assert!(timer_manager.index_mut(0).is_some());
    assert!(timer_manager.index_mut(1).is_none());
    assert!(timer_manager.index_mut(100).is_none());
}

#[test]
fn check_timer_manager_get_mut() {
    let mut timer_manager = TimerManager::new(4);
    let ch = Handle::<()>::new(100);
    let h_timer = timer_manager.allocate_for(ch);
    assert!(timer_manager.get_mut(h_timer).is_some());
    assert!(timer_manager.get_mut(Handle::<Timer>::new(200)).is_none());
}

#[test]
fn check_update_control_handle() {
    let mut timer_manager = TimerManager::new(4);
    let ch = Handle::<()>::new(100);
    let new_ch = Handle::<()>::new(200);
    let h_timer = timer_manager.allocate_for(ch);
    // valid request (but will change nothing)
    timer_manager.update_control_handle(h_timer, new_ch);
    // invalid request
    timer_manager.update_control_handle(Handle::<Timer>::new(200), new_ch);

    let t = timer_manager.get_mut(h_timer).unwrap();
    assert_eq!(t.handle(), h_timer);
    // the control handle was not changed because it was already set up when the control was created
    assert_eq!(t.control_handle(), ch);
    // the timer is ready (because it has a control handle)
    assert!(t.is_ready());
}

#[test]
fn check_update_control_handle_init_with_none() {
    let mut timer_manager = TimerManager::new(4);
    let new_ch = Handle::<()>::new(200);
    let h_timer = timer_manager.allocate_for(Handle::None);
    let t = timer_manager.get_mut(h_timer).unwrap();
    // timer is not ready because it DOES not have a control handle
    assert!(!t.is_ready());
    // valid request (this will change the control handle)
    timer_manager.update_control_handle(h_timer, new_ch);
    // invalid request
    timer_manager.update_control_handle(Handle::<Timer>::new(200), new_ch);

    let t = timer_manager.get_mut(h_timer).unwrap();
    assert_eq!(t.handle(), h_timer);
    // the control handle was changed to new_ch
    assert_eq!(t.control_handle(), new_ch);
    // now the timer is ready (because it has a control handle)
    assert!(t.is_ready());
}
