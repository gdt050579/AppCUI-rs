
#[derive(Copy,Clone,PartialEq, Debug)]
pub(crate) enum Dimension {
    Absolute(u32),
    Percentage(f32)
}
impl Dimension {
    pub(crate) fn from_str(value: &str) -> Option<Dimension> {
        if value.ends_with('%') {
            if let Ok(value) = value[0..value.len()-1].parse::<f32>() {
                return Some(Dimension::Percentage(value/100.0f32));
            }
        } else if let Ok(value) = value.parse::<u32>() {
            return Some(Dimension::Absolute(value));
        }
        None
    }
}