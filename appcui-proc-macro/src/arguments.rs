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
    // overwritebles (common events)
    pub on_paint: bool,
    pub on_key_pressed: bool,
    pub on_mouse_event: bool,
    pub on_default_action: bool,
    pub on_resize: bool,
    pub on_focus: bool,
    // control events
    pub command_bar_events: bool,
    pub menu_events: bool,
    pub button_events: bool,

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
            // overwritebles (common events)
            on_paint: false,
            on_key_pressed: false,
            on_mouse_event: false,
            on_default_action: false,
            on_resize: false,
            on_focus: false,
            // control events
            menu_events: false,
            command_bar_events: false,
            button_events: false
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
        } else {
            panic!("The value for `internal` attribute can only be 'true' or 'false'. Provided value was: {}",self.values[0].as_str());
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
    fn validate_overwrite_attribute(&mut self) {
        for trait_name in &self.values {
            match trait_name.as_str() {
                "OnPaint" => self.on_paint = true,
                "OnKeyPressed" => self.on_key_pressed = true,
                "OnMouseEvent" => self.on_mouse_event = true,
                "OnDefaultAction" => self.on_default_action = true,
                "OnResize" => self.on_resize = true,
                "OnFocus" => self.on_focus = true,
                other => {
                    panic!("Unknown trait to allow overwriting: '{other}'. Allowed traits are: OnPaint, OnKeyPressed, OnMouseEvent, OnDefaultAction, OnResize, OnFocus");
                }
            }
        }
    }
    fn validate_events_attribute(&mut self) {
        let mut limited_to_event_processor: bool;   // true = only event processor controls such as Window can process it
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
                "ButtonEvents" | "Button" => {
                    self.button_events = true;
                    limited_to_event_processor = true;
                }
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
    fn validate_key_value_pair(&mut self) {
        match self.key.as_str() {
            "base" => self.validate_base_attribute(),
            "overwrite" => self.validate_overwrite_attribute(),
            "events" => self.validate_events_attribute(),
            "debug" => self.validate_debug_attribute(),
            "internal" => self.validate_internal_attribute(),
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
    fn validate_expect_comma(&mut self, token: TokenTree) {
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
            self.validate_key_value_pair();
            self.state = State::ExpectKey;
        } else {
            panic!(
                "Expecting delimiter (',' comma) symbol but got:{}",
                token.to_string()
            );
        }
    }
    pub fn parse(&mut self, input: TokenStream) {
        for token in input.into_iter() {
            // println!("arg_token: {:?}", token);
            match self.state {
                State::ExpectKey => self.validate_expect_key(token),
                State::ExpectEqual => self.validate_expect_equal(token),
                State::ExpectValue => self.validate_expect_value(token),
                State::ExpectComma => self.validate_expect_comma(token),
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
            State::ExpectComma => self.validate_key_value_pair(),
        }
    }
}
