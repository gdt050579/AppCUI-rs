use std::fmt;

/// Enum representing different types of Markdown elements.
#[derive(Debug)]
pub enum MarkdownElement {
    Header(String, usize),
    Paragraph(Vec<InlineElement>),
    UnorderedList(Vec<ListItem>),
    OrderedList(Vec<ListItem>),
    HorizontalRule,
    CodeBlock(String),
    Table(Table)
}

// Enum representing Markdown  table with rows and cells.
#[derive(Debug)]
pub struct Table {
    pub headers: Vec<Vec<InlineElement>>,
    pub rows: Vec<Vec<Vec<InlineElement>>> // rows[row][cell][element]
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
    Code(String),
}

impl fmt::Display for InlineElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InlineElement::Link(text, _) => write!(f, "{}", text),
            InlineElement::Text(content) |
            InlineElement::Bold(content) |
            InlineElement::Italic(content) |
            InlineElement::Code(content) => write!(f, "{}", content),
        }
    }
}

pub struct MarkdownParser;

impl MarkdownParser {
    /// Parses a string input into a vector of MarkdownElements.
    /// It identifies headers, lists, paragraphs, and code blocks in the input.
    pub fn parse(input: &str) -> Vec<MarkdownElement> {
        let mut elements = Vec::new();
        let mut lines = input.lines();

        let mut in_code_block = false;
        let mut code_block_content = String::new();

        while let Some(line) = lines.next() {
            let trimmed = line.trim();

            // Check for the start or end of a code block
            if trimmed == "```" {
                if in_code_block {
                    // Close code block
                    elements.push(MarkdownElement::CodeBlock(code_block_content.clone()));
                    code_block_content.clear();
                }
                in_code_block = !in_code_block;
            } else if in_code_block {
                code_block_content.push_str(line);
                code_block_content.push('\n');
            } else if Self::is_table_header(trimmed) {
                elements.push(Self::parse_table(&mut lines, trimmed));
            } else if trimmed == "---" {
                elements.push(MarkdownElement::HorizontalRule);
            } else if trimmed.starts_with('#') {
                elements.push(Self::parse_header(trimmed));
            } else if trimmed.starts_with('-') {
                elements.push(Self::parse_list(&mut lines, trimmed, false));
            } else if trimmed.starts_with(|c: char| c.is_ascii_digit()) && trimmed[1..].starts_with('.')
            {
                elements.push(Self::parse_list(&mut lines, trimmed, true));
            } else if !trimmed.is_empty() {
                elements.push(Self::parse_paragraph(trimmed));
            }
        }

        elements
    }

    /// Check for table header
    fn is_table_header(line: &str) -> bool {
        line.starts_with('|') && line.ends_with('|')
    }

    fn parse_inline_cells(line: &str) -> Vec<Vec<InlineElement>> {
        line.trim()
            .trim_matches('|')
            .split('|')
            .map(|cell| {
                let content = cell.trim();
                Self::parse_inline(content)
            })
            .collect()
    }

    // Parse a table from lines.
    fn parse_table<'a>(
        lines: &mut impl Iterator<Item = &'a str>,
        header_line: &str,
    ) -> MarkdownElement {
        let headers = Self::parse_inline_cells(header_line);
        lines.next();
    
        let mut rows = Vec::new();

        for line in lines.by_ref() {
            let trimmed = line.trim();
            if trimmed.starts_with('|') && trimmed.ends_with('|') {
                rows.push(Self::parse_inline_cells(trimmed));
            } else {
                break; 
            }
        }
        
        MarkdownElement::Table(Table { headers, rows })
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
            } else if input[i..].starts_with('`') {
                let delimiter = '`';
                if let Some(end) = input[i + 1..].find(delimiter).map(|p| p + i + 1) {
                    elements.push(InlineElement::Code(input[i + 1..end].to_string()));
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
                let next_special = input[i..]
                    .find(|c| ['*', '_', '[', '`'].contains(&c))
                    .unwrap_or(input.len() - i);
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

        fn indentation_level(line: &str) -> usize {
            line.chars().take_while(|c| c.is_whitespace()).count()
        }

        let mut sublist_stack: Vec<(usize, Vec<ListItem>)> = Vec::new();

        let mut current_level = indentation_level(first_line);
        let content = if ordered {
            let no_digit_line = first_line.trim_start_matches(|c: char| c.is_ascii_digit());
            no_digit_line.trim_start_matches('.').trim()
        } else {
            first_line.trim_start_matches('-').trim()
        };
        list_items.push(ListItem::Simple(Self::parse_inline(content)));

        for next_line in lines.by_ref() {
            let trimmed = next_line.trim();
            let next_level = indentation_level(next_line);

            let is_ordered =
                trimmed.starts_with(|c: char| c.is_ascii_digit()) && trimmed[1..].starts_with('.');
            let is_unordered = trimmed.starts_with('-');

            if (ordered && is_ordered) || (!ordered && is_unordered) {
                let content = if ordered {
                    let no_digit_line = trimmed.trim_start_matches(|c: char| c.is_ascii_digit());
                    no_digit_line.trim_start_matches('.').trim()
                } else {
                    trimmed.trim_start_matches('-').trim()
                };

                let item = ListItem::Simple(Self::parse_inline(content));

                match () {
                    _ if next_level > current_level => {
                        // new sublist
                        sublist_stack.push((current_level, list_items));
                        list_items = vec![item];
                        current_level = next_level;
                    }
                    _ if next_level < current_level => {
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
                    }
                    _ => {
                        // Same level, add the item
                        list_items.push(item);
                    }
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

