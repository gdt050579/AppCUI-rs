use super::control_builder::ControlBuilder;
use crate::parameter_parser::*;
use proc_macro::*;

static mut COLORS_TYPE: FlagsSignature = FlagsSignature::new(&[
    "Black",
    "DarkBlue",
    "DarkGreen",
    "Teal",
    "DarkRed",
    "Magenta",
    "Olive",
    "Silver",
    "Gray",
    "Blue",
    "Green",
    "Aqua",
    "Red",
    "Pink",
    "Yellow",
    "White",
    "Transparent",
]);

static POSILITIONAL_PARAMETERS: &[PositionalParameter] = &[PositionalParameter::new("color", ParamType::String)];
static NAMED_PARAMETERS: &[NamedParameter] = &[NamedParameter::new("color", "color", ParamType::String)];

pub(crate) fn create(input: TokenStream) -> TokenStream {
    let mut cb = ControlBuilder::new("colorpicker", input, POSILITIONAL_PARAMETERS, NAMED_PARAMETERS, true);
    cb.init_control("ColorPicker::new");
    cb.add_enum_parameter("color", "Color", unsafe { &mut COLORS_TYPE }, None);
    cb.add_layout();    
    cb.finish_control_initialization();
    cb.add_basecontrol_operations();
    cb.into()
}
