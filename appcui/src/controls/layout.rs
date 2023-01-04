mod alignament;
mod coordonate;
mod size;
mod layout_information;
mod parameter;

use alignament::Alignament;
use coordonate::Coordonate;
use size::Size;
use layout_information::LayoutInformation;
use parameter::Parameter;

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
