use chrono::{NaiveDate, Datelike, Utc};
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
#[derive(PartialEq, Eq)]
enum HoveredDate {
    DoubleLeftArrow,
    LeftArrowYear,
    RightArrowYear,
    DoubleRightArrow,
    LeftArrowMonth,
    RightArrowMonth,
    Day(u32),
    None,
}
enum CharOrSpecialChar {
    Regular(char),
    Special(SpecialChar),
}

#[CustomControl(overwrite=OnPaint+OnDefaultAction+OnExpand+OnMouseEvent, internal=true)]
// +OnKeyPressed
pub struct DatePicker {
    header_y_ofs: i32,
    expanded_panel_y: i32,
    selected_date: NaiveDate,
    date_string: String,
    hover_date: HoveredDate,
    // date_size: DateSize,
}

impl DatePicker {
    const DAYS: [&'static str; 7] = ["Mo", "Tu", "We", "Th", "Fr", "Sa", "Su"];
    // pub fn new(date: NaiveDate, layout: Layout) -> Self {
    //     let mut dp = DatePicker {
    //         base: ControlBase::with_status_flags(layout, StatusFlags::Enabled | StatusFlags::Visible | StatusFlags::AcceptInput),
    //         header_y_ofs: 0,
    //         expanded_panel_y: 1,
    //         selected_date: date,
    //         date_string: Self::format_long_date(date),
    //         hover_date: HoveredDate::None,
    //         // date_size: DateSize::Large,
    //     };
    //     dp.set_size_bounds(6, 1, u16::MAX, 1);
    //     let date_len = dp.get_date_size();
    //     match date_len {
    //         DateSize::Large => {}
    //         DateSize::Small => {
    //             dp.date_string = Self::format_short_date(date);
    //         }
    //         DateSize::VerySmall => {
    //             dp.date_string = Self::format_very_short_date(date);
    //         }
    //     }
    //     dp
    // }

    pub fn new(date_str: &str, layout: Layout) -> Self {
        let date = date_str.parse::<NaiveDate>().unwrap();
        let mut dp = DatePicker {
            base: ControlBase::with_status_flags(layout, StatusFlags::Enabled | StatusFlags::Visible | StatusFlags::AcceptInput),
            header_y_ofs: 0,
            expanded_panel_y: 1,
            selected_date: date,
            date_string: Self::format_long_date(date),
            hover_date: HoveredDate::None,
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

    fn mouse_over_calendar(&self, x: i32, y: i32) -> HoveredDate {
        if !self.is_expanded() {
            return HoveredDate::None;
        }
        if y == 1 + self.expanded_panel_y {
            if x == 5 {
                return HoveredDate::LeftArrowYear;
            }
            if x == 12 {
                return HoveredDate::RightArrowYear;
            }
            if x == 2 || x == 3 {
                return HoveredDate::DoubleLeftArrow;
            }
            if x == 14 || x == 15 {
                return HoveredDate::DoubleRightArrow;
            }

            if x == 20 {
                return HoveredDate::LeftArrowMonth;
            }
            if x == 26 {
                return HoveredDate::RightArrowMonth;
            }
        }
        let mut col = self.get_first_day_index() * 4 + 3;
        let mut row = 4 + self.expanded_panel_y;
        let last_day = self.days_in_month() as i32;

        for i in 0..last_day {
            let day = i + 1;

            if(y == row) && (x == col || x == (col - 1)) {
                return HoveredDate::Day(day as u32);
            }

            col += 4;
            if col >= 30 {
                col = 3;
                row += 1;
            }
        }
        return HoveredDate::None;
    }

    pub fn format_very_short_date(selected_date: NaiveDate) -> String {
        selected_date.format("%d.%m.%y").to_string()
    }
    pub fn format_short_date(selected_date: NaiveDate) -> String {
        selected_date.format("%d.%m.%Y").to_string()
    }

    pub fn format_long_date(selected_date: NaiveDate) -> String {
        selected_date.format("%Y, %b, %d").to_string()
    }

    pub fn get_date_ints(&self) -> (i32, i32, i32) {
        (
            self.selected_date.year(),
            self.selected_date.month() as i32,
            self.selected_date.day() as i32,
        )
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

        let first_of_next_month = NaiveDate::from_ymd_opt(next_month_year, next_month, 1).unwrap();
        let first_of_current_month = NaiveDate::from_ymd_opt(year, month, 1).unwrap();

        first_of_next_month.signed_duration_since(first_of_current_month).num_days() as u32
    }

    fn get_first_day_index(&self) -> i32 {
        let first_day = self.selected_date.with_day(1).unwrap().format("%a").to_string();
        for i in 0..DatePicker::DAYS.len() {
            if first_day.starts_with(DatePicker::DAYS[i]) {
                return i as i32;
            }
        }
        0
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

        // header
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
            surface.write_char(
                (CALENDAR_WIDTH - 1) as i32,
                2 + self.expanded_panel_y,
                Character::with_attributes(SpecialChar::BoxMidleRight, col),
            );

            let year = self.selected_date.year();
            let mut year_format = TextFormat::single_line(7, 1 + self.expanded_panel_y, col, TextAlignament::Left);
            year_format.width = Some(4);
            surface.write_text(year.to_string().as_str(), &year_format);

            fn set_char(surface: &mut Surface, x: i32, y: i32, char_or_special: CharOrSpecialChar, condition: bool, theme: &Theme) {
                let attr = if condition { theme.menu.text.hovered } else { theme.menu.text.normal };
                let character = match char_or_special {
                    CharOrSpecialChar::Regular(c) => Character::with_attributes(c, attr),
                    CharOrSpecialChar::Special(sc) => Character::with_attributes(sc, attr),
                };
                surface.write_char(x, y, character);
            }
            let y_pos = 1 + self.expanded_panel_y;

            set_char(
                surface,
                5,
                y_pos,
                CharOrSpecialChar::Special(SpecialChar::TriangleLeft),
                self.hover_date == HoveredDate::LeftArrowYear,
                theme,
            );
            set_char(
                surface,
                12,
                y_pos,
                CharOrSpecialChar::Special(SpecialChar::TriangleRight),
                self.hover_date == HoveredDate::RightArrowYear,
                theme,
            );
            set_char(
                surface,
                2,
                y_pos,
                CharOrSpecialChar::Regular('<'),
                self.hover_date == HoveredDate::DoubleLeftArrow,
                theme,
            );
            set_char(
                surface,
                3,
                y_pos,
                CharOrSpecialChar::Regular('<'),
                self.hover_date == HoveredDate::DoubleLeftArrow,
                theme,
            );
            set_char(
                surface,
                14,
                y_pos,
                CharOrSpecialChar::Regular('>'),
                self.hover_date == HoveredDate::DoubleRightArrow,
                theme,
            );
            set_char(
                surface,
                15,
                y_pos,
                CharOrSpecialChar::Regular('>'),
                self.hover_date == HoveredDate::DoubleRightArrow,
                theme,
            );
            set_char(
                surface,
                20,
                y_pos,
                CharOrSpecialChar::Special(SpecialChar::TriangleLeft),
                self.hover_date == HoveredDate::LeftArrowMonth,
                theme,
            );
            set_char(
                surface,
                26,
                y_pos,
                CharOrSpecialChar::Special(SpecialChar::TriangleRight),
                self.hover_date == HoveredDate::RightArrowMonth,
                theme,
            );

            let month: String = self.selected_date.format("%b").to_string();
            let mut month_format = TextFormat::single_line(22, 1 + self.expanded_panel_y, col, TextAlignament::Left);
            month_format.width = Some(3);
            surface.write_text(&month.as_str(), &month_format);

            let mut day_format = TextFormat::single_line(2, 3 + self.expanded_panel_y, col, TextAlignament::Left);
            day_format.width = Some(2);
            for i in 0..7 {
                day_format.char_attr = theme.menu.text.inactive;
                surface.write_text(DatePicker::DAYS[i], &day_format);
                day_format.x += 4;
            }

            let mut day_row = 4 + self.expanded_panel_y;
            let mut day_col = self.get_first_day_index() * 4 + 3;

            let date_day = self.selected_date.day();
            let last_day = self.days_in_month();

            for i in 0..last_day {
                let day = i + 1;
                let mut day_format = TextFormat::single_line(day_col, day_row, col, TextAlignament::Right);
                if day == date_day {
                    day_format.char_attr = theme.menu.text.pressed_or_selectd;
                } else if self.hover_date == HoveredDate::Day(day) {
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
        self.hover_date = HoveredDate::None;
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
                let hd = self.mouse_over_calendar(p.x, p.y);
                if hd != self.hover_date {
                    self.hover_date = hd;
                }
                EventProcessStatus::Processed
            }
            MouseEvent::Pressed(data) => {
                
                let hd = self.mouse_over_calendar(data.x, data.y);
                if hd != HoveredDate::None {
                    match hd {
                        HoveredDate::DoubleLeftArrow => {
                            self.selected_date = self.selected_date.with_year(self.selected_date.year() - 10).unwrap();
                        }
                        HoveredDate::LeftArrowYear => {
                            self.selected_date = self.selected_date.with_year(self.selected_date.year() - 1).unwrap();
                        }
                        HoveredDate::RightArrowYear => {
                            self.selected_date = self.selected_date.with_year(self.selected_date.year() + 1).unwrap();
                        }
                        HoveredDate::DoubleRightArrow => {
                            self.selected_date = self.selected_date.with_year(self.selected_date.year() + 10).unwrap();
                        }
                        HoveredDate::LeftArrowMonth => {
                            self.selected_date = self.selected_date.with_month(self.selected_date.month() - 1).unwrap();
                        }
                        HoveredDate::RightArrowMonth => {
                            self.selected_date = self.selected_date.with_month(self.selected_date.month() + 1).unwrap();
                        }
                        HoveredDate::Day(day) => {
                            self.selected_date = self.selected_date.with_day(day).unwrap();
                        }
                        _ => {}
                    }
                    // MONTH PANICS ON DEC->JAN
                    // MONTH PANICS ON JAN->DEC
                    // MONTH PANICS ON 31
                    // YEAR PANICS ON 29 FEBRUARY


                    // self.date_string = Self::format_long_date(self.selected_date);
                    // self.raise_event(ControlEvent {
                    //     emitter: self.handle,
                    //     receiver: self.event_processor,
                    //     data: ControlEventData::DatePicker(EventData { date: self.selected_date }),
                    // });
                    return EventProcessStatus::Processed;
                }
                self.on_default_action();
                EventProcessStatus::Processed
            }
            _ => EventProcessStatus::Ignored,
        }
    }
}
