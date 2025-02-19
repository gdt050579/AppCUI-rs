mod linkheaderregistry;
mod parser;

use linkheaderregistry::LinkHeaderRegistry;
use parser::{InlineElement, MarkdownElement, MarkdownParser};
use std::cell::RefCell;
use crate::system::Theme;
use self::components::ScrollBars;
use crate::prelude::*;
use crate::ui::markdown::initialization_flags::Flags;

use std::sync::Mutex;
static DEBUG_STRING: Mutex<String> = Mutex::new(String::new());

#[CustomControl(overwrite=OnPaint+OnResize+OnMouseEvent+OnKeyPressed, internal=true)]
pub struct Markdown {
    surface: Surface,
    content: String,
    x: i32,
    y: i32,
    background: Option<Character>,
    flags: Flags,
    drag_point: Option<Point>,
    scrollbars: ScrollBars,
    link_header_registry: RefCell<LinkHeaderRegistry>,
    elements: Vec<MarkdownElement>,
}

impl Markdown {
    // FOR DEBUG -- REMOVE
    pub fn reset_debug_message() {
        let mut debug_string = DEBUG_STRING.lock().unwrap();
        *debug_string = String::new();
    }

    pub fn append_debug_message(message: &str) {
        let mut debug_string = DEBUG_STRING.lock().unwrap();
        debug_string.push_str(message);
    }

    pub fn set_debug_message(message: &str) {
        let mut debug_string = DEBUG_STRING.lock().unwrap();
        *debug_string = message.to_string();
    }

    pub fn get_debug_message() -> String {
        let debug_string = DEBUG_STRING.lock().unwrap();
        debug_string.clone()
    }
    //

    // Creates a new markdown component with a specified content, layout, and flags.
    pub fn new(content: String, layout: Layout, flags: Flags) -> Self {
        Self::reset_debug_message();
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
            content: content.clone(),
            x: 0,
            y: 0,
            flags,
            background: None,
            drag_point: None,
            scrollbars: ScrollBars::new(flags == Flags::ScrollBars),
            link_header_registry: RefCell::new(LinkHeaderRegistry::new()),
            elements: MarkdownParser::parse(&content),
        }
    }

    fn move_scroll_to(&mut self, x: i32, y: i32) {
        let sz = self.size();
        let surface_size = self.surface.size();
        self.x = if surface_size.width <= sz.width {
            0
        } else {
            x.max((sz.width as i32) - (surface_size.width as i32))
        };
        self.y = if surface_size.height <= sz.height {
            0
        } else {
            y.max((sz.height as i32) - (surface_size.height as i32))
        };
        self.x = self.x.min(0);
        self.y = self.y.min(0);
        self.scrollbars.set_indexes((-self.x) as u64, (-self.y) as u64);
    }

    fn update_scrollbars(&mut self, surface: &mut Surface) {
        let surface_size = surface.size();
        self.scrollbars.update(surface_size.width as u64, surface_size.height as u64, self.size());
    }

    fn update_scroll_pos_from_scrollbars(&mut self) {
        let h = -(self.scrollbars.horizontal_index() as i32);
        let v = -(self.scrollbars.vertical_index() as i32);
        self.move_scroll_to(h, v);
    }

    fn compute_dimensions(content: &str) -> (usize, usize) {
        let lines: Vec<&str> = content.split('\n').collect();

        let width = lines.iter().map(|line| line.chars().count()).max().unwrap_or(0);

        let height = lines.len();

        (width, height)
    }

    fn get_element_style(element: &InlineElement, theme: &Theme) -> CharAttribute {
        match element {
            InlineElement::Text(_) => theme.markdown.text,
            InlineElement::Bold(_) => theme.markdown.bold,
            InlineElement::Italic(_) => theme.markdown.italic,
            InlineElement::Link(_, _) => theme.markdown.link,
            InlineElement::Code(_) => theme.markdown.code,
        }
    }

    fn register_if_link(link_header_registry: &mut LinkHeaderRegistry, element: &InlineElement, x: i32, y: i32) -> bool {
        if let InlineElement::Link(_, link) = element {
            let link_width = link.chars().count() as i32;
            let link_str = &link.replace('#', "");
            link_header_registry.register_link_position(link_str, x, y, link_width);
            return true;
        }
        false
    }

    fn get_header_style(level: usize) -> (Color, CharFlags) {
        match level {
            1 => (Color::Red, CharFlags::None),
            2 => (Color::Yellow, CharFlags::None),
            3 => (Color::Pink, CharFlags::None),
            _ => (Color::Magenta, CharFlags::None),
        }
    }

    fn process_list_element(
        elements: &[InlineElement],
        indent: i32,
        x_pos: &mut i32,
        y_pos: &mut i32,
        xlsurface: &mut Surface,
        prefix: Option<String>,
        link_header_registry: &mut LinkHeaderRegistry,
        theme: &Theme
    ) {
        for (i, element) in elements.iter().enumerate() {
            if i == 0 {
                *x_pos = indent;
            }

            let style = Self::get_element_style(element, theme);
            Self::register_if_link(link_header_registry, element, *x_pos, *y_pos);

            let content_str = element.to_string();
            let formatted_content = if i == 0 {
                if let Some(ref prefix) = prefix {
                    format!("{} {}", prefix, content_str)
                } else {
                    format!("○ {}", content_str)
                }
            } else {
                content_str
            };

            xlsurface.write_string(
                *x_pos,
                *y_pos,
                &formatted_content,
                style,
                false,
            );

            *x_pos += formatted_content.chars().count() as i32;
        }
    }

    fn process_nested_list(
        depth: u8,
        nested_items: &Box<MarkdownElement>,
        x_pos: &mut i32,
        y_pos: &mut i32,
        xlsurface: &mut Surface,
        link_header_registry: &mut LinkHeaderRegistry,
        theme: &Theme
    ) {
        let indent = *x_pos + (depth as i32) * 4;

        match **nested_items {
            MarkdownElement::UnorderedList(ref items) => {
                for item in items.iter() {
                    match item {
                        parser::ListItem::Simple(ref elements) => {
                            let mut x = *x_pos;
                            Self::process_list_element(elements, indent, &mut x, y_pos, xlsurface, None, link_header_registry, theme);
                            *y_pos += 1;
                        }
                        parser::ListItem::Nested(ref nested) => {
                            Self::process_nested_list(depth + 1, nested, x_pos, y_pos, xlsurface, link_header_registry, theme);
                        }
                    }
                }
            }
            MarkdownElement::OrderedList(ref items) => {
                let mut index = 1;
                for item in items.iter() {
                    match item {
                        parser::ListItem::Simple(ref elements) => {
                            let mut x = *x_pos;
                            Self::process_list_element(
                                elements,
                                indent,
                                &mut x,
                                y_pos,
                                xlsurface,
                                Some(format!("{}.", index)),
                                link_header_registry,
                                theme
                            );
                            index += 1;
                            *y_pos += 1;
                        }
                        parser::ListItem::Nested(ref nested) => {
                            Self::process_nested_list(depth + 1, nested, x_pos, y_pos, xlsurface, link_header_registry, theme);
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

impl OnPaint for Markdown {
    fn on_paint(&self, surface: &mut Surface, _theme: &Theme) {
        if (self.has_focus()) && (self.flags == Flags::ScrollBars) {
            self.scrollbars.paint(surface, _theme, self);
            surface.reduce_clip_by(0, 0, 1, 1);
        }
        if let Some(back) = self.background {
            surface.clear(back);
        }

        // Inititialize vertical offset.
        let mut y_pos = self.y;

        for element in &self.elements {
            match element {
                MarkdownElement::Header(content, level) => {
                    let header_style = match level {
                        1 => _theme.markdown.h1,
                        2 => _theme.markdown.h2,
                        _ => _theme.markdown.h3,
                    };
            
                    self.link_header_registry.borrow_mut().register_header_position(&content, y_pos);
                    surface.write_string(self.x, y_pos, &content, header_style, false);
                }

                MarkdownElement::Paragraph(content) => {
                    let mut x_pos: i32 = self.x;

                    for element in content.iter() {
                        let style = Self::get_element_style(element, _theme);
                        Self::register_if_link(&mut *self.link_header_registry.borrow_mut(), element, x_pos, y_pos);
                        let content_str = element.to_string();

                        surface.write_string(x_pos, y_pos, &content_str, style, false);
                        x_pos += content_str.chars().count() as i32;
                    }
                }
                MarkdownElement::UnorderedList(items) => {
                    for item in items.iter() {
                        let mut x_pos: i32 = self.x + 4;

                        let elements = match item {
                            parser::ListItem::Simple(elements) => elements,
                            parser::ListItem::Nested(items) => {
                                Self::process_nested_list(1, &items, &mut x_pos, &mut y_pos, surface, &mut *self.link_header_registry.borrow_mut(), _theme);
                                continue;
                            }
                        };

                        for (i, element) in elements.iter().enumerate() {
                            let style = Self::get_element_style(element, _theme);
                            Self::register_if_link(&mut *self.link_header_registry.borrow_mut(), element, x_pos, y_pos);
                            let content_str = element.to_string();

                            let formatted_content = if i == 0 {
                                let prefix = "•";
                                format!("{} {}", prefix, content_str).to_string()
                            } else {
                                content_str
                            };

                            surface.write_string(
                                x_pos,
                                y_pos,
                                &formatted_content,
                                style,
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
                        let mut x_pos: i32 = self.x + 4;

                        let elements = match item {
                            parser::ListItem::Simple(elements) => elements,
                            parser::ListItem::Nested(items) => {
                                Self::process_nested_list(1, &items, &mut x_pos, &mut y_pos, surface, &mut *self.link_header_registry.borrow_mut(), _theme);
                                continue;
                            }
                        };

                        for (i, element) in elements.iter().enumerate() {
                            let style = Self::get_element_style(element, _theme);
                            Self::register_if_link(&mut *self.link_header_registry.borrow_mut(), element, x_pos, y_pos);
                            let content_str = element.to_string();

                            let formatted_content = if i == 0 {
                                let prefix = index;
                                index += 1;
                                format!("{}. {}", prefix, content_str).to_string()
                            } else {
                                content_str
                            };

                            surface.write_string(
                                x_pos,
                                y_pos,
                                &formatted_content,
                                style,
                                false,
                            );

                            x_pos += formatted_content.chars().count() as i32;
                        }

                        y_pos += 1;
                    }
                }
                MarkdownElement::HorizontalRule => {
                    let width = 80;
                    let line = "-".repeat(width as usize);

                    surface.write_string(
                        self.x,
                        y_pos,
                        &line,
                        _theme.markdown.text,
                        false,
                    );
                    y_pos += 1;
                }
                MarkdownElement::CodeBlock(code) => {
                    let code_lines = code.lines();
                    let x_pos = self.x + 4;

                    for line in code_lines {
                        surface.write_string(x_pos, y_pos, line, _theme.markdown.code_block, false);
                        y_pos += 1;
                    }
                }
                MarkdownElement::Table(table) => {
                    let mut column_widths = Vec::new();
                    let lines_count = table.rows.iter().count() + 2; // add the header and the separation line

                    for (i, header) in table.headers.iter().enumerate() {
                        let header_len = header.iter().map(|e| e.to_string().chars().count()).sum::<usize>();
                        if column_widths.len() <= i {
                            column_widths.push(header_len);
                        } else {
                            column_widths[i] = column_widths[i].max(header_len);
                        }
                    }

                    for row in &table.rows {
                        for (i, cell) in row.iter().enumerate() {
                            let cell_len = cell.iter().map(|e| e.to_string().chars().count()).sum::<usize>();
                            if column_widths.len() <= i {
                                column_widths.push(cell_len);
                            } else {
                                column_widths[i] = column_widths[i].max(cell_len);
                            }
                        }
                    }

                    let table_width: usize = column_widths.iter().sum();
                    let suplimentar_padding: usize = column_widths.iter().count() * 3;

                    let mut x_pos = self.x;
                    let rect = Rect::new(x_pos, y_pos, x_pos + (table_width + suplimentar_padding) as i32, y_pos + 1 + lines_count as i32);
                    surface.draw_rect(rect, LineType::Ascii, _theme.markdown.table);

                    x_pos += 1;
                    y_pos += 1;
                    for (i, header) in table.headers.iter().enumerate() {
                        let header_str = header.iter().map(|e| e.to_string()).collect::<String>();
                        let padded_header = format!("{:width$}", header_str, width = column_widths[i] + 2);
                        surface.write_string(
                            x_pos,
                            y_pos,
                            &padded_header,
                            _theme.markdown.table_header,
                            false,
                        );
                        x_pos += column_widths[i] as i32 + 3;
                        
                        let c = Character::new('|', _theme.markdown.table.foreground, _theme.markdown.table.background, _theme.markdown.table.flags);
                        surface.fill_vertical_line(x_pos - 1, y_pos, y_pos - 1 + lines_count as i32, c);
                    }
                    y_pos += 1;
                    let c = Character::new('-', _theme.markdown.table.foreground, _theme.markdown.table.background, _theme.markdown.table.flags);
                        surface.fill_horizontal_line(self.x + 1, y_pos, self.x + (table_width + suplimentar_padding) as i32 - 1, c);
                    y_pos += 1;

                    for row in &table.rows {
                        x_pos = self.x + 1;
                        for (i, cell) in row.iter().enumerate() {
                            let cell_str = cell.iter().map(|e| e.to_string()).collect::<String>();
                            let padded_cell = format!("{:width$}", cell_str, width = column_widths[i] + 2);
                            surface.write_string(
                                x_pos,
                                y_pos,
                                &padded_cell,
                                _theme.markdown.table,
                                false,
                            ); // shall modify if I want bold in table
                            x_pos += column_widths[i] as i32 + 3;
                        }
                        y_pos += 1;
                    }
                }
            }
            y_pos += 1;
        }
    }
}

impl OnResize for Markdown {
    fn on_resize(&mut self, _old_size: Size, _new_size: Size) {
        let paint_sz = self.surface.size();
        self.scrollbars.resize(paint_sz.width as u64, paint_sz.height as u64, &self.base);
        self.move_scroll_to(self.x, self.y);
    }
}

impl OnKeyPressed for Markdown {
    fn on_key_pressed(&mut self, key: Key, _character: char) -> EventProcessStatus {
        match key.value() {
            key!("Left") => {
                self.move_scroll_to(self.x + 1, self.y);
                EventProcessStatus::Processed
            }
            key!("Right") => {
                self.move_scroll_to(self.x - 1, self.y);
                EventProcessStatus::Processed
            }
            key!("Up") => {
                self.move_scroll_to(self.x, self.y + 1);
                EventProcessStatus::Processed
            }
            key!("Down") => {
                self.move_scroll_to(self.x, self.y - 1);
                EventProcessStatus::Processed
            }
            _ => EventProcessStatus::Ignored,
        }
    }
}

impl OnMouseEvent for Markdown {
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        if self.scrollbars.process_mouse_event(event) {
            self.update_scroll_pos_from_scrollbars();
            return EventProcessStatus::Processed;
        }
        match event {
            MouseEvent::Pressed(data) => {
                let mut y_header: Option<i32> = None;

                if let Some(link_header_id) = self.link_header_registry.borrow().check_for_link_at_position(data.x, data.y) {
                    if let Some(header_position) = self.link_header_registry.borrow().get_header_position(&link_header_id) {
                        y_header = Some(header_position);
                    }
                }

                if let Some(header_position) = y_header {
                    self.move_scroll_to(0, self.y - header_position);
                } else {
                    self.drag_point = Some(Point::new(data.x, data.y));
                }
                EventProcessStatus::Processed
            }
            MouseEvent::Released(data) => {
                if let Some(p) = self.drag_point {
                    self.move_scroll_to(self.x + data.x - p.x, self.y + data.y - p.y);
                }
                self.drag_point = None;
                EventProcessStatus::Processed
            }
            MouseEvent::Drag(data) => {
                if let Some(p) = self.drag_point {
                    self.move_scroll_to(self.x + data.x - p.x, self.y + data.y - p.y);
                }
                self.drag_point = Some(Point::new(data.x, data.y));
                EventProcessStatus::Processed
            }
            MouseEvent::Wheel(dir) => {
                match dir {
                    MouseWheelDirection::Up => self.move_scroll_to(self.x, self.y + 1),
                    MouseWheelDirection::Down => self.move_scroll_to(self.x, self.y - 1),
                    _ => {}
                };
                EventProcessStatus::Processed
            }
            _ => EventProcessStatus::Ignored,
        }
    }
}
