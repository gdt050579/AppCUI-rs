use crate::input::{MouseButton, MouseWheelDirection, Key, KeyModifier, KeyCode};

#[derive(Debug)]
pub(super) struct ParserError {
    error: String,
    line: String,
    start: Option<usize>,
    end: Option<usize>,
}
impl ParserError {
    pub(super) fn from_parser(error: &str, line: &str, start: Option<usize>, end: Option<usize>) -> Self {
        Self {
            error: String::from(error),
            line: String::from(line),
            start,
            end,
        }
    }
    pub(super) fn new(error: &str) -> Self {
        Self {
            error: String::from(error),
            line: String::new(),
            start: None,
            end: None,
        }
    }
    pub(super) fn get_error(&self) -> &str {
        return &self.error.as_str();
    }
    pub(super) fn to_string(&self) -> String {
        let mut err = String::with_capacity(256);
        err.push_str("Command parsing error: ");
        err.push_str(self.get_error());
        err.push_str("\n for line: \"");
        err.push_str(self.line.as_str());
        err.push_str("\"\n");
        if let (Some(s), Some(e)) = (self.start, self.end) {
            // add 12 spaces
            for _ in 0..12 {
                err.push(' ');
            }
            for (index, _) in self.line.as_str().char_indices() {
                if (index >= s) && (index < e) {
                    err.push('^');
                } else {
                    err.push(' ');
                }
            }
            err.push('\n');
        }
        err
    }
}

#[derive(Debug)]
pub(super) struct CommandParser<'a> {
    command: &'a str,
    params: [&'a str; 4],
    count: usize,
}
impl<'a> CommandParser<'a> {
    pub(super) fn new(command: &'a str) -> Result<Self, ParserError> {
        let mut cp = Self {
            command: "",
            params: ["", "", "", ""],
            count: 0,
        };
        cp.parse(command)?;
        Ok(cp)
    }
    fn is_space(value: u8) -> bool {
        (value == b' ') || (value == b'\t')
    }
    fn is_command(value: u8) -> bool {
        match value {
            b'a'..=b'z' => true,
            b'A'..=b'Z' => true,
            b'.' => true,
            _ => false,
        }
    }
    fn is_word(value: u8) -> bool {
        match value {
            b'a'..=b'z' => true,
            b'A'..=b'Z' => true,
            b'.' => true,
            b'_' => true,
            b'-' => true,
            b'+' => true,
            b'0'..=b'9' => true,
            _ => false,
        }
    }
    fn skip(buf: &[u8], start: usize, f: fn(u8) -> bool) -> usize {
        let len = buf.len();
        let mut start = start;
        while (start < len) && (f(buf[start])) {
            start += 1;
        }
        return start;
    }
    pub(super) fn get_command(&self) -> &str {
        self.command
    }
    pub(super) fn get_params_count(&self) -> usize {
        self.count
    }
    pub(super) fn get_param(&self, index: usize) -> Option<&'a str> {
        if index >= self.count {
            return None;
        }
        return Some(self.params[index]);
    }
    pub(super) fn get_bool(&self, index: usize) -> Option<bool> {
        if index >= self.count {
            return None;
        }
        match self.params[index] {
            "true" => Some(true),
            "false" => Some(false),
            _ => None,
        }
    }
    pub(super) fn get_key(&self, index: usize) -> Option<Key> {
        if index >= self.count {
            return None;
        }
        let mut k = Key::default();
        for part in self.params[index].split('+') {
            match part {
                "Ctrl" => k.modifier |= KeyModifier::Ctrl,
                "Alt" => k.modifier |= KeyModifier::Alt,
                "Shift" => k.modifier |= KeyModifier::Shift,
                "F1" => k.code = KeyCode::F1,
                "F2" => k.code = KeyCode::F2,
                "F3" => k.code = KeyCode::F3,
                "F4" => k.code = KeyCode::F4,
                "F5" => k.code = KeyCode::F5,
                "F6" => k.code = KeyCode::F6,
                "F7" => k.code = KeyCode::F7,
                "F8" => k.code = KeyCode::F8,
                "F9" => k.code = KeyCode::F9,
                "F10" => k.code = KeyCode::F10,
                "F11" => k.code = KeyCode::F11,
                "F12" => k.code = KeyCode::F12,
                "Enter" => k.code = KeyCode::Enter,
                "Escape" => k.code = KeyCode::Escape,
                "Insert" => k.code = KeyCode::Insert,
                "Delete" => k.code = KeyCode::Delete,
                "Backspace" => k.code = KeyCode::Backspace,
                "Tab" => k.code = KeyCode::Tab,
                "Left" => k.code = KeyCode::Left,
                "Up" => k.code = KeyCode::Up,
                "Down" => k.code = KeyCode::Down,
                "Right" => k.code = KeyCode::Right,
                "PageUp" => k.code = KeyCode::PageUp,
                "PageDown" => k.code = KeyCode::PageDown,
                "Home" => k.code = KeyCode::Home,
                "End" => k.code = KeyCode::End,
                "Space" => k.code = KeyCode::Space,
                "A" => k.code = KeyCode::A,
                "B" => k.code = KeyCode::B,
                "C" => k.code = KeyCode::C,
                "D" => k.code = KeyCode::D,
                "E" => k.code = KeyCode::E,
                "F" => k.code = KeyCode::F,
                "G" => k.code = KeyCode::G,
                "H" => k.code = KeyCode::H,
                "I" => k.code = KeyCode::I,
                "J" => k.code = KeyCode::J,
                "K" => k.code = KeyCode::K,
                "L" => k.code = KeyCode::L,
                "M" => k.code = KeyCode::M,
                "N" => k.code = KeyCode::N,
                "O" => k.code = KeyCode::O,
                "P" => k.code = KeyCode::P,
                "Q" => k.code = KeyCode::Q,
                "R" => k.code = KeyCode::R,
                "S" => k.code = KeyCode::S,
                "T" => k.code = KeyCode::T,
                "U" => k.code = KeyCode::U,
                "V" => k.code = KeyCode::V,
                "W" => k.code = KeyCode::W,
                "X" => k.code = KeyCode::X,
                "Y" => k.code = KeyCode::Y,
                "Z" => k.code = KeyCode::Z,
                "0" => k.code = KeyCode::N0,
                "1" => k.code = KeyCode::N1,
                "2" => k.code = KeyCode::N2,
                "3" => k.code = KeyCode::N3,
                "4" => k.code = KeyCode::N4,
                "5" => k.code = KeyCode::N5,
                "6" => k.code = KeyCode::N6,
                "7" => k.code = KeyCode::N7,
                "8" => k.code = KeyCode::N8,
                "9" => k.code = KeyCode::N9,
                _ => { return None; }
            }

        }   
        Some(k)   
    }
    pub(super) fn get_mouse_button(&self, index: usize) -> Option<MouseButton> {
        if index >= self.count {
            return None;
        }
        match self.params[index] {
            "left" => Some(MouseButton::Left),
            "right" => Some(MouseButton::Right),
            "center" => Some(MouseButton::Center),
            _ => None,
        }
    }
    pub(super) fn get_mouse_wheel(&self, index: usize) -> Option<MouseWheelDirection> {
        if index >= self.count {
            return None;
        }
        match self.params[index] {
            "left" => Some(MouseWheelDirection::Left),
            "right" => Some(MouseWheelDirection::Right),
            "up" => Some(MouseWheelDirection::Up),
            "down" => Some(MouseWheelDirection::Down),
            _ => None,
        }
    }
    pub(super) fn get_i32(&self, index: usize) -> Option<i32> {
        if index >= self.count {
            return None;
        }
        if let Ok(value) = self.params[index].parse::<i32>() {
            return Some(value);
        }
        return None;
    }
    pub(super) fn get_hash(&self, index: usize) -> Option<u64> {
        if index >= self.count {
            return None;
        }
        let txt = self.params[index];
        if !txt.starts_with("0x") {
            return None;
        }
        if let Ok(value) = u64::from_str_radix(&txt[2..],16) {
            return Some(value);
        }
        return None;
    }
    pub(super) fn parse(&mut self, command: &'a str) -> Result<(), ParserError> {
        let buf = command.as_bytes();
        let len = buf.len();
        let mut poz = 0usize;

        self.command = "";
        self.count = 0;
        self.params[0] = "";
        self.params[1] = "";
        self.params[2] = "";
        self.params[3] = "";
        // first the comman
        poz = CommandParser::skip(buf, poz, CommandParser::is_space);

        if poz >= len {
            return Err(ParserError::from_parser(
                "Expecting a valid command (not an empty line)",
                command,
                None,
                None,
            ));
        }
        let next = CommandParser::skip(buf, poz, CommandParser::is_command);
        self.command = &command[poz..next];
        poz = CommandParser::skip(buf, next, CommandParser::is_space);
        if poz >= len {
            // no parameters
            return Ok(());
        }
        // we expect '('
        if buf[poz] != b'(' {
            return Err(ParserError::from_parser(
                "Expecting '(' after the command !",
                command,
                Some(poz),
                Some(poz + 1),
            ));
        }
        let parantheze_poz = poz;
        poz += 1;
        loop {
            // skip some spaces
            poz = CommandParser::skip(buf, poz, CommandParser::is_space);
            // if we reached the end of the code --> error
            if poz >= len {
                return Err(ParserError::from_parser(
                    "Expecting ')' after the '(' ",
                    command,
                    Some(parantheze_poz),
                    Some(parantheze_poz + 1),
                ));
            }
            match buf[poz] {
                b')' => {
                    // found the ending ')'
                    return Ok(());
                }
                b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'-' | b'+' => {
                    if self.count >= 4 {
                        return Err(ParserError::from_parser(
                            "Too many parameters (max allowed is 4)",
                            command,
                            None,
                            None,
                        ));
                    }
                    let next = CommandParser::skip(buf, poz, CommandParser::is_word);
                    self.params[self.count] = &command[poz..next];
                    //println!("FOUND WORD: {}", self.params[self.count]);
                    poz = next;
                    self.count += 1;
                }
                b',' => {
                    return Err(ParserError::from_parser(
                        "Expecting a word but found ',' separator !",
                        command,
                        Some(poz),
                        Some(poz + 1),
                    ));
                }
                b'"' | b'\'' => {
                    // search for the first string end
                    let string_char = buf[poz];
                    let mut next = poz + 1;
                    while (next < len) && (buf[next] != string_char) {
                        next += 1;
                    }
                    if next >= len {
                        return Err(ParserError::from_parser(
                            "Invalid string (no ending '\"' character found)",
                            command,
                            Some(poz),
                            Some(len),
                        ));
                    }
                    if self.count >= 4 {
                        return Err(ParserError::from_parser(
                            "Too many parameters (max allowed is 4)",
                            command,
                            None,
                            None,
                        ));
                    }
                    self.params[self.count] = &command[poz + 1..next];
                    //println!("FOUND STRING: {}", self.params[self.count]);
                    poz = next + 1;
                    self.count += 1;
                }
                _ => {
                    return Err(ParserError::from_parser(
                        "Invalid character (expecting a word)",
                        command,
                        Some(poz),
                        Some(poz + 1),
                    ));
                }
            }
            // skip some spaces
            poz = CommandParser::skip(buf, poz, CommandParser::is_space);
            if (poz < len) && (buf[poz] == b',') {
                poz += 1;
            }
        }
    }
}
impl<'a> Default for CommandParser<'a> {
    fn default() -> Self {
        Self {
            command: "",
            params: ["", "", "", ""],
            count: 0,
        }
    }
}
