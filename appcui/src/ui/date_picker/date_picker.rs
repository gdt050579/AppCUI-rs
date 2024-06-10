use chrono::{Datelike, Days, Months, NaiveDate};
use date_picker::events::EventData;
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

#[CustomControl(overwrite=OnPaint+OnDefaultAction+OnExpand+OnMouseEvent+OnKeyPressed, internal=true)]
pub struct DatePicker {
    header_y_ofs: i32,
    expanded_panel_y: i32,
    selected_date: NaiveDate,
    date_string: String,
    hover_date: HoveredDate,
    virtual_date: NaiveDate,
}

impl DatePicker {
    const DAYS: [&'static str; 7] = ["Mo", "Tu", "We", "Th", "Fr", "Sa", "Su"];
    const MONTHS: [&'static str; 12] = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];

    /// Creates a new date picker with a NaiveDate and a layout.
    ///
    /// # Example
    /// ```rust,no_run
    /// use appcui::prelude::*;
    ///
    /// let date_picker = DatePicker::with_date(NaiveDate::from_ymd(2024, 6, 13), Layout::new("x:1,y:1,w:19"));
    /// ```
    pub fn with_date(date: NaiveDate, layout: Layout) -> Self {
        let mut dp = DatePicker {
            base: ControlBase::with_status_flags(layout, StatusFlags::Enabled | StatusFlags::Visible | StatusFlags::AcceptInput),
            header_y_ofs: 0,
            expanded_panel_y: 1,
            selected_date: date,
            date_string: Self::format_long_date(date),
            hover_date: HoveredDate::None,
            virtual_date: date,
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

    /// Creates a new date picker with a date string and a layout.
    /// The date string must be in the format "YYYY-MM-DD".
    ///
    /// # Example                                       
    /// ```rust,no_run
    /// use appcui::prelude::*;
    ///
    /// let date_picker = DatePicker::new("2024-06-13", Layout::new("x:1,y:1,w:19"));
    /// ```
    pub fn new(date_str: &str, layout: Layout) -> Self {
        let date = date_str.parse::<NaiveDate>().unwrap();
        Self::with_date(date, layout)
    }

    /// Sets the date of the date picker as a string representation.
    pub fn set_date_str(&mut self, date_str: &str) {
        self.selected_date = date_str.parse::<NaiveDate>().unwrap();
        self.date_string = Self::format_long_date(self.selected_date);
    }

    /// Sets the date of the date picker from a NaiveDate.
    pub fn set_date(&mut self, date: NaiveDate) {
        self.selected_date = date;
        self.date_string = Self::format_long_date(date);
    }

    /// Returns the date of the date picker as a NaiveDate.
    #[inline(always)]
    pub fn date(&self) -> NaiveDate {
        self.selected_date
    }

    fn update_date(&mut self, date: NaiveDate) {
        if date != self.selected_date {
            self.selected_date = date;
            self.date_string = Self::format_long_date(date);

            self.raise_event(ControlEvent {
                emitter: self.handle,
                receiver: self.event_processor,
                data: ControlEventData::DatePicker(EventData { date: self.selected_date }),
            });
        }
    }

    fn jump_to_month(date: NaiveDate, target_month: u32) -> NaiveDate {
        let year = date.year();
        let day = date.day();

        let mut new_date = NaiveDate::from_ymd_opt(year, target_month, 1).unwrap();

        // Get the last day of the target month
        let last_day_of_month = (1..=31)
            .rev()
            .find(|&d| NaiveDate::from_ymd_opt(year, target_month, d).is_some())
            .unwrap();

        // Adjust the day to be the minimum of the original day and the last day of the month
        new_date = new_date.with_day(day.min(last_day_of_month)).unwrap();

        new_date
    }
    fn mouse_over_calendar(&mut self, x: i32, y: i32) -> HoveredDate {
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

            if (y == row) && (x == col || x == (col - 1)) {
                self.virtual_date = self.virtual_date.with_day(day as u32).unwrap();
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

    fn format_very_short_date(selected_date: NaiveDate) -> String {
        selected_date.format("%d.%m.%y").to_string()
    }
    fn format_short_date(selected_date: NaiveDate) -> String {
        selected_date.format("%d.%m.%Y").to_string()
    }

    fn format_long_date(selected_date: NaiveDate) -> String {
        selected_date.format("%Y, %b, %d").to_string()
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
        let month = self.virtual_date.month();
        let year = self.virtual_date.year();
        let next_month = if month == 12 { 1 } else { month + 1 };
        let next_month_year = if month == 12 { year + 1 } else { year };

        let first_of_next_month = NaiveDate::from_ymd_opt(next_month_year, next_month, 1).unwrap();
        let first_of_current_month = NaiveDate::from_ymd_opt(year, month, 1).unwrap();

        first_of_next_month.signed_duration_since(first_of_current_month).num_days() as u32
    }

    fn get_first_day_index(&self) -> i32 {
        let first_day = self.virtual_date.with_day(1).unwrap().format("%a").to_string();
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

            let year = self.virtual_date.year();
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

            let month = Self::MONTHS[self.virtual_date.month0() as usize];
            let mut month_format = TextFormat::single_line(22, 1 + self.expanded_panel_y, col, TextAlignament::Left);
            month_format.width = Some(3);
            surface.write_text(&month, &month_format);

            let mut day_format = TextFormat::single_line(2, 3 + self.expanded_panel_y, col, TextAlignament::Left);
            day_format.width = Some(2);
            for i in 0..7 {
                day_format.char_attr = theme.menu.text.inactive;
                surface.write_text(DatePicker::DAYS[i], &day_format);
                day_format.x += 4;
            }

            let mut day_row = 4 + self.expanded_panel_y;
            let mut day_col = self.get_first_day_index() * 4 + 3;

            let last_day = self.days_in_month();

            for i in 0..last_day {
                let day = i + 1;
                let mut day_format = TextFormat::single_line(day_col, day_row, col, TextAlignament::Right);

                day_format.width = Some(2);
                surface.write_text(day.to_string().as_str(), &day_format);
                if day == self.selected_date.day()
                    && self.selected_date.month() == self.virtual_date.month()
                    && self.selected_date.year() == self.virtual_date.year()
                {
                    surface.fill_horizontal_line_with_size(
                        day_col - 2,
                        day_row,
                        4,
                        Character::with_attributes(0, theme.menu.text.pressed_or_selectd),
                    );
                } else if self.virtual_date.day() == day {
                    surface.fill_horizontal_line_with_size(day_col - 2, day_row, 4, Character::with_attributes(0, theme.menu.text.hovered));
                }
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
        self.virtual_date = self.selected_date;
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
                            self.virtual_date = self.virtual_date - Months::new(120);
                        }
                        HoveredDate::LeftArrowYear => {
                            self.virtual_date = self.virtual_date - Months::new(12);
                        }
                        HoveredDate::RightArrowYear => {
                            self.virtual_date = self.virtual_date + Months::new(12);
                        }
                        HoveredDate::DoubleRightArrow => {
                            self.virtual_date = self.virtual_date + Months::new(120);
                        }
                        HoveredDate::LeftArrowMonth => {
                            self.virtual_date = self.virtual_date - Months::new(1);
                        }
                        HoveredDate::RightArrowMonth => {
                            self.virtual_date = self.virtual_date + Months::new(1);
                        }
                        HoveredDate::Day(day) => {
                            self.update_date(self.virtual_date.with_day(day).unwrap());
                            self.on_default_action();
                        }

                        _ => {}
                    }

                    return EventProcessStatus::Processed;
                }
                self.on_default_action();
                EventProcessStatus::Processed
            }
            _ => EventProcessStatus::Ignored,
        }
    }
}

impl OnKeyPressed for DatePicker {
    fn on_key_pressed(&mut self, key: Key, _character: char) -> EventProcessStatus {
        let expanded = self.is_expanded();

        match key.value() {
            key!("F") | key!("S") | key!("O") | key!("N") | key!("D") => {
                let month = match key.value() {
                    key!("F") => 2,
                    key!("S") => 9,
                    key!("O") => 10,
                    key!("N") => 11,
                    key!("D") => 12,
                    _ => unreachable!(),
                };
                if expanded {
                    self.virtual_date = Self::jump_to_month(self.virtual_date, month);
                } else {
                    self.update_date(Self::jump_to_month(self.selected_date, month));
                }
                return EventProcessStatus::Processed;
            }

            key!("J") | key!("A") | key!("M") | key!("Shift+J") | key!("Shift+A") | key!("Shift+M") => {
                let mut val = 1i32;
                let month_char = match key.value() {
                    key!("J") => "J",
                    key!("A") => "A",
                    key!("M") => "M",
                    key!("Shift+J") => {
                        val = -1;
                        "J"
                    }
                    key!("Shift+A") => {
                        val = -1;
                        "A"
                    }
                    key!("Shift+M") => {
                        val = -1;
                        "M"
                    }
                    _ => unreachable!(),
                };
                let target_month: &mut NaiveDate = if expanded { &mut self.virtual_date } else { &mut self.selected_date };

                let month = {
                    let mut current_month = target_month.month() as i32 + val;
                    if current_month > 12 {
                        current_month = 1;
                    }
                    if current_month < 1 {
                        current_month = 12;
                    }
                    for _ in 0..Self::MONTHS.len() {
                        if Self::MONTHS[(current_month - 1) as usize].starts_with(month_char) {
                            break;
                        } else {
                            current_month += val;
                            if current_month > 12 {
                                current_month = 1;
                            }
                            if current_month < 1 {
                                current_month = 12;
                            }
                        }
                    }
                    current_month
                };
                if expanded {
                    self.virtual_date = Self::jump_to_month(self.virtual_date, month as u32);
                } else {
                    self.update_date(Self::jump_to_month(self.selected_date, month as u32));
                }
                return EventProcessStatus::Processed;
            }

            _ => {}
        }

        if !expanded {
            match key.value() {
                key!("Escape") => {
                    return EventProcessStatus::Ignored;
                }
                key!("Up") => {
                    self.update_date(self.selected_date + Days::new(1));
                    return EventProcessStatus::Processed;
                }

                key!("Down") => {
                    self.update_date(self.selected_date - Days::new(1));
                    return EventProcessStatus::Processed;
                }

                key!("Shift+Up") => {
                    self.update_date(self.selected_date + Months::new(1));
                    return EventProcessStatus::Processed;
                }

                key!("Shift+Down") => {
                    self.update_date(self.selected_date - Months::new(1));
                    return EventProcessStatus::Processed;
                }

                key!("Ctrl+Up") => {
                    self.update_date(self.selected_date + Months::new(12));
                    return EventProcessStatus::Processed;
                }

                key!("Ctrl+Down") => {
                    self.update_date(self.selected_date - Months::new(12));
                    return EventProcessStatus::Processed;
                }

                key!("Ctrl+Shift+Up") => {
                    self.update_date(self.selected_date + Months::new(120));
                    return EventProcessStatus::Processed;
                }

                key!("Ctrl+Shift+Down") => {
                    self.update_date(self.selected_date - Months::new(120));
                    return EventProcessStatus::Processed;
                }

                key!("Enter") | key!("Space") => {
                    self.on_default_action();
                    return EventProcessStatus::Processed;
                }
                _ => {}
            }
            EventProcessStatus::Ignored
        } else {
            match key.value() {
                key!("Escape") => {
                    self.pack();
                    return EventProcessStatus::Processed;
                }

                key!("Up") => {
                    self.virtual_date = self.virtual_date - Days::new(7);
                    return EventProcessStatus::Processed;
                }

                key!("Down") => {
                    self.virtual_date = self.virtual_date + Days::new(7);
                    return EventProcessStatus::Processed;
                }

                key!("Left") => {
                    self.virtual_date = self.virtual_date - Days::new(1);
                    return EventProcessStatus::Processed;
                }

                key!("Right") => {
                    self.virtual_date = self.virtual_date + Days::new(1);
                    return EventProcessStatus::Processed;
                }

                key!("Shift+Left") => {
                    self.virtual_date = self.virtual_date - Months::new(1);
                    return EventProcessStatus::Processed;
                }

                key!("Shift+Right") => {
                    self.virtual_date = self.virtual_date + Months::new(1);
                    return EventProcessStatus::Processed;
                }

                key!("Ctrl+Left") => {
                    self.virtual_date = self.virtual_date - Months::new(12);
                    return EventProcessStatus::Processed;
                }

                key!("Ctrl+Right") => {
                    self.virtual_date = self.virtual_date + Months::new(12);
                    return EventProcessStatus::Processed;
                }

                key!("Ctrl+Shift+Left") => {
                    self.virtual_date = self.virtual_date - Months::new(120);
                    return EventProcessStatus::Processed;
                }

                key!("Ctrl+Shift+Right") => {
                    self.virtual_date = self.virtual_date + Months::new(120);
                    return EventProcessStatus::Processed;
                }

                key!("Enter") => {
                    self.update_date(self.virtual_date);
                    self.on_default_action();
                    return EventProcessStatus::Processed;
                }
                _ => {}
            }

            EventProcessStatus::Ignored
        }
    }
}
