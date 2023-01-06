mod alignament;
mod anchors;
mod coordonate;
mod size;
mod layout_parameters;
mod layout_mode;
mod parameter;

use alignament::Alignament;
use anchors::Anchors;
use coordonate::Coordonate;
use size::Size;
use layout_parameters::LayoutParameters;
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
