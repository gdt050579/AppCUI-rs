use std::fmt;

/// Enum representing different types of Markdown elements.
#[derive(Debug)]
pub enum MarkdownElement {
    Header(String, usize),
    Paragraph(Vec<InlineElement>),
    UnorderedList(Vec<ListItem>),
    OrderedList(Vec<ListItem>),
}

/// Enum representing list items in Markdown. List items can be simple or nested.
#[derive(Debug)]
pub enum ListItem {
    Simple(Vec<InlineElement>),
    Nested(Box<MarkdownElement>),
}

/// Enum representing inline elements in Markdown (text, bold, italic, links).
#[derive(Debug)]
pub enum InlineElement {
    Text(String),
    Bold(String),
    Italic(String),
    Link(String, String),
}

// Temporary implementation to identify easier elements
impl fmt::Display for InlineElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InlineElement::Text(content) => write!(f, "{}", content),
            InlineElement::Bold(content) => write!(f, "**{}**", content),
            InlineElement::Italic(content) => write!(f, "_{}_", content),
            InlineElement::Link(text, url) => write!(f, "[{}]({})", text, url),
        }
    }
}

pub struct MarkdownParser;

impl MarkdownParser {
    /// Parses a string input into a vector of MarkdownElements.
    /// It identifies headers, lists, and paragraphs in the input.
    pub fn parse(input: &str) -> Vec<MarkdownElement> {
        let mut elements = Vec::new();
        let mut lines = input.lines();

        while let Some(line) = lines.next() {
            let trimmed = line.trim();

            if trimmed.starts_with('#') {
                elements.push(Self::parse_header(trimmed));
            } else if trimmed.starts_with('-') {
                elements.push(Self::parse_list(&mut lines, trimmed, false));
            } else if trimmed.starts_with(|c: char| c.is_digit(10)) && trimmed[1..].starts_with('.')
            {
                elements.push(Self::parse_list(&mut lines, trimmed, true));
            } else if !trimmed.is_empty() {
                elements.push(Self::parse_paragraph(trimmed));
            }
        }

        elements
    }

    /// Parses a header line (e.g., "# Header") into a `MarkdownElement::Header`
    fn parse_header(line: &str) -> MarkdownElement {
        let level = line.chars().take_while(|&c| c == '#').count();
        let content = line[level..].trim().to_string();
        MarkdownElement::Header(content, level)
    }

    /// Parses a paragraph line (e.g., "This is a paragraph") into a `MarkdownElement::Paragraph`.
    fn parse_paragraph(line: &str) -> MarkdownElement {
        let inline_elements = Self::parse_inline(line);
        MarkdownElement::Paragraph(inline_elements)
    }

    /// Parses inline elements within a line of text (e.g., bold, italic, links).
    fn parse_inline(input: &str) -> Vec<InlineElement> {
        let mut elements = Vec::new();
        let mut i = 0;

        while i < input.len() {
            if input[i..].starts_with("__") || input[i..].starts_with("**") {
                let delimiter = &input[i..i + 2];
                if let Some(end) = input[i + 2..].find(delimiter).map(|p| p + i + 2) {
                    elements.push(InlineElement::Bold(input[i + 2..end].to_string()));
                    i = end + 2;
                    continue;
                }
            } else if input[i..].starts_with('_') || input[i..].starts_with('*') {
                let delimiter = &input[i..i + 1];
                if let Some(end) = input[i + 1..].find(delimiter).map(|p| p + i + 1) {
                    elements.push(InlineElement::Italic(input[i + 1..end].to_string()));
                    i = end + 1;
                    continue;
                }
            } else if input[i..].starts_with('[') {
                if let Some(close_bracket) = input[i + 1..].find(']').map(|p| p + i + 1) {
                    if input[close_bracket + 1..].starts_with('(') {
                        if let Some(close_paren) = input[close_bracket + 2..].find(')').map(|p| p + close_bracket + 2) {
                            let text = input[i + 1..close_bracket].to_string();
                            let url = input[close_bracket + 2..close_paren].to_string();
                            elements.push(InlineElement::Link(text, url));
                            i = close_paren + 1;
                            continue;
                        }
                    }
                }
            } else {
                let next_special = input[i..].find(|c: char| c == '*' || c == '_' || c == '[').unwrap_or(input.len() - i);
                elements.push(InlineElement::Text(input[i..i + next_special].to_string()));
                i += next_special;
            }
        }
        elements
    }

    /// Parses a list in Markdown into either an `UnorderedList` or `OrderedList`.
    fn parse_list<'a>(
        lines: &mut impl Iterator<Item = &'a str>,
        first_line: &str,
        ordered: bool,
    ) -> MarkdownElement {
        let mut list_items = Vec::new();
        let mut current_level = 0;

        fn indentation_level(line: &str) -> usize {
            line.chars().take_while(|c| c.is_whitespace()).count()
        }

        let mut sublist_stack: Vec<(usize, Vec<ListItem>)> = Vec::new();

        current_level = indentation_level(first_line);
        let content = if ordered {
            let no_digit_line = first_line.trim_start_matches(|c: char| c.is_digit(10));
            no_digit_line.trim_start_matches('.').trim()
        } else {
            first_line.trim_start_matches('-').trim()
        };
        list_items.push(ListItem::Simple(Self::parse_inline(content)));

        while let Some(next_line) = lines.next() {
            let trimmed = next_line.trim();
            let next_level = indentation_level(next_line);

            let is_ordered =
                trimmed.starts_with(|c: char| c.is_digit(10)) && trimmed[1..].starts_with('.');
            let is_unordered = trimmed.starts_with('-');

            if (ordered && is_ordered) || (!ordered && is_unordered) {
                let content = if ordered {
                    let no_digit_line = trimmed.trim_start_matches(|c: char| c.is_digit(10));
                    no_digit_line.trim_start_matches('.').trim()
                } else {
                    trimmed.trim_start_matches('-').trim()
                };

                let item = ListItem::Simple(Self::parse_inline(content));

                if next_level > current_level {
                    // new sublist
                    sublist_stack.push((current_level, list_items));
                    list_items = vec![item];
                    current_level = next_level;
                } else if next_level < current_level {
                    // end the current sublist and go back to previous level
                    while let Some((prev_level, mut prev_items)) = sublist_stack.pop() {
                        if prev_level < next_level {
                            sublist_stack.push((prev_level, prev_items));
                            break;
                        }
                        prev_items.push(ListItem::Nested(Box::new(if ordered {
                            MarkdownElement::OrderedList(list_items)
                        } else {
                            MarkdownElement::UnorderedList(list_items)
                        })));
                        list_items = prev_items;
                        current_level = prev_level;
                    }
                    list_items.push(item);
                } else {
                    // Same level, add the item
                    list_items.push(item);
                }
            } else {
                break;
            }
        }

        while let Some((_, mut prev_items)) = sublist_stack.pop() {
            prev_items.push(ListItem::Nested(Box::new(if ordered {
                MarkdownElement::OrderedList(list_items)
            } else {
                MarkdownElement::UnorderedList(list_items)
            })));
            list_items = prev_items;
        }

        if ordered {
            MarkdownElement::OrderedList(list_items)
        } else {
            MarkdownElement::UnorderedList(list_items)
        }
    }
}
