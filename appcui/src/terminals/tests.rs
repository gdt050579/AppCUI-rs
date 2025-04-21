use crate::{
    input::Key,
    terminals::{KeyPressedEvent, SystemEvent},
};

#[test]
fn check_system_event_snould_close() {
    assert_eq!(SystemEvent::AppClose.should_close(), true);
    assert_eq!(
        SystemEvent::KeyPressed(KeyPressedEvent {
            key: Key::None,
            character: 'a'
        })
        .should_close(),
        false
    );
}
