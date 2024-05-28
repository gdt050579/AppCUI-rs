mod appcui_traits;
mod arguments;
mod templates;
mod traits_configuration;
mod builder;
mod base_control_type;
mod struct_definition;
#[cfg(test)]
mod tests;

pub(crate) use struct_definition::StructDefinition;
pub(crate) use appcui_traits::AppCUITrait;
pub(crate) use traits_configuration::TraitImplementation;
pub(crate) use traits_configuration::TraitsConfig;
pub(crate) use builder::build;
pub(crate) use base_control_type::BaseControlType;