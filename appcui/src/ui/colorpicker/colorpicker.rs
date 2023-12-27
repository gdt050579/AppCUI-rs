use crate::prelude::*;
use crate::ui::colorpicker::events::EventData;

const MINSPACE_FOR_COLOR_DRAWING: u32 = 5;
const MIN_WIDTH_FOR_COLOR_NAME: u32 = 8;
const MINSPACE_FOR_DROPBUTTON_DRAWING: u32 = 3;
const NUMBER_OF_COLORS: i32 = 16;
const COLOR_MATRIX_WIDTH: i32 = 4;
const COLOR_MATRIX_HEIGHT: i32 = 4;
const ONE_POSITION_TO_RIGHT: i32 = 1;
const ONE_POSITION_TO_LEFT: i32 = -1;
const SPACES_PER_COLOR: i32 = 3;
const TRANSPARENT_CHECKBOX_X_OFFSET: i32 = 15;
const TRANSPARENT_CHECKBOX_X_LAST_OFFSET: i32 = 29;
static REVERSED_COLORS: [Color; 16] = [
    Color::White,
    Color::White,
    Color::White,
    Color::White,
    Color::White,
    Color::White,
    Color::White,
    Color::Black,
    Color::Black,
    Color::White,
    Color::Black,
    Color::Black,
    Color::White,
    Color::White,
    Color::Black,
    Color::Black,
];

#[CustomControl(overwrite=OnPaint+OnDefaultAction+OnKeyPressed+OnMouseEvent+OnExpand, internal=true)]
pub struct ColorPicker {
    color: Color,
    header_y_ofs: i32,
    expanded_panel_y: i32,
    mouse_on_color_index: i32,
}
impl ColorPicker {
    pub fn new(color: Color, layout: Layout) -> Self {
        let mut cp = ColorPicker {
            base: ControlBase::new(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
            header_y_ofs: 0,
            expanded_panel_y: 1,
            mouse_on_color_index: -1,
            color,
        };
        cp.set_size_bounds(7, 1, u16::MAX, 1);
        cp
    }
    #[inline(always)]
    pub fn get_color(&self) -> Color {
        self.color
    }
    #[inline(always)]
    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    fn next_color(&mut self, expanded: bool, offset: i32) {
        let mut result = ((self.color as u8) as i32) + offset;
        if expanded {
            // specific cases
            // when the cursor is on the first line (the first 4 colors), it should be able to move to transparent checkbox
            // as well the logic below enphasize this
            let transparent = (Color::Transparent as u8) as i32;
            if (result == COLOR_MATRIX_WIDTH) && (offset == ONE_POSITION_TO_RIGHT) {
                result = transparent; // Move to the right with 1 position
            } else {
                if (result == transparent + 1) && (offset == ONE_POSITION_TO_RIGHT) {
                    result = 0;
                } else {
                    if (result == -1) && (offset == ONE_POSITION_TO_LEFT) {
                        result = transparent;
                    } else {
                        if (result == transparent - 1) && (offset == ONE_POSITION_TO_LEFT) {
                            result = COLOR_MATRIX_WIDTH - 1;
                        } else {
                            if result < 0 {
                                result += NUMBER_OF_COLORS;
                            }
                            if result >= NUMBER_OF_COLORS {
                                result -= NUMBER_OF_COLORS;
                            }
                        }
                    }
                }
            }
        } else {
            result = result.clamp(0, NUMBER_OF_COLORS);
        }
        if let Some(col) = Color::from_value(result) {
            self.color = col;
            self.raise_event(ControlEvent {
                emitter: self.handle,
                receiver: self.event_processor,
                data: ControlEventData::ColorPickerEvent(EventData { color: col }),
            });
        }
    }

    fn mouse_to_color_index(&self, x: i32, y: i32) -> i32 {
        if !self.is_expanded() {
            return -1;
        }
        if (x > 0)
            && (x < SPACES_PER_COLOR * COLOR_MATRIX_WIDTH + 1)
            && (y > self.expanded_panel_y)
            && (y < self.expanded_panel_y + COLOR_MATRIX_HEIGHT + 1)
        {
            return ((x - 1) / SPACES_PER_COLOR) + (y - (self.expanded_panel_y + 1)) * COLOR_MATRIX_WIDTH;
        }
        if (y == 1 + self.expanded_panel_y) && (x >= TRANSPARENT_CHECKBOX_X_OFFSET) && (x <= TRANSPARENT_CHECKBOX_X_LAST_OFFSET) {
            return (Color::Transparent as u8) as i32;
        }
        return -1;
    }
}
impl OnPaint for ColorPicker {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        // first paint the header
        let size = self.get_size();
        let col_text = match () {
            _ if !self.is_enabled() => theme.button.text.inactive,
            _ if self.has_focus() => theme.button.text.focused,
            _ if self.is_mouse_over() => theme.button.text.hovered,
            _ => theme.button.text.normal,
        };

        let space_char = Character::with_attributes(' ', col_text);
        if size.width > MINSPACE_FOR_COLOR_DRAWING {
            surface.fill_horizontal_line(0, self.header_y_ofs, (size.width - MINSPACE_FOR_COLOR_DRAWING) as i32, space_char);
            surface.write_char(
                1,
                self.header_y_ofs,
                Character::new(SpecialChar::BlockCentered, self.color, Color::Transparent, CharFlags::None),
            );
            if size.width > MIN_WIDTH_FOR_COLOR_NAME {
                let mut format = TextFormat::single_line(3, self.header_y_ofs, col_text, TextAlignament::Left);
                format.width = Some((size.width - MIN_WIDTH_FOR_COLOR_NAME) as u16);
                surface.write_text(self.color.get_name(), &format);
            }
        }
        if size.width >= MINSPACE_FOR_DROPBUTTON_DRAWING {
            let px = (size.width - MINSPACE_FOR_DROPBUTTON_DRAWING) as i32;
            surface.fill_horizontal_line_with_size(px, self.header_y_ofs, 3, space_char);
            surface.write_char(px + 1, self.header_y_ofs, Character::with_attributes(SpecialChar::TriangleDown, col_text));
        }
        // assuming the control is expanded
        if self.is_expanded() {
            let size = self.get_expanded_size();
            let col = theme.menu.text.normal;
            let mut space_char = Character::with_attributes(' ', col);
            surface.fill_rect(
                Rect::with_size(0, self.expanded_panel_y, size.width as u16, (size.height - 1) as u16),
                space_char,
            );
            surface.draw_rect(
                Rect::with_size(0, self.expanded_panel_y, size.width as u16, (size.height - 1) as u16),
                LineType::Single,
                col,
            );
            for y in 0..COLOR_MATRIX_HEIGHT {
                for x in 0..COLOR_MATRIX_WIDTH {
                    space_char.background = Color::from_value(y * COLOR_MATRIX_WIDTH + x).unwrap();
                    surface.fill_horizontal_line_with_size(
                        x * SPACES_PER_COLOR + 1,
                        y + 1 + self.expanded_panel_y,
                        SPACES_PER_COLOR as u32,
                        space_char,
                    );
                    if space_char.background == self.color {
                        surface.write_char(
                            x * SPACES_PER_COLOR + ((SPACES_PER_COLOR + 1) >> 1),
                            y + 1 + self.expanded_panel_y,
                            Character::new(
                                SpecialChar::CheckMark,
                                REVERSED_COLORS[(y * COLOR_MATRIX_WIDTH + x) as usize],
                                space_char.background,
                                CharFlags::None,
                            ),
                        );
                    }
                    if self.mouse_on_color_index == (y * COLOR_MATRIX_WIDTH + x) {
                        let x_p = x * SPACES_PER_COLOR + 1;
                        let y_p = y + 1 + self.expanded_panel_y;
                        let c_attr = CharAttribute::new(
                            REVERSED_COLORS[(y * COLOR_MATRIX_WIDTH + x) as usize],
                            space_char.background,
                            CharFlags::None,
                        );
                        surface.write_char(x_p, y_p, Character::with_attributes(SpecialChar::TriangleLeft, c_attr));
                        surface.write_char(x_p + 2, y_p, Character::with_attributes(SpecialChar::TriangleRight, c_attr));
                    }
                }
            }

            // transparent part
            let attr = match () {
                _ if self.color == Color::Transparent => theme.menu.text.focused,
                _ if self.mouse_on_color_index == 16 => theme.menu.text.hovered,
                _ => theme.menu.text.normal,
            };
            surface.write_string(TRANSPARENT_CHECKBOX_X_OFFSET, 1 + self.expanded_panel_y, "[ ] Transparent", attr, false);
            if self.color == Color::Transparent {
                surface.write_char(
                    TRANSPARENT_CHECKBOX_X_OFFSET + 1,
                    1 + self.expanded_panel_y,
                    Character::with_attributes(SpecialChar::CheckMark, theme.menu.symbol.normal),
                );
                surface.set_cursor(TRANSPARENT_CHECKBOX_X_OFFSET + 1, 1 + self.expanded_panel_y);
            }
        }
    }
}
impl OnDefaultAction for ColorPicker {
    fn on_default_action(&mut self) {
        if self.is_expanded() {
            self.pack();
        } else {
            self.expand(
                Size::new((TRANSPARENT_CHECKBOX_X_LAST_OFFSET as u32) + 2, 7),
                Size::new(self.get_size().width, 7),
            );
        }
    }
}
impl OnExpand for ColorPicker {
    fn on_expand(&mut self, direction: ExpandedDirection) {
        match direction {
            ExpandedDirection::OnTop => {
                self.expanded_panel_y = 0;
                self.header_y_ofs = (self.get_expanded_size().height as i32) - 1;
            }
            ExpandedDirection::OnBottom => {
                self.expanded_panel_y = 1;
                self.header_y_ofs = 0;
            }
        }
        self.mouse_on_color_index = -1;
    }
    fn on_pack(&mut self) {
        self.expanded_panel_y = 1;
        self.header_y_ofs = 0;
    }
}
impl OnKeyPressed for ColorPicker {
    fn on_key_pressed(&mut self, key: Key, _character: char) -> EventProcessStatus {
        let expanded = self.is_expanded();

        match key.get_compact_code() {
            key!("Escape") => {
                if expanded {
                    self.pack();
                    return EventProcessStatus::Processed;
                } else {
                    return EventProcessStatus::Ignored;
                }
            }
            key!("Space") | key!("Enter") => {
                self.on_default_action();
                return EventProcessStatus::Processed;
            }
            key!("Up") => {
                self.next_color(expanded, if expanded { -COLOR_MATRIX_WIDTH } else { -1 });
                return EventProcessStatus::Processed;
            }
            key!("Down") => {
                self.next_color(expanded, if expanded { COLOR_MATRIX_WIDTH } else { 1 });
                return EventProcessStatus::Processed;
            }
            key!("Left") => {
                self.next_color(expanded, -1);
                return EventProcessStatus::Processed;
            }
            key!("Right") => {
                self.next_color(expanded, 1);
                return EventProcessStatus::Processed;
            }
            _ => {}
        }
        EventProcessStatus::Ignored
    }
}
impl OnMouseEvent for ColorPicker {
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        match event {
            MouseEvent::Enter => EventProcessStatus::Processed,
            MouseEvent::Leave => EventProcessStatus::Processed,
            MouseEvent::Over(p) => {
                let idx = self.mouse_to_color_index(p.x, p.y);
                if idx != self.mouse_on_color_index {
                    self.mouse_on_color_index = idx;
                    return EventProcessStatus::Processed;
                }
                return EventProcessStatus::Ignored;
            }
            MouseEvent::Pressed(data) => {
                let idx = self.mouse_to_color_index(data.x, data.y);
                if let Some(col) = Color::from_value(idx) {
                    if col != self.color {
                        self.color = col;
                        self.raise_event(ControlEvent {
                            emitter: self.handle,
                            receiver: self.event_processor,
                            data: ControlEventData::ColorPickerEvent(EventData { color: col }),
                        });
                    }
                }
                self.on_default_action();
                return EventProcessStatus::Processed;
            }
            _ => return EventProcessStatus::Ignored,
        }
    }
}

/*
#include "ControlContext.hpp"

namespace AppCUI
{
constexpr int32 COLORPICEKR_HEIGHT                 = 7;
constexpr uint32 NO_COLOR_OBJECT                   = 0xFFFFFFFF;
constexpr int32 SPACES_PER_COLOR                   = 3;
constexpr int32 TRANSPARENT_CHECKBOX_X_OFFSET      = 15;
constexpr int32 TRANSPARENT_CHECKBOX_X_LAST_OFFSET = 29;
constexpr int32 ONE_POSITION_TO_RIGHT              = 1;
constexpr int32 ONE_POSITION_TO_LEFT               = -1;
constexpr int32 COLOR_MATRIX_WIDTH                 = 4;
constexpr int32 COLOR_MATRIX_HEIGHT                = 4;
constexpr int32 NUMBER_OF_COLORS                   = 16;
constexpr uint32 MINSPACE_FOR_COLOR_DRAWING        = 5;
constexpr uint32 MINSPACE_FOR_DROPBUTTON_DRAWING   = 3;
constexpr int32 COLOR_NAME_OFFSET                  = 3;

void ColorPickerContext::OnExpandView(Graphics::Clip& expandedClip)
{
// done
}
void ColorPickerContext::PaintHeader(int x, int y, uint32 width, Graphics::Renderer& renderer)
{
// done
}
void ColorPickerContext::PaintColorBox(Graphics::Renderer& renderer)
{
    // done
}
void ColorPickerContext::Paint(Graphics::Renderer& renderer)
{
    // done
}
uint32 ColorPickerContext::MouseToObject(int x, int y)
{
    // done
}
bool ColorPickerContext::OnMouseOver(int x, int y)
{
    auto obj = MouseToObject(x, y);
    if (obj != this->colorObject)
    {
        this->colorObject = obj;
        return true;
    }
    return false;
}
void ColorPickerContext::OnMousePressed(int x, int y, Input::MouseButton /*button*/)
{
    auto obj = MouseToObject(x, y);
    if (obj != NO_COLOR_OBJECT)
    {
        this->color = static_cast<Color>((uint8) obj);
        host->RaiseEvent(Event::ColorPickerSelectedColorChanged);
    }
}
void ColorPickerContext::NextColor(int32 offset, bool isExpanded)
{
    // done
}
bool ColorPickerContext::OnKeyEvent(Input::Key keyCode)
{
    // done
}

bool ColorPicker::OnMouseLeave()
{
    return true;
}
bool ColorPicker::OnMouseEnter()
{
    return true;
}
bool ColorPicker::OnMouseOver(int x, int y)
{
    return reinterpret_cast<ColorPickerContext*>(this->Context)->OnMouseOver(x, y);
}
void ColorPicker::OnMousePressed(int x, int y, Input::MouseButton button)
{
    reinterpret_cast<ColorPickerContext*>(this->Context)->OnMousePressed(x, y, button);
    OnHotKey();
}
*/
