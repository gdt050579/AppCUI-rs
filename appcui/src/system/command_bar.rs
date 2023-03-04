use crate::{
    graphics::{CharAttribute, CharFlags, Character, Surface},
    input::{Key, KeyCode, KeyModifier, MouseEvent}, controls::events::EventProcessStatus,
};

use super::Theme;

const MAX_KEYS: usize = 64; // no bigger than 255
const MAX_SHIFT_STATES: usize = 8;
const INVALID_INDEX: u32 = 0xFFFFFFFF;

#[derive(Default)]
struct Item {
    text: String,
    key: &'static str,
    left: i32,
    right: i32,
    command: u32,
    version: u32,
    size: u16,
}
pub struct CommandBar {
    width: u32,
    y: i32,
    version: u32,
    modifier: KeyModifier,
    items: Vec<Item>,
    indexes: [Vec<u32>; MAX_SHIFT_STATES],
    has_shifts: [bool; MAX_SHIFT_STATES],
    hovered_index: u32,
    pressed_index: u32,
}

impl CommandBar {
    pub(crate) fn new(width: u32, height: u32) -> Self {
        let mut obj = Self {
            width,
            y: (height as i32) - 1,
            version: 1,
            items: Vec::with_capacity(MAX_KEYS * MAX_SHIFT_STATES),
            indexes: Default::default(),
            has_shifts: [false; MAX_SHIFT_STATES],
            modifier: KeyModifier::None,
            hovered_index: INVALID_INDEX,
            pressed_index: INVALID_INDEX,
        };
        for vec in &mut obj.indexes {
            vec.reserve(MAX_KEYS);
        }
        for _ in 0..(MAX_KEYS * MAX_SHIFT_STATES) {
            obj.items.push(Item {
                text: String::new(),
                key: "",
                left: -1,
                right: -1,
                command: 0,
                version: 0,
                size: 0,
            });
        }
        obj
    }
    pub(crate) fn set_desktop_size(&mut self, width: u32, height: u32) {
        self.width = width;
        self.y = (height as i32) - 1;
        self.update_positions();
    }

    pub(crate) fn set_key_modifier(&mut self, modifier: KeyModifier) {
        if modifier != self.modifier {
            self.modifier = modifier;
            self.hovered_index = INVALID_INDEX;
            self.pressed_index = INVALID_INDEX;
        }
    }

    pub(crate) fn clear(&mut self) {
        self.version += 1;
        for has_shift in &mut self.has_shifts {
            *has_shift = false;
        }
        for vec in &mut self.indexes {
            vec.clear();
        }
        self.hovered_index = INVALID_INDEX;
        self.pressed_index = INVALID_INDEX;
    }

    pub fn set(&mut self, key: Key, text: &str, command: u32) -> bool {
        if key.code == KeyCode::None {
            return false;
        }
        let key_index = (key.code as u8) as usize;
        if key_index >= MAX_KEYS {
            return false;
        }
        let shift_state = key.modifier.get_value() as usize;
        if shift_state >= MAX_SHIFT_STATES {
            return false;
        }
        let item = &mut self.items[shift_state * MAX_KEYS + key_index];

        item.text.clear();
        item.text.push_str(text);
        item.text.push(' '); // one extra space
        item.command = command;
        item.left = -1;
        item.right = -1;
        item.key = key.code.get_name_padded();
        item.version = self.version;
        item.size = (item.key.len() + item.text.chars().count()) as u16;

        self.has_shifts[shift_state] = true;

        true
    }

    pub(crate) fn update_positions(&mut self) {
        // recompute all positions regardless of the shift state
        for shift_state in 0..MAX_SHIFT_STATES {
            let vidx = &mut self.indexes[shift_state];
            vidx.clear();
            if self.has_shifts[shift_state] == false {
                continue;
            }
            let start_index = MAX_KEYS * shift_state;
            let end_index = start_index + MAX_KEYS;
            let mut x = if shift_state == 0 {
                0
            } else {
                KeyModifier::get_name_from_index(shift_state).len() as i32
            };
            for idx in start_index..end_index {
                let item = &mut self.items[idx];
                if item.version != self.version {
                    continue;
                }
                vidx.push(idx as u32);
                item.left = x;
                item.right = x + item.size as i32;
                x = item.right + 1;
                if x > (self.width as i32) {
                    break;
                }
            }
        }
    }

    pub(crate) fn paint(&self, surface: &mut Surface, theme: &Theme) {
        surface.fill_horizontal_line(
            0,
            self.y,
            self.width as i32,
            Character::with_attributes(' ', theme.menu.text.normal),
        );
        let modifier_name = self.modifier.get_name();
        if modifier_name.len() > 0 {
            surface.write_string(0, self.y, modifier_name, theme.menu.text.inactive, false);
        }
        let shift_idx = self.modifier.get_value() as usize;
        if (shift_idx >= MAX_SHIFT_STATES) || (self.has_shifts[shift_idx] == false) {
            return;
        }
        for idx in &self.indexes[shift_idx] {
            let item = &self.items[(*idx) as usize];

            // write the key
            let col_key = match () {
                _ if (*idx) == self.pressed_index => theme.menu.shortcut.pressed_or_selectd,
                _ if (*idx) == self.hovered_index => theme.menu.shortcut.hovered,
                _ => theme.menu.shortcut.normal,
            };
            surface.write_string(item.left, self.y, item.key, col_key, false);

            // write the text
            let col_text = match () {
                _ if (*idx) == self.pressed_index => theme.menu.text.pressed_or_selectd,
                _ if (*idx) == self.hovered_index => theme.menu.text.hovered,
                _ => theme.menu.text.normal,
            };
            surface.write_string(
                item.left + (item.key.len() as i32),
                self.y,
                &item.text,
                col_text,
                false,
            );
        }
    }

    pub(crate) fn on_mouse_event(&mut self, event: &MouseEvent)->EventProcessStatus {
        EventProcessStatus::Ignored
    }
}

/*

    class CommandBarController
    {
        CommandBarField Fields[MAX_COMMANDBAR_SHIFTSTATES][(uint32) Input::Key::Count];
        CommandBarFieldIndex VisibleFields[MAX_COMMANDBAR_SHIFTSTATES][(uint32) Input::Key::Count];
        int IndexesCount[MAX_COMMANDBAR_SHIFTSTATES];
        bool HasKeys[MAX_COMMANDBAR_SHIFTSTATES];

        struct
        {
            int Y, Width;
        } BarLayout;

        string_view ShiftStatus;

        Application::Config* Cfg;
        Input::Key CurrentShiftKey;
        int LastCommand;
        CommandBarField* PressedField;
        CommandBarField* HoveredField;
        uint32 ClearCommandUniqueID;
        bool RecomputeScreenPos;

        void ComputeScreenPos();
        bool CleanFieldStatus();
        CommandBarField* MousePositionToField(int x, int y);

      public:
        CommandBarController(uint32 desktopWidth, uint32 desktopHeight, Application::Config* cfg);
        void Paint(Graphics::Renderer& renderer);
        void Clear();
        bool Set(Input::Key keyCode, const ConstString& caption, int Command);
        bool SetShiftKey(Input::Key keyCode);
        bool OnMouseMove(int x, int y, bool& repaint);
        bool OnMouseDown();
        bool OnMouseUp(int& command);
        int GetCommandForKey(Input::Key keyCode);
    };



#include "Internal.hpp"

namespace AppCUI::Internal
{
using namespace Input;

CommandBarController::CommandBarController(
      uint32 desktopWidth, uint32 desktopHeight, Application::Config* cfg)
{
    this->Cfg = cfg;
    SetDesktopSize(desktopWidth, desktopHeight);
    ClearCommandUniqueID = 0;
    for (uint32 tr = 0; tr < MAX_COMMANDBAR_SHIFTSTATES; tr++)
    {
        CommandBarField* b = &Fields[tr][0];
        CommandBarField* e = b + (uint32) Key::Count;
        while (b < e)
        {
            b->ClearCommandUniqueID = ClearCommandUniqueID;
            b++;
        }
    }
    CurrentShiftKey = Input::Key::None;
    PressedField    = nullptr;
    HoveredField    = nullptr;
    LastCommand     = 0;
    ShiftStatus     = string_view("", 0);
    Clear();
}
void CommandBarController::SetDesktopSize(uint32 desktopWidth, uint32 desktopHeight)
{
    this->RecomputeScreenPos = true;
}

bool CommandBarController::Set(Input::Key keyCode, const ConstString& caption, int Command)
{

    RecomputeScreenPos      = true;
    return true;
}
void CommandBarController::Paint(Graphics::Renderer& renderer)
{

}
void CommandBarController::ComputeScreenPos()
{

    this->HoveredField = nullptr;
    this->PressedField = nullptr;
    RecomputeScreenPos = false;
}
bool CommandBarController::SetShiftKey(Input::Key keyCode)
{
    if (keyCode != CurrentShiftKey)
    {
        CurrentShiftKey = keyCode;
        ComputeScreenPos();
        return true;
    }
    return false;
}
CommandBarField* CommandBarController::MousePositionToField(int x, int y)
{
    if (RecomputeScreenPos)
        ComputeScreenPos();
    uint32 shift = ((uint32) CurrentShiftKey) >> ((uint32) Utils::KeyUtils::KEY_SHIFT_BITS);
    CHECK(shift < MAX_COMMANDBAR_SHIFTSTATES, nullptr, "");
    if (HasKeys[shift] == false)
        return nullptr;
    if (y < this->BarLayout.Y)
        return nullptr;
    CommandBarFieldIndex* bi = &VisibleFields[shift][0];
    CommandBarFieldIndex* ei = bi + IndexesCount[shift];
    while (bi < ei)
    {
        if ((x >= bi->Field->StartScreenPos) && (x < bi->Field->EndScreenPos))
            return (bi->Field);
        bi++;
    }
    return nullptr;
}
bool CommandBarController::CleanFieldStatus()
{
    if ((this->HoveredField) || (this->PressedField))
    {
        this->HoveredField = nullptr;
        this->PressedField = nullptr;
        return true;
    }
    return false;
}
bool CommandBarController::OnMouseMove(int x, int y, bool& repaint)
{
    repaint = false;
    if (y < this->BarLayout.Y)
    {
        repaint = CleanFieldStatus();
        return false; // sunt in afara lui
    }
    if (this->HoveredField)
    {
        // cached position
        if ((x >= this->HoveredField->StartScreenPos) && (x < this->HoveredField->EndScreenPos))
            return true;
    }
    CommandBarField* field = MousePositionToField(x, y);
    if (field != this->HoveredField)
    {
        this->HoveredField = field;
        repaint            = true;
    }
    return true;
}
bool CommandBarController::OnMouseDown()
{
    if (this->HoveredField)
    {
        this->PressedField = this->HoveredField;
        return true;
    }
    return false;
}
bool CommandBarController::OnMouseUp(int& command)
{
    if (this->PressedField)
    {
        command = this->PressedField->Command;
        if (command < 0)
            command = -1;
        this->PressedField = nullptr;
        return true;
    }
    command = -1;
    return false;
}
int CommandBarController::GetCommandForKey(Input::Key keyCode)
{
    uint32 index = (((uint32) keyCode) & 0xFF);
    uint32 shift = (((uint32) keyCode) >> Utils::KeyUtils::KEY_SHIFT_BITS);
    CHECK(index < (uint32) Input::Key::Count, -1, "Invalid key code !");
    CHECK((shift < MAX_COMMANDBAR_SHIFTSTATES), -1, "Invalid shift combination !");
    CommandBarField* b = &Fields[shift][index];
    // if ClearCommandUniqueID is not thee same as the current one, then its an old item and we discard it
    if (b->ClearCommandUniqueID != ClearCommandUniqueID)
        return -1;
    return b->Command;
}
} // namespace AppCUI::Internal


*/
