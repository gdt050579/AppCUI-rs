pub(super) struct CommandParser<'a> {
    command: &'a str,
    params: [&'a str; 3],
    count: usize,
}
impl<'a> CommandParser<'a> {
    pub(super) fn new(command: &'a str) -> Result<Self, &'static str> {
        let mut cp = Self {
            command: "",
            params: ["", "", ""],
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
    pub(super) fn parse(&mut self, command: &'a str) -> Result<(), &'static str> {
        let buf = command.as_bytes();
        let len = buf.len();
        let mut poz = 0usize;

        self.command = "";
        self.count = 0;
        self.params[0] = "";
        self.params[1] = "";
        self.params[2] = "";
        // first the comman
        poz = CommandParser::skip(buf, poz, CommandParser::is_space);

        if poz >= len {
            return Err("Expecting a command !");
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
            return Err("Expecting '(' after the command !");
        }
        poz += 1;
        loop {
            // skip some spaces
            poz = CommandParser::skip(buf, poz, CommandParser::is_space);
            // if we reached the end of the code --> error
            if poz >= len {
                return Err("Expecting ')' after the '(' ");
            }
            match buf[poz] {
                b')' => {
                    // found the ending ')'
                    return Ok(());
                }
                b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' => {
                    if self.count >= 3 {
                        return Err("Too many parameters (max allowed is 3)");
                    }
                    let next = CommandParser::skip(buf, poz, CommandParser::is_word);
                    self.params[self.count] = &command[poz..next];
                    poz = next;
                    self.count += 1;
                }
                b',' => {
                    return Err("Expecting a word but found ',' separator !");
                }
                _ => {
                    return Err("Invalid character (expecting a word)");
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
            params: ["", "", ""],
            count: 0,
        }
    }
}
