use chartbar::Value;
use flat_string::FlatString;

use crate::graphics::Rect;
use crate::prelude::*;
use crate::ui::chartbar::initialization_flags::{Flags, Type, YAxes};

//use crate::ui::chart::{events::EventData, Type};

struct YAxesData {
    label: FlatString<14>,
    min: i32,
    max: i32,
    step: i32,
    left_space: u32,
}

// impl YAxes
// {
//     fn set_label(&mut self, label: FlatString<14>) { self.label = label; }
//     fn set_interval(&mut self, min: i32, max: i32) { self.min = min; self.max = max; }
//     fn set_step(&mut self, step: i32) { self.step = step ; }
//     fn set_left_space(&mut self, left_space: u32) { self.left_space = left_space; }

//     fn label(&self) -> FlatString<14> { self.label }
//     fn interval(&self) -> (i32,i32) { (self.min,self.max) }
//     fn step(&self) -> i32 { self.step }
//     fn left_space(&self) -> u32 { self.left_space }

// }

#[CustomControl(overwrite =[OnPaint,OnKeyPressed,OnMouseEvent,OnResize], internal = true)]
pub struct ChartBar {
    ox_label: String,

    min_on_size: i32,
    max_on_size: i32,

    y_axes: Option<YAxesData>,

    max_bar_height: u32,

    chart_type: Type,
    distance: u8,
    bar_width: u8,

    flags: Flags,
    comp: ScrollBars,

    data: Vec<Value>,

    top_view: i32,

    y_axes_type: YAxes,

    left_offset: u32,
}

impl ChartBar {
    /// Creates a excel like chart that allows the user to show data  
    /// #Examples
    /// '''rust,no_run
    /// use appcui::prelude::*
    /// let mut chart = ChartBar::new(Vec::from([1,2,3,4,5,8,9]),chart::Type::VerticalBar,false,Layout::new("d:c,w:100%,h:100%"));
    ///
    pub fn new(ox_label: &str, chart_type: chartbar::Type, distance: u8, flags: Flags, bar_width: u8, y_axes_type: YAxes, layout: Layout) -> Self {
        let mut status_flags = StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput;
        if flags.contains(Flags::ScrollBars) {
            status_flags |= StatusFlags::IncreaseBottomMarginOnFocus;
            status_flags |= StatusFlags::IncreaseRightMarginOnFocus;
        }
        let c = Self {
            base: ControlBase::with_status_flags(layout, status_flags),
            ox_label: String::from(ox_label),

            bar_width: bar_width.clamp(1, 40),

            chart_type: chart_type,
            distance: distance.clamp(0, 10),
            comp: ScrollBars::new(flags.contains(Flags::ScrollBars)),
            flags: flags,

            data: Vec::new(),
            top_view: 0,

            left_offset: 0,

            min_on_size: i32::MAX,
            max_on_size: i32::MIN,

            max_bar_height: 50,

            y_axes: None,
            y_axes_type,
        };
        c
    }

    #[inline(always)]
    fn oy_label(&self) -> &str {
        self.y_axes.as_ref().map(|f| f.label.as_str()).unwrap_or("")
    }

    #[inline(always)]
    fn yaxes_interval(&self) -> (i32, i32) {
        self.y_axes.as_ref().map(|f| (f.min, f.max)).unwrap_or((0, i32::MAX))
    }

    #[inline(always)]
    fn step(&self) -> i32 {
        self.y_axes.as_ref().map(|f| f.step).unwrap_or(2)
    }

    #[inline(always)]
    fn left_space(&self) -> u32 {
        self.y_axes.as_ref().map(|y| y.left_space).unwrap_or(0)
    }

    fn update_min_max_on_wiew_width(&mut self) {
        match self.y_axes_type {
            YAxes::MinMax(v1, v2) => {
                self.min_on_size = v1;
                self.max_on_size = v2;
            }
            YAxes::Auto => {
                self.min_on_size = i32::MAX;
                self.max_on_size = i32::MIN;

                for c in self.data.iter() {
                    self.min_on_size = self.min_on_size.min(c.value());
                    self.max_on_size = self.max_on_size.max(c.value());
                }
            }
            YAxes::Visible => {
                let bar_width = (self.bar_width + self.distance) as u32;
                let start = self.left_offset / bar_width;

                self.min_on_size = i32::MAX;
                self.max_on_size = i32::MIN;

                for (index, c) in self.data[start as usize..].iter().enumerate() {
                    let x = index as u32 * bar_width + self.left_space();
                    if x > self.size().width {
                        break;
                    }
                    self.min_on_size = self.min_on_size.min(c.value());
                    self.max_on_size = self.max_on_size.max(c.value());
                }
            }
        }
    }

    pub fn write_string_on_y_axes(&self, surface: &mut Surface, theme: &Theme, y: i32, label: &String) {
        let attr = theme.editor.normal;
        let left_space = self.left_space();
        let mut index_copy = -5;
        for (index, c) in label.as_bytes().iter().enumerate() {
            if index >= left_space.saturating_sub(3) as usize {
                index_copy = index as i32;
                break;
            }
            surface.write_char(0 + index as i32, y, Character::with_attributes(*c as char, attr));
        }

        if index_copy >= 0 {
            while index_copy < left_space.saturating_sub(1) as i32 {
                surface.write_char(0 + index_copy, y, Character::with_attributes('.', attr));
                index_copy += 1;
            }
        }
    }

    pub fn set_max_bar_height(&mut self, value: u32) {
        self.max_bar_height = value;
    }

    pub fn set_axes_left_space(&mut self, val: u32) {
        if self.y_axes.is_some() {
            self.y_axes.as_mut().unwrap().left_space = val;
        }
    }

    pub fn update_scroll_pos_from_scrollbars(&mut self) {
        self.left_offset = self.comp.horizontal_index() as u32;
        self.top_view = self.comp.vertical_index() as i32 * -1;
    }

    fn update_scrollbars_size(&mut self) {
        let len = self.data.len() as u64;
        let bar_width = (self.distance + self.bar_width) as u64;
        let total_width = len * bar_width + self.left_space() as u64;
        let total_height = self.max_bar_height as u64 + self.size().height as u64 - 2;
        self.comp.resize(total_width, total_height, &self.base);
    }

    pub fn update_scrollbars(&mut self) {
        self.comp.set_indexes(self.left_offset as u64, (self.top_view * -1) as u64);
    }

    pub fn add_value(&mut self, value: Value) {
        let len = self.data.len() as u32;
        let bar_width = (self.bar_width + self.distance) as u32;
        let w = self.size().width.saturating_sub(self.left_space()) / bar_width;

        self.data.push(value);
        if len == 1 {}
        if self.flags.contains(Flags::AutoScroll) {
            self.left_offset = len.saturating_sub(w) * bar_width;
        }
        self.update_min_max_on_wiew_width();
        self.left_offset = 0;

        self.update_scrollbars();
        self.update_scrollbars_size();
        self.on_resize(self.size(), self.size());
    }

    pub fn clear_values(&mut self) {
        self.data.clear();
        self.left_offset = 0;
        self.update_scrollbars_size();
        self.update_scrollbars();
    }

    pub fn set_yaxes(&mut self, label: &str, min: i32, max: i32, step: i32, left_space: u32) {
        self.y_axes = Some(YAxesData {
            label: FlatString::<14>::from_str(label),
            min,
            max,
            step,
            left_space,
        });
    }
}

impl OnPaint for ChartBar {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        //pregatim terenul pentru scrollbars
        if self.has_focus() && (self.flags.contains(Flags::ScrollBars)) {
            self.comp.paint(surface, theme, self);
            if self.flags.contains(Flags::ScrollBars) {
                surface.reduce_clip_by(0, 0, 1, 1);
            } else {
                surface.reduce_clip_by(0, 0, 0, 1);
            }
        }
        //curatam pagina de caracterele anterioare
        surface.clear(Character::with_attributes(' ', theme.editor.normal));

        //separam axele ox si oy de restul chartului
        let left_space = self.left_space() as i32;
        let sz = self.size();
        let lineattr = if self.is_enabled() { theme.lines.normal } else { theme.lines.inactive };
        surface.draw_vertical_line(left_space, 0, sz.height as i32 - 2, LineType::Single, lineattr);

        surface.draw_horizontal_line(left_space, sz.height as i32 - 2, sz.width as i32, LineType::Single, lineattr);
        surface.write_string(
            sz.width as i32 - self.ox_label.bytes().count() as i32,
            sz.height as i32 - 1,
            self.ox_label.as_str(),
            theme.editor.normal,
            false,
        );

        surface.write_char(
            left_space,
            sz.height as i32 - 2,
            Character::with_attributes(SpecialChar::BoxBottomLeftCornerSingleLine, lineattr),
        );

        let interval = self.yaxes_interval();
        let max = interval.1;
        let step = self.step();

        let bar_width = self.bar_width as u32 + self.distance as u32;

        let start = self.left_offset / bar_width;
        let h = (sz.height - 1) as i32;

        let mut min = self.data[start as usize].relative_size(self.max_bar_height, self.min_on_size, self.max_on_size);

        let d = (self.max_on_size - self.min_on_size) as u32;
        let mut i = 2;
        min += 1;
        while i <= max + h {
            if h - i - self.top_view <= h - 2 {
                let v = (i as u32 * d / self.max_bar_height) as i32 + self.min_on_size;
                self.write_string_on_y_axes(surface, theme, h - i - self.top_view, &format!("{}", v));
                surface.draw_horizontal_line(left_space + 1, h - i - self.top_view, sz.width as i32, LineType::RoofLine, lineattr);
            }
            min += step as u32;
            i += step;
        }

        for (index, c) in self.data[start as usize..].iter().enumerate() {
            let x = index as u32 * bar_width + left_space as u32 + 1;
            if x > sz.width as u32 {
                break;
            }
            let val = c.relative_size(self.max_bar_height, self.min_on_size, self.max_on_size) as i32 + 1;

            if h - val - self.top_view as i32 - 1 <= h - val + val.max(1) - 2 {
                let rect = Rect::new(
                    x as i32,
                    h - val - self.top_view as i32 - 1,
                    x as i32 + self.bar_width.max(1) as i32 - 1,
                    h - val + val.max(1) - 2,
                );
                surface.fill_rect(rect, Character::with_attributes(' ', c.attr()));
            }
        }

        self.write_string_on_y_axes(surface, theme, 0, &String::from(self.oy_label()));
    }
}

impl OnMouseEvent for ChartBar {
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        if self.comp.process_mouse_event(event) {
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
                if is_over(point, 0, 1, 6, 1) && self.oy_label().bytes().count() >= 4 {}

                let sz = self.size();

                if self.chart_type == Type::VerticalBar {
                    let bar_width = self.bar_width as u32 + self.distance as u32;

                    let start = self.left_offset / bar_width;
                    let h = (sz.height - 1) as i32;
                    let left_space = self.left_space() as i32;

                    for (index, c) in self.data[start as usize..].iter().enumerate() {
                        let x = index as u32 * bar_width + left_space as u32 + 1;
                        if x > sz.width as u32 {
                            break;
                        }
                        let val = c.relative_size(self.max_bar_height, self.min_on_size, self.max_on_size) as i32 + 1;
                        //let rect1 = Rect::with_size(x as i32, h - 1, self.bar_width as u16, val as u16);
                        if h - val - self.top_view as i32 - 1 <= h - val + val.max(1) - 2 {
                            if is_over(
                                point,
                                x as i32,
                                h - val - self.top_view as i32 - 2,
                                x as i32 + self.bar_width.max(1) as i32 - 1,
                                h - val + val.max(1) - 2,
                            ) {
                                self.show_tooltip_on_point(format!("{},{},{}", c.value(), val, self.max_on_size).as_str(), point.x, point.y);
                                return EventProcessStatus::Processed;
                            } else {
                                self.hide_tooltip();
                            }
                        }
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

impl OnKeyPressed for ChartBar {
    fn on_key_pressed(&mut self, key: Key, _character: char) -> EventProcessStatus {
        match key.value() {
            key!("Left") => {
                let bar_width = (self.bar_width + self.distance) as u32;
                self.left_offset = self.left_offset.saturating_sub(bar_width);

                self.update_min_max_on_wiew_width();
                self.update_scrollbars();
                self.update_scrollbars_size();
                return EventProcessStatus::Processed;
            }
            key!("Right") => {
                let len = self.data.len() as u32;
                let bar_width = (self.bar_width + self.distance) as u32;
                let w = (self.size().width.saturating_sub(self.left_space()) / bar_width) as u32;
                let new_poz = self.left_offset.saturating_add(bar_width);
                let final_pos = len.saturating_sub(w) * bar_width;
                if new_poz <= final_pos {
                    self.left_offset = new_poz;
                }

                self.update_min_max_on_wiew_width();
                self.update_scrollbars();
                self.update_scrollbars_size();
                return EventProcessStatus::Processed;
            }
            key!("Home") => {
                self.left_offset = 0;

                self.update_min_max_on_wiew_width();
                self.update_scrollbars();
                self.update_scrollbars_size();
                return EventProcessStatus::Processed;
            }
            key!("End") => {
                let len = self.data.len() as u32;
                let bar_width = (self.bar_width + self.distance) as u32;
                let w = self.base.size().width.saturating_sub(self.left_space()) / bar_width;
                self.left_offset = len.saturating_sub(w) * bar_width;

                self.update_min_max_on_wiew_width();
                self.update_scrollbars();
                self.update_scrollbars_size();
                return EventProcessStatus::Processed;
            }
            key!("Down") => {
                let new_pos = self.top_view - 1;

                if new_pos * -1 <= self.max_bar_height as i32 {
                    self.top_view = new_pos;
                }

                self.update_scrollbars();
                self.update_scrollbars_size();
                return EventProcessStatus::Processed;
            }

            key!("Up") => {
                let new_pos = self.top_view + 1;

                if new_pos <= 0 {
                    self.top_view = new_pos;
                }

                self.update_scrollbars();
                self.update_scrollbars_size();
                return EventProcessStatus::Processed;
            }
            _ => {}
        };
        if self.comp.should_repaint() {
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

impl OnResize for ChartBar {
    fn on_resize(&mut self, _old_size: Size, _new_size: Size) {
        self.update_min_max_on_wiew_width();
        self.update_scrollbars();
        self.update_scrollbars_size();
    }
}
