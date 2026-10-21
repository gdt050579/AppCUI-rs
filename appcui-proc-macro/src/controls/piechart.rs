use super::control_builder::ControlBuilder;
use crate::bar::{parse_bar_attr, parse_custom_char};
use crate::parameter_parser::*;
use proc_macro::*;

static FLAGS: FlagsSignature = FlagsSignature::new(&["DimSlicesOnSelection"]);

static SHAPES: &[(&str, &str)] = &[
    ("Circle", "circle"),
    ("Square", "square"),
    ("RoundSquare", "roundsquare"),
    ("RoundSquare", "round-square"),
    ("Rectangle", "rectangle"),
    ("Rectangle", "rect"),
    ("RoundRectangle", "roundrectangle"),
    ("RoundRectangle", "round-rectangle"),
    ("RoundRectangle", "roundrect"),
    ("RoundRectangle", "round-rect"),
    ("Ellipse", "ellipse"),
    ("Triangle", "triangle"),
    ("RoundTriangle", "roundtriangle"),
    ("RoundTriangle", "round-triangle"),
];

static RENDERERS: &[(&str, &str)] = &[
    ("HalfBlocks", "halfblocks"),
    ("HalfBlocks", "half-blocks"),
    ("Solid", "solid"),
    ("Shade75", "shade75"),
    ("Shade50", "shade50"),
    ("Shade25", "shade25"),
    ("Grid", "grid"),
    ("GridDouble", "griddouble"),
    ("GridDouble", "grid-double"),
    ("CrossHatch", "crosshatch"),
    ("CrossHatch", "cross-hatch"),
    ("DiagonalUp", "diagonalup"),
    ("DiagonalUp", "diagonal-up"),
    ("DiagonalDown", "diagonaldown"),
    ("DiagonalDown", "diagonal-down"),
    ("Braille", "braille"),
    ("Custom", "custom"),
];

static HOLLOW_KINDS: &[(&str, &str)] = &[
    ("None", "none"),
    ("Percentage", "percentage"),
    ("Percentage", "percent"),
    ("Absolute", "absolute"),
    ("Absolute", "abs"),
];

static LABEL_LAYOUTS: &[(&str, &str)] = &[("None", "none"), ("Inside", "inside"), ("Outside", "outside")];

static LABEL_CONTENTS: &[(&str, &str)] = &[
    ("Value", "value"),
    ("Label", "label"),
    ("Percentage", "percentage"),
    ("Percentage", "percent"),
    ("LabelAndValue", "labelandvalue"),
    ("LabelAndValue", "label-and-value"),
    ("LabelAndValue", "label+value"),
    ("LabelAndPercentage", "labelandpercentage"),
    ("LabelAndPercentage", "label-and-percentage"),
    ("LabelAndPercentage", "label+percentage"),
];

static DIRECTIONS: &[(&str, &str)] = &[
    ("Clockwise", "clockwise"),
    ("Clockwise", "cw"),
    ("CounterClockwise", "counterclockwise"),
    ("CounterClockwise", "counter-clockwise"),
    ("CounterClockwise", "ccw"),
];

static SLICE_POSITIONAL: &[PositionalParameter] = &[
    PositionalParameter::new("value", ParamType::String),
    PositionalParameter::new("label", ParamType::String),
];
static SLICE_NAMED: &[NamedParameter] = &[
    NamedParameter::new("value", "value", ParamType::String),
    NamedParameter::new("val", "value", ParamType::String),
    NamedParameter::new("v", "value", ParamType::String),
    NamedParameter::new("label", "label", ParamType::String),
    NamedParameter::new("text", "label", ParamType::String),
    NamedParameter::new("caption", "label", ParamType::String),
    NamedParameter::new("attr", "attr", ParamType::String),
    NamedParameter::new("attribute", "attr", ParamType::String),
    NamedParameter::new("charattr", "attr", ParamType::String),
    NamedParameter::new("color", "attr", ParamType::String),
];

static POSILITIONAL_PARAMETERS: &[PositionalParameter] = &[];
static NAMED_PARAMETERS: &[NamedParameter] = &[
    NamedParameter::new("values", "values", ParamType::List),
    NamedParameter::new("data", "values", ParamType::List),
    NamedParameter::new("slices", "values", ParamType::List),
    NamedParameter::new("flags", "flags", ParamType::Flags),
    // shape
    NamedParameter::new("shape", "shape", ParamType::String),
    // renderer
    NamedParameter::new("renderer", "renderer", ParamType::String),
    NamedParameter::new("render", "renderer", ParamType::String),
    NamedParameter::new("draw-mode", "renderer", ParamType::String),
    NamedParameter::new("dm", "renderer", ParamType::String),
    // hole
    NamedParameter::new("hollow-thickness", "hollow", ParamType::String),
    NamedParameter::new("hollow", "hollow", ParamType::String),
    NamedParameter::new("hole", "hollow", ParamType::String),
    // labels
    NamedParameter::new("label-layout", "label-layout", ParamType::String),
    NamedParameter::new("labels", "label-layout", ParamType::String),
    NamedParameter::new("ll", "label-layout", ParamType::String),
    NamedParameter::new("label-content", "label-content", ParamType::String),
    NamedParameter::new("content", "label-content", ParamType::String),
    NamedParameter::new("lc", "label-content", ParamType::String),
    // numeric format
    NamedParameter::new("numeric-format", "numeric-format", ParamType::String),
    NamedParameter::new("nf", "numeric-format", ParamType::String),
    // rotation
    NamedParameter::new("start-angle", "start-angle", ParamType::Integer),
    NamedParameter::new("angle", "start-angle", ParamType::Integer),
    NamedParameter::new("direction", "direction", ParamType::String),
    NamedParameter::new("dir", "direction", ParamType::String),
    // selection
    NamedParameter::new("selected-slice", "selected", ParamType::Integer),
    NamedParameter::new("selected", "selected", ParamType::Integer),
    NamedParameter::new("select", "selected", ParamType::Integer),
];

pub(crate) fn create(input: TokenStream) -> TokenStream {
    let mut cb = ControlBuilder::new("piechart", input, POSILITIONAL_PARAMETERS, NAMED_PARAMETERS, true);
    cb.init_control("PieChart::new");
    cb.add_layout();
    cb.add_flags_parameter("flags", "piechart::Flags", &FLAGS);
    cb.finish_control_initialization();
    cb.call_method_with_string_parameter_parser("set_shape", "shape", |repr| {
        format!("piechart::DrawModeShape::{}", find("shape", SHAPES, repr))
    });
    cb.call_method_with_string_parameter_parser("set_renderer", "renderer", parse_renderer);
    cb.call_method_with_string_parameter_parser("set_hollow_thickness", "hollow", parse_hollow);
    cb.call_method_with_string_parameter_parser("set_label_layout", "label-layout", |repr| {
        format!("piechart::LabelLayout::{}", find("label-layout", LABEL_LAYOUTS, repr))
    });
    cb.call_method_with_string_parameter_parser("set_label_content", "label-content", |repr| {
        format!("piechart::LabelContent::{}", find("label-content", LABEL_CONTENTS, repr))
    });
    cb.call_method_with_integer_parameter_and_range("set_start_angle", "start-angle", 0, 360);
    cb.call_method_with_string_parameter_parser("set_direction", "direction", |repr| {
        format!("piechart::Direction::{}", find("direction", DIRECTIONS, repr))
    });
    if cb.has_parameter("numeric-format") {
        let str_repr = String::from(cb.get_string_representation());
        let tmp = if let Some(d) = cb.get_dict("numeric-format") {
            crate::numericformat::builder::create_from_dict(&str_repr, d)
        } else if let Some(v) = cb.get_value("numeric-format") {
            let mut parsed = crate::parameter_parser::parse(v).unwrap_or_else(|e| {
                panic!("Invalid numeric-format: {v} !{e:?}");
            });
            crate::numericformat::builder::create_from_dict(v, &mut parsed)
        } else {
            panic!("Invalid numeric-format ! Expected a number format (e.g. 'dec', 'hex, prefix: 0x' or '{{dec, group: 3}}')");
        };
        cb.add("control.set_number_format(");
        cb.add(&tmp);
        cb.add(");\n");
    }
    // the slices are added after the settings, so that every setting is applied only once
    if cb.has_parameter("values") {
        let Some(list) = cb.get_list("values") else {
            panic!("Parameter 'values' in piechart must be a list of slices: values: [10, {{20, 'Label', attr: red}}, ...] !");
        };
        let mut code = String::with_capacity(64 * list.len());
        let mut temp_s = String::with_capacity(16);
        for item in list.iter_mut() {
            temp_s.clear();
            temp_s.push_str(item.get_string());
            if let Some(d) = item.get_dict() {
                code.push_str(&parse_slice_from_dict(d, &temp_s));
            } else if item.is_list() {
                panic!("Invalid values format - a slice must be a number or a dictionary {{value, label: 'text', attr: red}} !");
            } else {
                code.push_str(&format!("control.add_value({}, \"\", None);\n", parse_slice_value(item.get_string())));
            }
        }
        cb.add_line(&code);
    }
    cb.call_method_with_integer_parameter_and_range("set_selected_slice", "selected", 0, i32::MAX);
    cb.add_basecontrol_operations();
    cb.into()
}

fn find(key: &str, names: &[(&'static str, &'static str)], repr: &str) -> &'static str {
    let repr = repr.trim();
    match crate::utils::find_string_in_array(names, repr) {
        Some(name) => name,
        None => panic!("Invalid {key}: {repr} - expected one of: {}", crate::utils::join_strings(names)),
    }
}

fn parse_renderer(repr: &str) -> String {
    let repr = repr.trim();
    if let Some(fncall) = crate::fncall::FnCall::new(repr) {
        if let Some(name) = fncall.match_name(RENDERERS) {
            if name == "Custom" {
                assert!(
                    fncall.params_count() == 1,
                    "Invalid renderer - expecting Custom(character), for example Custom('#') or Custom(code: 0x2588) !"
                );
                return format!("piechart::DrawModeRenderer::Custom({})", parse_custom_char(fncall.param(0).unwrap(), "renderer"));
            }
            assert!(fncall.params_count() == 0, "Invalid renderer - {name} takes no parameters !");
            return format!("piechart::DrawModeRenderer::{name}");
        }
    }
    // a single character (for example `renderer: '#'`) is the same as Custom('#')
    if repr.chars().count() == 1 {
        return format!("piechart::DrawModeRenderer::Custom({})", parse_custom_char(repr, "renderer"));
    }
    panic!(
        "Invalid renderer: {repr} - expected one of: {}, Custom(character) or a single character",
        crate::utils::join_strings(RENDERERS)
    );
}

fn parse_u8(repr: &str, key: &str, max: u8) -> u8 {
    match repr.trim().parse::<u8>() {
        Ok(v) if v <= max => v,
        _ => panic!("Invalid {key} - expecting a number between 0 and {max} but got {repr} !"),
    }
}

fn parse_hollow(repr: &str) -> String {
    let repr = repr.trim();
    // `50%` is the same as Percentage(50)
    if let Some(percent) = repr.strip_suffix('%') {
        return format!("piechart::HollowThickness::Percentage({})", parse_u8(percent, "hollow", 100));
    }
    let Some(fncall) = crate::fncall::FnCall::new(repr) else {
        panic!("Invalid hollow: {repr} - expected None, Percentage(n), Absolute(n) or n% !");
    };
    let name = find("hollow", HOLLOW_KINDS, fncall.name());
    let expected = if name == "None" { 0 } else { 1 };
    assert!(
        fncall.params_count() == expected,
        "Invalid hollow: {repr} - expected None, Percentage(n), Absolute(n) or n% !"
    );
    match name {
        "None" => String::from("piechart::HollowThickness::None"),
        "Percentage" => format!("piechart::HollowThickness::Percentage({})", parse_u8(fncall.param(0).unwrap(), "hollow", 100)),
        _ => format!("piechart::HollowThickness::Absolute({})", parse_u8(fncall.param(0).unwrap(), "hollow", 100)),
    }
}

fn parse_slice_value(repr: &str) -> String {
    match repr.trim().parse::<u64>() {
        Ok(v) => format!("{v}u64"),
        Err(_) => panic!("Invalid values format - expecting a positive integer (u64) but got {repr} !"),
    }
}

fn parse_slice_from_dict(dict: &mut NamedParamsMap, param_list: &str) -> String {
    dict.validate_positional_parameters(param_list, SLICE_POSITIONAL).unwrap();
    dict.validate_named_parameters(param_list, SLICE_NAMED).unwrap();
    let value = match dict.get("value") {
        Some(v) => parse_slice_value(v.get_string()),
        None => panic!("Invalid values format - missing 'value' ! Expected {{value, label: 'text', attr: red}}"),
    };
    let label = dict.get("label").map(|v| v.get_string().to_string()).unwrap_or_default();
    let attr = match dict.get_mut("attr") {
        Some(v) if v.is_dict() => {
            let inner = v.get_dict().expect("attr was checked to be a dictionary");
            format!("Some({})", crate::chars::builder::create_attr_from_dict(param_list, inner))
        }
        Some(v) => format!("Some({})", parse_bar_attr(v.get_string(), "attr")),
        None => String::from("None"),
    };
    format!("control.add_value({value}, {label:?}, {attr});\n")
}
