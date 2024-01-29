use super::menuitem_type::MenuItemType;
use crate::{
    parameter_parser::{self, *},
    token_stream_to_string::TokenStreamToString,
};
use proc_macro::*;
use std::str::FromStr;

static POSILITIONAL_PARAMETERS: &[PositionalParameter] = &[PositionalParameter::new("caption", ParamType::String)];
static NAMED_PARAMETERS: &[NamedParameter] = &[
    NamedParameter::new("caption", "caption", ParamType::String),
    NamedParameter::new("text", "caption", ParamType::String),
    NamedParameter::new("shortcut", "shortcut", ParamType::String),
    NamedParameter::new("shortcutkey", "shortcut", ParamType::String),
    NamedParameter::new("cmd", "cmd", ParamType::Integer),
    NamedParameter::new("command", "cmd", ParamType::Integer),
    NamedParameter::new("cmd-id", "cmd", ParamType::Integer),
    NamedParameter::new("command-id", "cmd", ParamType::Integer),
    NamedParameter::new("enable", "enable", ParamType::Bool),
    NamedParameter::new("enabled", "enable", ParamType::Bool),
    NamedParameter::new("check", "checked", ParamType::Bool),
    NamedParameter::new("checked", "checked", ParamType::Bool),
    NamedParameter::new("select", "select", ParamType::Bool),
    NamedParameter::new("selected", "select", ParamType::Bool),
    NamedParameter::new("items", "items", ParamType::Dict),    // should be LIST
    NamedParameter::new("subitems", "items", ParamType::Dict), // should be LIST
    NamedParameter::new("type", "type", ParamType::String),
];

fn get_menu_type(param_list: &str, dict: &mut NamedParamsMap) -> MenuItemType {
    if let Some(value) = dict.get("type") {
        if let Some(m_type) = MenuItemType::from_hash(crate::utils::compute_hash(value.get_string())) {
            m_type
        } else {
            Error::new(
                param_list,
                "Invalid type (allowed are Command,Line,Checkbox,SubMenu or SingleChoice)",
                value.get_start_pos(),
                value.get_end_pos(),
            )
            .panic();
            panic!();
        }
    } else {
        // logic based on data types
        if dict.contains("items") {
            return MenuItemType::SubMenu;
        }
        if dict.contains("checked") {
            return MenuItemType::CheckBox;
        }
        if dict.contains("select") {
            return MenuItemType::SingleChoice;
        }
        if dict.contains("caption") && (dict.get_parameters_count()==1) {
            if dict.get("caption").unwrap().get_string().chars().all(|c| c=='-') {
                return MenuItemType::Line;
            }            
        }
        return MenuItemType::Command;
    }
}
fn build_menuitem_command(param_list: &str, dict: &mut NamedParamsMap) -> String {
    String::new()
}
fn build_menuitem_checkbox(param_list: &str, dict: &mut NamedParamsMap) -> String {
    String::new()
}
fn build_menuitem_singlechoice(param_list: &str, dict: &mut NamedParamsMap) -> String {
    String::new()
}
fn build_menuitem_submenu(param_list: &str, dict: &mut NamedParamsMap) -> String {
    String::new()
}
fn build_menuitem_line(param_list: &str, dict: &mut NamedParamsMap) -> String {
    String::new()
}
fn menuitem_from_dict(param_list: &str, dict: &mut NamedParamsMap) -> String {
    dict.validate_positional_parameters(param_list, POSILITIONAL_PARAMETERS).unwrap();
    dict.validate_named_parameters(param_list, NAMED_PARAMETERS).unwrap();

    let menuitem_type = get_menu_type(param_list, dict);
    match menuitem_type {
        MenuItemType::Command => build_menuitem_command(param_list,dict),
        MenuItemType::CheckBox => build_menuitem_checkbox(param_list,dict),
        MenuItemType::SingleChoice => build_menuitem_singlechoice(param_list,dict),
        MenuItemType::SubMenu => build_menuitem_submenu(param_list,dict),
        MenuItemType::Line => build_menuitem_line(param_list,dict),
    }
}
pub(crate) fn create(input: TokenStream) -> TokenStream {
    let s = input.validate_one_string_parameter("menuitem");
    let mut d = parameter_parser::parse(&s).unwrap();
    let res = menuitem_from_dict(&s, &mut d);
    TokenStream::from_str(&res).expect(format!("Fail to convert 'menuitem!' macro content to token stream").as_str())
}
