use crate::system::{timer::TimerManager, Handle};

use super::Command;

#[test]
fn check_command_interval() {
    assert_eq!(Command::None.iterval(),None);
    assert_eq!(Command::Start(100).iterval(),Some(100));
    assert_eq!(Command::SetInterval(200).iterval(),Some(200));
    assert_eq!(Command::Stop.iterval(),None);
    assert_eq!(Command::Pause.iterval(),None);
    assert_eq!(Command::Resume.iterval(),None);    
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
    assert_eq!(h.is_none(), true);
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
    assert_eq!(timer_manager.control_handle(1).is_none(), true);
    assert_eq!(timer_manager.control_handle(100).is_none(), true);
}


#[test]
fn check_timer_manager_index_mut() {
    let mut timer_manager = TimerManager::new(4);
    let ch = Handle::<()>::new(100);
    let _ = timer_manager.allocate_for(ch);
    assert_eq!(timer_manager.index_mut(0).is_some(), true);
    assert_eq!(timer_manager.index_mut(1).is_none(), true);
    assert_eq!(timer_manager.index_mut(100).is_none(), true);
}
