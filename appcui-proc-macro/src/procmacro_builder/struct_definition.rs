#[derive(Debug)]
pub(crate) struct StructDefinition {
    pub(crate) name: String,
    pub(crate) template_type: String,
    pub(crate) template_def: String,
}

impl StructDefinition {
    fn skip_while(buf: &[u8], pos: usize, condition: fn(u8) -> bool) -> usize {
        let len = buf.len();
        let mut pos = pos;
        while (pos < len) && (condition(buf[pos])) {
            pos += 1;
        }
        pos
    }
    fn skip_spaces(buf: &[u8], pos: usize) -> usize {
        StructDefinition::skip_while(buf, pos, |value| {
            matches!(value, b' ' | b'\t' | b'\n' | b'\r')
        })
    }
    fn skip_word(buf: &[u8], pos: usize) -> usize {
        StructDefinition::skip_while(
            buf,
            pos,
            |value| matches!(value, b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_'),
        )
    }
}
impl From<&str> for StructDefinition {
    fn from(value: &str) -> Self {
        if let Some(mut pos) = value.find("struct") {
            pos += 6;
            let buf = value.as_bytes();
            let len = buf.len();

            pos = StructDefinition::skip_spaces(buf, pos);
            let start_name = pos;
            pos = StructDefinition::skip_word(buf, pos);
            let end_name = pos;
            pos = StructDefinition::skip_spaces(buf, pos);

            if pos == len {
                panic!("Unexpected end of structure definition (expecting a '{{' or an '<' character) ! ");
            }
            match buf[pos] {
                b'{' => {
                    // normal format --> without template
                    StructDefinition {
                        name: String::from(&value[start_name..end_name]),
                        template_type: String::new(),
                        template_def: String::new(),
                    }
                }
                b'<' => {
                    // we have a template
                    let start_template = pos;
                    let end_template =
                        StructDefinition::skip_while(buf, pos, |value| value != b'{');
                    if end_template == len {
                        panic!("Unexpected end of structure definition (expecting a '{{' after the template definition) ! ");
                    }
                    // find the template type
                    let start_tamplate_type = StructDefinition::skip_spaces(buf, pos + 1);
                    let end_template_type = StructDefinition::skip_word(buf, start_tamplate_type);
                    if end_template_type==start_tamplate_type {
                        panic!("Expecting a valid template type (e.g. <T>");
                    }

                    let mut template_type = String::with_capacity(8);
                    template_type.push('<');
                    template_type.push_str(&value[start_tamplate_type..end_template_type]);
                    template_type.push('>');
                    StructDefinition {
                        name: String::from(&value[start_name..end_name]),
                        template_type,
                        template_def: String::from(&value[start_template..end_template]),
                    }
                }
                _ => {
                    panic!(
                        "Unexpected word in after a structure name: {}",
                        &value[pos..]
                    );
                }
            }
        } else {
            panic!("Expecting a structure definition !");
        }
    }
}