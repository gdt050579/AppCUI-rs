pub(crate) struct Params {
    pub(crate) times: u32,
    pub(crate) filter: String,
}
impl Params {
    pub(crate) fn new(args: &[String]) -> Result<Self, String> {
        let mut me = Self { times: 10, filter: String::new() };
        let mut index = 0;
        while index < args.len() {
            let arg = args[index].as_str();
            match arg {
                "--times" => {
                    me.times = Self::parse_times(args, index + 1)?;
                    index += 2;
                }
                "--filter" => {
                    me.filter = Self::parse_filter(args, index + 1)?;
                    index += 2;
                }
                _ => {
                    return Err(format!("Unknown argument: {}", arg));
                }
            }
        }

        Ok(me)
    }
    fn parse_times(arg: &[String], index: usize) -> Result<u32, String> {
        if index >= arg.len() {
            return Err("Missing argument for --times".to_string());
        }
        let value = arg[index].parse::<u32>().map_err(|e| e.to_string())?;
        if value == 0 {
            return Err("Times must be greater than 0".to_string());
        }
        Ok(value)
    }
    fn parse_filter(arg: &[String], index: usize) -> Result<String, String> {
        if index >= arg.len() {
            return Err("Missing argument for --filter".to_string());
        }
        Ok(arg[index].clone())
    }
}