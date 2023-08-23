use crate::{
    appcui_traits::{AppCUITrait, TraitType},
    traits_configuration::{TraitsConfig, TraitImplementation},
};

use super::utils;
use proc_macro::*;

enum State {
    ExpectKey,
    ExpectEqual,
    ExpectValue,
    ExpectComma,
}
pub struct Arguments {
    pub base: String,
    pub event_processor_list: String,
    pub root: &'static str,
    pub debug_mode: bool,
    pub internal_mode: bool,
    pub window_control: bool,
    pub desktop_control: bool,

    // control events
    pub command_bar_events: bool,
    pub menu_events: bool,
    pub button_events: bool,
    pub checkbox_events: bool,
    pub window_events: bool,

    // internal
    state: State,
    key: String,
    values: Vec<String>,
}

impl Arguments {
    pub fn new(base_control: &str) -> Arguments {
        Arguments {
            base: String::from(base_control),
            state: State::ExpectKey,
            root: "appcui",
            key: String::new(),
            values: Vec::with_capacity(8),
            event_processor_list: String::new(),
            debug_mode: false,
            internal_mode: false,
            window_control: false,
            desktop_control: false,
            // control events
            menu_events: false,
            command_bar_events: false,
            button_events: false,
            checkbox_events: false,
            window_events: false,
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
    fn validate_base_attribute(&mut self) {
        self.validate_one_value();
        if !utils::validate_struct_name(self.values[0].as_str()) {
            panic!("Invalid name for a base struct. A valid name should contains letters, numbers or underline and must not start with a number.");
        }
        self.base.clear();
        self.base.push_str(self.values[0].as_str());
    }

    fn validate_internal_attribute(&mut self) {
        self.validate_one_value();
        if let Some(value) = utils::string_to_bool(self.values[0].as_str()) {
            self.root = if value { "crate" } else { "appcui" };
            self.internal_mode = value;
        } else {
            panic!("The value for `internal` attribute can only be 'true' or 'false'. Provided value was: {}",self.values[0].as_str());
        }
    }
    fn validate_window_control(&mut self) {
        self.validate_one_value();
        if let Some(value) = utils::string_to_bool(self.values[0].as_str()) {
            self.window_control = value;
        } else {
            panic!("The value for `window` attribute can only be 'true' or 'false'. Provided value was: {}",self.values[0].as_str());
        }
    }
    fn validate_desktop_control(&mut self) {
        self.validate_one_value();
        if let Some(value) = utils::string_to_bool(self.values[0].as_str()) {
            self.desktop_control = value;
        } else {
            panic!("The value for `desktop` attribute can only be 'true' or 'false'. Provided value was: {}",self.values[0].as_str());
        }
    }
    fn validate_debug_attribute(&mut self) {
        self.validate_one_value();
        if let Some(value) = utils::string_to_bool(self.values[0].as_str()) {
            self.debug_mode = value;
        } else {
            panic!("The value for `debug` attribute can only be 'true' or 'false'. Provided value was: {}",self.values[0].as_str());
        }
    }
    fn validate_overwrite_attribute(&mut self, config: &mut TraitsConfig) {
        for trait_name in &self.values {
            if let Some(appcui_trait) = AppCUITrait::new(&trait_name) {
                if appcui_trait.get_trait_type() != TraitType::RawEvent {
                    panic!(
                        "Trait {trait_name} can not be used with the 'overwrite' attribute. Allowed traits for the 'overwrite' attribute are: {}", 
                        AppCUITrait::traits_of_type(TraitType::RawEvent)
                    );
                }
                // now try to update the trait
                if config.get(appcui_trait).can_be_overwritten() {
                    config.clear(appcui_trait);
                } else {
                    panic!(
                        "Trait {trait_name} can not be overwritten (for proc macro: {})",config.get_name());
                }
            } else {
                panic!(
                    "Unknown trait to allow overwriting: '{trait_name}'. Allowed traits are: {}",
                    AppCUITrait::traits_of_type(TraitType::RawEvent)
                );
            }
        }
    }
    fn validate_events_attribute(&mut self) {
        let mut limited_to_event_processor = true; // true = only event processor controls such as Window can process it
                                                   // false = all controls can process it

        for trait_name in &self.values {
            match trait_name.as_str() {
                "CommandBarEvents" | "CommandBar" => {
                    self.command_bar_events = true;
                    limited_to_event_processor = false;
                }
                "MenuEvents" | "Menu" => {
                    self.menu_events = true;
                    limited_to_event_processor = false;
                }
                "ButtonEvents" | "Button" => self.button_events = true,
                "CheckBoxEvents" | "CheckBox" => self.checkbox_events = true,
                "WindowEvents" | "Window" => self.window_events = true,

                other => {
                    panic!("Unknown event/control: '{other}'. Events that could be process are from : CommandBar, Menu");
                }
            }
            if limited_to_event_processor {
                // add trait name
                if self.event_processor_list.len() > 0 {
                    self.event_processor_list.push(',');
                }
                self.event_processor_list.push_str(&trait_name);
            }
        }
    }
    fn validate_key_value_pair(&mut self, config: &mut TraitsConfig) {
        match self.key.as_str() {
            "base" => self.validate_base_attribute(),
            "overwrite" => self.validate_overwrite_attribute(config),
            "events" => self.validate_events_attribute(),
            "debug" => self.validate_debug_attribute(),
            "internal" => self.validate_internal_attribute(),
            "window" => self.validate_window_control(),
            "desktop" => self.validate_desktop_control(),
            _ => {
                panic!("Unknown attribute `{}` for AppCUI. Accepted attributes are 'base' , 'overwrite' and 'debug' !",self.key.as_str());
            }
        }
    }

    fn validate_expect_key(&mut self, token: TokenTree) {
        if let TokenTree::Ident(ident) = token {
            self.key = ident.to_string();
            self.values.clear();
            self.state = State::ExpectEqual;
        } else {
            panic!(
                "Expecting a key (a-zA-Z0-9) but got: `{}`",
                token.to_string()
            );
        }
    }
    fn validate_expect_equal(&mut self, token: TokenTree) {
        if let TokenTree::Punct(punctuation) = token {
            if (punctuation.as_char() != '=') && (punctuation.as_char() != ':') {
                panic!(
                    "Expecting asign ('=' or ':') symbol but got: {}",
                    punctuation.as_char()
                );
            }
            self.state = State::ExpectValue;
        } else {
            panic!(
                "Expecting asign ('=' or ':') symbol but got: {}",
                token.to_string()
            );
        }
    }
    fn validate_expect_value(&mut self, token: TokenTree) {
        if let TokenTree::Ident(ident) = token {
            self.values.push(ident.to_string());
            self.state = State::ExpectComma;
        } else if let TokenTree::Literal(literal) = token {
            self.values.push(literal.to_string());
            self.state = State::ExpectComma;
        } else {
            panic!(
                "Expecting a value (a-zA-Z0-9) but got: `{}`",
                token.to_string()
            );
        }
    }
    fn validate_expect_comma(&mut self, token: TokenTree, config: &mut TraitsConfig) {
        if let TokenTree::Punct(punctuation) = token {
            if punctuation.as_char() == '+' {
                self.state = State::ExpectValue;
                return;
            }
            if punctuation.as_char() != ',' {
                panic!(
                    "Expecting delimiter (',' comma) symbol but got: {}",
                    punctuation.as_char()
                );
            }
            self.validate_key_value_pair(config);
            self.state = State::ExpectKey;
        } else {
            panic!(
                "Expecting delimiter (',' comma) symbol but got:{}",
                token.to_string()
            );
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
                panic!("Unexpected end of procedural macro attribute (expecting an asignament character ':' or '=') after key: '{}'", self.key.as_str());
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
