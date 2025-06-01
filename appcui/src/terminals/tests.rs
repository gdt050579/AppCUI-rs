use crate::{
    input::Key,
    terminals::{KeyPressedEvent, SystemEvent},
};

#[test]
fn check_system_event_snould_close() {
    assert!(SystemEvent::AppClose.should_close());
    assert!(
        !SystemEvent::KeyPressed(KeyPressedEvent {
            key: Key::None,
            character: 'a'
        })
        .should_close()
    );
}
