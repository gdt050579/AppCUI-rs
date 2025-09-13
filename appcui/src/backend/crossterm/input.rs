use crate::backend::SystemEventReader;
use crate::input::{Key, KeyCode, KeyModifier, MouseButton, MouseWheelDirection};
use crate::system::SystemEvent;
use crate::system::{KeyModifierChangedEvent, KeyPressedEvent, MouseButtonDownEvent, MouseButtonUpEvent, MouseMoveEvent, MouseWheelEvent};
use crossterm::event::KeyCode as CrosstermKeyCode;
use crossterm::event::{self, Event, KeyEvent, KeyEventKind, MouseButton as CrosstermMouseButton, MouseEvent, MouseEventKind};

pub(crate) struct Input {
    last_modifiers: KeyModifier,
}

impl Input {
    pub(super) fn new() -> Self {
        Self {
            last_modifiers: KeyModifier::None,
        }
    }
}

impl SystemEventReader for Input {
    fn read(&mut self) -> Option<SystemEvent> {
        match event::read() {
            Ok(Event::Key(key_event)) => self.handle_key_event(key_event),
            Ok(Event::Mouse(mouse_event)) => self.handle_mouse_event(mouse_event),
            Ok(Event::Resize(width, height)) => Some(SystemEvent::Resize(crate::graphics::Size::new(width as u32, height as u32))),
            Ok(Event::FocusGained) => None,
            Ok(Event::FocusLost) => None,
            Ok(Event::Paste(_)) => None,
            Err(_) => None,
        }
    }
}

impl Input {
    fn handle_key_event(&mut self, key_event: KeyEvent) -> Option<SystemEvent> {
        // Only handle key press events, not releases
        if key_event.kind != KeyEventKind::Press {
            return None;
        }

        let modifiers = self.convert_modifiers(key_event.modifiers);

        // Check for modifier changes
        if modifiers != self.last_modifiers {
            let old_modifiers = self.last_modifiers;
            self.last_modifiers = modifiers;
            return Some(SystemEvent::KeyModifierChanged(KeyModifierChangedEvent {
                new_state: modifiers,
                old_state: old_modifiers,
            }));
        }

        let key_code = self.convert_key_code(key_event.code);
        let key = Key::new(key_code, modifiers);
        let character = match key_event.code {
            crossterm::event::KeyCode::Char(c) => c,
            _ => '\0',
        };

        Some(SystemEvent::KeyPressed(KeyPressedEvent { key, character }))
    }

    fn handle_mouse_event(&mut self, mouse_event: MouseEvent) -> Option<SystemEvent> {
        let x = mouse_event.column as i32;
        let y = mouse_event.row as i32;

        match mouse_event.kind {
            MouseEventKind::Down(button) => {
                let button = self.convert_mouse_button(button);
                Some(SystemEvent::MouseButtonDown(MouseButtonDownEvent { x, y, button }))
            }
            MouseEventKind::Up(button) => {
                let button = self.convert_mouse_button(button);
                Some(SystemEvent::MouseButtonUp(MouseButtonUpEvent { x, y, button }))
            }
            MouseEventKind::Moved => Some(SystemEvent::MouseMove(MouseMoveEvent {
                x,
                y,
                button: MouseButton::None,
            })),
            MouseEventKind::Drag(button) => {
                let button = self.convert_mouse_button(button);
                Some(SystemEvent::MouseMove(MouseMoveEvent { x, y, button }))
            }
            MouseEventKind::ScrollUp => Some(SystemEvent::MouseWheel(MouseWheelEvent {
                x,
                y,
                direction: MouseWheelDirection::Up,
            })),
            MouseEventKind::ScrollDown => Some(SystemEvent::MouseWheel(MouseWheelEvent {
                x,
                y,
                direction: MouseWheelDirection::Down,
            })),
            MouseEventKind::ScrollLeft => Some(SystemEvent::MouseWheel(MouseWheelEvent {
                x,
                y,
                direction: MouseWheelDirection::Left,
            })),
            MouseEventKind::ScrollRight => Some(SystemEvent::MouseWheel(MouseWheelEvent {
                x,
                y,
                direction: MouseWheelDirection::Right,
            })),
        }
    }

    fn convert_key_code(&self, code: crossterm::event::KeyCode) -> KeyCode {
        match code {
            CrosstermKeyCode::F(1) => KeyCode::F1,
            CrosstermKeyCode::F(2) => KeyCode::F2,
            CrosstermKeyCode::F(3) => KeyCode::F3,
            CrosstermKeyCode::F(4) => KeyCode::F4,
            CrosstermKeyCode::F(5) => KeyCode::F5,
            CrosstermKeyCode::F(6) => KeyCode::F6,
            CrosstermKeyCode::F(7) => KeyCode::F7,
            CrosstermKeyCode::F(8) => KeyCode::F8,
            CrosstermKeyCode::F(9) => KeyCode::F9,
            CrosstermKeyCode::F(10) => KeyCode::F10,
            CrosstermKeyCode::F(11) => KeyCode::F11,
            CrosstermKeyCode::F(12) => KeyCode::F12,
            CrosstermKeyCode::Enter => KeyCode::Enter,
            CrosstermKeyCode::Esc => KeyCode::Escape,
            CrosstermKeyCode::Insert => KeyCode::Insert,
            CrosstermKeyCode::Delete => KeyCode::Delete,
            CrosstermKeyCode::Backspace => KeyCode::Backspace,
            CrosstermKeyCode::Tab => KeyCode::Tab,
            CrosstermKeyCode::Left => KeyCode::Left,
            CrosstermKeyCode::Up => KeyCode::Up,
            CrosstermKeyCode::Down => KeyCode::Down,
            CrosstermKeyCode::Right => KeyCode::Right,
            CrosstermKeyCode::PageUp => KeyCode::PageUp,
            CrosstermKeyCode::PageDown => KeyCode::PageDown,
            CrosstermKeyCode::Home => KeyCode::Home,
            CrosstermKeyCode::End => KeyCode::End,
            CrosstermKeyCode::Char(' ') => KeyCode::Space,
            CrosstermKeyCode::Char('a') | CrosstermKeyCode::Char('A') => KeyCode::A,
            CrosstermKeyCode::Char('b') | CrosstermKeyCode::Char('B') => KeyCode::B,
            CrosstermKeyCode::Char('c') | CrosstermKeyCode::Char('C') => KeyCode::C,
            CrosstermKeyCode::Char('d') | CrosstermKeyCode::Char('D') => KeyCode::D,
            CrosstermKeyCode::Char('e') | CrosstermKeyCode::Char('E') => KeyCode::E,
            CrosstermKeyCode::Char('f') | CrosstermKeyCode::Char('F') => KeyCode::F,
            CrosstermKeyCode::Char('g') | CrosstermKeyCode::Char('G') => KeyCode::G,
            CrosstermKeyCode::Char('h') | CrosstermKeyCode::Char('H') => KeyCode::H,
            CrosstermKeyCode::Char('i') | CrosstermKeyCode::Char('I') => KeyCode::I,
            CrosstermKeyCode::Char('j') | CrosstermKeyCode::Char('J') => KeyCode::J,
            CrosstermKeyCode::Char('k') | CrosstermKeyCode::Char('K') => KeyCode::K,
            CrosstermKeyCode::Char('l') | CrosstermKeyCode::Char('L') => KeyCode::L,
            CrosstermKeyCode::Char('m') | CrosstermKeyCode::Char('M') => KeyCode::M,
            CrosstermKeyCode::Char('n') | CrosstermKeyCode::Char('N') => KeyCode::N,
            CrosstermKeyCode::Char('o') | CrosstermKeyCode::Char('O') => KeyCode::O,
            CrosstermKeyCode::Char('p') | CrosstermKeyCode::Char('P') => KeyCode::P,
            CrosstermKeyCode::Char('q') | CrosstermKeyCode::Char('Q') => KeyCode::Q,
            CrosstermKeyCode::Char('r') | CrosstermKeyCode::Char('R') => KeyCode::R,
            CrosstermKeyCode::Char('s') | CrosstermKeyCode::Char('S') => KeyCode::S,
            CrosstermKeyCode::Char('t') | CrosstermKeyCode::Char('T') => KeyCode::T,
            CrosstermKeyCode::Char('u') | CrosstermKeyCode::Char('U') => KeyCode::U,
            CrosstermKeyCode::Char('v') | CrosstermKeyCode::Char('V') => KeyCode::V,
            CrosstermKeyCode::Char('w') | CrosstermKeyCode::Char('W') => KeyCode::W,
            CrosstermKeyCode::Char('x') | CrosstermKeyCode::Char('X') => KeyCode::X,
            CrosstermKeyCode::Char('y') | CrosstermKeyCode::Char('Y') => KeyCode::Y,
            CrosstermKeyCode::Char('z') | CrosstermKeyCode::Char('Z') => KeyCode::Z,
            CrosstermKeyCode::Char('0') => KeyCode::N0,
            CrosstermKeyCode::Char('1') => KeyCode::N1,
            CrosstermKeyCode::Char('2') => KeyCode::N2,
            CrosstermKeyCode::Char('3') => KeyCode::N3,
            CrosstermKeyCode::Char('4') => KeyCode::N4,
            CrosstermKeyCode::Char('5') => KeyCode::N5,
            CrosstermKeyCode::Char('6') => KeyCode::N6,
            CrosstermKeyCode::Char('7') => KeyCode::N7,
            CrosstermKeyCode::Char('8') => KeyCode::N8,
            CrosstermKeyCode::Char('9') => KeyCode::N9,
            _ => KeyCode::None,
        }
    }

    fn convert_modifiers(&self, modifiers: crossterm::event::KeyModifiers) -> KeyModifier {
        use crossterm::event::KeyModifiers;

        let mut result = KeyModifier::None;

        if modifiers.contains(KeyModifiers::ALT) {
            result |= KeyModifier::Alt;
        }
        if modifiers.contains(KeyModifiers::CONTROL) {
            result |= KeyModifier::Ctrl;
        }
        if modifiers.contains(KeyModifiers::SHIFT) {
            result |= KeyModifier::Shift;
        }

        result
    }

    fn convert_mouse_button(&self, button: CrosstermMouseButton) -> MouseButton {
        match button {
            CrosstermMouseButton::Left => MouseButton::Left,
            CrosstermMouseButton::Right => MouseButton::Right,
            CrosstermMouseButton::Middle => MouseButton::Center,
        }
    }
}
