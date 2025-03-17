use proc_macro::{TokenStream, TokenTree, Delimiter};
use std::collections::HashMap;


#[derive(Debug, Eq, PartialEq)]
enum TokenType {
    Attribute,
    Visibility,
    EnumKeyword,
    Name,
    TwoPoints,
    Comma,
    Equal,
    Generics,
    Other,
    GroupWithParenthesis,
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
                    "pub" => Self::Visibility,
                    "enum" => Self::EnumKeyword,
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
                Delimiter::Parenthesis => Self::GroupWithParenthesis,
                _ => Self::Other,
            },
        }
    }
}

#[derive(Debug)]
pub(crate) struct EnumVariant {
    pub(crate) name: String,
    pub(crate) fields: Option<String>,
    pub(crate) attributes: HashMap<String, String>,
}

#[derive(Debug)]
pub(crate) struct Enum {
    pub(crate) name: String,
    pub(crate) _generics: String,
    pub(crate) variants: Vec<EnumVariant>,
}

impl Enum {
    fn token_to_string(token: &TokenTree) -> String {
        match token {
            TokenTree::Ident(ident) => ident.to_string(),
            TokenTree::Punct(punct) => punct.to_string(),
            TokenTree::Literal(literal) => {
                let mut res = literal.to_string();
                if (res.starts_with("\"") && res.ends_with("\""))
                    || (res.starts_with("'") && res.ends_with("'"))
                {
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

    fn parse_attributes_params(ast: TokenStream, prefix: &str, attr: &mut HashMap<String, String>) -> Result<(), String> {
        let tokens: Vec<_> = ast.into_iter().collect();
        let mut index = 0;
        while index < tokens.len() {
            index = Self::parse_one_attribute(&tokens, index, prefix, attr)?;
        }
        Ok(())
    }

    fn parse_one_attribute(tokens: &[TokenTree], start: usize, prefix: &str, attr: &mut HashMap<String, String>) -> Result<usize, String> {
        let mut index = start;
        let token_type = TokenType::try_from(tokens, index, "Expecting a valid attribute name")?;
        if token_type != TokenType::Name {
            return Err(format!("Expecting an attribute name but found: '{:?}'", &tokens[index]));
        }
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
            TokenType::GroupWithParenthesis => {
                if let TokenTree::Group(group) = &tokens[index] {
                    let content = group.stream();
                    Self::parse_attributes_params(content, key.as_str(), attr)?;
                    index += 1;
                } else {
                    return Err(format!("Expecting a group with parenthesis but found: '{:?}'", tokens[index]));
                }
            }
            _ => {
                return Err(format!("Expecting '=' or a value definition but found: '{:?}' for attribute: '{}'", &tokens[index], key));
            }
        }
        if index < tokens.len() {
            let token_type = TokenType::try_from(tokens, index, "Expecting a comma")?;
            if token_type != TokenType::Comma {
                return Err(format!("Expecting a comma but found: '{:?}'", &tokens[index]));
            }
            index += 1;
        }
        Ok(index)
    }

    fn parse_attributes(tokens: &[TokenTree], start: usize, attr: Option<&mut HashMap<String, String>>) -> Result<usize, String> {
        if start >= tokens.len() {
            return Err(format!(
                "Unexpected end of token stream (provided index: {}, tokens size: {})",
                start,
                tokens.len()
            ));
        }
        if let TokenTree::Punct(punct) = &tokens[start] {
            if punct.as_char() != '#' {
                return Err(format!("Expecting '#' for an attribute but found: '{}'", punct.as_char()));
            }
        }
        if start + 1 >= tokens.len() {
            return Err(format!(
                "Unexpected end of token stream. Expecting a [..] after '#' (provided index: {}, tokens size: {})",
                start + 1,
                tokens.len()
            ));
        }
        if let TokenTree::Group(group) = &tokens[start + 1] {
            if group.delimiter() != Delimiter::Bracket {
                return Err(format!("Expecting '[' after '#' but found: '{:?}'", group.delimiter()));
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
            else {
                return  Ok(start); // no pub specifier
            }
        }
        Err(format!("Expecting 'pub' keyword but found: '{:?}'", tokens[start]))
    }

    fn parse_variant(tokens: &[TokenTree], start: usize, variants: &mut Vec<EnumVariant>) -> Result<usize, String> {
        let mut index = start;
        let mut attributes = HashMap::new();

        // Parse attributes
        while index < tokens.len() {
            if let TokenTree::Punct(punct) = &tokens[index] {
                if punct.as_char() == '#' {
                    index = Self::parse_attributes(tokens, index, Some(&mut attributes))?;
                    continue;
                }
            }
            break;
        }

        // Next token shall be variant name
        if index >= tokens.len() {
            return Err("Unexpected end of token stream while parsing variant name".to_string());
        }
        let name_token = &tokens[index];
        let token_type = TokenType::from(name_token);
        if token_type != TokenType::Name {
            return Err(format!("Expecting variant name but found: '{:?}'", name_token));
        }
        let name = Self::token_to_string(name_token);
        index += 1;

        // Check if the variant has associated data
        let mut fields = None;
        if index < tokens.len() {
            if let TokenTree::Group(group) = &tokens[index] {
                // Accept both tuple variants (parenthesis) and struct variants (braces)
                let delim = group.delimiter();
                if delim == Delimiter::Parenthesis || delim == Delimiter::Brace {
                    fields = Some(group.stream().to_string());
                    index += 1;
                }
            }
        }
        // A comma may follow the variant.
        if index < tokens.len() {
            if let TokenTree::Punct(punct) = &tokens[index] {
                if punct.as_char() == ',' {
                    index += 1;
                }
            }
        }
        variants.push(EnumVariant { name, fields, attributes });
        Ok(index)
    }

    // Parse all enum variants inside {...}
    fn parse_variants(ast: TokenStream) -> Result<Vec<EnumVariant>, String> {
        let tokens: Vec<_> = ast.into_iter().collect();
        let mut variants = Vec::new();
        let mut index = 0;
        while index < tokens.len() {
            if let TokenTree::Punct(punct) = &tokens[index] {
                if punct.as_char() == ',' {
                    index += 1;
                    continue;
                }
            }
            index = Self::parse_variant(&tokens, index, &mut variants)?;
        }
        Ok(variants)
    }

    // Parse the generic section of enum
    fn parse_enum_generics(tokens: &[TokenTree], start: usize) -> Result<(String, usize), String> {
        if start >= tokens.len() {
            return Ok((String::new(), start));
        }
        let token_type = TokenType::try_from(tokens, start, "Expecting generics or enum body")?;
        if token_type != TokenType::Generics {
            return Ok((String::new(), start));
        }
        let mut depth = 0;
        let mut index = start;
        while index < tokens.len() {
            if let TokenTree::Punct(punct) = &tokens[index] {
                match punct.as_char() {
                    '<' => depth += 1,
                    '>' => {
                        depth -= 1;
                        if depth == 0 {
                            index += 1;
                            break;
                        }
                    }
                    _ => {}
                }
            }
            index += 1;
        }
        let generics = tokens[start..index]
            .iter()
            .map(Self::token_to_string)
            .collect::<Vec<_>>()
            .join("");
        Ok((generics, index))
    }

    /// Parse an enum definition from a TokenStream
    pub(crate) fn from(ast: TokenStream) -> Result<Self, String> {
        let tokens: Vec<_> = ast.into_iter().collect();
        let mut index = 0;

        // 1. Parse enum-level atributes
        let mut enum_attributes = HashMap::new();
        while index < tokens.len() {
            if let TokenTree::Punct(punct) = &tokens[index] {
                if punct.as_char() == '#' {
                    index = Self::parse_attributes(&tokens, index, Some(&mut enum_attributes))?;
                    continue;
                }
            }
            break;
        }

        // 2. Skip visibility if present
        index = Self::skip_visibility(&tokens, index)?;

        // 3. Expect "enum" keyword.
        if index >= tokens.len() {
            return Err("Unexpected end of token stream; expecting 'enum' keyword".to_string());
        }
        if TokenType::from(&tokens[index]) != TokenType::EnumKeyword {
            return Err(format!("Expecting 'enum' keyword but found: '{:?}'", tokens[index]));
        }
        index += 1;

        // 4. Expect the enum name.
        if index >= tokens.len() {
            return Err("Unexpected end of token stream; expecting enum name".to_string());
        }
        let name = Self::token_to_string(&tokens[index]);
        index += 1;

        // 5. Parse generics.
        let (_generics, new_index) = Self::parse_enum_generics(&tokens, index)?;
        index = new_index;

        // 6. Expect the enum body as a group with braces.
        if index >= tokens.len() {
            return Err("Unexpected end of token stream; expecting enum body".to_string());
        }
        let body = match &tokens[index] {
            TokenTree::Group(group) if group.delimiter() == Delimiter::Brace => group.stream(),
            _ => return Err(format!("Expecting enum body in braces but found: '{:?}'", tokens[index])),
        };

        // 7. Extract variants
        let variants = Self::parse_variants(body)?;
        Ok(Enum { name, _generics, variants })
    }
}