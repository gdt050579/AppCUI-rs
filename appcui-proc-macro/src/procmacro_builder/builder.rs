use super::{arguments::Arguments, templates, utils, AppCUITrait, BaseControlType, TraitImplementation, TraitsConfig};
use proc_macro::TokenStream;
use std::fmt::Write;
use std::str::FromStr;

fn generate_commands(code: &mut String, a: &Arguments, struct_name: &str) -> String {
    let mut cmd_code = String::with_capacity(1024);
    cmd_code.push_str(templates::COMMANDS_TEMPLATE);
    let mut temp = String::with_capacity(256);

    // step 1 --> build the module name
    temp.clear();
    for mut ch in struct_name.chars() {
        if (ch >= 'A') && (ch <= 'Z') {
            ch = (((ch as u32) as u8) | 0x20) as char;
        }
        temp.push(ch);
    }
    cmd_code = cmd_code.replace("$(MOD_NAME)", &temp);

    // step 2 --> generate the list of enum variants
    temp.clear();
    let mut idx = 0u32;
    for cmd in &a.commands {
        let _ = write!(temp, "{} = {}, ", &cmd, idx).unwrap();
        idx += 1;
    }
    cmd_code = cmd_code.replace("$(COMMANDS_IDS)", &temp);

    // step 3 --> generate the conversion code (from u32 to commands)
    temp.clear();
    let mut idx = 0u32;
    for cmd in &a.commands {
        let _ = writeln!(temp, "{} => Ok(Commands::{}),", idx, &cmd).unwrap();
        idx += 1;
    }
    cmd_code = cmd_code.replace("$(U32_TO_COMMANDS)", &temp);

    // step 2 --> generate the conversion code (from commands to u32)
    temp.clear();
    let mut idx = 0u32;
    for cmd in &a.commands {
        let _ = writeln!(temp, "Commands::{} => {},", &cmd, idx).unwrap();
        idx += 1;
    }
    cmd_code = cmd_code.replace("$(COMMANDS_TO_U32)", &temp);
    cmd_code
}

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
        let cmd_gen_code = generate_commands(&mut code, &a, &struct_name);
        code.push_str(&cmd_gen_code);
    }
    // replace templates
    code = code
        .replace("$(STRUCT_NAME)", &struct_name)
        .replace("$(BASE)", &a.base)
        .replace("$(ROOT)", a.root)
        .replace("$(MODAL_RESULT_TYPE)", &a.modal_result_type);
    //println!("{}", code);
    TokenStream::from_str(&code).expect("Fail to convert string to token stream")
}
