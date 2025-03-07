pub(crate) mod linkregistry;
pub(crate) mod parser;
use self::components::ScrollBars;
use crate::prelude::*;
use crate::system::Theme;
use crate::ui::markdown::initialization_flags::Flags;
use linkregistry::LinkRegistry;
use parser::{InlineElement, MarkdownElement, MarkdownParser, Table};
use std::cell::RefCell;

use super::events::EventData;

#[CustomControl(overwrite=OnPaint+OnResize+OnMouseEvent+OnKeyPressed, internal=true)]
pub struct Markdown {
    surface: Surface,
    x: i32,
    y: i32,
    background: Option<Character>,
    flags: Flags,
    drag_point: Option<Point>,
    scrollbars: ScrollBars,
    link_registry: RefCell<LinkRegistry>,
    elements: Vec<MarkdownElement>,
}

impl Markdown {
    /// Creates a new markdown component with a specified content, layout, and flags.
    /// This method initializes a markdown rendering control with the provided parameters.
    ///
    /// # Parameters
    /// - `content`: The markdown-formatted string to be displayed.
    /// - `layout`: The layout configuration for positioning and sizing.
    /// - `flags`: Initialization flags that define specific behaviors (e.g., enabling scrollbars).
    /// 
    /// # Example
    /// ```rust,no_run
    /// use appcui::prelude::*;
    /// let m = Markdown::new("< a markdown text >", Layout::new("d: c"), markdown::Flags::ScrollBars);
    /// ```
    pub fn new(content: &str, layout: Layout, flags: Flags) -> Self {
        let (width, height) = Self::compute_dimension(content);
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
            x: 0,
            y: 0,
            flags,
            background: None,
            drag_point: None,
            scrollbars: ScrollBars::new(flags == Flags::ScrollBars),
            link_registry: RefCell::new(LinkRegistry::new()),
            elements: MarkdownParser::parse(content) ,
        }
    }

    /// Sets new content in the markdown component.
    /// This method resets the scroll position, reparses the content, 
    /// and adjusts the surface and scrollbars accordingly.
    ///
    /// # Parameters
    /// - `content`: The new markdown content to be set.
    pub fn set_content(&mut self, content: &str) {
        self.x = 0;
        self.y = 0;
        self.elements = MarkdownParser::parse(content);
        self.link_registry.replace(LinkRegistry::new());
  
        let (width, height) = Self::compute_dimension(content);
        self.surface.resize(Size::new(width as u32, height as u32));
        
        let paint_sz = self.surface.size();
        self.scrollbars.resize(paint_sz.width as u64, paint_sz.height as u64, &self.base);
        self.move_scroll_to(self.x, self.y);
    }

    fn compute_dimension(content: &str) -> (usize, usize) {
        let lines: Vec<&str> = content.lines().collect();
        let height = lines.len();
        let width = lines.iter().map(|line| line.len()).max().unwrap_or(0); 
    
        (width, height)
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

    fn update_scroll_pos_from_scrollbars(&mut self) {
        let h = -(self.scrollbars.horizontal_index() as i32);
        let v = -(self.scrollbars.vertical_index() as i32);
        self.move_scroll_to(h, v);
    }

    fn get_element_style(element: &InlineElement, theme: &Theme, hovered: bool) -> CharAttribute {
        match element {
            InlineElement::Text(_) => theme.markdown.text,
            InlineElement::Bold(_) => theme.markdown.bold,
            InlineElement::Italic(_) => theme.markdown.italic,
            InlineElement::Link(_, _) => if !hovered { theme.markdown.link } else { theme.text.highlighted },
            InlineElement::Code(_) => theme.markdown.code,
        }
    }

    fn register_if_link(
        link_registry: &mut LinkRegistry,
        element: &InlineElement,
        x: i32,
        y: i32,
    ) -> Option<String> {
        if let InlineElement::Link(display_str, link) = element {
            let link_str = if link.starts_with("#") {
                link.trim_start_matches("#").to_string()
            } else {
                link.to_string()
            };
    
            let link_width = display_str.chars().count() as i32;
            link_registry.register_link_position(&link_str, x, y, link_width, !link.starts_with("#"));
    
            return Some(link_str);
        }
        None
    }

    fn process_list_element(
        elements: &[InlineElement],
        indent: i32,
        x_pos: &mut i32,
        y_pos: &mut i32,
        xlsurface: &mut Surface,
        prefix: Option<String>,
        link_registry: &mut LinkRegistry,
        theme: &Theme,
        inactive: bool
    ) {
        for (i, element) in elements.iter().enumerate() {
            if i == 0 {
                *x_pos = indent;
            }

            let link_identifier = Self::register_if_link(link_registry, element, *x_pos, *y_pos);
            let is_hovered = if let Some(ref id) = link_identifier {
                link_registry.is_hovered(id)
            } else {
                false
            };
            let style = Self::get_element_style(element, theme, is_hovered);
            let attr = if !inactive { style } else { theme.text.inactive };

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

            xlsurface.write_string(*x_pos, *y_pos, &Self::replace_tabs(&formatted_content), attr, false);

            *x_pos += formatted_content.chars().count() as i32;
        }
    }

    fn process_nested_list(
        depth: u8,
        nested_items: &MarkdownElement,
        x_pos: &mut i32,
        y_pos: &mut i32,
        xlsurface: &mut Surface,
        link_registry: &mut LinkRegistry,
        theme: &Theme,
        inactive: bool
    ) {
        let indent = *x_pos + (depth as i32) * 4;

        match *nested_items {
            MarkdownElement::UnorderedList(ref items) => {
                for item in items.iter() {
                    match item {
                        parser::ListItem::Simple(ref elements) => {
                            let mut x = *x_pos;
                            Self::process_list_element(elements, indent, &mut x, y_pos, xlsurface, None, link_registry, theme, inactive);
                            *y_pos += 1;
                        }
                        parser::ListItem::Nested(ref nested) => {
                            Self::process_nested_list(depth + 1, nested, x_pos, y_pos, xlsurface, link_registry, theme, inactive);
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
                                link_registry,
                                theme,
                                inactive
                            );
                            index += 1;
                            *y_pos += 1;
                        }
                        parser::ListItem::Nested(ref nested) => {
                            Self::process_nested_list(depth + 1, nested, x_pos, y_pos, xlsurface, link_registry, theme, inactive);
                        }
                    }
                }
            }
            _ => {}
        }
    }

    fn replace_tabs(string_to_print: &str) -> String {
        string_to_print.replace("\t", "    ")
    }

    fn paint_codeblock(&self, code: &str, y_pos: &mut i32, surface: &mut Surface, theme: &Theme, left_padding: Option<i32>) {
        let left_padding = left_padding.unwrap_or(4);
        let content = Self::replace_tabs(code);
        let code_lines: Vec<&str> = content.lines().collect();
        let max_width = code_lines.iter().map(|line| line.len()).max().unwrap_or(0);

        let attr = if self.is_enabled() { theme.markdown.code_block } else { theme.text.inactive };
        
        for line in code_lines {
            let formatted_line = format!(" {:width$} ", line, width = max_width);
            surface.write_string(self.x + left_padding - 1, *y_pos, &Self::replace_tabs(&formatted_line), attr, false);
            *y_pos += 1;
        }
    }

    fn paint_header(&self, content: &str, y_pos: i32, level: &usize, surface: &mut Surface, theme: &Theme) {
        let content = Self::replace_tabs(content);
        let header_style = match level {
            1 => theme.markdown.h1,
            2 => theme.markdown.h2,
            _ => theme.markdown.h3,
        };

        let attr = if self.is_enabled() { header_style } else { theme.text.inactive };

        self.link_registry.borrow_mut().register_header_position(&content, y_pos);
        surface.write_string(self.x, y_pos, &Self::replace_tabs(&content), attr, false);
    }

    fn paint_table(&self, table: &Table, y_pos: &mut i32, surface: &mut Surface, theme: &Theme) {

        let (attr, attr_header) = if self.is_enabled() {
            (theme.markdown.table, theme.markdown.table_header)
        } else {
            (theme.text.inactive, theme.text.inactive)
        };

        fn compute_column_widths(table: &Table) -> Vec<usize> {
            let mut column_widths = Vec::new();

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

            column_widths
        }

        let lines_count = table.rows.len() + 2; // the header and the separator
        let column_widths = compute_column_widths(table);

        let table_width: usize = column_widths.iter().sum();
        let suplimentar_padding: usize = column_widths.len() * 3;

        // draw contour
        let mut x_pos = self.x;
        let rect = Rect::new(
            x_pos,
            *y_pos,
            x_pos + (table_width + suplimentar_padding) as i32,
            *y_pos + 1 + lines_count as i32,
        );
        surface.draw_rect(rect, LineType::Single, attr);

        // draw horizontal line
        x_pos += 1;
        *y_pos += 2;
        surface.draw_horizontal_line(
            self.x + 1,
            *y_pos,
            self.x + (table_width + suplimentar_padding) as i32 - 1,
            LineType::Single,
            attr,
        );
        *y_pos -= 1;

        // write headers
        for (i, header) in table.headers.iter().enumerate() {
            let header_str = header.iter().map(|e| e.to_string()).collect::<String>();
            let padded_header = format!("{:width$}", header_str, width = column_widths[i] + 2);
            let content = Self::replace_tabs(&padded_header);
            surface.write_string(x_pos, *y_pos, &content, attr_header, false);
            x_pos += column_widths[i] as i32 + 3;

            // draw vertical line
            surface.draw_vertical_line(x_pos - 1, *y_pos, *y_pos - 1 + lines_count as i32, LineType::Single, attr);
        }
        *y_pos += 2;

        for (row_index, row) in table.rows.iter().enumerate() {
            x_pos = self.x + 1;
            for (i, cell) in row.iter().enumerate() {
                let cell_str = cell.iter().map(|e| e.to_string()).collect::<String>();
                let padded_cell = format!("{:width$}", cell_str, width = column_widths[i] + 2);
                let content = Self::replace_tabs(&padded_cell);
                surface.write_string(x_pos, *y_pos, &content, attr, false);
                x_pos += column_widths[i] as i32 + 3;
                if row_index == 0 && i < (row.len() - 1) {
                    // cross separators
                    surface.write_char(
                        x_pos - 1,
                        *y_pos - 1,
                        Character::with_attributes(SpecialChar::BoxCrossSingleLine, attr),
                    );

                    // horizontal separators
                    surface.write_char(
                        rect.left(),
                        *y_pos - 1,
                        Character::with_attributes(SpecialChar::BoxMidleLeft, attr),
                    );
                    surface.write_char(
                        rect.right(),
                        *y_pos - 1,
                        Character::with_attributes(SpecialChar::BoxMidleRight, attr),
                    );

                    // vertical separators
                    surface.write_char(
                        x_pos - 1,
                        rect.top(),
                        Character::with_attributes(SpecialChar::BoxMidleTop, attr),
                    );
                    surface.write_char(
                        x_pos - 1,
                        rect.bottom(),
                        Character::with_attributes(SpecialChar::BoxMidleBottom, attr),
                    );
                }
            }
            *y_pos += 1;
        }
    }

    fn paint_paragraph(&self, content: &[InlineElement], y_pos: i32, surface: &mut Surface, theme: &Theme) {
        let mut x_pos: i32 = self.x;
        
        for element in content.iter() {
            let link_identifier = {
                let mut registry = self.link_registry.borrow_mut();
                Self::register_if_link(&mut registry, element, x_pos, y_pos)
            };
            
            let is_hovered = if let Some(ref id) = link_identifier {
                self.link_registry.borrow().is_hovered(id)
            } else {
                false
            };
            
            let style = Self::get_element_style(element, theme, is_hovered);
            let attr = if self.is_enabled() { style } else { theme.text.inactive };
            let content_str = element.to_string();
            
            surface.write_string(x_pos, y_pos, &Self::replace_tabs(&content_str), attr, false);
            x_pos += content_str.chars().count() as i32;
        }
    }
    
    fn paint_unordered_list(
        &self,
        items: &[parser::ListItem],
        mut y_pos: i32,
        surface: &mut Surface,
        theme: &Theme,
    ) -> i32 {
        for item in items.iter() {
            let mut x_pos: i32 = self.x + 4;

            let elements = match item {
                parser::ListItem::Simple(elements) => elements,
                parser::ListItem::Nested(items) => {
                    Self::process_nested_list(
                        1,
                        items,
                        &mut x_pos,
                        &mut y_pos,
                        surface,
                        &mut self.link_registry.borrow_mut(),
                        theme,
                        !self.is_enabled()
                    );
                    continue;
                }
            };

            for (i, element) in elements.iter().enumerate() {
                let link_identifier = {
                    let mut registry = self.link_registry.borrow_mut();
                    Self::register_if_link(&mut registry, element, x_pos, y_pos)
                };
                let is_hovered = if let Some(ref id) = link_identifier {
                    self.link_registry.borrow().is_hovered(id)
                } else {
                    false
                };
                let style = Self::get_element_style(element, theme, is_hovered);
                let attr = if self.is_enabled() { style } else { theme.text.inactive };
                let content_str = element.to_string();

                let formatted_content = if i == 0 {
                    let prefix = "•";
                    format!("{} {}", prefix, content_str).to_string()
                } else {
                    content_str
                };

                surface.write_string(x_pos, y_pos, &Self::replace_tabs(&formatted_content), attr, false);

                x_pos += formatted_content.chars().count() as i32;
            }

            y_pos += 1;
        }
        y_pos
    }

    fn send_link(&self, link: &str) {
        self.raise_event(ControlEvent {
            emitter: self.handle,
            receiver: self.event_processor,
            data: ControlEventData::Markdown(EventData { event_type: markdown::events::Data::LinkClickEvent(link.to_string()) })
        });
    }

    fn send_back_navigation_command(&self) {
        self.raise_event(ControlEvent {
            emitter: self.handle,
            receiver: self.event_processor,
            data: ControlEventData::Markdown(EventData { event_type: markdown::events::Data::BackEvent }),
        });
    }

    fn paint_ordered_list(
        &self,
        items: &[parser::ListItem],
        mut y_pos: i32,
        surface: &mut Surface,
        theme: &Theme,
    ) -> i32 {
        let mut index = 1;
                    for item in items.iter() {
                        let mut x_pos: i32 = self.x + 4;

                        let elements = match item {
                            parser::ListItem::Simple(elements) => elements,
                            parser::ListItem::Nested(items) => {
                                Self::process_nested_list(
                                    1,
                                    items,
                                    &mut x_pos,
                                    &mut y_pos,
                                    surface,
                                    &mut self.link_registry.borrow_mut(),
                                    theme,
                                    !self.is_enabled()
                                );
                                continue;
                            }
                        };

                        for (i, element) in elements.iter().enumerate() {
                            let link_identifier = {
                                let mut registry = self.link_registry.borrow_mut();
                                Self::register_if_link(&mut registry, element, x_pos, y_pos)
                            };
                            let is_hovered = if let Some(ref id) = link_identifier {
                                self.link_registry.borrow().is_hovered(id)
                            } else {
                                false
                            };
                            let style = Self::get_element_style(element, theme, is_hovered);
                            let attr = if self.is_enabled() { style } else { theme.text.inactive };
                            let content_str = element.to_string();

                            let formatted_content = if i == 0 {
                                let prefix = index;
                                index += 1;
                                format!("{}. {}", prefix, content_str).to_string()
                            } else {
                                content_str
                            };

                            surface.write_string(x_pos, y_pos, &Self::replace_tabs(&formatted_content), attr, false);

                            x_pos += formatted_content.chars().count() as i32;
                        }

                        y_pos += 1;
                    }
                    y_pos
                }
}

impl OnPaint for Markdown {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        if (self.has_focus()) && (self.flags == Flags::ScrollBars) {
            self.scrollbars.paint(surface, theme, self);
            surface.reduce_clip_by(0, 0, 1, 1);
        }
        if let Some(back) = self.background {
            surface.clear(back);
        }

        // Inititialize vertical offset.
        let mut y_pos = self.y;

        for element in &self.elements {
            match element {
                MarkdownElement::Header(content, level) => self.paint_header(content, y_pos, level, surface, theme),
                MarkdownElement::Paragraph(content) => self.paint_paragraph(content, y_pos, surface, theme),
                MarkdownElement::UnorderedList(items) => { y_pos = self.paint_unordered_list(items, y_pos, surface, theme) },
                MarkdownElement::OrderedList(items) => { y_pos = self.paint_ordered_list(items, y_pos, surface, theme) },
                MarkdownElement::HorizontalRule => {
                    surface.draw_horizontal_line(self.x, y_pos, surface.size().width as i32, LineType::Single, theme.markdown.text);
                    y_pos += 1;
                }
                MarkdownElement::CodeBlock(code) => self.paint_codeblock(code, &mut y_pos, surface, theme, None),
                MarkdownElement::Table(table) => self.paint_table(table, &mut y_pos, surface, theme),
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
            key!("Backspace") => {
                self.send_back_navigation_command();
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
            MouseEvent::Over(data) => {
                let mut tmp = None;
                if let Some(link_header_id) = self.link_registry.borrow().check_for_link_at_position(data.x, data.y) {
                    tmp = Some(link_header_id);
                }

                if let Some(link_header_id) = tmp {
                    self.link_registry.borrow_mut().set_link_hovered(&link_header_id);
                } else {
                    self.link_registry.borrow_mut().clear_hovered();
                }
                EventProcessStatus::Processed
            }
            MouseEvent::Pressed(data) => {
                self.drag_point = Some(Point::new(data.x, data.y));
                EventProcessStatus::Processed
            }
            MouseEvent::Released(data) => {
                let mut y_header: Option<i32> = None;

    
                if let Some(link_id) = self.link_registry.borrow().check_for_link_at_position(data.x, data.y) {
                    if let Some(is_external) = self.link_registry.borrow().is_link_external(&link_id) {
                        if is_external {
                            self.send_link(&link_id);
                            return EventProcessStatus::Processed;
                        } else if let Some(header_position) = self.link_registry.borrow().get_header_position(&link_id) {
                            y_header = Some(header_position);
                        }
                    }
                }

                if let Some(header_position) = y_header {
                    self.move_scroll_to(0, self.y - header_position);
                    return EventProcessStatus::Processed;
                }

                if let Some(p) = self.drag_point {
                    self.move_scroll_to(self.x + data.x - p.x, self.y + data.y - p.y);
                    self.drag_point = None;
                    return EventProcessStatus::Processed;
                }
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
