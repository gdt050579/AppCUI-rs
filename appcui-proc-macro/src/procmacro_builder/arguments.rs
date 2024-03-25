use std::collections::HashSet;

use super::{
    appcui_traits::{AppCUITrait, TraitType},
    traits_configuration::TraitsConfig,
    BaseControlType,
};

use proc_macro::*;

enum State {
    ExpectKey,
    ExpectEqual,
    ExpectValue,
    ExpectComma,
}
pub(crate) struct Arguments {
    pub base_control_type: BaseControlType,
    pub root: &'static str,
    pub debug_mode: bool,
    pub internal_mode: bool,
    pub window_control: bool,
    pub desktop_control: bool,
    pub base: String,
    pub modal_result_type: String,
    pub commands: Vec<String>,
    // internal
    state: State,
    key: String,
    values: Vec<String>,
}

impl Arguments {
    pub fn new(base_control_type: BaseControlType) -> Arguments {
        Arguments {
            base_control_type,
            base: base_control_type.to_string(),
            modal_result_type: String::from("()"),
            state: State::ExpectKey,
            root: "appcui",
            key: String::new(),
            values: Vec::with_capacity(8),
            commands: Vec::new(),
            debug_mode: false,
            internal_mode: false,
            window_control: false,
            desktop_control: false,
        }
    }

    fn validate_one_value(&self) {
        if self.values.len() != 1 {
            panic!(
                "Expecting one value for key: '{}', but got {} => {:?}",
                self.key,
                self.values.len(),
                self.values
            );
        }
    }
    // fn validate_base_attribute(&mut self) {
    //     if self.internal_mode==false {
    //         panic!("Base attribute is reserved for internal usage wuthin AppCUI framework !");
    //     }
    //     self.validate_one_value();
    //     if !utils::validate_struct_name(self.values[0].as_str()) {
    //         panic!("Invalid name for a base struct. A valid name should contains letters, numbers or underline and must not start with a number.");
    //     }
    //     self.base.clear();
    //     self.base.push_str(self.values[0].as_str());
    // }
    fn validate_modal_response(&mut self) {
        if self.base_control_type != BaseControlType::ModalWindow {
            panic!("Attribute 'response' is only available for ModalWindows !");
        }
        self.validate_one_value();
        if let Err(desc) = crate::utils::validate_name(self.values[0].as_str(), false) {
            panic!("Invalid name ('{}') for the modal response => {}", self.values[0].as_str(), desc);
        }
        self.base.clear();
        self.base.push_str("ModalWindow<");
        self.base.push_str(self.values[0].as_str());
        self.base.push('>');
        self.modal_result_type.clear();
        self.modal_result_type.push_str(self.values[0].as_str());
    }

    fn validate_internal_attribute(&mut self) {
        self.validate_one_value();
        if let Some(value) = crate::utils::to_bool(self.values[0].as_str()) {
            self.root = if value { "crate" } else { "appcui" };
            self.internal_mode = value;
        } else {
            panic!(
                "The value for `internal` attribute can only be 'true' or 'false'. Provided value was: {}",
                self.values[0].as_str()
            );
        }
    }
    fn validate_window_control(&mut self) {
        self.validate_one_value();
        if let Some(value) = crate::utils::to_bool(self.values[0].as_str()) {
            self.window_control = value;
        } else {
            panic!(
                "The value for `window` attribute can only be 'true' or 'false'. Provided value was: {}",
                self.values[0].as_str()
            );
        }
    }
    fn validate_desktop_control(&mut self) {
        self.validate_one_value();
        if let Some(value) = crate::utils::to_bool(self.values[0].as_str()) {
            self.desktop_control = value;
        } else {
            panic!(
                "The value for `desktop` attribute can only be 'true' or 'false'. Provided value was: {}",
                self.values[0].as_str()
            );
        }
    }
    fn validate_debug_attribute(&mut self) {
        self.validate_one_value();
        if let Some(value) = crate::utils::to_bool(self.values[0].as_str()) {
            self.debug_mode = value;
        } else {
            panic!(
                "The value for `debug` attribute can only be 'true' or 'false'. Provided value was: {}",
                self.values[0].as_str()
            );
        }
    }
    fn validate_overwrite_attribute(&mut self, config: &mut TraitsConfig) {
        for trait_name in &self.values {
            if let Some(appcui_trait) = AppCUITrait::new(trait_name) {
                if appcui_trait.get_trait_type() != TraitType::RawEvent {
                    panic!(
                        "Trait {trait_name} can not be used with the 'overwrite' attribute. Allowed traits for the 'overwrite' attribute are: {}",
                        config.traits_of_type(TraitType::RawEvent)
                    );
                }
                // now try to update the trait
                if config.get(appcui_trait).can_be_overwritten() {
                    config.clear(appcui_trait);
                } else {
                    panic!("Trait {trait_name} can not be overwritten (for proc macro: {})", config.get_name());
                }
            } else {
                panic!(
                    "Unknown trait to allow overwriting: '{trait_name}'. Allowed traits are: {}",
                    config.traits_of_type(TraitType::RawEvent)
                );
            }
        }
    }
    fn validate_commands(&mut self) {
        let mut h = HashSet::with_capacity(self.values.len() * 2);
        for command_name in &self.values {
            if let Err(desc) = crate::utils::validate_name(command_name.as_str(), false) {
                panic!("Invalid ID: '{}' => {}", command_name, desc);
            }
            let hash = crate::utils::compute_hash(command_name);
            if h.contains(&hash) {
                panic!("Commands must be unique. Duplicate command: {}", command_name);
            }
            h.insert(hash);
        }
        // all good --> move current value vector into commands and create a new one for values
        self.commands = std::mem::replace(&mut self.values, Vec::new());
    }
    fn validate_events_attribute(&mut self, config: &mut TraitsConfig) {
        for trait_name in &self.values {
            if let Some(appcui_trait) = AppCUITrait::new(trait_name) {
                if appcui_trait.get_trait_type() != TraitType::ControlEvent {
                    panic!(
                        "Trait {trait_name} can not be used with the 'overwrite' attribute. Allowed traits for the 'overwrite' attribute are: {}",
                        config.traits_of_type(TraitType::ControlEvent)
                    );
                }
                // now try to update the trait
                if config.get(appcui_trait).can_be_overwritten() {
                    config.clear(appcui_trait);
                } else {
                    panic!("Trait {trait_name} can not be overwritten (for proc macro: {})", config.get_name());
                }
            } else {
                panic!(
                    "Unknown trait to allow overwriting: '{trait_name}'. Allowed traits are: {}",
                    config.traits_of_type(TraitType::ControlEvent)
                );
            }
        }
    }
    fn validate_key_value_pair(&mut self, config: &mut TraitsConfig) {
        match self.key.as_str() {
            //"base" => self.validate_base_attribute(),
            "overwrite" => self.validate_overwrite_attribute(config),
            "events" => self.validate_events_attribute(config),
            "debug" => self.validate_debug_attribute(),
            "internal" => self.validate_internal_attribute(),
            "window" => self.validate_window_control(),
            "response" => self.validate_modal_response(),
            "desktop" => self.validate_desktop_control(),
            "commands" => self.validate_commands(),
            _ => {
                panic!(
                    "Unknown attribute `{}` for AppCUI. Accepted attributes are 'overwrite', 'events', 'debug', 'response', 'commands' !",
                    self.key.as_str()
                );
            }
        }
    }

    fn validate_expect_key(&mut self, token: TokenTree) {
        if let TokenTree::Ident(ident) = token {
            self.key = ident.to_string();
            self.values.clear();
            self.state = State::ExpectEqual;
        } else {
            panic!("Expecting a key (a-zA-Z0-9) but got: `{}`", token.to_string());
        }
    }
    fn validate_expect_equal(&mut self, token: TokenTree) {
        if let TokenTree::Punct(punctuation) = token {
            if (punctuation.as_char() != '=') && (punctuation.as_char() != ':') {
                panic!("Expecting asign ('=' or ':') symbol but got: {}", punctuation.as_char());
            }
            self.state = State::ExpectValue;
        } else {
            panic!("Expecting asign ('=' or ':') symbol but got: {}", token.to_string());
        }
    }
    fn validate_expect_value(&mut self, token: TokenTree) {
        match token {
            TokenTree::Group(group) => {
                if group.delimiter() != Delimiter::Bracket {
                    panic!(
                        "Expecting a value or a list of value [value-1, value-2, ... value-n] for key: {}",
                        self.key
                    );
                }
                let mut expect_value = true;
                for inner_token in group.stream() {
                    if expect_value {
                        if let TokenTree::Ident(val) = inner_token {
                            self.values.push(val.to_string());
                        } else {
                            panic!("Expecting a proper format for the list associated with the key: '{}'. It should use this format `[value-1, value-2, ... value-n]` but found `{}`",self.key,inner_token.to_string())
                        }
                    } else if let TokenTree::Punct(p) = inner_token {
                        if p.as_char() != ',' {
                            panic!("Expecting a separatoe ',' for the list associated with the key: '{}'. It should use this format `[value-1, value-2, ... value-n]` but found `{}`",self.key,p.to_string())
                        }
                    } else {
                        panic!("Expecting a separatoe ',' for the list associated with the key: '{}'. It should use this format `[value-1, value-2, ... value-n]` but found `{}`",self.key,inner_token.to_string())
                    }
                    expect_value = !expect_value;
                }
                self.state = State::ExpectComma;
            }
            TokenTree::Ident(ident) => {
                self.values.push(ident.to_string());
                self.state = State::ExpectComma;
            }
            TokenTree::Literal(literal) => {
                self.values.push(literal.to_string());
                self.state = State::ExpectComma;
            }
            _ => {
                panic!("Expecting a value (a-zA-Z0-9) but got: `{}`", token.to_string());
            }
        }
    }
    fn validate_expect_comma(&mut self, token: TokenTree, config: &mut TraitsConfig) {
        if let TokenTree::Punct(punctuation) = token {
            if punctuation.as_char() == '+' {
                self.state = State::ExpectValue;
                return;
            }
            if punctuation.as_char() != ',' {
                panic!("Expecting delimiter (',' comma) symbol but got: {}", punctuation.as_char());
            }
            self.validate_key_value_pair(config);
            self.state = State::ExpectKey;
        } else {
            panic!("Expecting delimiter (',' comma) symbol but got:{}", token.to_string());
        }
    }
    pub(crate) fn parse(&mut self, input: TokenStream, config: &mut TraitsConfig) {
        for token in input.into_iter() {
            // println!("arg_token: {:?}", token);
            match self.state {
                State::ExpectKey => self.validate_expect_key(token),
                State::ExpectEqual => self.validate_expect_equal(token),
                State::ExpectValue => self.validate_expect_value(token),
                State::ExpectComma => self.validate_expect_comma(token, config),
            }
        }
        match self.state {
            State::ExpectKey => {}
            State::ExpectEqual => {
                panic!(
                    "Unexpected end of procedural macro attribute (expecting an asignament character ':' or '=') after key: '{}'",
                    self.key.as_str()
                );
            }
            State::ExpectValue => {
                panic!(
                    "Unexpected end of procedural macro attribute (expecting a value for key: '{}')",
                    self.key.as_str()
                );
            }
            State::ExpectComma => self.validate_key_value_pair(config),
        }
    }
}
