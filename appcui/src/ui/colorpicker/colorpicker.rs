use crate::prelude::*;
use crate::ui::colorpicker::events::EventData;

const MINSPACE_FOR_COLOR_DRAWING: u32 = 5;
const MIN_WIDTH_FOR_COLOR_NAME: u32 = 8;
const MINSPACE_FOR_DROPBUTTON_DRAWING: u32 = 3;

#[CustomControl(overwrite=OnPaint+OnDefaultAction+OnKeyPressed+OnMouseEvent, internal=true)]
pub struct ColorPicker {
    color: Color,
    header_y_ofs: i32,
}
impl ColorPicker {
    pub fn new(color: Color, layout: Layout) -> Self {
        let mut cp = ColorPicker {
            base: ControlBase::new(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
            header_y_ofs: 0,
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
}
impl OnPaint for ColorPicker {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        // first paint the header
        let col_text = match () {
            _ if !self.is_enabled() => theme.button.text.inactive,
            _ if self.has_focus() => theme.button.text.focused,
            _ if self.is_mouse_over() => theme.button.text.hovered,
            _ => theme.button.text.normal,
        };
        let size = self.get_size();
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
    }
}
impl OnDefaultAction for ColorPicker {
    fn on_default_action(&mut self) {}
}
impl OnKeyPressed for ColorPicker {
    fn on_key_pressed(&mut self, _key: Key, _character: char) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
impl OnMouseEvent for ColorPicker {
    fn on_mouse_event(&mut self, _event: &MouseEvent) -> EventProcessStatus {
        EventProcessStatus::Ignored
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

constexpr static Color reverse_color[] = {
    Color::White, Color::White, Color::White, Color::White, Color::White, Color::White, Color::White, Color::Black,
    Color::Black, Color::White, Color::Black, Color::Black, Color::White, Color::White, Color::Black, Color::Black,
};
void ColorPickerContext::OnExpandView(Graphics::Clip& expandedClip)
{
    Size size;
    if (!AppCUI::Application::GetApplicationSize(size))
        return;
    expandedClip.ClipRect.Height = COLORPICEKR_HEIGHT;
    this->headerYOffset          = 0;
    this->yOffset                = 1;
    this->colorObject            = NO_COLOR_OBJECT;
    if (expandedClip.ScreenPosition.Y + COLORPICEKR_HEIGHT >= (int32) size.Height)
    {
        this->headerYOffset = COLORPICEKR_HEIGHT - 1;
        this->yOffset       = 0;
        expandedClip.ScreenPosition.Y -= this->headerYOffset;
        expandedClip.ClipRect.Y = expandedClip.ScreenPosition.Y;
    }
}
void ColorPickerContext::PaintHeader(int x, int y, uint32 width, Graphics::Renderer& renderer)
{
    auto cbc = this->Cfg->Button.Text.GetColor(this->GetControlState(ControlStateFlags::ProcessHoverStatus));

    if (width > MINSPACE_FOR_COLOR_DRAWING)
    {
        renderer.FillHorizontalLine(x, y, x + (int) width - (int) (MINSPACE_FOR_COLOR_DRAWING), ' ', cbc);
        renderer.WriteSingleLineText(
              x + COLOR_NAME_OFFSET,
              y,
              width - (int) (MINSPACE_FOR_COLOR_DRAWING - 1),
              ColorUtils::GetColorName(this->color),
              cbc);
        renderer.WriteSpecialCharacter(
              x + 1, y, SpecialChars::BlockCentered, ColorPair{ this->color, Color::Transparent });
    }
    if (width >= MINSPACE_FOR_DROPBUTTON_DRAWING)
    {
        renderer.WriteSingleLineText(x + (int) width - (int32) MINSPACE_FOR_DROPBUTTON_DRAWING, y, "   ", cbc);
        renderer.WriteSpecialCharacter(x + (int) width - 2, y, SpecialChars::TriangleDown, cbc);
    }
}
void ColorPickerContext::PaintColorBox(Graphics::Renderer& renderer)
{
    const auto col = Cfg->Menu.Text.Normal;
    renderer.FillRect(0, this->yOffset, this->Layout.Width - 1, this->yOffset + COLORPICEKR_HEIGHT - 2, ' ', col);
    // draw colors (4x4 matrix)
    for (auto y = 0U; y < COLOR_MATRIX_HEIGHT; y++)
    {
        for (auto x = 0U; x < COLOR_MATRIX_WIDTH; x++)
        {
            auto c = static_cast<Color>(y * COLOR_MATRIX_WIDTH + x);
            renderer.FillHorizontalLineSize(
                  x * SPACES_PER_COLOR + 1, y + 1 + this->yOffset, SPACES_PER_COLOR, ' ', ColorPair{ Color::Black, c });
            if (c == color)
            {
                auto c2 = reverse_color[y * COLOR_MATRIX_WIDTH + x];
                renderer.WriteSpecialCharacter(
                      x * SPACES_PER_COLOR + ((SPACES_PER_COLOR + 1) >> 1),
                      y + 1 + this->yOffset,
                      SpecialChars::CheckMark,
                      ColorPair{ c2, c });
            }
            if (y * COLOR_MATRIX_WIDTH + x == colorObject)
            {
                auto c2 = reverse_color[y * COLOR_MATRIX_WIDTH + x];
                renderer.WriteSpecialCharacter(
                      x * SPACES_PER_COLOR + 1, y + 1 + this->yOffset, SpecialChars::TriangleRight, ColorPair{ c2, c });
                renderer.WriteSpecialCharacter(
                      x * SPACES_PER_COLOR + SPACES_PER_COLOR,
                      y + 1 + this->yOffset,
                      SpecialChars::TriangleLeft,
                      ColorPair{ c2, c });
                renderer.SetCursor(x * SPACES_PER_COLOR + ((SPACES_PER_COLOR + 1) >> 1), y + 1 + this->yOffset);
            }
        }
    }
    if (colorObject == (uint32) Color::Transparent)
    {
        renderer.WriteSingleLineText(
              TRANSPARENT_CHECKBOX_X_OFFSET, 1 + this->yOffset, "[ ] Transparent", Cfg->Menu.Text.PressedOrSelected);
        if (color == Color::Transparent)
            renderer.WriteSpecialCharacter(
                  TRANSPARENT_CHECKBOX_X_OFFSET + 1,
                  1 + this->yOffset,
                  SpecialChars::CheckMark,
                  Cfg->Menu.Symbol.PressedOrSelected);
        renderer.SetCursor(TRANSPARENT_CHECKBOX_X_OFFSET + 1, 1 + this->yOffset);
    }
    else
    {
        renderer.WriteSingleLineText(TRANSPARENT_CHECKBOX_X_OFFSET, 1 + this->yOffset, "[ ] Transparent", col);
        if (color == Color::Transparent)
            renderer.WriteSpecialCharacter(
                  TRANSPARENT_CHECKBOX_X_OFFSET + 1,
                  1 + this->yOffset,
                  SpecialChars::CheckMark,
                  Cfg->Menu.Symbol.Normal);
    }


    renderer.DrawVerticalLine(
          SPACES_PER_COLOR * COLOR_MATRIX_WIDTH + 1, 1 + this->yOffset, COLOR_MATRIX_HEIGHT + this->yOffset, col, true);
    renderer.DrawRect(
          0, this->yOffset, this->Layout.Width - 1, this->yOffset + COLORPICEKR_HEIGHT - 2, col, LineType::Single);
}
void ColorPickerContext::Paint(Graphics::Renderer& renderer)
{
    PaintHeader(0, this->headerYOffset, this->Layout.Width, renderer);
    if (this->Flags & GATTR_EXPANDED)
        PaintColorBox(renderer);
}
uint32 ColorPickerContext::MouseToObject(int x, int y)
{
    if (!(this->Flags & GATTR_EXPANDED))
        return NO_COLOR_OBJECT;
    if ((x > 0) && (x < SPACES_PER_COLOR * COLOR_MATRIX_WIDTH + 1) && (y > this->yOffset) &&
        (y < this->yOffset + COLOR_MATRIX_HEIGHT + 1))
        return (((x - 1) / SPACES_PER_COLOR) + (y - (this->yOffset + 1)) * COLOR_MATRIX_WIDTH);
    if ((y == 1 + this->yOffset) && (x >= TRANSPARENT_CHECKBOX_X_OFFSET) && (x <= TRANSPARENT_CHECKBOX_X_LAST_OFFSET))
        return (uint32) (Color::Transparent);
    return NO_COLOR_OBJECT;
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
    if (colorObject == NO_COLOR_OBJECT)
        colorObject = (uint32) Color::Black;

    if (isExpanded)
    {
        auto result = (int32) colorObject + offset;
        // specific cases
        // when the cursor is on the first line (the first 4 colors), it should be able to move to transparent checkbox
        // as well the logic below enphasize this
        if ((result == COLOR_MATRIX_WIDTH) && (offset == ONE_POSITION_TO_RIGHT))
            result = static_cast<int32>(Color::Transparent); // Move to the right with 1 position
        else if ((result == static_cast<int32>(Color::Transparent) + 1) && (offset == ONE_POSITION_TO_RIGHT))
            result = 0;
        else if ((result == -1) && (offset == ONE_POSITION_TO_LEFT))
            result = static_cast<int32>(Color::Transparent);
        else if ((result == static_cast<int32>(Color::Transparent) - 1) && (offset == ONE_POSITION_TO_LEFT))
            result = COLOR_MATRIX_WIDTH - 1;
        else
        {
            if (result < 0)
                result += NUMBER_OF_COLORS;
            if (result >= NUMBER_OF_COLORS)
                result -= NUMBER_OF_COLORS;
        }
        colorObject = (uint32) result;
    }
    else
    {
        auto result = (int32) this->color + offset;
        if (result < 0)
            result = 0;
        if (result >= NUMBER_OF_COLORS)
            result = NUMBER_OF_COLORS;
        color = static_cast<Color>((uint8) result);
        host->RaiseEvent(Event::ColorPickerSelectedColorChanged);
    }
}
bool ColorPickerContext::OnKeyEvent(Input::Key keyCode)
{
    bool isExpanded = (this->Flags & GATTR_EXPANDED) != 0;
    switch (keyCode)
    {
    case Key::Space:
    case Key::Enter:
        if ((isExpanded) && (colorObject != NO_COLOR_OBJECT))
        {
            this->color = static_cast<Color>((uint8) colorObject);
            host->RaiseEvent(Event::ColorPickerSelectedColorChanged);
        }
        return true;
    case Key::Up:
        NextColor(isExpanded ? -(COLOR_MATRIX_WIDTH) : -1, isExpanded);
        return true;
    case Key::Down:
        NextColor(isExpanded ? COLOR_MATRIX_WIDTH : 1, isExpanded);
        return true;
    case Key::Left:
        NextColor(-1, isExpanded);
        return true;
    case Key::Right:
        NextColor(1, isExpanded);
        return true;
    }
    return false;
}

ColorPicker::ColorPicker(string_view layout, Graphics::Color _color)
    : Control(new ColorPickerContext(), "", layout, false)
{
    auto Members              = reinterpret_cast<ColorPickerContext*>(this->Context);
    Members->Layout.MinWidth  = 7;
    Members->Layout.MinHeight = 1;
    Members->Layout.MaxHeight = 1;
    Members->Flags            = GATTR_ENABLE | GATTR_VISIBLE | GATTR_TABSTOP;
    Members->color            = _color;
    Members->headerYOffset    = 0;
    Members->yOffset          = 1;
    Members->colorObject      = NO_COLOR_OBJECT;
    Members->host             = this;
}
ColorPicker::~ColorPicker()
{
}
void ColorPicker::Paint(Graphics::Renderer& renderer)
{
    reinterpret_cast<ColorPickerContext*>(this->Context)->Paint(renderer);
}
bool ColorPicker::OnKeyEvent(Input::Key keyCode, char16 /*UnicodeChar*/)
{
    bool result = reinterpret_cast<ColorPickerContext*>(this->Context)->OnKeyEvent(keyCode);
    switch (keyCode)
    {
    case Key::Space:
    case Key::Enter:
        OnHotKey();
        return true;
    }
    return result;
}
void ColorPicker::OnHotKey()
{
    SetChecked(!IsChecked());
    if (IsChecked())
        this->ExpandView();
    else
    {
        this->PackView();
        RaiseEvent(Event::ColorPickerClosed);
    }
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
void ColorPicker::OnExpandView(Graphics::Clip& expandedClip)
{
    reinterpret_cast<ColorPickerContext*>(this->Context)->OnExpandView(expandedClip);
}
void ColorPicker::OnPackView()
{
    reinterpret_cast<ColorPickerContext*>(this->Context)->headerYOffset = 0; // reset position
}
void ColorPicker::SetColor(Graphics::Color color)
{
    reinterpret_cast<ColorPickerContext*>(this->Context)->color = color;
}
Graphics::Color ColorPicker::GetColor()
{
    return reinterpret_cast<ColorPickerContext*>(this->Context)->color;
}
} // namespace AppCUI


*/
