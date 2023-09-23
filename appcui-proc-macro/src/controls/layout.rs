use crate::parameter_parser::NamedParamsMap;

pub (super) fn add_layout(s: &mut String, params: &NamedParamsMap) {
    s.push_str("Layout::new(\"d:c\")");
}