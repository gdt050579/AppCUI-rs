use crate::parameter_parser::NamedParamsMap;

static LAYOUT_PARAMS: [&str;10] = ["x", "y", "left", "top", "right", "bottom", "align", "dock", "width", "height"];
fn copy_layout_params(s: &mut String, params: &NamedParamsMap) {
    let mut one_added = false;
    for p in LAYOUT_PARAMS {
        if let Some(value) = params.get(p) {
            if one_added {
                s.push_str(" , ");
            }
            s.push_str(p);
            s.push(':');
            s.push_str(value.get_string());
            one_added = true;
        }
    }
}
pub(super) fn add_layout(s: &mut String, params: &NamedParamsMap) {
    s.push_str("Layout::new(\"");
    copy_layout_params(s, params);
    s.push_str("\") ");
}
