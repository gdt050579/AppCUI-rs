#[repr(u8)]
#[derive(Default, Clone, Copy)]
pub(crate) enum ParamType {
    #[default]
    String,
    Bool,
    Flags,
    Alignament,
    Layout,
}

#[derive(Default)]
pub(crate) struct ParamSignature {
    name: &'static str,
    key: &'static str,
    param_type: ParamType,
    optional: bool,
}
impl ParamSignature {
    pub(crate) const fn optional(name: &'static str, key: &'static str, param_type: ParamType) -> Self {
        Self {
            name,
            key,
            param_type,
            optional: true,
        }
    }
    pub(crate) const fn mandatory(name: &'static str, key: &'static str, param_type: ParamType) -> Self {
        Self {
            name,
            key,
            param_type,
            optional: false,
        }
    }
    #[inline(always)]
    pub(crate) fn is_mandatory(&self) -> bool {
        self.optional == false
    }
    #[inline(always)]
    pub(crate) fn get_key(&self) -> &'static str {
        self.key
    }
    #[inline(always)]
    pub(crate) fn get_name(&self) -> &'static str {
        self.name
    }
    #[inline(always)]
    pub(crate) fn get_param_type(&self) -> ParamType {
        self.param_type
    }
}
