use chartbar::Value;

use crate::ui::chartbar::initialization_flags::{Flags, Type};
use crate::prelude::*;

//use crate::ui::chart::{events::EventData, Type};

#[CustomControl(overwrite =[OnPaint,OnKeyPressed,OnMouseEvent,OnResize], internal = true)]
pub struct ChartBar {
    m_ox_label: String,
    m_oy_label: String,

    m_x_extra: i32,
    m_y_extra: i32,

    m_min_val: i32,
    m_max_val: i32,

    m_y_axes_left_space: i32,

    m_chart_type: Type,
    m_distance: u8,
    m_bar_width: u8,

    m_flags: Flags,
    m_comp: ScrollBars,

    m_data: Vec<Value<i32>>,
    m_top_view: usize,
    m_left_view: usize,

    m_left_offset: i32,

    m_pos: usize,
}

impl ChartBar {
    /// Creates a excel like chart that allows the user to show data  
    /// #Examples
    /// '''rust,no_run
    /// use appcui::prelude::*
    /// let mut chart = ChartBar::new(Vec::from([1,2,3,4,5,8,9]),chart::Type::VerticalBar,false,Layout::new("d:c,w:100%,h:100%"));
    ///

    pub fn new(ox_label: &str, oy_label: &str, chart_type: chartbar::Type, distance: u8, flags: Flags, bar_width: u8, layout: Layout) -> Self {
        let mut status_flags = StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput;
        if flags.contains(Flags::ScrollBars) {
            status_flags |= StatusFlags::IncreaseBottomMarginOnFocus;
            status_flags |= StatusFlags::IncreaseRightMarginOnFocus;
        }
        let c = Self {
            base: ControlBase::with_status_flags(layout, status_flags),
            m_ox_label: String::from(ox_label),
            m_oy_label: String::from(oy_label),

            m_x_extra: oy_label.bytes().count() as i32,
            m_y_extra: 0,
            m_bar_width: bar_width.clamp(1, 40),

            m_chart_type: chart_type,
            m_distance: distance.clamp(0,10),
            m_comp: ScrollBars::new(flags.contains(Flags::ScrollBars)),
            m_flags: flags,

            m_data: Vec::new(),
            m_top_view: 0,
            m_left_view: 0,

            m_left_offset: 0,

            m_pos: usize::MAX,
            m_y_axes_left_space: 6,
        };
        c
    }

    fn update_scroll_pos_from_scrollbars(&mut self) {
        self.m_left_offset = self.m_comp.horizontal_index() as i32;
    }

    fn update_scrollbars_size(&mut self) {
        self.m_comp
            .resize(self.m_data.len() as u64 * (self.m_distance + self.m_bar_width) as u64 + self.m_y_axes_left_space as u64, 0, &self.base);
    }

    pub fn update_scrollbars(&mut self) {
        self.m_comp.set_indexes(self.m_left_offset as u64, self.m_top_view as u64);
    }

    pub fn add_value(&mut self, value: Value<i32>) {
        let w = (self.size().width.saturating_sub(6) / (self.m_bar_width as u32 + self.m_distance as u32)) as usize;

        self.m_data.push(value);
        if self.m_data.len() == 1 {
            self.m_left_view = 0;
        }
        if self.m_flags.contains(Flags::AutoScroll) {
            self.m_left_offset = self.m_data.len().saturating_sub(w) as i32;
        }
        self.update_scrollbars();
        self.update_scrollbars_size();
    }

    pub fn clear_values(&mut self) {
        self.m_data.clear();
        self.m_left_view = 0;
        self.update_scrollbars_size();
        self.update_scrollbars();
    }

   
}

impl OnPaint for ChartBar {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        //pregatim terenul pentru scrollbars
        if self.has_focus() && (self.m_flags.contains(Flags::ScrollBars)) {
            self.m_comp.paint(surface, theme, self);
            if self.m_flags.contains(Flags::ScrollBars) {
                surface.reduce_clip_by(0, 0, 1, 1);
            } else {
                surface.reduce_clip_by(0, 0, 0, 1);
            }
        }
        //curatam pagina de caracterele anterioare
        surface.clear(Character::with_attributes(' ', theme.editor.normal));

        //separam axele ox si oy de restul chartului
        let len = self.m_data.len();
        let length = self.m_oy_label.bytes().count();
        let sz = self.size();
        let lineattr = if self.is_enabled() { theme.lines.normal } else { theme.lines.inactive };
        let oy_label_width = if length >= 4 { 2 } else { length as u8 - 1 };

        let bar_width = self.m_bar_width as i32 + self.m_distance as i32;

        let start = self.m_left_offset / bar_width;
        let h = (sz.height - 1) as i32;

        for (index, c) in self.m_data[start as usize..].iter().enumerate() {
            let x = index as i32 * bar_width + self.m_y_axes_left_space;
            if x > sz.width as i32 {
                break;
            }
            let rect1 = Rect::with_size(x, h - c.value() - 1, self.m_bar_width as u16, c.value() as u16);
            surface.fill_rect(rect1, Character::with_attributes(' ', c.attr()));
        }

        surface.draw_vertical_line(5, 0, sz.height as i32 - 2, LineType::Single, lineattr);
        if length >= 4 {
            surface.write_string(0, 0, ".....", theme.editor.normal, false);
        }
        surface.write_string(0, 0, &self.m_oy_label[0..(oy_label_width + 1) as usize], theme.editor.normal, false);

        surface.draw_horizontal_line(5, sz.height as i32 - 2, sz.width as i32, LineType::Single, lineattr);
        surface.write_string(
            sz.width as i32 - self.m_ox_label.bytes().count() as i32,
            sz.height as i32 - 1,
            self.m_ox_label.as_str(),
            theme.editor.normal,
            false,
        );

        surface.write_char(
            5,
            sz.height as i32 - 2,
            Character::with_attributes(SpecialChar::BoxBottomLeftCornerSingleLine, lineattr),
        );
        surface.write_char(
            5,
            sz.height as i32 / 2,
            Character::with_attributes(SpecialChar::BoxCrossSingleLine, lineattr),
        );

        surface.draw_horizontal_line(0, sz.height as i32 / 2, sz.width as i32, LineType::Single, lineattr);

        // let mut poz = 6;
        // if self.m_chart_type == Type::VerticalBar {
        //     let mut y: i32 = 0;
        //     let mut idx = self.m_left_view;
        //     let count = len;
        //     let width = (self.size().width.saturating_sub(6) / (self.m_bar_width as u32  + self.m_distance as u32)) as i32;

        //     while y < width && idx < count {
        //         // let rect = Rect::new(
        //         //     poz ,
        //         //     h - self.m_data[idx as usize].get_value() - 1,
        //         //     poz + self.m_bar_width as i32,
        //         //     h - 2,
        //         // );
        //         let rect1 = Rect::with_size(poz, h-self.m_data[idx as usize].get_value() - 1 , self.m_bar_width as u16, self.m_data[idx as usize].get_value() as u16);
        //         // let rect2 = Rect::with_alignament(poz, h - 1, self.m_bar_width as u16, self.m_data[idx as usize].get_value() , Alignment::Center );
        //         surface.fill_rect(
        //             rect1,
        //             Character::new(
        //                 32,
        //                 self.m_data[idx as usize].get_color().foreground,
        //                 self.m_data[idx as usize].get_color().background,
        //                 self.m_data[idx as usize].get_color().flags,
        //             ),
        //         );
        //         y += 1;
        //         idx += 1;
        //         poz += self.m_bar_width as i32 + self.m_distance as i32;
        //     }

        // }

        // if self.m_chart_type == Type::HorizontalBar {
        //     for i in self.m_data.iter() {
        //         surface.fill_horizontal_line(
        //             1 + self.m_x_extra,
        //             poz + self.m_y_extra,
        //             i.value() + self.m_x_extra,
        //             char!("' ',black,dr"),
        //         );
        //         let rect = Rect::new(self.m_x_extra, poz, self.m_x_extra + i.value(), poz + self.m_bar_width as i32);
        //         surface.fill_rect(rect, char!("' ',black,dr"));
        //         poz += 1 + self.m_distance as i32;
        //     }
        // }

        // if self.m_chart_type == Type::Line {}
    }
}

impl OnKeyPressed for ChartBar {
    fn on_key_pressed(&mut self, key: Key, _character: char) -> EventProcessStatus {
        match key.value() {
            key!("Left") => {
                self.m_left_view = self.m_left_view.saturating_sub(1);
                self.update_scrollbars();
                self.update_scrollbars_size();
                return EventProcessStatus::Processed;
            }
            key!("Right") => {
                let w = (self.size().width.saturating_sub(6) / (self.m_bar_width as u32 + self.m_distance as u32)) as usize;
                let new_poz = self.m_left_view.saturating_add(1);
                let len = self.m_data.len();
                if new_poz + w < len + 2 {
                    self.m_left_view = new_poz
                }
                self.update_scrollbars();
                self.update_scrollbars_size();
                return EventProcessStatus::Processed;
            }
            key!("Home") => {
                return EventProcessStatus::Processed;
            }
            _ => {}
        };
        if self.m_comp.should_repaint() {
            EventProcessStatus::Processed
        } else {
            EventProcessStatus::Ignored
        }
    }
}

pub fn is_over(poz: &Point, top: i32, left: i32, bottom: i32, right: i32) -> bool {
    if poz.x >= top && poz.x <= bottom && poz.y >= left && poz.y <= right {
        return true;
    }
    return false;
}

impl OnMouseEvent for ChartBar {
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        if self.m_comp.process_mouse_event(event) {
            self.update_scroll_pos_from_scrollbars();
            return EventProcessStatus::Processed;
        }
        match event {
            MouseEvent::Enter => {
                self.hide_tooltip();
            }
            MouseEvent::Leave => {
                self.hide_tooltip();
            }
            MouseEvent::Over(point) => {
                if is_over(point, 0, 1, 6, 1) && self.m_oy_label.bytes().count() >= 4 {}

                let sz = self.size();
                let len = self.m_data.len();
                let h = (sz.height - 1) as i32;

                let mut poz = 6;
                if self.m_chart_type == Type::VerticalBar {
                    let mut y: i32 = 0;
                    let mut idx = self.m_left_view;
                    let count = len;
                    let width = (self.size().width.saturating_sub(6) / (self.m_bar_width + self.m_distance) as u32) as i32;

                    while y < width && idx < count {
                        if is_over(
                            point,
                            poz,
                            h - self.m_data[idx as usize].value() - 1,
                            poz + self.m_bar_width as i32 - 1,
                            h - 2,
                        ) {
                            self.show_tooltip_on_point(
                                format!("{},{}", self.m_data[idx].value(), self.m_data[idx].label()).as_str(),
                                point.x,
                                point.y,
                            );
                            return EventProcessStatus::Processed;
                        } else {
                            self.hide_tooltip();
                        }
                        y += 1;
                        idx += 1;
                        poz += self.m_distance as i32 + self.m_bar_width as i32;
                    }

                    return EventProcessStatus::Processed;
                }
                return EventProcessStatus::Ignored;
            }
            MouseEvent::Pressed(mouse_event_data) => {
                if mouse_event_data.button == MouseButton::Right {
                    // De implementat un meniu de schimbat culori la fiecare bara cand o apesi
                }
            }
            MouseEvent::Released(_mouse_event_data) => {}
            MouseEvent::DoubleClick(_mouse_event_data) => {}
            MouseEvent::Drag(_mouse_event_data) => {}
            MouseEvent::Wheel(_mouse_wheel_direction) => {}
        }
        EventProcessStatus::Ignored
    }
}

impl OnResize for ChartBar {
    fn on_resize(&mut self, _old_size: Size, _new_size: Size) {
        // de reparat bug cu cand maresti lungimea ferestrei
        self.update_scrollbars();
        self.update_scrollbars_size();
    }
}
