use super::menuitem_type::MenuItemType;
use crate::parameter_parser::*;

static POSILITIONAL_PARAMETERS: &[PositionalParameter] = &[
    PositionalParameter::new("caption", ParamType::String),
    PositionalParameter::new("shortcut", ParamType::String),
    PositionalParameter::new("cmd", ParamType::String),
];
static NAMED_PARAMETERS: &[NamedParameter] = &[
    NamedParameter::new("caption", "caption", ParamType::String),
    NamedParameter::new("text", "caption", ParamType::String),
    NamedParameter::new("shortcut", "shortcut", ParamType::String),
    NamedParameter::new("shortcutkey", "shortcut", ParamType::String),
    NamedParameter::new("key", "shortcut", ParamType::String),
    NamedParameter::new("cmd", "cmd", ParamType::String),
    NamedParameter::new("command", "cmd", ParamType::String),
    NamedParameter::new("cmd-id", "cmd", ParamType::String),
    NamedParameter::new("command-id", "cmd", ParamType::String),
    NamedParameter::new("enable", "enable", ParamType::Bool),
    NamedParameter::new("enabled", "enable", ParamType::Bool),
    NamedParameter::new("check", "checked", ParamType::Bool),
    NamedParameter::new("checked", "checked", ParamType::Bool),
    NamedParameter::new("select", "select", ParamType::Bool),
    NamedParameter::new("selected", "select", ParamType::Bool),
    NamedParameter::new("items", "items", ParamType::List),
    NamedParameter::new("subitems", "items", ParamType::List),
    NamedParameter::new("type", "type", ParamType::String),
    NamedParameter::new("class", "class", ParamType::String),
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
        if dict.contains("caption") && (dict.get_parameters_count() == 1) && dict.get("caption").unwrap().get_string().chars().all(|c| c == '-') {
            return MenuItemType::Separator;
        }
        MenuItemType::Command
    }
}
fn add_caption(s: &mut String, dict: &mut NamedParamsMap) {
    if let Some(value) = dict.get("caption") {
        s.push('"');
        s.push_str(value.get_string());
        s.push('"')
    } else {
        panic!("Missing 'caption' for menuitem !");
    }
}
fn add_shortcut(s: &mut String, dict: &mut NamedParamsMap) {
    if let Some(value) = dict.get("shortcut") {
        let r = crate::key::builder::create_string(value.get_string());
        s.push_str(&r);
    } else {
        s.push_str("Key::None");
    }
}
fn add_command_id(s: &mut String, dict: &mut NamedParamsMap, class: Option<&str>) {
    if let Some(value) = dict.get("cmd") {
        let id = value.get_string();
        if id.contains("::") {
            let w: Vec<_> = id.split("::").collect();
            if (w.len() > 3) || (w.len() < 2) {
                panic!("Full qualifer format format for a command must be: '<module>::Commands::<Command>' ");
            }
            if w.len() == 3 {
                // format <module>::Commands::<command>
                if let Err(desc) = crate::utils::validate_name(w[0], false) {
                    panic!("Invalid class name '{}' => {}", w[0], desc);
                }
                if !crate::utils::equal_ignore_case(w[1], "commands") {
                    panic!("Full qualifer format format for a command must be: '<module>::Commands::<Command>' (you have to use `Commands` for the middle part !");
                }
                if let Err(desc) = crate::utils::validate_name(w[2], false) {
                    panic!("Invalid command name '{}' => {}", w[2], desc);
                }
                // add the module name (the class) first (lowercase)
                for ch in w[0].chars() {
                    s.push(ch.to_ascii_lowercase());
                }
                s.push_str("::Commands::");
                s.push_str(w[2]);
            } else {
                // format <module>::<command>
                if let Err(desc) = crate::utils::validate_name(w[0], false) {
                    panic!("Invalid class name '{}' => {}", w[0], desc);
                }
                if let Err(desc) = crate::utils::validate_name(w[1], false) {
                    panic!("Invalid command name '{}' => {}", w[1], desc);
                }
                // add the module name (the class) first (lowercase)
                for ch in w[0].chars() {
                    s.push(ch.to_ascii_lowercase());
                }
                s.push_str("::Commands::");
                s.push_str(w[1]);
            }
        } else {
            // validate if the class can be build
            let c = if dict.contains("class") {
                dict.get("class").unwrap().get_string()
            } else {
                class.unwrap_or_default()
            };
            if c.is_empty() {
                panic!("Unknwon class nane (or empty) for command. Either specify it in the `class` attribute (e.g. class=MyWin) or specify the command with its full qualifier (e.g. command='mywin::Command::<name>').");
            }
            if let Err(desc) = crate::utils::validate_name(c, true) {
                panic!("Invalid class name '{}' => {}", c, desc);
            }
            if let Err(desc) = crate::utils::validate_name(id, false) {
                panic!("Invalid command name '{}' => {}", id, desc);
            }
            // add the module name (the class) first (lowercase)
            for ch in c.chars() {
                s.push(ch.to_ascii_lowercase());
            }
            s.push_str("::Commands::");
            s.push_str(id);
        }
    } else {
        panic!("Missing 'command' for menuitem !");
    }
}
fn add_enable_status(s: &mut String, dict: &mut NamedParamsMap) {
    if let Some(value) = dict.get_bool("enable") {
        if !value {
            s.push_str("item.set_enabled(false);\n");
        }
    }
}
fn build_menuitem_command(_param_list: &str, dict: &mut NamedParamsMap, class: Option<&str>) -> String {
    let mut s = String::from("{\nlet mut item = menu::Command::new(");
    add_caption(&mut s, dict);
    s.push_str(", ");
    add_shortcut(&mut s, dict);
    s.push_str(", ");
    add_command_id(&mut s, dict, class);
    s.push_str(");\n");
    add_enable_status(&mut s, dict);
    s.push_str("\nitem\n}");
    s
}
fn build_menuitem_checkbox(_param_list: &str, dict: &mut NamedParamsMap, class: Option<&str>) -> String {
    let mut s = String::from("{\nlet mut item = menu::CheckBox::new(");
    add_caption(&mut s, dict);
    s.push_str(", ");
    add_shortcut(&mut s, dict);
    s.push_str(", ");
    add_command_id(&mut s, dict, class);
    if dict.get_bool("checked").unwrap_or(false) {
        s.push_str(",true");
    } else {
        s.push_str(",false");
    }
    s.push_str(");\n");
    add_enable_status(&mut s, dict);
    s.push_str("\nitem\n}");
    s
}
fn build_menuitem_singlechoice(_param_list: &str, dict: &mut NamedParamsMap, class: Option<&str>) -> String {
    let mut s = String::from("{\nlet mut item = menu::SingleChoice::new(");
    add_caption(&mut s, dict);
    s.push_str(", ");
    add_shortcut(&mut s, dict);
    s.push_str(", ");
    add_command_id(&mut s, dict, class);
    if dict.get_bool("select").unwrap_or(false) {
        s.push_str(",true");
    } else {
        s.push_str(",false");
    }
    s.push_str(");\n");
    add_enable_status(&mut s, dict);
    s.push_str("\nitem\n}");
    s
}
fn get_class(dict: &mut NamedParamsMap, inherit: Option<&str>) -> Option<String> {
    if let Some(value) = dict.get("class") {
        let c = value.get_string();
        if c.is_empty() {
            panic!("Unknwon class nane (or empty) for command. Either specify it in the `class` attribute (e.g. class=MyWin) or specify the command with its full qualifier (e.g. command='mywin::Command::<name>').");
        }
        if let Err(desc) = crate::utils::validate_name(c, true) {
            panic!("Invalid class name '{}' => {}", c, desc);
        }
        Some(String::from(c))
    } else if let Some(name) = inherit {
        return Some(String::from(name));
    } else {
        return None;
    }
}
pub(super) fn build_menu(param_list: &str, dict: &mut NamedParamsMap, class: Option<&str>) -> String {
    let mut s = String::from("{\nlet mut menu = Menu::new(");
    let class_name = get_class(dict, class);
    add_caption(&mut s, dict);
    s.push_str(");\n");
    // we should add menu items
    if let Some(value) = dict.get_list("items") {
        for subitem in value {
            s.push_str("menu.add(");
            if let Some(d) = subitem.get_dict() {
                let result = menuitem_from_dict(param_list, d, class_name.as_deref());
                s.push_str(&result);
            } else {
                panic!("Invalid format for a menu subitems. Within the `items` attribute all items must be declared within `{{..}}` !");
            }
            s.push_str(");\n");
        }
    }
    s.push_str("\nmenu\n}");
    s
}
fn build_menuitem_submenu(param_list: &str, dict: &mut NamedParamsMap, class: Option<&str>) -> String {
    let mut s = String::from("{\nlet mut item = menu::SubMenu::new(");
    // check to see if I provide the class or if I have to use the provded one
    // the class that I provive as a key takes priority.
    let class_name = get_class(dict, class);
    let m = build_menu(param_list, dict, class_name.as_deref());
    s.push_str(&m);
    s.push_str(");\n");
    add_enable_status(&mut s, dict);
    s.push_str("\nitem\n}");
    s
}
fn build_menuitem_separator() -> String {
    String::from("menu::Separator::new()")
}
pub(super) fn menuitem_from_dict(param_list: &str, dict: &mut NamedParamsMap, class: Option<&str>) -> String {
    dict.validate_positional_parameters(param_list, POSILITIONAL_PARAMETERS).unwrap();
    dict.validate_named_parameters(param_list, NAMED_PARAMETERS).unwrap();

    let menuitem_type = get_menu_type(param_list, dict);
    match menuitem_type {
        MenuItemType::Command => build_menuitem_command(param_list, dict, class),
        MenuItemType::CheckBox => build_menuitem_checkbox(param_list, dict, class),
        MenuItemType::SingleChoice => build_menuitem_singlechoice(param_list, dict, class),
        MenuItemType::SubMenu => build_menuitem_submenu(param_list, dict, class),
        MenuItemType::Separator => build_menuitem_separator(),
    }
}
