use super::{arguments::Arguments, templates, utils, AppCUITrait, BaseControlType, TraitImplementation, TraitsConfig};
use proc_macro::TokenStream;
use std::str::FromStr;

pub(crate) fn build(args: TokenStream, input: TokenStream, base_control: BaseControlType, config: &mut TraitsConfig) -> TokenStream {
    let mut a = Arguments::new(base_control);
    a.parse(args, config);
    let mut base_definition = "{\n    base: ".to_string();
    base_definition.push_str(&a.base);
    base_definition.push_str(", ");
    let mut code = input.to_string().replace("{", base_definition.as_str());
    let struct_name = utils::extract_structure_name(code.as_str());
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
    for (appcui_trait, trait_impl) in config.iter() {
        match trait_impl {
            TraitImplementation::None => {}
            TraitImplementation::Default | TraitImplementation::DefaultNonOverwritable => {
                code.push_str(appcui_trait.get_default_implementation());
            }
            TraitImplementation::BaseFallback | TraitImplementation::BaseFallbackNonOverwritable => {
                code.push_str(appcui_trait.get_basefallback_implementation());
            }
        }
        code.push_str("\n");
    }
    // add commands
    if !a.commands.is_empty() {
        code.push_str("mod ");
        for mut ch in struct_name.chars() {
            if (ch>='A') && (ch<='Z') {
                ch = (((ch as u32) as u8) | 0x20) as char;
            }
            code.push(ch);
        }
        code.push_str(" {\n");
        code.push_str("\t#[repr(u32)]\n\tpub enum Commands {\n");
        for cmd in &a.commands {
            code.push_str("\t\t");
            code.push_str(cmd.as_str());
            code.push_str(", \n");
        }
        code.push_str("\t}\n");
        code.push_str("}\n");
    }
    // replace templates
    code = code
        .replace("$(STRUCT_NAME)", &struct_name)
        .replace("$(BASE)", &a.base)
        .replace("$(ROOT)", a.root)
        .replace("$(MODAL_RESULT_TYPE)",&a.modal_result_type);
    //println!("{}", code);
    TokenStream::from_str(&code).expect("Fail to convert string to token stream")
}
