use super::*;
use std::collections::HashMap;

pub(crate) struct NamedParamsMap<'a> {
    named: HashMap<u64, Value<'a>>,
    ordered: Vec<Value<'a>>,
}
enum ValueType<'a> {
    Undetermined,
    Bool(bool),
    Integer(i32),
    Percentage(f32),
    List(Vec<Value<'a>>),
    Dict(NamedParamsMap<'a>),
}

pub(crate) struct Value<'a> {
    raw_data: &'a str,
    data_type: ValueType<'a>,
}
impl<'a> Value<'a> {
    pub(crate) fn is_dict(&self) -> bool {
        match self.data_type {
            ValueType::Dict(_) => true,
            _ => false,
        }
    }
    pub(crate) fn is_list(&self) -> bool {
        match self.data_type {
            ValueType::List(_) => true,
            _ => false,
        }
    }
    pub(crate) fn is_value(&self) -> bool {
        match self.data_type {
            ValueType::List(_) | ValueType::Dict(_) => false,
            _ => true,
        }
    }
    pub(crate) fn get_bool(&mut self) -> Option<bool> {
        if !self.is_value() {
            return None;
        }
        if let ValueType::Bool(value) = &self.data_type {
            return Some(*value);
        }
        if utils::equal_ignore_case("true", self.raw_data) {
            self.data_type = ValueType::Bool(true);
            return Some(true);
        }
        if utils::equal_ignore_case("false", self.raw_data) {
            self.data_type = ValueType::Bool(false);
            return Some(false);
        }
        None
    }
}

fn parse_vec<'a>(text: &'a str, tokenizer: &Tokenizer, index_start: usize, index_end: usize) -> Result<Vec<Value<'a>>, Error> {
    let mut v: Vec<Value> = Vec::with_capacity(8);
    let mut pos = index_start;
    while pos < index_end {
        let format = tokenizer.analyze(text, pos, index_end, true, false)?;
        let next = tokenizer.get(pos).get_next(pos);
        let data_type = match tokenizer.get(pos).get_type() {
            TokenType::OpenBrace => ValueType::Dict(parse_dict(text, tokenizer, pos + 1, next - 1)?),
            TokenType::OpenSquareBracket => ValueType::List(parse_vec(text, tokenizer, pos + 1, next - 1)?),
            _ => ValueType::Undetermined,
        };
        v.push(Value {
            raw_data: tokenizer.get(pos).get_text(text),
            data_type,
        });
        pos = format.get_next_pos();
    }
    Ok(v)
}

fn parse_dict<'a>(text: &'a str, tokenizer: &Tokenizer, index_start: usize, index_end: usize) -> Result<NamedParamsMap<'a>, Error> {
    let mut r = NamedParamsMap {
        named: HashMap::with_capacity(8),
        ordered: Vec::with_capacity(8),
    };
    let mut pos = index_start;
    let mut allow_value = true;
    while pos < index_end {
        let format = tokenizer.analyze(text, pos, index_end, allow_value, true)?;
        if format.is_key_value() {
            let key_token = tokenizer.get(pos);
            let key = utils::compute_hash(key_token.get_text(text));
            if r.named.contains_key(&key) {
                return Err(Error::with_token(
                    text,
                    format!("Duplicate parameter: '{}'", key_token.get_text(text)).as_str(),
                    &key_token,
                ));
            }
            let next = tokenizer.get(pos + 2).get_next(pos + 2);
            let data_type = match tokenizer.get(pos + 2).get_type() {
                TokenType::OpenBrace => ValueType::Dict(parse_dict(text, tokenizer, pos + 3, next - 1)?),
                TokenType::OpenSquareBracket => ValueType::List(parse_vec(text, tokenizer, pos + 3, next - 1)?),
                _ => ValueType::Undetermined,
            };
            r.named.insert(
                key,
                Value {
                    raw_data: key_token.get_text(text),
                    data_type,
                },
            );
        } else {
            let next = tokenizer.get(pos).get_next(pos);
            let data_type = match tokenizer.get(pos).get_type() {
                TokenType::OpenBrace => ValueType::Dict(parse_dict(text, tokenizer, pos + 1, next - 1)?),
                TokenType::OpenSquareBracket => ValueType::List(parse_vec(text, tokenizer, pos + 1, next - 1)?),
                _ => ValueType::Undetermined,
            };
            r.ordered.push(Value {
                raw_data: tokenizer.get(pos).get_text(text),
                data_type,
            });
        }
        // first time a key:value is found, seding values in order is not possible anymore
        allow_value &= !format.is_key_value();
        pos = format.get_next_pos();
    }
    Ok(r)
}
pub(crate) fn parse<'a>(text: &'a str) -> Result<NamedParamsMap<'a>, Error> {
    let t = Tokenizer::new(text)?;
    let r = parse_dict(text, &t, 0, t.count())?;
    Ok(r)
}
