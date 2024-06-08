#[derive(Copy,Clone,PartialEq, Debug)]
pub(crate) enum Coordonate {
    Absolute(i32),
    Percentage(f32),
}

impl Coordonate {
    pub(crate) fn from_str(value: &str) -> Option<Coordonate> {
        if value.ends_with('%') {
            if let Ok(value) = value[0..value.len() - 1].parse::<f32>() {
                return Some(Coordonate::Percentage(value / 100.0f32));
            }
        } else if let Ok(value) = value.parse::<i32>() {
            return Some(Coordonate::Absolute(value));
        }
        None
    }
}
 