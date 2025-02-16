use chartbar::Value;
use flat_string::FlatString;

use crate::graphics::Rect;
use crate::prelude::*;
use crate::ui::chartbar::initialization_flags::{Fit, Flags, Type, YAxes};

struct YAxesData {
    label: FlatString<14>,
    min: i32,
    max: i32,
    step: i32,
    left_space: u32,
}

#[CustomControl(overwrite =[OnPaint,OnKeyPressed,OnMouseEvent,OnResize], internal = true)]
pub struct ChartBar {
    ox_label: String,

    min_on_size: i32,
    max_on_size: i32,

    y_axes: Option<YAxesData>,
    y_axes_type: YAxes,

    max_bar_height: u32,

    chart_type: Type,
    fit: Fit,
    distance: u8,
    bar_width: u8,

    flags: Flags,
    comp: ScrollBars,

    data: Vec<Value>,

    top_view: i32,
    left_offset: u32,
}

impl ChartBar {
    pub fn default_chart() -> Self {
        Self {
            base: ControlBase::default(),
            ox_label: String::from(""),

            bar_width: 1,

            chart_type: Type::VerticalBar,
            distance: 0,
            comp: ScrollBars::new(false),
            flags: Flags::None,
            fit: Fit::None,
            data: Vec::new(),
            top_view: 0,
            left_offset: 0,

            min_on_size: 0,
            max_on_size: 100,

            max_bar_height: 50,

            y_axes: Some(YAxesData {
                label: FlatString::from_str("11"),
                min: 0,
                max: 100,
                step: 5,
                left_space: 5,
            }),
            y_axes_type: YAxes::Auto,
        }
    }

    /// Creates a excel like chart that allows the user to show data  
    /// #Examples
    /// '''rust,no_run
    /// use appcui::prelude::*
    /// let mut chart = ChartBar::new(Vec::from([1,2,3,4,5,8,9]),chart::Type::VerticalBar,false,Layout::new("d:c,w:100%,h:100%"));
    ///
    pub fn new(f: Flags, layout: Layout) -> Self {
        let mut status_flags = StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput;

        if f.contains(Flags::ScrollBars) {
            status_flags |= StatusFlags::IncreaseBottomMarginOnFocus;
            status_flags |= StatusFlags::IncreaseRightMarginOnFocus;
        }

        let mut c_type = Type::VerticalBar;
        if f.contains(Flags::Line) {
            c_type = Type::Line
        };
        if f.contains(Flags::VerticalBar) {
            c_type = Type::VerticalBar
        };

        let fits = if f.contains(Flags::FitToHeight) { Fit::FitToHeight } else { Fit::None };

        let y_axes = if f.contains(Flags::ManualYAxesSize) {
            YAxes::MinMax(0, 100)
        } else if f.contains(Flags::AdaptivYAXesOnData) {
            YAxes::Auto
        } else {
            YAxes::Visible
        };

        Self {
            base: ControlBase::with_status_flags(layout, status_flags),
            y_axes_type: y_axes,
            chart_type: c_type,
            fit: fits,
            flags: f,
            comp: ScrollBars::new(f.contains(Flags::ScrollBars)),
            ..Self::default_chart()
        }
    }

    pub fn set_fit_to_window_height(&mut self, b: bool) {
        self.fit = if b == true { Fit::FitToHeight } else { Fit::None };
    }

    #[inline(always)]
    fn oy_label(&self) -> &str {
        self.y_axes.as_ref().map(|f| f.label.as_str()).unwrap_or("?")
    }

    #[inline(always)]
    fn yaxes_interval(&self) -> (i32, i32) {
        self.y_axes.as_ref().map(|f| (f.min, f.max)).unwrap_or((0, 50))
    }

    #[inline(always)]
    fn step(&self) -> i32 {
        self.y_axes.as_ref().map(|f| f.step).unwrap_or(2)
    }

    #[inline(always)]
    fn left_space(&self) -> u32 {
        self.y_axes.as_ref().map(|y| y.left_space).unwrap_or(4)
    }

    pub fn get_bars_count(&self) -> usize {
        self.data.len()
    }

    pub fn index(&self, idx: usize) -> Option<&Value> {
        if idx < self.data.len() {
            Some(&self.data[idx])
        } else {
            None
        }
    }

    pub fn index_mut(&mut self, idx: usize) -> Option<&mut Value> {
        if idx < self.data.len() {
            Some(&mut self.data[idx])
        } else {
            None
        }
    }

    fn update_min_max(&mut self) {
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

    fn write_string_on_y_axes(&self, surface: &mut Surface, y: i32, label: &str, attr: CharAttribute) {
        let left_space = self.left_space();
        let mut index_copy = -5;
        for (index, c) in label.as_bytes().iter().enumerate() {
            if index >= left_space.saturating_sub(3) as usize {
                index_copy = index as i32;
                break;
            }
            surface.write_char(index as i32, y, Character::with_attributes(*c as char, attr));
        }

        if index_copy >= 0 {
            while index_copy < left_space.saturating_sub(1) as i32 {
                surface.write_char(index_copy, y, Character::with_attributes('.', attr));
                index_copy += 1;
            }
        }
    }

    pub fn set_max_bar_height(&mut self, value: u32) {
        self.max_bar_height = if self.fit == Fit::FitToHeight { self.base.size().height } else { value };
    }

    pub fn set_axes_left_space(&mut self, val: u32) {
        if self.y_axes.is_some() {
            self.y_axes.as_mut().unwrap().left_space = val;
        }
    }

    fn update_scroll_pos_from_scrollbars(&mut self) {
        self.left_offset = self.comp.horizontal_index() as u32;
        self.top_view = -(self.comp.vertical_index() as i32);
    }

    fn update_scrollbars_size(&mut self) {
        let len = self.data.len() as u64;
        let bar_width = (self.distance + self.bar_width) as u64;
        let total_width = len * bar_width + self.left_space() as u64;
        let total_height = (self.max_bar_height as u64 + self.size().height as u64).saturating_sub(2);
        self.comp.resize(total_width, total_height, &self.base);
    }

    fn update_scrollbars(&mut self) {
        self.comp.set_indexes(self.left_offset as u64, -self.top_view as u64);
    }

    pub fn add_value(&mut self, value: Value) {
        let bar_width = (self.bar_width + self.distance) as u32;
        let len = self.data.len() as u32 * bar_width;
        let w = self.size().width.saturating_sub(self.left_space());

        self.data.push(value);

        if self.flags.contains(Flags::AutoScroll) && len - self.left_offset * bar_width > w {
            self.left_offset += bar_width;
        }
        self.top_view = 0;
        self.update_min_max();

        self.update_scrollbars();
        self.update_scrollbars_size();
        self.on_resize(self.size(), self.size());
    }

    pub fn add_data(&mut self, data: &Vec<i32>) {
        for &i in data {
            self.add_value(Value::new(i));
        }
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
    pub fn set_chart_type(&mut self, t: chartbar::Type) {
        self.chart_type = t;
    }
    pub fn set_manul_size_for_yaxes(&mut self, min: i32, max: i32) {
        self.y_axes_type = YAxes::MinMax(min, max);
    }
    pub fn set_adaptive_size_for_yaxes(&mut self) {
        self.y_axes_type = YAxes::Auto;
    }
    pub fn set_adaptive_size_on_view_for_yaxes(&mut self) {
        self.y_axes_type = YAxes::Visible;
    }
    pub fn set_distance_between_bars(&mut self, distance: u8) {
        self.distance = distance;
    }
    pub fn set_bar_width(&mut self, width: u8) {
        self.bar_width = width;
    }
}

impl OnPaint for ChartBar {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        let mut output: [u8; 16] = [0; 16];
        const FMT_NUMBER: FormatNumber = FormatNumber::new(10);

        //pregatim terenul pentru scrollbars
        if self.has_focus() && (self.flags.contains(Flags::ScrollBars)) {
            self.comp.paint(surface, theme, self);
            if self.flags.contains(Flags::ScrollBars) {
                surface.reduce_clip_by(0, 0, 1, 1);
            } else {
                surface.reduce_clip_by(0, 0, 0, 1);
            }
        }
        let (back_attr, line_attr, bar_color) = match () {
            _ if !self.is_enabled() => (theme.editor.normal, theme.lines.inactive, Some(theme.editor.inactive.foreground)),
            _ if self.has_focus() => (theme.editor.focused, theme.lines.focused, None),
            _ if self.is_mouse_over() => (theme.editor.hovered, theme.lines.hovered, Some(theme.editor.hovered.foreground)),
            _ => (theme.editor.normal, theme.lines.normal, Some(theme.editor.normal.foreground)),
        };
        let default_color = theme.editor.focused.foreground;
        //curatam pagina de caracterele anterioare
        surface.clear(Character::with_attributes(' ', back_attr));
        //separam axele ox si oy de restul chartului
        let left_space = self.left_space() as i32;
        let sz = self.size();
        surface.draw_vertical_line(left_space, 0, sz.height as i32 - 2, LineType::Single, line_attr);

        surface.draw_horizontal_line(left_space, sz.height as i32 - 2, sz.width as i32, LineType::Single, line_attr);
        surface.write_string(
            sz.width as i32 - self.ox_label.len() as i32,
            sz.height as i32 - 1,
            self.ox_label.as_str(),
            back_attr,
            false,
        );

        surface.write_char(
            left_space,
            sz.height as i32 - 2,
            Character::with_attributes(SpecialChar::BoxBottomLeftCornerSingleLine, line_attr),
        );

        let interval = self.yaxes_interval();
        let max = interval.1;
        let step = self.step();

        let bar_width = self.bar_width as u32 + self.distance as u32;

        let start = self.left_offset.saturating_div(bar_width);
        let h = (sz.height - 1) as i32;

        let d = (self.max_on_size.saturating_sub(self.min_on_size)) as u32;
        let mut i = 0;

        if self.y_axes.is_some() {
            while i <= max + h - 2 {
                if h - 2 - i - self.top_view < h - 2 {
                    let v = (((i as u32).saturating_mul(d) / self.max_bar_height.max(1)) as i32).saturating_add(self.min_on_size);
                    if let Some(txt) = FMT_NUMBER.write_number(v, &mut output) {
                        self.write_string_on_y_axes(surface, h - i - self.top_view, txt, back_attr);
                    }
                    surface.draw_horizontal_line(left_space + 1, h - i - self.top_view, sz.width as i32, LineType::RoofLine, line_attr);
                }
                i += step;
            }
        }

        if self.chart_type == Type::VerticalBar {
            for (index, c) in self.data[start as usize..].iter().enumerate() {
                let x = index as u32 * bar_width + left_space as u32 + 1;
                if x > sz.width {
                    break;
                }
                let val = c.relative_size(self.max_bar_height, self.min_on_size, self.max_on_size).max(1) as i32;

                if h - val - self.top_view - 1 <= h - val + val.max(1) - 2 {
                    let rect = Rect::new(
                        x as i32,
                        h - val - self.top_view - 1,
                        x as i32 + self.bar_width.max(1) as i32 - 1,
                        h - val + val.max(1) - 2,
                    );
                    surface.fill_rect(rect, Character::with_attributes(' ', c.attr(bar_color, default_color)));
                }
            }
        } else if self.chart_type == Type::Line {
            let mut prev_val = 1;
            for (index, c) in self.data[start as usize..].iter().enumerate() {
                let x = index as u32 * bar_width + left_space as u32 + 1;
                if x > sz.width {
                    break;
                }
                let val = c.relative_size(self.max_bar_height, self.min_on_size, self.max_on_size).max(1) as i32;
                let bar_attr = c.attr(bar_color, default_color);
                if h - val - self.top_view - 1 <= h - val + val.max(1) - 2 {
                    surface.write_char(x as i32, h - val - self.top_view - 1, Character::with_attributes(' ', bar_attr));
                }
                if h - prev_val - self.top_view - 1 < h - val - self.top_view - 1 {
                    surface.fill_vertical_line(
                        x as i32,
                        h - prev_val - self.top_view - 1,
                        (h - val - self.top_view - 1).min(h - 1),
                        Character::with_attributes(' ', bar_attr),
                    );
                } else {
                    surface.fill_vertical_line(
                        x as i32,
                        h - val - self.top_view - 1,
                        (h - prev_val - self.top_view - 1).min(h - 1),
                        Character::with_attributes(' ', bar_attr),
                    );
                }
                prev_val = val;
            }
        }

        self.write_string_on_y_axes(surface, 0, self.oy_label(), back_attr);
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
                return EventProcessStatus::Processed;
            }
            MouseEvent::Leave => {
                self.hide_tooltip();
                return EventProcessStatus::Processed;
            }
            MouseEvent::Over(point) => {
                let sz = self.size();
                let bar_width = self.bar_width as u32 + self.distance as u32;

                let start = self.left_offset / bar_width;
                let h = (sz.height - 1) as i32;
                let left_space = self.left_space() as i32;

                if self.chart_type == Type::VerticalBar {
                    for (index, c) in self.data[start as usize..].iter().enumerate() {
                        let x = index as u32 * bar_width + left_space as u32 + 1;
                        if x > sz.width {
                            break;
                        }
                        let val = c.relative_size(self.max_bar_height, self.min_on_size, self.max_on_size).max(1) as i32;
                        //let rect1 = Rect::with_size(x as i32, h - 1, self.bar_width as u16, val as u16);
                        if h - val - self.top_view - 1 <= h - val + val.max(1) - 2 {
                            if is_over(
                                point,
                                x as i32,
                                h - val - self.top_view - 1,
                                x as i32 + self.bar_width.max(1) as i32 - 1,
                                h - val + val.max(1) - 2,
                            ) {
                                self.show_tooltip_on_point(format!("{},{}", c.value(), c.label()).as_str(), point.x, point.y);
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
                if Flags::contains(&self.flags, Flags::ScrollBars) {
                    let bar_width = (self.bar_width + self.distance) as u32;
                    self.left_offset = self.left_offset.saturating_sub(bar_width);

                    self.update_min_max();
                    self.update_scrollbars();
                    self.update_scrollbars_size();
                }
                return EventProcessStatus::Processed;
            }
            key!("Right") => {
                if Flags::contains(&self.flags, Flags::ScrollBars) {
                    let bar_width = (self.bar_width + self.distance) as u32;
                    let len = self.data.len() as u32 * bar_width;
                    let w = self.size().width.saturating_sub(self.left_space());

                    let new_poz = self.left_offset.saturating_add(bar_width);
                    let final_pos = len.saturating_sub(w);

                    if new_poz <= final_pos + 1 {
                        self.left_offset = new_poz;
                    }

                    self.update_min_max();
                    self.update_scrollbars();
                    self.update_scrollbars_size();
                }
                return EventProcessStatus::Processed;
            }
            key!("Home") => {
                if Flags::contains(&self.flags, Flags::ScrollBars) {
                    self.left_offset = 0;

                    self.update_min_max();
                    self.update_scrollbars();
                    self.update_scrollbars_size();
                    return EventProcessStatus::Processed;
                }
            }
            key!("End") => {
                if Flags::contains(&self.flags, Flags::ScrollBars) {
                    let bar_width = (self.bar_width + self.distance) as u32;
                    let len = self.data.len() as u32 * bar_width;
                    let w = self.base.size().width.saturating_sub(self.left_space());
                    self.left_offset = len.saturating_sub(w) + bar_width;

                    self.update_min_max();
                    self.update_scrollbars();
                    self.update_scrollbars_size();
                }
                return EventProcessStatus::Processed;
            }
            key!("Down") => {
                if Flags::contains(&self.flags, Flags::ScrollBars) {
                    let new_pos = self.top_view - 1;

                    if -new_pos <= self.max_bar_height as i32 {
                        self.top_view = new_pos;
                    }

                    self.update_scrollbars();
                    self.update_scrollbars_size();
                }
                return EventProcessStatus::Processed;
            }

            key!("Up") => {
                if Flags::contains(&self.flags, Flags::ScrollBars) {
                    let new_pos = self.top_view + 1;

                    if new_pos <= 0 {
                        self.top_view = new_pos;
                    }

                    self.update_scrollbars();
                    self.update_scrollbars_size();
                }
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
    false
}

impl OnResize for ChartBar {
    fn on_resize(&mut self, _old_size: Size, _new_size: Size) {
        self.set_max_bar_height(self.size().height);
        self.update_min_max();
        self.update_scrollbars();
        self.update_scrollbars_size();
    }
}
