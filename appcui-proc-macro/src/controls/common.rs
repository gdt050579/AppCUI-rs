use crate::parameter_parser::ParamSignature;
use crate::parameter_parser::ParamType;

pub (super) static SIGNATURE: &[ParamSignature] = &[
    // generic characteristics 
    ParamSignature::optional("visible", "visible", ParamType::Bool),
    ParamSignature::optional("enabled", "enabled", ParamType::Bool),
    ParamSignature::optional("enable", "enabled", ParamType::Bool),
    // layout
    ParamSignature::optional("x", "x", ParamType::Layout),
    ParamSignature::optional("y", "y", ParamType::Layout),
    ParamSignature::optional("left", "left", ParamType::Layout),
    ParamSignature::optional("l", "left", ParamType::Layout),
    ParamSignature::optional("right", "right", ParamType::Layout),
    ParamSignature::optional("r", "right", ParamType::Layout),
    ParamSignature::optional("top", "top", ParamType::Layout),
    ParamSignature::optional("t", "top", ParamType::Layout),
    ParamSignature::optional("bottom", "bottom", ParamType::Layout),
    ParamSignature::optional("b", "bottom", ParamType::Layout),
    ParamSignature::optional("width", "width", ParamType::Layout),
    ParamSignature::optional("w", "width", ParamType::Layout),
    ParamSignature::optional("height", "height", ParamType::Layout),
    ParamSignature::optional("h", "height", ParamType::Layout),
    ParamSignature::optional("align", "align", ParamType::Alignament),
    ParamSignature::optional("a", "align", ParamType::Alignament),   
    ParamSignature::optional("alignament", "align", ParamType::Alignament),   
    ParamSignature::optional("dock", "dock", ParamType::Alignament),
    ParamSignature::optional("d", "dock", ParamType::Alignament),   
];

pub(super) fn add_string(s: &mut String, text: &str) {
    s.push('"');
    s.push_str(text);
    s.push('"');
}