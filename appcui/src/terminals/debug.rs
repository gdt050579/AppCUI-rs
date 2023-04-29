mod terminal;
mod command_parser;
mod mouse_hold_command;
mod mouse_release_command;
mod mouse_click_command;
mod mouse_move_command;
mod mouse_drag_command;
mod mouse_wheel_command;
mod paint_command;
mod command;

#[cfg(test)]
mod tests;

pub (crate) use self::terminal::DebugTerminal;