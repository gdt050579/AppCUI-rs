pub(crate) enum Value {
    None,
    Percent(f32),
    Integer(i32),
    Error(String),
}

impl Value {
    pub(crate) fn new(value: &str, field_name: &str, should_be_positive: bool) -> Self {
        let text = value.trim();
        if text.is_empty() {
            return Value::None;
        }
        if text.ends_with('%') {
            if let Ok(percent) = text[..text.len() - 1].parse::<f32>() {
                if should_be_positive && percent < 0.0f32 {
                    return Value::Error(format!("Negative percentage value are not allowed for: {field_name} ({text})"));
                }
                return Value::Percent(percent / 100.0f32);
            } else {
                return Value::Error(format!("Invalid percentage value: {field_name} ({text})"));
            }
        }
        if let Ok(integer) = text.parse::<i32>() {
            if should_be_positive && integer < 0 {
                return Value::Error(format!("Negative value are not allowed for: {field_name} ({text})"));
            }
            return Value::Integer(integer);
        }
        Value::Error(format!("Invalid value: {field_name} ({text}) (expected integer or percentage)"))
    }
}
