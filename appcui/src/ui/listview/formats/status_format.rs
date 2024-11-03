use crate::utils::FormatNumber;

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub enum Status {
    Running(f32),
    Queued,
    Paused(f32),
    Stopped,
    Error,
    Completed,
}
impl Ord for Status {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if let Some(res) = self.partial_cmp(other) {
            res
        } else {
            std::cmp::Ordering::Equal
        }
    }
}
impl Eq for Status {
    
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
    #[inline(always)]
    pub(crate) fn proc(value: f32) -> f64 {
        if value < 0.0 {
            0.0
        } else if value > 1.0 {
            100.0
        } else {
            (value * 100.0) as f64
        }
    }
    pub(crate) fn string_representation<'a>(&self, output_buffer: &'a mut [u8]) -> &'a str {
        match self {
            Status::Paused(value) => PERCENTAGE_PAUSED.write_float(Status::proc(*value), output_buffer).unwrap_or(""),
            Status::Completed => "Completed",
            Status::Running(value) => PERCENTAGE_RUNNING.write_float(Status::proc(*value), output_buffer).unwrap_or(""),
            Status::Error => "Error",
            Status::Queued => "Queued",
            Status::Stopped => "Stopped",
        }
    }
}
