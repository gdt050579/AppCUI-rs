use flat_string::FlatString;

enum SetData {
    Interval(u32),
    List(Vec<char>),
}
pub struct Set {
    name: FlatString<22>,
    count: u32,
    data: SetData,
}
impl Set {
    pub fn with_interval(name: &str, start_code_point: u32, end_code_point: u32) -> Option<Self> {
        let start = start_code_point.min(end_code_point);
        let end = start_code_point.max(end_code_point);
        if (char::from_u32(start).is_none()) || (char::from_u32(start).is_none()) {
            // invalid characters
            return None;
        }
        Some(Self {
            name: FlatString::from_str(name),
            count: end + 1 - start,
            data: SetData::Interval(start),
        })
    }
    pub fn new(name: &str, string_list: &str) -> Option<Self> {
        if string_list.is_empty() {
            return None;
        }
        let v: Vec<char> = string_list.chars().collect();
        Some(Self {
            name: FlatString::from_str(name),
            count: v.len() as u32,
            data: SetData::List(v),
        })
    }

    #[inline(always)]
    pub(super) fn count(&self) -> u32 {
        self.count
    }

    #[inline(always)]
    pub(super) fn char(&self, index: u32) -> Option<char> {
        if index >= self.count {
            return None;
        }
        match &self.data {
            SetData::Interval(start) => char::from_u32(start + index),
            SetData::List(items) => Some(items[index as usize]),
        }
    }

    #[inline(always)]
    pub(super) fn name(&self) -> &str {
        self.name.as_str()
    }
}
