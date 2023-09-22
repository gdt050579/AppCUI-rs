use super::*;
use std::collections::HashMap;

pub(crate) struct NamedParamsMap<'a> {
    named: HashMap<u64, Value<'a>>,
}
pub(crate) enum ValueType<'a> {
    Integer(i32),
    String(&'a str),
    Percentage(i32),
    List(Vec<Value<'a>>),
    Dict(NamedParamsMap<'a>),
}
pub(crate) struct Value<'a> {
    raw_data: &'a str,
    data_type: ValueType<'a>,
}

fn parse_dict<'a>(text: &str, tokenizer: &Tokenizer, index_start: usize, index_end: usize) -> Result<NamedParamsMap<'a>, Error> {
    let r = NamedParamsMap {
        named: HashMap::with_capacity(8),
    };
    let mut pos = index_start;
    let mut allow_value = true;
    while pos < index_end {
        let format = tokenizer.analyze(text, pos, index_end, allow_value, true)?;

        // first time a key:value is found, seding values in order is not possible anymore
        allow_value &= !format.is_key_value();
        pos = format.get_next_pos();
    }
    Ok(r)
}
pub(crate) fn parse<'a>(text: &str) -> Result<NamedParamsMap<'a>, Error> {
    let t = Tokenizer::new(text)?;
    let r = parse_dict(text, &t, 0, t.count())?;
    Ok(r)
}
