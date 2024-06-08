use chrono::{DateTime, Datelike, NaiveDate, Utc};
use AppCUIProcMacro::CustomControl;

const MINSPACE_FOR_SHORT_DATE: u32 = 15;
const MINSPACE_FOR_LONG_DATE: u32 = 18;
const MINSPACE_FOR_DROPBUTTON_DRAWING: u32 = 3;
const MINSPACE_FOR_DATE_DRAWING: u32 = 5;
const MIN_WIDTH_FOR_DATE_NAME: u32 = 6;
const CALENDAR_WIDTH: u32 = 30;
const CALENDAR_HEIGHT: u32 = 12;
enum DateSize {
    Large,
    Small,
    VerySmall,
}

#[CustomControl(overwrite=OnPaint+OnDefaultAction+OnExpand+OnMouseEvent, internal=true)]
// +OnKeyPressed
pub struct DatePicker {
    header_y_ofs: i32,
    expanded_panel_y: i32,
    selected_date: DateTime<Utc>,
    date_string: String,
    // date_size: DateSize,
}

impl DatePicker {
    pub fn new(date: DateTime<Utc>, layout: Layout) -> Self {
        let mut dp = DatePicker {
            base: ControlBase::with_status_flags(layout, StatusFlags::Enabled | StatusFlags::Visible | StatusFlags::AcceptInput),
            header_y_ofs: 0,
            expanded_panel_y: 1,
            selected_date: date,
            date_string: Self::format_long_date(date),
            // date_size: DateSize::Large,
        };
        dp.set_size_bounds(6, 1, u16::MAX, 1);
        let date_len = dp.get_date_size();
        match date_len {
            DateSize::Large => {}
            DateSize::Small => {
                dp.date_string = Self::format_short_date(date);
            }
            DateSize::VerySmall => {
                dp.date_string = Self::format_very_short_date(date);
            }
        }
        dp
    }

    pub fn format_very_short_date(selected_date: DateTime<Utc>) -> String {
        selected_date.format("%d.%m.%y").to_string()
    }
    pub fn format_short_date(selected_date: DateTime<Utc>) -> String {
        selected_date.format("%d.%m.%Y").to_string()
    }

    pub fn format_long_date(selected_date: DateTime<Utc>) -> String {
        selected_date.format("%Y, %b, %d").to_string()
    }

    pub fn get_date_ints(&self) -> (i32, i32, i32) {
        (self.selected_date.year(), self.selected_date.month() as i32, self.selected_date.day() as i32)
    }

    fn get_date_size(&self) -> DateSize {
        let width: u32 = self.size().width;
        if width > MINSPACE_FOR_LONG_DATE {
            DateSize::Large
        } else if width > MINSPACE_FOR_SHORT_DATE {
            DateSize::Small
        } else {
            DateSize::VerySmall
        }
    }
    fn get_printed_chars(&self) -> u32 {
        let wdth = self.size().width - MIN_WIDTH_FOR_DATE_NAME;
        if (self.date_string.len() as u32) > wdth {
            wdth
        } else {
            self.date_string.len() as u32
        }
    }

    fn days_in_month(&self) -> u32 {
        let month = self.selected_date.month();
        let year = self.selected_date.year();
        let next_month = if month == 12 { 1 } else { month + 1 };
        let next_month_year = if month == 12 { year + 1 } else { year };
        
        let first_of_next_month = NaiveDate::from_ymd_opt(next_month_year, next_month, 1)
            .unwrap();
        let first_of_current_month = NaiveDate::from_ymd_opt(year, month, 1)
            .unwrap();
            
        first_of_next_month.signed_duration_since(first_of_current_month)
            .num_days() as u32
    }
}

impl OnPaint for DatePicker {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        let size = self.size();
        let col_text = match () {
            _ if !self.is_enabled() => theme.button.text.inactive,
            _ if self.has_focus() => theme.button.text.focused,
            _ if self.is_mouse_over() => theme.button.text.hovered,
            _ => theme.button.text.normal,
        };

        let space_char = Character::with_attributes(' ', col_text);

        // expanded calendar
        let date_size = self.get_date_size();
        let mut format = TextFormat::single_line(1, self.header_y_ofs, col_text, TextAlignament::Left);
        surface.fill_horizontal_line(0, self.header_y_ofs, (size.width - MINSPACE_FOR_DATE_DRAWING) as i32, space_char);
        format.width = Some((size.width - MIN_WIDTH_FOR_DATE_NAME) as u16);
        match date_size {
            DateSize::Large => {
                surface.write_text(Self::format_long_date(self.selected_date).as_str(), &format);
            }
            DateSize::Small => {
                surface.write_text(Self::format_short_date(self.selected_date).as_str(), &format);
            }
            DateSize::VerySmall => {
                surface.write_text(Self::format_very_short_date(self.selected_date).as_str(), &format);
            }
        }

        if size.width >= MINSPACE_FOR_DROPBUTTON_DRAWING {
            let px = (size.width - MINSPACE_FOR_DROPBUTTON_DRAWING) as i32;
            surface.fill_horizontal_line_with_size(px, self.header_y_ofs, 3, space_char);
            surface.write_char(px + 1, self.header_y_ofs, Character::with_attributes(SpecialChar::TriangleDown, col_text));
        }

        // expanded calendar
        if self.is_expanded() {
            let size = self.expanded_size();
            let col = theme.menu.text.normal;
            let space_char = Character::with_attributes(' ', col);
            surface.fill_rect(
                Rect::with_size(0, self.expanded_panel_y, size.width as u16, (size.height - 1) as u16),
                space_char,
            );
            surface.draw_rect(
                Rect::with_size(0, self.expanded_panel_y, size.width as u16, (size.height - 1) as u16),
                LineType::Single,
                col,
            );
            surface.draw_horizontal_line(1, 2 + self.expanded_panel_y, (CALENDAR_WIDTH - 1) as i32, LineType::SingleRound, col);
            surface.write_char(0, 2 + self.expanded_panel_y, Character::with_attributes(SpecialChar::BoxMidleLeft, col));
            surface.write_char((CALENDAR_WIDTH - 1) as i32, 2 + self.expanded_panel_y, Character::with_attributes(SpecialChar::BoxMidleRight, col));
            
            let year = self.selected_date.year();
            let mut year_format = TextFormat::single_line(7, 1 + self.expanded_panel_y, col, TextAlignament::Left);
            year_format.width = Some(4);
            surface.write_text(year.to_string().as_str(), &year_format);
            surface.write_char(5, 1 + self.expanded_panel_y, Character::with_attributes(SpecialChar::TriangleLeft, col));
            surface.write_char(12, 1 + self.expanded_panel_y, Character::with_attributes(SpecialChar::TriangleRight, col));
            surface.write_char(2, 1 + self.expanded_panel_y, Character::with_attributes('<', col));
            surface.write_char(3, 1 + self.expanded_panel_y, Character::with_attributes('<', col));
            surface.write_char(14, 1 + self.expanded_panel_y, Character::with_attributes('>', col));
            surface.write_char(15, 1 + self.expanded_panel_y, Character::with_attributes('>', col));
            
            let month: String = self.selected_date.format("%b").to_string();
            let mut month_format = TextFormat::single_line(22, 1 + self.expanded_panel_y, col, TextAlignament::Left);
            month_format.width = Some(3);
            surface.write_char(20, 1 + self.expanded_panel_y, Character::with_attributes(SpecialChar::TriangleLeft, col));
            surface.write_text(&month.as_str(), &month_format);
            surface.write_char(26, 1 + self.expanded_panel_y, Character::with_attributes(SpecialChar::TriangleRight, col));
        
        
            let days = vec!["Mo", "Tu", "We", "Th", "Fr", "Sa", "Su"];
            let mut day_format = TextFormat::single_line(2, 3 + self.expanded_panel_y, col, TextAlignament::Left);
            day_format.width = Some(2);
            for i in 0..7 {
                day_format.char_attr = theme.menu.text.inactive;
                surface.write_text(days[i], &day_format);
                day_format.x += 4;
            }

            let mut day_row = 4 + self.expanded_panel_y;
            let mut day_col = 2i32;

            let first_day = self.selected_date.with_day(1).unwrap().format("%a").to_string();  
            let date_day = self.selected_date.day(); 
            let mut first_day_index = 0i32;
            for i in 0..days.len() {
                if first_day.starts_with(days[i]) {
                    first_day_index = i as i32;
                    break;
                }
            }
            let last_day = self.days_in_month();

            for i in 0..last_day {
                let day = i + 1;
                if day == 1 {
                    day_col = first_day_index * 4 + 3;
                }
                let mut day_format = TextFormat::single_line(day_col, day_row, col, TextAlignament::Right);
                if day == date_day{                    
                    day_format.char_attr = theme.menu.text.hovered;
                }
                day_format.width = Some(2);
                surface.write_text(day.to_string().as_str(), &day_format);
                day_col += 4;
                if day_col >= 30 {
                    day_col = 3;
                    day_row += 1;
                }
            }
        }
    }
}

impl OnDefaultAction for DatePicker {
    fn on_default_action(&mut self) {
        if self.is_expanded() {
            self.pack();
        } else {
            self.expand(Size::new(CALENDAR_WIDTH, CALENDAR_HEIGHT), Size::new(CALENDAR_WIDTH, CALENDAR_HEIGHT));
        }
    }
}
impl OnExpand for DatePicker {
    fn on_expand(&mut self, direction: ExpandedDirection) {
        match direction {
            ExpandedDirection::OnTop => {
                self.expanded_panel_y = 0;
                self.header_y_ofs = (self.expanded_size().height as i32) - 1;
            }
            ExpandedDirection::OnBottom => {
                self.expanded_panel_y = 1;
                self.header_y_ofs = 0;
            }
        }
        // self.mouse_on_color_index = -1;
    }
    fn on_pack(&mut self) {
        self.expanded_panel_y = 1;
        self.header_y_ofs = 0;
    }
}

impl OnMouseEvent for DatePicker {
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        match event {
            MouseEvent::Enter => {
                if !self.is_expanded() && (self.date_string.len() as i32) > (self.get_printed_chars() as i32) {
                    self.show_tooltip(self.date_string.as_str())
                }
                EventProcessStatus::Processed
            }

            MouseEvent::Leave => {
                self.hide_tooltip();
                EventProcessStatus::Processed
            }
            MouseEvent::Over(p) => {
                // let idx = self.mouse_to_color_index(p.x, p.y);
                // if idx != self.mouse_on_color_index {
                //     self.mouse_on_color_index = idx;
                //     return EventProcessStatus::Processed;
                // }
                EventProcessStatus::Ignored
            }
            MouseEvent::Pressed(data) => {
            //     let idx = self.mouse_to_color_index(data.x, data.y);
            //     if let Some(col) = Color::from_value(idx) {
            //         if col != self.color {
            //             self.color = col;
            //             self.raise_event(ControlEvent {
            //                 emitter: self.handle,
            //                 receiver: self.event_processor,
            //                 data: ControlEventData::ColorPicker(EventData { color: col }),
            //             });
            //         }
            //     }
                self.on_default_action();
                EventProcessStatus::Processed
            }
            _ => EventProcessStatus::Ignored,
        }
    }
}
