use chrono::{DateTime, Local, Utc};
use AppCUIProcMacro::CustomControl;

const MINSPACE_FOR_SHORT_DATE: u32 = 15;
const MINSPACE_FOR_LONG_DATE: u32 = 18;
const MINSPACE_FOR_DROPBUTTON_DRAWING: u32 = 3;
const MINSPACE_FOR_DATE_DRAWING: u32 = 5;
const MIN_WIDTH_FOR_DATE_NAME: u32 = 6;

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

    fn get_date_size(&self, width: u32) -> DateSize {
        if width > MINSPACE_FOR_LONG_DATE {
            DateSize::Large
        } else if width > MINSPACE_FOR_SHORT_DATE {
            DateSize::Small
        } else {
            DateSize::VerySmall
        }
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
        let date_size = self.get_date_size(size.width);
        match date_size {
            DateSize::Large => {
                surface.fill_horizontal_line(0, self.header_y_ofs, (size.width - MINSPACE_FOR_DATE_DRAWING) as i32, space_char);
                let mut format = TextFormat::single_line(1, self.header_y_ofs, col_text, TextAlignament::Left);
                format.width = Some((size.width - MIN_WIDTH_FOR_DATE_NAME) as u16);
                surface.write_text(Self::format_long_date(self.selected_date).as_str(), &format);
            }
            DateSize::Small => {
                surface.fill_horizontal_line(0, self.header_y_ofs, (size.width - MINSPACE_FOR_DATE_DRAWING) as i32, space_char);
                let mut format = TextFormat::single_line(1, self.header_y_ofs, col_text, TextAlignament::Left);
                format.width = Some((size.width - MIN_WIDTH_FOR_DATE_NAME) as u16);
                surface.write_text(Self::format_short_date(self.selected_date).as_str(), &format);
            }
            DateSize::VerySmall => {
                surface.fill_horizontal_line(0, self.header_y_ofs, (size.width - MINSPACE_FOR_DATE_DRAWING) as i32, space_char);
                let mut format = TextFormat::single_line(1, self.header_y_ofs, col_text, TextAlignament::Left);
                format.width = Some((size.width - MIN_WIDTH_FOR_DATE_NAME) as u16);
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
        }
    }
}

impl OnDefaultAction for DatePicker {
    fn on_default_action(&mut self) {
        if self.is_expanded() {
            self.pack();
        } else {
            self.expand(Size::new(30, 11), Size::new(30, 11));
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
                if !self.is_expanded() && self.date_string.len() as i32 > ((self.size().width as i32) - MIN_WIDTH_FOR_DATE_NAME as i32) {
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
            // MouseEvent::Pressed(data) => {
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
            //     self.on_default_action();
            //     EventProcessStatus::Processed
            // }
            _ => EventProcessStatus::Ignored,
        }
    }
}
