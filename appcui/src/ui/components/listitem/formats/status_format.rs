use crate::utils::FormatNumber;

#[derive(Copy, Clone)]
pub enum Status {
    Running(f32),
    Queued,
    Paused(f32),
    Stopped,
    Error,
    Completed,
}
impl Status {
    fn order(&self) -> u32 {
        match self {
            Status::Running(_) => 0,
            Status::Paused(_) => 1,
            Status::Queued => 2,
            Status::Stopped => 3,
            Status::Error => 4,
            Status::Completed => 5,
        }
    }
    fn value(&self) -> f32 {
        match self {
            Status::Running(value) => *value,
            Status::Paused(value) => *value,
            _ => 0.0,
        }
    }
}
impl PartialEq for Status {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Status::Running(_) => {
                matches!(other, Status::Running(_))
            }
            Status::Paused(_) => {
                matches!(other, Status::Paused(_))
            }
            Status::Queued => {
                matches!(other, Status::Queued)
            }
            Status::Stopped => {
                matches!(other, Status::Stopped)
            }
            Status::Error => {
                matches!(other, Status::Error)
            }
            Status::Completed => {
                matches!(other, Status::Completed)
            }
        }
    }
}
impl Eq for Status {}
impl Ord for Status {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_order = self.order();
        let other_order = other.order();
        match self_order.cmp(&other_order) {
            std::cmp::Ordering::Equal => {
                match self {
                    Status::Running(value) => {
                        if let Some(result) = value.partial_cmp(&other.value()) {
                            result
                        } else {
                            std::cmp::Ordering::Equal
                        }
                    }
                    Status::Paused(value) => {
                        if let Some(result) = value.partial_cmp(&other.value()) {
                            result
                        } else {
                            std::cmp::Ordering::Equal
                        }
                    }
                    _ => std::cmp::Ordering::Equal,
                }
            }
            other => other,
        }
    }
}
impl PartialOrd for Status {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum StatusFormat {
    Hashtag,
    Graphical,
    Arrow,
    Block,
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
