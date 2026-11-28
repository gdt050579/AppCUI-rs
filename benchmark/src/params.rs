use appcui::graphics::Size;

pub(crate) struct Params {
    pub(crate) times: u32,
    pub(crate) filter: String,
    pub(crate) screen_size: Size,
}
impl Params {
    pub(crate) fn new(args: &[String]) -> Result<Self, String> {
        let mut me = Self { times: 10, filter: String::new(), screen_size: Size::new(180, 60) };
        let mut index = 1; // first argument is the command
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
                "--screen-size" => {
                    me.screen_size = Self::parse_screen_size(args, index + 1)?;
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
    fn parse_screen_size(arg: &[String], index: usize) -> Result<Size, String> {
        if index >= arg.len() {
            return Err("Missing argument for --screen-size".to_string());
        }
        if arg[index].contains('x') {
            let (w, h) = arg[index].split_once('x').unwrap();
            let width = w.parse::<u32>().map_err(|e| e.to_string())?;
            let height = h.parse::<u32>().map_err(|e| e.to_string())?;
            if (width == 0) || (height == 0) || (width > 1000) || (height > 1000) {
                return Err("Screen size (width or height) must be between 1 and 1000".to_string());
            }
            Ok(Size::new(width, height))
        } else {
            Err("Screen size must be in the format WIDTHxHEIGHT (e.g. 180x60)".to_string())
        }
    }
}