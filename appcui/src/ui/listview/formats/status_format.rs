use crate::utils::FormatNumber;

#[derive(Copy, Clone, PartialEq)]
pub enum Status {
    Running(f32),
    Queued,
    Paused(f32),
    Stopped,
    Error,
    Completed,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum StatusFormat {
    Hashtag,
    Graphical,
    Arrow,
}

const PERCENTAGE_RUNNING: FormatNumber = FormatNumber::new(10).suffix("%").fill(4, b' ');
const PERCENTAGE_PAUSED: FormatNumber = FormatNumber::new(10).suffix("%)").prefix("Paused (");


impl Status {
    pub(crate) fn to_str<'a>(&self, output_buffer: &'a mut [u8]) -> &'a str {
        match self {
            Status::Paused(value) => PERCENTAGE_PAUSED.write_float(*value as f64, output_buffer).unwrap_or(""),
            Status::Completed => "Completed",
            Status::Running(value) => PERCENTAGE_RUNNING.write_float(*value as f64, output_buffer).unwrap_or(""),
            Status::Error => "Error",
            Status::Queued => "Queued",
            Status::Stopped => "Stopped",
        }
    }  
}