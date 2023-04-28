struct CommandParser {
    command: &str,
    params: [&str;3],
    count: u32
}
impl CommandParser {
    fn new()->Self {
        Self {
            command: "",
            params: ["","",""],
            count: 0
        }
    }
}