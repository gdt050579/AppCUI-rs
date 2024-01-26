#[repr(u8)]
#[derive(Default, Clone, Copy)]
pub(crate) enum ParamType {
    #[default]
    String,
    Bool,
    Flags,
    Alignament,
    Color,
    Size,
    Layout,
    Dict,
    Integer,
}

pub(crate) struct PositionalParameter {
    key: &'static str,
    param_type: ParamType,
}
impl PositionalParameter {
    #[inline(always)]
    pub(crate) const fn new(key: &'static str, param_type: ParamType) -> Self {
        Self {
            key,
            param_type,
        }
    }
    #[inline(always)]
    pub(crate) fn get_key(&self) -> &'static str {
        self.key
    }
    #[inline(always)]
    pub(crate) fn get_param_type(&self) -> ParamType {
        self.param_type
    }
}
pub(crate) struct NamedParameter {
    name: &'static str,
    key: &'static str,
    param_type: ParamType,
}
impl NamedParameter {
    #[inline(always)]
    pub(crate) const fn new(name: &'static str, key: &'static str, param_type: ParamType) -> Self {
        Self {
            name,
            key,
            param_type,
        }
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

