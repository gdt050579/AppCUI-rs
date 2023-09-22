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


fn parse_dict<'a>(tokenizer: &Tokenizer, index_start: usize, index_end: usize) -> Result<NamedParamsMap<'a>, Error> {
    let r = NamedParamsMap {
        named: HashMap::with_capacity(8),
    };
    let mut pos = index_start;
    while pos < index_end {
        let tok = tokenizer.get(pos);
    }
    Ok(r)
}
pub(crate) fn parse<'a>(text: &str) -> Result<NamedParamsMap<'a>, Error> {
    let t = Tokenizer::new(text)?;
    let r = parse_dict(&t, 0, t.count())?;
    Ok(r)
}
