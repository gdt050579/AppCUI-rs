use std::{
    collections::{HashMap, HashSet},
    iter::Map,
};

use super::{
    appcui_traits::{AppCUITrait, TraitType},
    traits_configuration::TraitsConfig,
    BaseControlType,
};

use proc_macro::*;

enum ExpectNext {
    Key,
    Equal,
    Value,
    Comma,
    Template,
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
    pub emitted_events: Vec<String>,
    pub custom_events: Vec<String>,
    pub template_events: HashMap<String, Vec<String>>,
    // internal
    expect_next: ExpectNext,
    key: String,
    template_content: String,
    template_depth: u32,
    values: Vec<String>,
}

impl Arguments {
    pub fn new(base_control_type: BaseControlType) -> Arguments {
        Arguments {
            base_control_type,
            base: base_control_type.as_string(),
            modal_result_type: String::from("()"),
            expect_next: ExpectNext::Key,
            root: "appcui",
            key: String::new(),
            values: Vec::with_capacity(8),
            commands: Vec::new(),
            emitted_events: Vec::new(),
            custom_events: Vec::new(),
            template_events: HashMap::new(),
            debug_mode: false,
            internal_mode: false,
            window_control: false,
            desktop_control: false,
            template_content: String::new(),
            template_depth: 0,
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
                if appcui_trait.trait_type() != TraitType::RawEvent {
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
        self.commands = std::mem::take(&mut self.values);
    }
    fn validate_emitted_events(&mut self) {
        let mut h = HashSet::with_capacity(self.values.len() * 2);
        for event_name in &self.values {
            if let Err(desc) = crate::utils::validate_name(event_name.as_str(), false) {
                panic!("Invalid Evenat name:: '{}' => {}", event_name, desc);
            }
            let hash = crate::utils::compute_hash(event_name);
            if h.contains(&hash) {
                panic!("Events name must be unique. Duplicate event name: {}", event_name);
            }
            h.insert(hash);
        }
        // all good --> move current value vector into emitted events and create a new one for values
        self.emitted_events = std::mem::take(&mut self.values);
    }
    fn validate_custom_events(&mut self) {
        let mut h = HashSet::with_capacity(self.values.len() * 2);
        for trait_name in &self.values {
            if let Err(desc) = crate::utils::validate_name(trait_name.as_str(), false) {
                panic!("Invalid trait name: '{}' => {}", trait_name, desc);
            }
            // analyze format
            if !trait_name.ends_with("Events") {
                panic!("Custom events should have their trait name in the format ( <struct_name>Events - ex: MyCustomButtonEvents, where MyCustomButton is the actual custom control). This is required as a trait will be automatically generated with this name for you to implement.")
            }
            let hash = crate::utils::compute_hash(trait_name);
            if h.contains(&hash) {
                panic!("Custom events trait names must be unique. Duplicate trait name: {}", trait_name);
            }
            h.insert(hash);
        }
        // all good --> move current value vector into custom events and create a new one for values
        self.custom_events = std::mem::take(&mut self.values);
    }
    fn validate_events_attribute(&mut self, config: &mut TraitsConfig) {
        for trait_name in &self.values {
            if let Some(appcui_trait) = AppCUITrait::new(trait_name) {
                if appcui_trait.trait_type() != TraitType::ControlEvent {
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
            "emit" => self.validate_emitted_events(),
            "custom_events" => self.validate_custom_events(),
            _ => {
                panic!(
                    "Unknown attribute `{}` for AppCUI. Accepted attributes are 'overwrite', 'events', 'debug', 'response', 'commands', 'emit', 'custom_events' !",
                    self.key.as_str()
                );
            }
        }
    }

    fn validate_expect_key(&mut self, token: TokenTree) {
        if let TokenTree::Ident(ident) = token {
            self.key = ident.to_string();
            self.values.clear();
            self.expect_next = ExpectNext::Equal;
        } else {
            panic!("Expecting a key (a-zA-Z0-9) but got: `{}`", token);
        }
    }
    fn validate_expect_equal(&mut self, token: TokenTree) {
        if let TokenTree::Punct(punctuation) = token {
            if (punctuation.as_char() != '=') && (punctuation.as_char() != ':') {
                panic!("Expecting asign ('=' or ':') symbol but got: {}", punctuation.as_char());
            }
            self.expect_next = ExpectNext::Value;
        } else {
            panic!("Expecting asign ('=' or ':') symbol but got: {}", token);
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
                            panic!("Expecting a proper format for the list associated with the key: '{}'. It should use this format `[value-1, value-2, ... value-n]` but found `{}`",self.key,inner_token)
                        }
                    } else if let TokenTree::Punct(p) = inner_token {
                        if p.as_char() != ',' {
                            panic!("Expecting a separatoe ',' for the list associated with the key: '{}'. It should use this format `[value-1, value-2, ... value-n]` but found `{}`",self.key,p)
                        }
                    } else {
                        panic!("Expecting a separatoe ',' for the list associated with the key: '{}'. It should use this format `[value-1, value-2, ... value-n]` but found `{}`",self.key,inner_token)
                    }
                    expect_value = !expect_value;
                }
                self.expect_next = ExpectNext::Comma;
            }
            TokenTree::Ident(ident) => {
                self.values.push(ident.to_string());
                self.expect_next = ExpectNext::Comma;
            }
            TokenTree::Literal(literal) => {
                self.values.push(literal.to_string());
                self.expect_next = ExpectNext::Comma;
            }
            _ => {
                panic!("Expecting a value (a-zA-Z0-9) but got: `{}`", token);
            }
        }
    }
    fn validate_template(&mut self) {
        // Validate if a tmplate is correct (from example it should contain only letters and no spaces or other punctuation marks)
        // step 1 --> grab the last control in values list
        if self.values.is_empty() {
            let mut list_of_generic_controls = String::new();
            let mut index = 0u8;
            while let Some(appcui_trait) = AppCUITrait::with_discriminant(index) {
                if appcui_trait.is_generic() {
                    if !list_of_generic_controls.is_empty() {
                        list_of_generic_controls.push_str(", ");
                    }
                    list_of_generic_controls.push_str(appcui_trait.name());
                }
                index += 1;
            }
            panic!("Generic type '{}' without a proper control. You should haved used it as a template Control<Type> and not just <Type>. The following controls supports generic: {}, please select one of them for your generic type !",self.template_content,list_of_generic_controls);
        }
        if let Some(last_control) = AppCUITrait::new(self.values.last().unwrap().as_str()) {
            if last_control.is_generic() == false {
                panic!(
                    "Events of type `{}` are not generic and can not be used with a templetize form => '{}<{}>'",
                    last_control.name(),
                    last_control.name(),
                    self.template_content
                );
            }
            // now check if the template name is valid
            // we should also get a slice from the original name
            // add to a hash map
            self.template_events
                .entry(last_control.name().to_string())
                .or_insert_with(Vec::new)
                .push(self.template_content.clone());
        } else {
            // do nothing --> upon validation the template is not valid !! and error will occur anyway !
        }
    }
    fn validate_expect_template(&mut self, token: TokenTree) {
        match token {
            TokenTree::Group(g) => {
                panic!(
                    "Invalid group delimiter {} in a template definition : {}",
                    g.to_string(),
                    self.template_content
                );
            }
            TokenTree::Ident(id) => {
                self.template_content.push_str(id.to_string().as_str());
                self.template_content.push(' ');
            }
            TokenTree::Punct(punctuation) => match punctuation.as_char() {
                '<' => {
                    self.template_content.push('<');
                    self.template_depth += 1;
                }
                '>' => {
                    self.template_depth -= 1;
                    if self.template_depth > 0 {
                        self.template_content.push('>');
                    } else {
                        // template is over
                        self.validate_template();
                        self.expect_next = ExpectNext::Comma;
                    }
                }
                other_punctuation_mark @ _ => {
                    self.template_content.push(other_punctuation_mark);
                }
            },
            TokenTree::Literal(lit) => {
                self.template_content.push_str(lit.to_string().as_str());
                self.template_content.push(' ');
            }
        }
    }
    fn validate_expect_comma(&mut self, token: TokenTree, config: &mut TraitsConfig) {
        if let TokenTree::Punct(punctuation) = token {
            if punctuation.as_char() == '+' {
                self.expect_next = ExpectNext::Value;
                return;
            }
            if punctuation.as_char() == '<' {
                self.template_content.clear();
                self.template_depth = 1;
                self.expect_next = ExpectNext::Template;
                return;
            }
            if punctuation.as_char() != ',' {
                panic!("Expecting delimiter (',' comma) symbol but got: {}", punctuation.as_char());
            }
            self.validate_key_value_pair(config);
            self.expect_next = ExpectNext::Key;
        } else {
            panic!("Expecting a punctuation symbol (e.g. ',' comma), but got:{}", token);
        }
    }
    pub(crate) fn parse(&mut self, input: TokenStream, config: &mut TraitsConfig) {
        for token in input.into_iter() {
            // println!("arg_token: {:?}", token);
            match self.expect_next {
                ExpectNext::Key => self.validate_expect_key(token),
                ExpectNext::Equal => self.validate_expect_equal(token),
                ExpectNext::Value => self.validate_expect_value(token),
                ExpectNext::Comma => self.validate_expect_comma(token, config),
                ExpectNext::Template => self.validate_expect_template(token),
            }
        }
        match self.expect_next {
            ExpectNext::Key => {}
            ExpectNext::Equal => {
                panic!(
                    "Unexpected end of procedural macro attribute (expecting an asignament character ':' or '=') after key: '{}'",
                    self.key.as_str()
                );
            }
            ExpectNext::Value => {
                panic!(
                    "Unexpected end of procedural macro attribute (expecting a value for key: '{}')",
                    self.key.as_str()
                );
            }
            ExpectNext::Template => {
                panic!(
                    "Unexpected end of procedural macro attribute (incomplete template for the value of key: '{}')",
                    self.key.as_str()
                );
            }
            ExpectNext::Comma => self.validate_key_value_pair(config),
        }
    }
}
