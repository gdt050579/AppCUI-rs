use std::collections::HashMap;

use super::{
    named_params_map::NamedParamsMap,
    value::{Value, ValueType},
    *,
};

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
        let tok = tokenizer.get(pos);
        v.push(Value {
            raw_data: tok.get_text(text),
            data_type,
            validated: false,
            start: tok.get_start(),
            end: tok.get_end()
        });
        pos = format.get_next_pos();
    }
    Ok(v)
}

fn parse_dict<'a>(text: &'a str, tokenizer: &Tokenizer, index_start: usize, index_end: usize) -> Result<NamedParamsMap<'a>, Error> {
    let mut r = NamedParamsMap {
        named: HashMap::with_capacity(8),
        ordered: Vec::with_capacity(8),
        non_named_count: 0,
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
            r.ordered.push(Value {
                raw_data: key_token.get_text(text),
                data_type,
                validated: false,
                start: key_token.get_start(),
                end: key_token.get_end()
            });
            r.named.insert(key, (r.ordered.len() - 1) as u32);
        } else {
            let next = tokenizer.get(pos).get_next(pos);
            let data_type = match tokenizer.get(pos).get_type() {
                TokenType::OpenBrace => ValueType::Dict(parse_dict(text, tokenizer, pos + 1, next - 1)?),
                TokenType::OpenSquareBracket => ValueType::List(parse_vec(text, tokenizer, pos + 1, next - 1)?),
                _ => ValueType::Undetermined,
            };
            let tok = tokenizer.get(pos);
            r.ordered.push(Value {
                raw_data: tok.get_text(text),
                data_type,
                validated: false,
                start: tok.get_start(),
                end: tok.get_end()
            });
            r.non_named_count += 1;
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
