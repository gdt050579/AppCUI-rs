use super::control_builder::ControlBuilder;
use crate::parameter_parser::*;
use proc_macro::*;

static RESIZE_BEHAVIOR: FlagsSignature = FlagsSignature::new(&["PreserveAspectRatio", "PreserveLeftPanelSize", "PreserveRightPanelSize"]);

static POSILITIONAL_PARAMETERS: &[PositionalParameter] = &[PositionalParameter::new("pos", ParamType::String)];
static NAMED_PARAMETERS: &[NamedParameter] = &[
    NamedParameter::new("pos", "pos", ParamType::Coordonate),
    NamedParameter::new("resize", "resize", ParamType::String),
    NamedParameter::new("on-resize", "resize", ParamType::String),
    NamedParameter::new("resize-behavior", "resize", ParamType::String),
    NamedParameter::new("rb", "resize", ParamType::String),
    NamedParameter::new("min-left-width", "mlw", ParamType::Dimension),
    NamedParameter::new("minleftwidth", "mlw", ParamType::Dimension),
    NamedParameter::new("mlw", "mlw", ParamType::Dimension),
    NamedParameter::new("min-right-width", "mrw", ParamType::Dimension),
    NamedParameter::new("minrightwidth", "mrw", ParamType::Dimension),
    NamedParameter::new("mrw", "mrw", ParamType::Dimension),
];

pub(crate) fn create(input: TokenStream) -> TokenStream {
    let mut cb = ControlBuilder::new("vsplitter", input, POSILITIONAL_PARAMETERS, NAMED_PARAMETERS, true);
    cb.init_control("VSplitter::new");
    cb.add_coordonate_parameter("pos", None);
    cb.add_layout();
    cb.add_enum_parameter("resize", "vsplitter::ResizeBehavior", &RESIZE_BEHAVIOR, Some("PreserveAspectRatio"));
    cb.finish_control_initialization();
    if cb.has_parameter("mlw") {
        cb.add("control.set_min_width(vsplitter::Panel::Left");
        cb.add_dimension_parameter("mlw", None);
        cb.add(");\n");
    }
    if cb.has_parameter("mrw") {
        cb.add("control.set_min_width(vsplitter::Panel::Right");
        cb.add_dimension_parameter("mrw", None);
        cb.add(");\n");
    }
    cb.add_basecontrol_operations();
    cb.into()
}
