use AppCUIProcMacro::AppCUIControl;

use super::WindowFlags;
use crate::controls::events::*;
use crate::controls::*;
use crate::controls::menu::MenuBar;
use crate::graphics::*;
use crate::input::*;
use crate::system::*;


#[AppCUIControl(overwrite=OnPaint)]
pub struct Window {
    title: String,
    flags: WindowFlags,
    menu: Option<MenuBar>,
    resize_move_mode: bool,
}

impl Window {
    pub fn new(title: &str, layout: Layout, flags: WindowFlags) -> Self {
        Window {
            base: ControlBase::new(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
            title: String::from(title),
            flags,
            menu: None,
            resize_move_mode: false,
        }
    }
}
impl OnPaint for Window {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        let color_window = match () {
            _ if !self.has_focus() => theme.window.inactive,
            _ if self.flags.contains(WindowFlags::WarningWindow) => theme.window.warning,
            _ if self.flags.contains(WindowFlags::ErrorWindow) => theme.window.error,
            _ if self.flags.contains(WindowFlags::NotifyWindow) => theme.window.info,
            _  => theme.window.normal,
        };
        // set some colors
        let color_title: CharAttribute;
        let color_border: CharAttribute;
        let line_type: LineType;

        // initialization
        if self.has_focus() {

        } else {
            color_title              = theme.text.normal;
            color_border             = theme.border.normal;
            line_type                = LineType::Single;
            self.resize_move_mode = false;
        }
    }
/*
    CREATE_TYPECONTROL_CONTEXT(WindowControlContext, Members, );
    ColorPair colorTitle, colorWindow, colorBorder, colorStartEndSeparators, tmpCol, tmpHK;
    LineType lineType;


    const auto sepColor = Members->Focused ? Members->Cfg->Lines.Normal : Members->Cfg->Lines.Inactive;

    if (Members->Focused)
    {
        colorTitle  = Members->Cfg->Text.Focused;
        colorBorder = Members->dragStatus == WindowDragStatus::None ? Members->Cfg->Border.Focused
                                                                    : Members->Cfg->Border.PressedOrSelected;
        lineType    = Members->dragStatus == WindowDragStatus::None ? LineType::Double : LineType::Single;
        if (Members->ResizeMoveMode)
            colorBorder = Members->Cfg->Border.PressedOrSelected;
    }
    else
    {

    }
    renderer.Clear(' ', colorWindow);
    renderer.DrawRectSize(0, 0, Members->Layout.Width, Members->Layout.Height, colorBorder, lineType);

    auto* btn = Members->ControlBar.Items;
    for (uint32 tr = 0; tr < Members->ControlBar.Count; tr++, btn++)
    {
        if ((!btn->IsVisible()) || (btn->IsHidden()))
            continue;
        bool fromLeft = (btn->Layout == WindowControlsBarLayout::TopBarFromLeft) ||
                        (btn->Layout == WindowControlsBarLayout::BottomBarFromLeft);
        bool showChecked        = false;
        colorStartEndSeparators = colorBorder;
        auto state              = ControlState::Normal;

        if (Members->ControlBar.Current == tr)
        {
            // hover or pressed
            if (Members->ControlBar.IsCurrentItemPressed)
                state = ControlState::PressedOrSelected;
            else
            {
                showChecked = ((Members->Focused) && (btn->IsChecked()));
                state       = ControlState::Hovered;
            }
        }
        else
        {
            if (Members->Focused)
            {
                showChecked = btn->IsChecked();
                state       = ControlState::Focused;
            }
            else
                state = ControlState::Inactive;
        }
        // bool hoverOrPressed = (state == ControlState::Hovered) || (state == ControlState::PressedOrSelected);
        bool drawSeparators = false;
        switch (btn->Type)
        {
        case WindowBarItemType::CloseButton:
            renderer.WriteSingleLineText(
                  btn->X, btn->Y, "[ ]", Members->GetSymbolColor(state, colorStartEndSeparators));
            renderer.WriteCharacter(
                  btn->X + 1, btn->Y, 'x', Members->GetSymbolColor(state, Members->Cfg->Symbol.Close));
            break;
        case WindowBarItemType::MaximizeRestoreButton:
            renderer.WriteSingleLineText(
                  btn->X, btn->Y, "[ ]", Members->GetSymbolColor(state, colorStartEndSeparators));
            renderer.WriteSpecialCharacter(
                  btn->X + 1,
                  btn->Y,
                  Members->Maximized ? SpecialChars::ArrowUpDown : SpecialChars::ArrowUp,
                  Members->GetSymbolColor(state, Members->Cfg->Symbol.Maximized));
            break;
        case WindowBarItemType::WindowResize:
            if (Members->Focused)
                renderer.WriteSpecialCharacter(
                      btn->X,
                      btn->Y,
                      SpecialChars::BoxBottomRightCornerSingleLine,
                      Members->GetSymbolColor(state, Members->Cfg->Symbol.Resize));
            break;
        case WindowBarItemType::HotKeY:
            renderer.WriteCharacter(btn->X, btn->Y, '[', colorStartEndSeparators);
            tmpCol = Members->Focused ? Members->Cfg->Text.Normal : Members->Cfg->Text.Inactive;
            renderer.WriteSingleLineText(btn->X + 1, btn->Y, KeyUtils::GetKeyName(Members->HotKey), tmpCol);
            renderer.WriteCharacter(btn->X + btn->Size - 1, btn->Y, ']', colorStartEndSeparators);
            break;
        case WindowBarItemType::Tag:
            renderer.WriteCharacter(btn->X, btn->Y, '[', colorStartEndSeparators);
            tmpCol = Members->Focused ? Members->Cfg->Text.Emphasized2 : Members->Cfg->Text.Inactive;
            renderer.WriteSingleLineText(btn->X + 1, btn->Y, btn->Text, tmpCol);
            renderer.WriteCharacter(btn->X + btn->Size - 1, btn->Y, ']', colorStartEndSeparators);
            break;

        case WindowBarItemType::Button:
        case WindowBarItemType::SingleChoice:
            switch (state)
            {
            case ControlState::Hovered:
                tmpCol = Members->Cfg->Button.Text.Hovered;
                tmpHK  = Members->Cfg->Button.Text.Hovered;
                break;
            case ControlState::Normal:
                tmpCol = Members->Cfg->Text.Normal;
                tmpHK  = Members->Cfg->Text.HotKey;
                break;
            case ControlState::Focused:
                tmpCol = Members->Cfg->Text.Normal;
                tmpHK  = Members->Cfg->Text.HotKey;
                break;
            case ControlState::PressedOrSelected:
                tmpCol = Members->Cfg->Button.Text.PressedOrSelected;
                tmpHK  = Members->Cfg->Button.Text.PressedOrSelected;
                break;
            default:
                tmpHK = tmpCol = Members->Cfg->Text.Inactive;
                break;
            }
            if (showChecked)
                renderer.WriteSingleLineText(
                      btn->X,
                      btn->Y,
                      btn->Text,
                      Members->Cfg->Button.Text.PressedOrSelected,
                      Members->Cfg->Button.HotKey.PressedOrSelected,
                      btn->HotKeyOffset);
            else
                renderer.WriteSingleLineText(btn->X, btn->Y, btn->Text, tmpCol, tmpHK, btn->HotKeyOffset);
            drawSeparators = true;
            break;
        case WindowBarItemType::CheckBox:
            switch (state)
            {
            case ControlState::Hovered:
                tmpCol = Members->Cfg->Button.Text.Hovered;
                tmpHK  = Members->Cfg->Button.Text.Hovered;
                break;
            case ControlState::Normal:
                tmpCol = Members->Cfg->Text.Normal;
                tmpHK  = Members->Cfg->Text.HotKey;
                break;
            case ControlState::Focused:
                tmpCol = Members->Cfg->Text.Normal;
                tmpHK  = Members->Cfg->Text.HotKey;
                break;
            case ControlState::PressedOrSelected:
                tmpCol = Members->Cfg->Button.Text.PressedOrSelected;
                tmpHK  = Members->Cfg->Button.Text.PressedOrSelected;
                break;
            default:
                tmpHK = tmpCol = Members->Cfg->Text.Inactive;
                break;
            }
            renderer.FillHorizontalLine(btn->X, btn->Y, btn->X + 1, ' ', tmpCol);
            renderer.WriteSingleLineText(btn->X + 2, btn->Y, btn->Text, tmpCol, tmpHK, btn->HotKeyOffset);
            if (btn->IsChecked())
            {
                // tmpCol = (Members->Focused && (!hoverOrPressed)) ? wcfg->ControlBar.CheckMark : c_i->Text;
                tmpCol = Members->GetSymbolColor(state, Members->Cfg->Symbol.Checked);
                renderer.WriteSpecialCharacter(btn->X, btn->Y, SpecialChars::CheckMark, tmpCol);
            }
            drawSeparators = true;
            break;
        case WindowBarItemType::Text:
            tmpCol = Members->Focused ? Members->Cfg->Text.Normal : Members->Cfg->Text.Inactive;
            renderer.WriteSingleLineText(btn->X, btn->Y, btn->Text, tmpCol);
            drawSeparators = true;
            break;
        }
        // separators
        if (drawSeparators)
        {
            if ((uint8) btn->Flags & (uint8) WindowBarItemFlags::LeftGroupMarker)
                renderer.WriteCharacter(btn->X - 1, btn->Y, '[', colorStartEndSeparators);
            else if (fromLeft)
                renderer.WriteCharacter(btn->X - 1, btn->Y, '|', sepColor);
            if ((uint8) btn->Flags & (uint8) WindowBarItemFlags::RightGroupMarker)
                renderer.WriteCharacter(btn->X + btn->Size, btn->Y, ']', colorStartEndSeparators);
            else if (!fromLeft)
                renderer.WriteCharacter(btn->X + btn->Size, btn->Y, '|', sepColor);
        }
    }

    // Title
    if (Members->TitleMaxWidth >= 2)
    {
        WriteTextParams params(
              WriteTextFlags::SingleLine | WriteTextFlags::ClipToWidth | WriteTextFlags::FitTextToWidth |
                    WriteTextFlags::OverwriteColors | WriteTextFlags::LeftMargin | WriteTextFlags::RightMargin,
              TextAlignament::Center);
        params.X     = Members->TitleLeftMargin;
        params.Y     = 0;
        params.Color = colorTitle;
        params.Width = Members->TitleMaxWidth;
        renderer.WriteText(Members->Text, params);
    }
    // menu
    if (Members->menu)
        Members->menu->Paint(renderer);


 */
}