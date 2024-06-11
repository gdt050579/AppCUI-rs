use super::control_builder::ControlBuilder;
use crate::parameter_parser::*;
use proc_macro::*;

static RESIZE_BEHAVIOR: FlagsSignature = FlagsSignature::new(&["PreserveAspectRatio", "PreserveTopPanelSize", "PreserveBottomPanelSize"]);

static POSILITIONAL_PARAMETERS: &[PositionalParameter] = &[PositionalParameter::new("pos", ParamType::String)];
static NAMED_PARAMETERS: &[NamedParameter] = &[
    NamedParameter::new("pos", "pos", ParamType::Coordonate),
    NamedParameter::new("resize", "resize", ParamType::String),
    NamedParameter::new("on-resize", "resize", ParamType::String),
    NamedParameter::new("resize-behavior", "resize", ParamType::String),
    NamedParameter::new("rb", "resize", ParamType::String),
    NamedParameter::new("min-top-height", "mthbh", ParamType::Dimension),
    NamedParameter::new("mintopheight", "mth", ParamType::Dimension),
    NamedParameter::new("mth", "mth", ParamType::Dimension),
    NamedParameter::new("min-bottom-height", "mbh", ParamType::Dimension),
    NamedParameter::new("minbottomheight", "mbh", ParamType::Dimension),
    NamedParameter::new("mbh", "mbh", ParamType::Dimension),
];

pub(crate) fn create(input: TokenStream) -> TokenStream {
    let mut cb = ControlBuilder::new("hsplitter", input, POSILITIONAL_PARAMETERS, NAMED_PARAMETERS, true);
    cb.init_control("HSplitter::new");
    cb.add_coordonate_parameter("pos", None);
    cb.add_layout();
    cb.add_enum_parameter("resize", "hsplitter::ResizeBehavior", &RESIZE_BEHAVIOR, Some("PreserveAspectRatio"));
    cb.finish_control_initialization();
    if cb.has_parameter("mth") {
        cb.add("control.set_min_height(hsplitter::Panel::Top");
        cb.add_dimension_parameter("mth", None);
        cb.add(");\n");
    }
    if cb.has_parameter("mbh") {
        cb.add("control.set_min_height(hsplitter::Panel::Bottom");
        cb.add_dimension_parameter("mbh", None);
        cb.add(");\n");
    }
    cb.add_basecontrol_operations();
    cb.into()
}
