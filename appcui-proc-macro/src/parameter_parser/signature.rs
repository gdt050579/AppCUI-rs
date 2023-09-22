#[repr(u8)]
pub (crate) enum ParamType {
    String,
    Bool,
    Flags,
    Alignament,
    Layout,
}
pub (crate) struct ParamSignature {
    name: &'static str,
    param_type: ParamType,
    order: u8,
    mandatory: bool, 
}
pub (crate) struct FunctionSignature {
    params: [ParamSignature; 32],
    count: u8
}