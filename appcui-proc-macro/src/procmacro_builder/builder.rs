use crate::utils;

use super::StructDefinition;
use super::{arguments::Arguments, templates, AppCUITrait, BaseControlType, TraitImplementation, TraitsConfig};
use proc_macro::TokenStream;
use std::fmt::Write;
use std::str::FromStr;

fn generate_commands(a: &Arguments) -> String {
    let mut cmd_code = String::with_capacity(1024);
    cmd_code.push_str(templates::COMMANDS_TEMPLATE);
    let mut temp = String::with_capacity(256);

    // step 1 --> generate the list of enum variants
    temp.clear();
    for (idx, cmd) in a.commands.iter().enumerate() {
        write!(temp, "{} = {}, ", &cmd, idx).unwrap();
    }
    cmd_code = cmd_code.replace("$(COMMANDS_IDS)", &temp);

    // step 2 --> generate the conversion code (from u32 to commands)
    temp.clear();
    for (idx, cmd) in a.commands.iter().enumerate() {
        writeln!(temp, "{} => Ok(Commands::{}),", idx, &cmd).unwrap();
    }
    cmd_code = cmd_code.replace("$(U32_TO_COMMANDS)", &temp);

    // step 3 --> generate the conversion code (from commands to u32)
    temp.clear();
    for (idx, cmd) in a.commands.iter().enumerate() {
        writeln!(temp, "Commands::{} => {},", &cmd, idx).unwrap();
    }
    cmd_code = cmd_code.replace("$(COMMANDS_TO_U32)", &temp);
    cmd_code
}
fn generate_emitted_events(a: &Arguments) -> String {
    let mut cmd_code = String::with_capacity(1024);
    cmd_code.push_str(templates::EMIT_EVENTS_TEMPLATE);
    let mut temp = String::with_capacity(256);

    // step 1 --> generate the list of enum variants
    temp.clear();
    for (idx, event_name) in a.emitted_events.iter().enumerate() {
        write!(temp, "{} = {}, ", &event_name, idx).unwrap();
    }
    cmd_code = cmd_code.replace("$(EVENTS_IDS)", &temp);

    // step 2 --> generate the conversion code (from u32 to events)
    temp.clear();
    for (idx, cmd) in a.emitted_events.iter().enumerate() {
        writeln!(temp, "{} => Ok(Events::{}),", idx, &cmd).unwrap();
    }
    cmd_code = cmd_code.replace("$(U32_TO_EVENTS)", &temp);

    // step 3 --> generate the conversion code (from events to u32)
    temp.clear();
    for (idx, cmd) in a.emitted_events.iter().enumerate() {
        writeln!(temp, "Events::{} => {},", &cmd, idx).unwrap();
    }
    cmd_code = cmd_code.replace("$(EVENTS_TO_U32)", &temp);
    cmd_code
}
pub(crate) fn generate_inner_module(a: &Arguments, config: &mut TraitsConfig, code: &mut String, base_control: BaseControlType) {
    // add commands
    if a.commands.is_empty() {
        // if CommandBarEvents is present, we should add commands as well
        if config.get(AppCUITrait::CommandBarEvents) == TraitImplementation::None {
            panic!("Overwriting `CommandBarEvents` implies you should also define the `commands` attribute with possible values !");
        }
        // if MenuEvents is present, we should add commands as well
        if config.get(AppCUITrait::MenuEvents) == TraitImplementation::None {
            panic!("Overwriting `MenuEvents` implies you should also define the `commands` attribute with possible values !");
        }
    } else {
        if (config.get(AppCUITrait::CommandBarEvents) != TraitImplementation::None)
            && (config.get(AppCUITrait::MenuEvents) != TraitImplementation::None)
        {
            panic!(
                "The 'commands` attribute can only be used if one of the CommandBarEvents or MenuEvents is overwritten (via `events` attributie) !"
            );
        }
        code.push_str(generate_commands(a).as_str());
    }
    if !a.emitted_events.is_empty() {
        if base_control != BaseControlType::CustomControl {
            panic!("The 'emit' attribute can only be used with a CustomControl !");
        }
        code.push_str(generate_emitted_events(a).as_str());
    }
}
pub(crate) fn generate_custom_event_traits(a: &mut Arguments) -> String {
    let mut cmd_code = String::with_capacity(1024);
    cmd_code.push_str(templates::CUSTOM_EVENTS);
    let mut temp = String::with_capacity(256);

    // step 1 --> generate all custom traits and reduce the names
    for trait_name in a.custom_events.iter_mut() {
        temp.clear();
        temp.push_str(templates::CUSTOM_TRAIT_DEF);
        temp = temp.replace("$(TRAIT_NAME)", trait_name);
        // remove the Events part from trait_name
        trait_name.truncate(trait_name.len() - 6); // 6 = sizeof(Events);
                                                   // now its just the name (replace the structura name)
        temp = temp
            .replace("$(STRUC_NAME)", trait_name)
            .replace("$(MOD_NAME)", trait_name.to_ascii_lowercase().as_str());
        cmd_code.push_str(&temp);
    }

    // step 2 --> generate all proxy calls
    temp.clear();
    for trait_name in a.custom_events.iter() {
        // at this point the trait name does not have the Events part at its end
        let hash = utils::compute_hash(trait_name);
        write!(temp, "0x{:X} => {{", hash).unwrap();
        temp.push_str(templates::CUSTOM_EVENT_CONVERTOR);
        temp.push_str("}\n");
        temp = temp
            .replace("$(STRUC_NAME)", trait_name)
            .replace("$(MOD_NAME)", &trait_name.to_lowercase());
    }

    cmd_code.replace("$(CUSTOM_EVENT_CLASS_PROXY_CALL)", &temp)
}
fn generate_selector_events(a: &mut Arguments) -> String {
    let mut s = String::new();
    for trait_name in a.template_events[&AppCUITrait::GenericSelectorEvents].iter() {
        s.push_str(templates::SELECT_ON_SELECTION_CHANGE_DEF.replace("$(TYPE)", trait_name).as_str());
    }
    return templates::SELECTOR_TRAIT_DEF.replace("$(TYPE_ID_TRANSLATION_FOR_SELECTOR)", s.as_str());
}
fn generate_dropdownlist_events(a: &mut Arguments) -> String {
    let mut s = String::new();
    for trait_name in a.template_events[&AppCUITrait::GenericDropDownListEvents].iter() {
        s.push_str(templates::SELECT_ON_DROPDOWNLIST_CHANGE_DEF.replace("$(TYPE)", trait_name).as_str());
    }
    return templates::DROPDOWNLIST_TRAIT_DEF.replace("$(TYPE_ID_TRANSLATION_FOR_DROPDOWNLIST)", s.as_str());
}
fn generate_numeric_selector_events(a: &mut Arguments) -> String {
    if !a.template_events.contains_key(&AppCUITrait::GenericNumericSelectorEvents) {
        panic!("Missing generic type for NumericSelectorEvents event (Have you used evets=NumericSelectorEvents<Type> ?)");
    }
    let mut s = String::new();
    for trait_name in a.template_events[&AppCUITrait::GenericNumericSelectorEvents].iter() {
        s.push_str(templates::NUMERIC_SELECT_ON_VALUE_CHANGE_DEF.replace("$(TYPE)", trait_name).as_str());
    }
    return templates::NUMERIC_SELECTOR_TRAIT_DEF.replace("$(TYPE_ID_TRANSLATION_FOR_NUMERIC_SELECTOR)", s.as_str());
}

fn generate_listview_events(a: &mut Arguments) -> String {
    if !a.template_events.contains_key(&AppCUITrait::GenericListViewEvents) {
        panic!("Missing generic type for ListView event (Have you used events=ListVewEvents<Type> ?)");
    }
    let mut on_current_item_changed_code = String::new();
    let mut on_group_collapsed_code = String::new();
    let mut on_group_expanded_code = String::new();
    let mut on_selection_changed_code = String::new();
    let mut on_item_action_code = String::new();
    for trait_name in a.template_events[&AppCUITrait::GenericListViewEvents].iter() {
        on_current_item_changed_code.push_str(templates::LISTVIEW_ON_CURRENT_ITEM_CHANGED_DEF.replace("$(TYPE)", trait_name).as_str());
        on_group_collapsed_code.push_str(templates::LISTVIEW_ON_GROUP_COLLAPSED_DEF.replace("$(TYPE)", trait_name).as_str());
        on_group_expanded_code.push_str(templates::LISTVIEW_ON_GROUP_EXPANDED_DEF.replace("$(TYPE)", trait_name).as_str());
        on_selection_changed_code.push_str(templates::LISTVIEW_ON_SELECTION_CHANGED_DEF.replace("$(TYPE)", trait_name).as_str());
        on_item_action_code.push_str(templates::LISTVIEW_ON_ITEM_ACTION_DEF.replace("$(TYPE)", trait_name).as_str());
    }
    templates::LISTVIEW_TRAIT_DEF
        .replace(
            "$(TYPE_ID_TRANSLATION_FOR_LISTVIEW_ON_CURRENT_ITEM_CHANGED)",
            &on_current_item_changed_code,
        )
        .replace("$(TYPE_ID_TRANSLATION_FOR_LISTVIEW_ON_GROUP_COLLAPSED)", &on_group_collapsed_code)
        .replace("$(TYPE_ID_TRANSLATION_FOR_LISTVIEW_ON_GROUP_EXPANDED)", &on_group_expanded_code)
        .replace("$(TYPE_ID_TRANSLATION_FOR_LISTVIEW_ON_SELECTION_CHANGED)", &on_selection_changed_code)
        .replace("$(TYPE_ID_TRANSLATION_FOR_LISTVIEW_ON_ITEM_ACTION)", &on_item_action_code)
}

fn generate_treeview_events(a: &mut Arguments) -> String {
    if !a.template_events.contains_key(&AppCUITrait::GenericTreeViewEvents) {
        panic!("Missing generic type for TreeView event (Have you used events=TreeVewEvents<Type> ?)");
    }
    let mut on_current_item_changed_code = String::new();
    let mut on_item_collapsed_code = String::new();
    let mut on_item_expanded_code = String::new();
    let mut on_selection_changed_code = String::new();
    let mut on_item_action_code = String::new();
    for trait_name in a.template_events[&AppCUITrait::GenericTreeViewEvents].iter() {
        on_current_item_changed_code.push_str(templates::TREEVIEW_ON_CURRENT_ITEM_CHANGED_DEF.replace("$(TYPE)", trait_name).as_str());
        on_item_collapsed_code.push_str(templates::TREEVIEW_ON_ITEM_COLLAPSED_DEF.replace("$(TYPE)", trait_name).as_str());
        on_item_expanded_code.push_str(templates::TREEVIEW_ON_ITEM_EXPANDED_DEF.replace("$(TYPE)", trait_name).as_str());
        on_selection_changed_code.push_str(templates::TREEVIEW_ON_SELECTION_CHANGED_DEF.replace("$(TYPE)", trait_name).as_str());
        on_item_action_code.push_str(templates::TREEVIEW_ON_ITEM_ACTION_DEF.replace("$(TYPE)", trait_name).as_str());
    }
    templates::TREEVIEW_TRAIT_DEF
        .replace(
            "$(TYPE_ID_TRANSLATION_FOR_TREEVIEW_ON_CURRENT_ITEM_CHANGED)",
            &on_current_item_changed_code,
        )
        .replace("$(TYPE_ID_TRANSLATION_FOR_TREEVIEW_ON_ITEM_COLLAPSED)", &on_item_collapsed_code)
        .replace("$(TYPE_ID_TRANSLATION_FOR_TREEVIEW_ON_ITEM_EXPANDED)", &on_item_expanded_code)
        //.replace("$(TYPE_ID_TRANSLATION_FOR_LISTVIEW_ON_SELECTION_CHANGED)", &on_selection_changed_code)
        .replace("$(TYPE_ID_TRANSLATION_FOR_TREEVIEW_ON_ITEM_ACTION)", &on_item_action_code)
}

pub(crate) fn build(args: TokenStream, input: TokenStream, base_control: BaseControlType, config: &mut TraitsConfig) -> TokenStream {
    let mut a = Arguments::new(base_control);
    a.parse(args, config);
    let mut base_definition = "{\n    base: ".to_string();
    base_definition.push_str(&a.base);
    base_definition.push_str(", ");
    let mut code = input.to_string().replace('{', base_definition.as_str());
    let struct_data = StructDefinition::from(code.as_str());
    let has_inner_module = !a.commands.is_empty() || !a.emitted_events.is_empty();
    let mut struct_name_hash = String::new();
    code.insert_str(0, "#[repr(C)]\n");
    code.insert_str(0, templates::IMPORTS);
    if a.internal_mode {
        code.insert_str(0, templates::IMPORTS_INTERNAL);
        if a.window_control {
            // we need to overwrite NotWindow and make sure that WindowControl is set up
            config.clear(AppCUITrait::NotWindow);
            config.clear(AppCUITrait::NotModalWindow);
            config.clear(AppCUITrait::WindowControl);
            config.clear(AppCUITrait::OnWindowRegistered);
            config.set(AppCUITrait::WindowControl, TraitImplementation::Default);
            config.set(AppCUITrait::NotModalWindow, TraitImplementation::Default);
        }
        if a.desktop_control {
            // we need to overwrite NotDesktop and make sure that DesktopControl is set up
            config.clear(AppCUITrait::NotDesktop);
            config.clear(AppCUITrait::DesktopControl);
            config.set(AppCUITrait::DesktopControl, TraitImplementation::Default);
        }
    }
    if !a.custom_events.is_empty() {
        if config.get(AppCUITrait::CustomEvents) != TraitImplementation::Default {
            panic!("Support for custom events (via 'custom_events' attribute) is only allowed for a Window or a ModalWindow !");
        } else {
            // we clear the custom event to avoid default implementation
            config.clear(AppCUITrait::CustomEvents);
            // generate the code
            code.push_str(generate_custom_event_traits(&mut a).as_str());
        }
    }
    for (appcui_trait, trait_impl) in config.iter() {
        match trait_impl {
            TraitImplementation::None => {
                if appcui_trait.is_generic() {
                    match appcui_trait {
                        AppCUITrait::GenericSelectorEvents => code.push_str(generate_selector_events(&mut a).as_str()),
                        AppCUITrait::GenericDropDownListEvents => code.push_str(generate_dropdownlist_events(&mut a).as_str()),
                        AppCUITrait::GenericNumericSelectorEvents => code.push_str(generate_numeric_selector_events(&mut a).as_str()),
                        AppCUITrait::GenericListViewEvents => code.push_str(generate_listview_events(&mut a).as_str()),
                        AppCUITrait::GenericTreeViewEvents => code.push_str(generate_treeview_events(&mut a).as_str()),
                        _ => {}
                    }
                }
            }
            TraitImplementation::Default | TraitImplementation::DefaultNonOverwritable => {
                code.push_str(appcui_trait.default_implementation());
            }
            TraitImplementation::BaseFallback | TraitImplementation::BaseFallbackNonOverwritable => {
                code.push_str(appcui_trait.basefallback_implementation());
            }
        }
        code.push('\n');
    }
    // if commands or emit is available - build the inner module
    if has_inner_module {
        code.push_str(struct_data.access.as_str());
        code.push_str("mod $(MOD_NAME) {\nuse $(ROOT)::prelude::*;\n");
        generate_inner_module(&a, config, &mut code, base_control);
        code.push_str("}\n");
        // add the CommandBar events wrapper if needed
        if config.get(AppCUITrait::CommandBarEvents) == TraitImplementation::None {
            code.push_str(templates::COMMANDBAR_EVENTS);
        }
        // add the MenudBar events wrapper if needed
        if config.get(AppCUITrait::MenuEvents) == TraitImplementation::None {
            code.push_str(templates::MENU_EVENTS);
        }
        // add raise events support
        if !a.emitted_events.is_empty() {
            code.push_str(templates::RAISE_EVENTS_TEMPLATE);
            write!(struct_name_hash, "0x{:X}", utils::compute_hash(struct_data.name.as_str())).unwrap();
        }
    }

    // replace templates
    code = code
        .replace("$(STRUCT_NAME)", &struct_data.name)
        .replace("$(MOD_NAME)", struct_data.name.to_lowercase().as_str())
        .replace("$(BASE)", &a.base)
        .replace("$(ROOT)", a.root)
        .replace("$(MODAL_RESULT_TYPE)", &a.modal_result_type)
        .replace("$(STRUCT_NAME_HASH)", &struct_name_hash);
    // check templates
    if struct_data.template_type.is_empty() {
        code = code.replace("$(TEMPLATE_TYPE)", "").replace("$(TEMPLATE_DEF)", "");
    } else {
        code = code
            .replace("$(TEMPLATE_TYPE)", &struct_data.template_type)
            .replace("$(TEMPLATE_DEF)", &struct_data.template_def);
    }
    //println!("{}", code);
    TokenStream::from_str(&code).expect("Fail to convert string to token stream")
}
