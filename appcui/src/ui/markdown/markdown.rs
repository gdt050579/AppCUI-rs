mod parser;
use parser::{InlineElement, MarkdownElement, MarkdownParser};

use self::components::ScrollBars;
use crate::prelude::*;
use crate::ui::markdown::initialization_flags::Flags;

#[CustomControl(overwrite=OnPaint, internal=true)]
pub struct Markdown {
    surface: Surface,
    content: String,
    x: i32,
    y: i32,
    background: Option<Character>,
    flags: Flags,
    drag_point: Option<Point>,
    scrollbars: ScrollBars,
}

impl Markdown {
    // Creates a new markdown component with a specified content, layout, and flags.
    pub fn new(content: String, layout: Layout, flags: Flags) -> Self {
        let (width, height) = (100, 150); //Markdown::compute_dimensions(&content);

        Self {
            base: ControlBase::with_status_flags(
                layout,
                (StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput)
                    | if flags == Flags::ScrollBars {
                        StatusFlags::IncreaseBottomMarginOnFocus | StatusFlags::IncreaseRightMarginOnFocus
                    } else {
                        StatusFlags::None
                    },
            ),
            surface: Surface::new(width as u32, height as u32),
            content,
            x: 0,
            y: 0,
            flags,
            background: None,
            drag_point: None,
            scrollbars: ScrollBars::new(flags == Flags::ScrollBars),
        }
    }

    fn compute_dimensions(content: &str) -> (usize, usize) {
        let lines: Vec<&str> = content.split('\n').collect();

        let width = lines.iter().map(|line| line.chars().count()).max().unwrap_or(0);

        let height = lines.len();

        (width, height)
    }
}

impl OnPaint for Markdown {
    fn on_paint(&self, xlsurface: &mut Surface, _theme: &Theme) {
        // Parse the content using the md parser.
        let elements = MarkdownParser::parse(&self.content);

        let mut y_pos = 0;

        for element in elements {
            match element {
                MarkdownElement::Header(content, level) => {
                    let color = match level {
                        1 => Color::Red,
                        2 => Color::Yellow,
                        3 => Color::Pink,
                        _ => Color::Magenta,
                    };
                    xlsurface.write_string(0, y_pos, &content, CharAttribute::new(color, Color::White, CharFlags::Bold), false);
                }

                MarkdownElement::Paragraph(content) => {
                    let mut x_pos: i32 = 0;

                    for element in content.iter() {
                        let (color, char_flags) = match element {
                            InlineElement::Text(_) => (Color::Black, CharFlags::None),
                            InlineElement::Bold(_) => (Color::Red, CharFlags::Bold),
                            InlineElement::Italic(_) => (Color::Blue, CharFlags::Italic),
                            InlineElement::Link(_, _) => (Color::Pink, CharFlags::Underline),
                            InlineElement::Code(_) => (Color::Magenta, CharFlags::None),
                        };

                        let content_str = element.to_string();

                        xlsurface.write_string(x_pos, y_pos, &content_str, CharAttribute::new(color, Color::White, char_flags), false);
                        x_pos += content_str.chars().count() as i32;
                    }
                }
                MarkdownElement::UnorderedList(items) => {
                    for item in items.iter() {
                        let mut x_pos: i32 = 0;

                        let elements = match item {
                            parser::ListItem::Simple(elements) => elements,
                            parser::ListItem::Nested(items) => {
                                process_nested_list(1, &items, &mut x_pos, &mut y_pos, xlsurface);
                                continue;
                            }
                        };

                        for (i, element) in elements.iter().enumerate() {
                            let (color, char_flags) = match element {
                                InlineElement::Text(_) => (Color::Green, CharFlags::None),
                                InlineElement::Bold(_) => (Color::Red, CharFlags::Bold),
                                InlineElement::Italic(_) => (Color::Blue, CharFlags::Italic),
                                InlineElement::Link(_, _) => (Color::Pink, CharFlags::Underline),
                                InlineElement::Code(_) => (Color::Magenta, CharFlags::None),
                            };

                            let content_str = element.to_string();

                            let formatted_content = if i == 0 {
                                let prefix = "•";
                                format!("{} {}", prefix, content_str).to_string()
                            } else {
                                content_str
                            };

                            xlsurface.write_string(
                                x_pos,
                                y_pos,
                                &formatted_content,
                                CharAttribute::new(color, Color::White, char_flags),
                                false,
                            );

                            x_pos += formatted_content.chars().count() as i32;
                        }

                        y_pos += 1;
                    }
                }
                MarkdownElement::OrderedList(items) => {
                    let mut index = 1;
                    for item in items.iter() {
                        let mut x_pos: i32 = 0;

                        let elements = match item {
                            parser::ListItem::Simple(elements) => elements,
                            parser::ListItem::Nested(items) => {
                                process_nested_list(1, &items, &mut x_pos, &mut y_pos, xlsurface);
                                continue;
                            }
                        };

                        for (i, element) in elements.iter().enumerate() {
                            let (color, char_flags) = match element {
                                InlineElement::Text(_) => (Color::Green, CharFlags::None),
                                InlineElement::Bold(_) => (Color::Red, CharFlags::Bold),
                                InlineElement::Italic(_) => (Color::Blue, CharFlags::Italic),
                                InlineElement::Link(_, _) => (Color::Pink, CharFlags::Underline),
                                InlineElement::Code(_) => (Color::Magenta, CharFlags::None),
                            };

                            let content_str = element.to_string();

                            let formatted_content = if i == 0 {
                                let prefix = index;
                                index += 1;
                                format!("{}. {}", prefix, content_str).to_string()
                            } else {
                                content_str
                            };

                            xlsurface.write_string(
                                x_pos,
                                y_pos,
                                &formatted_content,
                                CharAttribute::new(color, Color::White, char_flags),
                                false,
                            );

                            x_pos += formatted_content.chars().count() as i32;
                        }

                        y_pos += 1;
                    }
                },
                MarkdownElement::HorizontalRule => {
                    let width = 80;
                    let line = "-".repeat(width as usize);

                    xlsurface.write_string(0, y_pos, &line, CharAttribute::new(Color::Gray, Color::White, CharFlags::None), false);
                    y_pos += 1;
                },
                MarkdownElement::CodeBlock(code) => {
                    let code_lines = code.lines();
                    let code_color = Color::Gray;
                    let background_color = Color::White;
                    let flags = CharFlags::None;
                
                    for line in code_lines {
                       
                        xlsurface.write_string(0, y_pos, line, CharAttribute::new(code_color, background_color, flags), false);
                        y_pos += 1; 
                    }
                }
            }
            y_pos += 1;
        }
    }
}

fn process_nested_list(depth: u8, nested_items: &Box<MarkdownElement>, x_pos: &mut i32, y_pos: &mut i32, xlsurface: &mut Surface) {
    match **nested_items {
        MarkdownElement::UnorderedList(ref items) => {
            let indent = depth as i32 * 4;

            for item in items.iter() {
                match item {
                    parser::ListItem::Simple(ref elements) => {
                        // Process simple list items
                        for (i, element) in elements.iter().enumerate() {
                            if i == 0 {
                                *x_pos = indent;
                            }
                            let (color, char_flags) = match element {
                                InlineElement::Text(_) => (Color::Green, CharFlags::None),
                                InlineElement::Bold(_) => (Color::Red, CharFlags::Bold),
                                InlineElement::Italic(_) => (Color::Blue, CharFlags::Italic),
                                InlineElement::Link(_, _) => (Color::Pink, CharFlags::Underline),
                                InlineElement::Code(_) => (Color::Magenta, CharFlags::None),
                            };

                            let content_str = element.to_string();

                            let formatted_content = if i == 0 { format!("○ {}", content_str) } else { content_str };

                            xlsurface.write_string(
                                *x_pos,
                                *y_pos,
                                &formatted_content,
                                CharAttribute::new(color, Color::White, char_flags),
                                false,
                            );

                            *x_pos += formatted_content.chars().count() as i32;
                        }
                    }
                    parser::ListItem::Nested(ref nested) => {
                        process_nested_list(depth + 1, nested, x_pos, y_pos, xlsurface);
                    }
                }
                *y_pos += 1;
            }
        },
        MarkdownElement::OrderedList(ref items) => {
            let indent = depth as i32 * 4;
            let mut index = 1;

            for item in items.iter() {
                match item {
                    parser::ListItem::Simple(ref elements) => {
                        // Process simple list items
                        for (i, element) in elements.iter().enumerate() {
                            if i == 0 {
                                *x_pos = indent;
                            }
                            let (color, char_flags) = match element {
                                InlineElement::Text(_) => (Color::Green, CharFlags::None),
                                InlineElement::Bold(_) => (Color::Red, CharFlags::Bold),
                                InlineElement::Italic(_) => (Color::Blue, CharFlags::Italic),
                                InlineElement::Link(_, _) => (Color::Pink, CharFlags::Underline),
                                InlineElement::Code(_) => (Color::Magenta, CharFlags::None),
                            };

                            let content_str = element.to_string();

                            let prefix = index;
                            let formatted_content = if i == 0 { format!("{}. {}",prefix , content_str) } else { content_str };

                            xlsurface.write_string(
                                *x_pos,
                                *y_pos,
                                &formatted_content,
                                CharAttribute::new(color, Color::White, char_flags),
                                false,
                            );
                            index += 1;
                            *x_pos += formatted_content.chars().count() as i32;
                        }
                    }
                    parser::ListItem::Nested(ref nested) => {
                        process_nested_list(depth + 1, nested, x_pos, y_pos, xlsurface);
                    }
                }
                *y_pos += 1;
            }
        }
        _ => {
        }
    }
}
