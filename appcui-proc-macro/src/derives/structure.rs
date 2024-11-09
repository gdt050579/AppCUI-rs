use proc_macro::{TokenStream, TokenTree};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
enum TokenType {
    Attribute,
    Vizibility,
    StructKeyword,
    Name,
    TwoPoints,
    Comma,
    Equal,
    Generics,
    Other,
    GroupWithParenthesis,
}

#[derive(Debug, Eq, PartialEq)]
enum TokenTreeType {
    Ident,
    Punct,
    Literal,
    Group,
    None,
}

impl TokenType {
    fn try_from(tokens: &[TokenTree], start: usize, error: &str) -> Result<Self, String> {
        if start >= tokens.len() {
            return Err(format!(
                "Unexpected end of token stream: {} (provided index: {}, tokens size: {})",
                error,
                start,
                tokens.len()
            ));
        }
        let token = &tokens[start];
        Ok(Self::from(token))
    }
}
impl From<&TokenTree> for TokenType {
    fn from(value: &TokenTree) -> Self {
        match value {
            TokenTree::Ident(ident) => {
                let value = ident.to_string();
                match value.as_str() {
                    "pub" => Self::Vizibility,
                    "struct" => Self::StructKeyword,
                    _ => Self::Name,
                }
            }
            TokenTree::Punct(punct) => {
                let value = punct.as_char();
                match value {
                    '#' => Self::Attribute,
                    ':' => Self::TwoPoints,
                    '=' => Self::Equal,
                    ',' => Self::Comma,
                    '<' => Self::Generics,
                    _ => Self::Other,
                }
            }
            TokenTree::Literal(_) => Self::Other,
            TokenTree::Group(group) => match group.delimiter() {
                proc_macro::Delimiter::Parenthesis => Self::GroupWithParenthesis,
                _ => Self::Other,
            },
        }
    }
}

#[derive(Debug)]
pub(crate) struct StructureField {
    pub(crate) name: String,
    pub(crate) ty: String,
    pub(crate) attributes: HashMap<String, String>,
}

#[derive(Debug)]
pub(crate) struct Structure {
    pub(crate) name: String,
    pub(crate) _generics: String,
    pub(crate) fields: Vec<StructureField>,
}
impl Structure {
    fn parse_one_attribute(tokens: &[TokenTree], start: usize, prefix: &str, attr: &mut HashMap<String, String>) -> Result<usize, String> {
        let mut index = start;
        let token_type = TokenType::try_from(tokens, index, "Expecting a valid attribute name")?;
        if token_type != TokenType::Name {
            return Err(format!("Expecting an attribute name definition but found: '{:?}'", &tokens[index]));
        };
        let key = if prefix.is_empty() {
            Self::token_to_string(&tokens[index])
        } else {
            format!("{}.{}", prefix, Self::token_to_string(&tokens[index]))
        };
        if attr.contains_key(&key) {
            return Err(format!("Duplicated attribute: '{}'", key));
        }
        index += 1;
        if index >= tokens.len() {
            // we've reached th end of the token stream
            attr.insert(key, String::new());
            return Ok(tokens.len());
        }
        let token_type = TokenType::try_from(tokens, index, "Expecting a valid attribute definition")?;
        match token_type {
            TokenType::Equal | TokenType::TwoPoints => {
                index += 1;
                if index >= tokens.len() {
                    return Err("Unexpected end of token stream (expecting a valid attribute value)".to_string());
                }
                attr.insert(key, Self::token_to_string(&tokens[index]));
                index += 1;
            }
            TokenType::Comma => {
                attr.insert(key, String::new());
                return Ok(index + 1);
            }
            TokenType::GroupWithParenthesis => match &tokens[index] {
                TokenTree::Group(group) => {
                    let content = group.stream();
                    Self::parse_attributes_params(content, key.as_str(), attr)?;
                    index += 1;
                }
                _ => {
                    return Err(format!("Expecting a group with parenthesis but found: '{:?}'", tokens[index]));
                }
            },
            _ => {
                return Err(format!(
                    "Expecting an equal sign or a value definition but found: '{:?}' for attribute: '{}'",
                    &tokens[index], key
                ));
            }
        }
        if index >= tokens.len() {
            return Ok(tokens.len());
        }
        let token_type = TokenType::try_from(tokens, index, "Expecting a comma")?;
        if token_type != TokenType::Comma {
            return Err(format!("Expecting a comma but found: '{:?}'", &tokens[index]));
        }
        Ok(index + 1)
    }
    fn parse_attributes_params(ast: TokenStream, prefix: &str, attr: &mut HashMap<String, String>) -> Result<(), String> {
        // allowed formats: key = value, key: value, key, key_group(...)
        let v: Vec<_> = ast.clone().into_iter().collect();
        let mut index = 0;
        while index < v.len() {
            index = Self::parse_one_attribute(v.as_slice(), index, prefix, attr)?;
        }
        Ok(())
    }
    fn parse_attributes(tokens: &[TokenTree], start: usize, attr: Option<&mut HashMap<String, String>>) -> Result<usize, String> {
        // assume that the first token is a # character
        if start >= tokens.len() {
            return Err(format!(
                "Unexpected end of token stream (provided index: {}, tokens size: {}",
                start,
                tokens.len()
            ));
        }
        // assume that the first token is a # character
        if let TokenTree::Punct(punct) = &tokens[start] {
            if punct.as_char() != '#' {
                return Err(format!("Expecting a '#' character but found: '{}'", punct.as_char()));
            }
        }
        // the next token should be a gourp with [...]
        if start + 1 >= tokens.len() {
            return Err(format!(
                "Unexpected end of token stream. Expecting a [..] after # character (provided index: {}, tokens size: {}",
                start + 1,
                tokens.len()
            ));
        }
        if let TokenTree::Group(group) = &tokens[start + 1] {
            if group.delimiter() != proc_macro::Delimiter::Bracket {
                return Err(format!(
                    "Expecting a bracket ('[') after the '#' character but found: '{:?}'",
                    group.delimiter()
                ));
            }
            if let Some(attr) = attr {
                Self::parse_attributes_params(group.stream(), "", attr)?;
            }
        }
        Ok(start + 2)
    }
    fn skip_visibility(tokens: &[TokenTree], start: usize) -> Result<usize, String> {
        if start >= tokens.len() {
            return Err(format!(
                "Unexpected end of token stream (provided index: {}, tokens size: {}",
                start,
                tokens.len()
            ));
        }
        if let TokenTree::Ident(ident) = &tokens[start] {
            let value = ident.to_string();
            if value == "pub" {
                if start + 1 >= tokens.len() {
                    return Err(format!(
                        "Unexpected end of token stream after the 'pub' keyword (provided index: {}, tokens size: {}",
                        start + 1,
                        tokens.len()
                    ));
                }
                if let TokenTree::Group(group) = &tokens[start + 1] {
                    if group.delimiter() != proc_macro::Delimiter::Parenthesis {
                        return Err(format!(
                            "Expecting a parenthesis after the 'pub' keyword but found: '{:?}'",
                            group.delimiter()
                        ));
                    }
                    return Ok(start + 2);
                }
                return Ok(start + 1);
            }
        }
        Err(format!("Expecting 'pub' keyword but found: '{:?}'", tokens[start]))
    }
    fn token_to_string(token: &TokenTree) -> String {
        match token {
            TokenTree::Ident(ident) => ident.to_string(),
            TokenTree::Punct(punct) => punct.to_string(),
            TokenTree::Literal(literal) => {
                let mut res = literal.to_string();
                if (res.starts_with("\"") && res.ends_with("\"")) || (res.starts_with("'") && res.ends_with("'")) {
                    res.truncate(res.len() - 1);
                    if res.len() > 1 {
                        res.remove(0);
                    }
                }
                res
            }
            TokenTree::Group(group) => group.stream().to_string(),
        }
    }
    fn tokens_to_generics(tokens: &[TokenTree], start: usize, end: usize) -> String {
        let mut s = String::new();
        let mut last_token_type = TokenTreeType::None;
        for token in tokens.iter().take(end).skip(start) {
            let (content, token_type) = match token {
                TokenTree::Group(group) => (group.stream().to_string(), TokenTreeType::Group),
                TokenTree::Ident(ident) => (ident.to_string(), TokenTreeType::Ident),
                TokenTree::Punct(punct) => (punct.to_string(), TokenTreeType::Punct),
                TokenTree::Literal(literal) => (literal.to_string(), TokenTreeType::Literal),
            };
            if (token_type == last_token_type) && (token_type != TokenTreeType::Punct) {
                s.push(' ');
            }
            s.push_str(&content);
            last_token_type = token_type;
        }
        s
    }
    fn type_to_string(tokens: &[TokenTree], start: usize, end: usize) -> String {
        let mut s = String::new();
        let mut skip_next = false;
        for token in &tokens[start..end] {
            let content = if skip_next {
                skip_next = false;
                None
            } else {
                match token {
                    TokenTree::Group(group) => {
                        let (l, r) = match group.delimiter() {
                            proc_macro::Delimiter::Parenthesis => ("(", ")"),
                            proc_macro::Delimiter::Brace => ("{", "}"),
                            proc_macro::Delimiter::Bracket => ("[", "]"),
                            proc_macro::Delimiter::None => (" ", " "),
                        };
                        let group_tokens: Vec<_> = group.stream().clone().into_iter().collect();
                        let inner_str = Self::type_to_string(group_tokens.as_slice(), 0, group_tokens.len());
                        Some(format!("{}{}{}", l, inner_str.as_str(), r))
                    }
                    TokenTree::Ident(ident) => Some(ident.to_string()),
                    TokenTree::Punct(punct) => {
                        let value = punct.as_char();
                        match value {
                            '\'' => {
                                skip_next = true;
                                None
                            }
                            _ => Some(format!("{}", value)),
                        }
                    }
                    TokenTree::Literal(literal) => Some(literal.to_string()),
                }
            };
            if let Some(content) = content {
                s.push_str(&content);
            }
        }
        s
    }
    fn group_to_stream(token: &TokenTree) -> Result<TokenStream, String> {
        match token {
            TokenTree::Group(group) => {
                if group.delimiter() == proc_macro::Delimiter::Brace {
                    Ok(group.stream())
                } else {
                    Err(format!("Unexpected token: '{:?}' -> Expecting a brace '{{' !", group.delimiter()))
                }
            }
            _ => Err(format!("Unexpected token: '{}' -> Expecting a brace '{{' !", token)),
        }
    }
    fn parse_field_type(tokens: &[TokenTree], start: usize) -> Result<usize, String> {
        let mut depth = 0;
        let t = &tokens[start..];
        for (index, token) in t.iter().enumerate() {
            if let TokenTree::Punct(punct) = token {
                let value = punct.as_char();
                match value {
                    ',' => {
                        if depth == 0 {
                            return Ok(index + start);
                        }
                    }
                    '<' => {
                        depth += 1;
                    }
                    '>' => {
                        depth -= 1;
                        if depth < 0 {
                            return Err("Unexpected '>' character in field definition ".to_string());
                        }
                    }
                    '#' => {
                        return Err("Invalid attribute in field definition ('#')".to_string());
                    }
                    _ => {}
                }
            }
        }
        if depth != 0 {
            return Err("Unexpected end of token stream (expecting a valid field type definition). Too many '<' characters (make sure that the template/generics definition is correct)".to_string());
        }
        Ok(tokens.len())
    }
    fn parse_field(tokens: &[TokenTree], start: usize, fields: &mut Vec<StructureField>) -> Result<usize, String> {
        let mut index = start;
        let mut attributes = HashMap::new();
        let name;
        loop {
            if index >= tokens.len() {
                return Err("Unexpected end of token stream (expecting a field definition)".to_string());
            }
            let token = &tokens[index];
            let token_type = TokenType::from(token);
            match token_type {
                TokenType::Attribute => {
                    index = Self::parse_attributes(tokens, index, Some(&mut attributes))?;
                }
                TokenType::Vizibility => {
                    index = Self::skip_visibility(tokens, index)?;
                }
                TokenType::Name => {
                    name = Self::token_to_string(token);
                    index += 1;
                    break;
                }
                _ => {
                    return Err(format!("Expecting a valid field definition but found: '{:?}'", token));
                }
            }
        }
        if index >= tokens.len() {
            return Err("Unexpected end of token stream (expecting a field type definition after field name)".to_string());
        }
        let token = &tokens[index];
        let token_type = TokenType::from(token);
        if token_type != TokenType::TwoPoints {
            return Err(format!("Expecting a colon (:) after the field name but found: '{:?}'", token));
        }
        index += 1;
        if index >= tokens.len() {
            return Err("Unexpected end of token stream (expecting a field type definition)".to_string());
        }
        let next = Self::parse_field_type(tokens, index)?;

        fields.push(StructureField {
            name,
            ty: Self::type_to_string(tokens, index, next),
            attributes,
        });
        Ok((next + 1).min(tokens.len()))
    }
    fn parse_fileds(ast: TokenStream) -> Result<Vec<StructureField>, String> {
        let v: Vec<_> = ast.clone().into_iter().collect();
        let mut result = Vec::new();
        let mut index = 0;
        while index < v.len() {
            index = Self::parse_field(&v, index, &mut result)?;
        }
        Ok(result)
    }
    fn parse_struct_generics_and_lifetime(tokens: &[TokenTree], start: usize) -> Result<usize, String> {
        // skip the generics definition
        let mut depth = 0;
        let mut index = start;
        loop {
            if index >= tokens.len() {
                return Err("Unexpected end of token stream (expecting a stuct definition with a lifetime/generics: `struct Name <lifetime or generics> {...}`".to_string());
            }
            let token = &tokens[index];
            if let TokenTree::Punct(punct) = token {
                let value = punct.as_char();
                match value {
                    '<' => {
                        depth += 1;
                    }
                    '>' => {
                        depth -= 1;
                    }
                    _ => {}
                }
            }
            if depth == 0 {
                break;
            }
            index += 1;
        }

        Ok(index + 1)
    }
    pub(crate) fn from(ast: TokenStream) -> Result<Self, String> {
        let v: Vec<_> = ast.clone().into_iter().collect();

        let mut index = 0;
        loop {
            if index >= v.len() {
                return Err("Unexpected end of token stream (expecting a stuct definition: `struct <Name> {...}`".to_string());
            }
            let token = &v[index];
            let token_type = TokenType::from(token);
            match token_type {
                TokenType::Attribute => {
                    index = Self::parse_attributes(&v, index, None)?;
                }
                TokenType::Vizibility => {
                    index = Self::skip_visibility(&v, index)?;
                }
                TokenType::StructKeyword => {
                    index += 1;
                    break;
                }
                _ => {
                    return Err(format!(
                        "Expecting a valid structure definition (with the keyword 'struct'), but found: '{:?}'",
                        token
                    ));
                }
            }
        }
        if index >= v.len() {
            return Err("Unexpected end of token stream (expecting a structure name)".to_string());
        }
        let name = Self::token_to_string(&v[index]);
        index += 1;
        let token_type = TokenType::try_from(
            v.as_slice(),
            index,
            "Expecting structure content {...} or a lifetime/generics definition <...> !",
        )?;
        let generics = if token_type == TokenType::Generics {
            let next = Self::parse_struct_generics_and_lifetime(v.as_slice(), index)?;
            let generics = Self::tokens_to_generics(v.as_slice(), index, next);
            index = next;
            generics
        } else {
            String::new()
        };

        let content = Self::group_to_stream(&v[index])?;
        let fields = Self::parse_fileds(content)?;

        Ok(Self { name, _generics: generics, fields })
    }
}
