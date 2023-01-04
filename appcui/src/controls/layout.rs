use super::Alignament;

pub struct Layout<'a> {
    format: &'a str
}
impl Layout<'_> {
    pub fn new(format: &str)->Layout {
        Layout {format: format}
    }
}

pub (in crate) struct ControlLayout {

}
