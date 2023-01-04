mod alignament;
mod coordonate;
mod size;
mod layout_information;

use alignament::Alignament;
use coordonate::Coordonate;
use size::Size;
use layout_information::LayoutInformation;

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
