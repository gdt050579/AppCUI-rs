use AppCUIProcMacro::AppCUIControl;

use super::DragStatus;
use super::WindowFlags;
use super::bar_item::BarItem;
use crate::controls::events::*;
use crate::controls::menu::MenuBar;
use crate::controls::*;
use crate::graphics::*;
use crate::input::*;
use crate::system::*;
use crate::utils::VectorIndex;

#[AppCUIControl(overwrite=OnPaint)]
pub struct Window {
    title: String,
    flags: WindowFlags,
    menu: Option<MenuBar>,
    bar_items: Vec<BarItem>,
    current_bar_item: VectorIndex,
    resize_move_mode: bool,
    drag_status: DragStatus,
    title_max_width: u16,
    title_left_margin: i32,
}

impl Window {
    pub fn new(title: &str, layout: Layout, flags: WindowFlags) -> Self {
        Window {
            base: ControlBase::new(
                layout,
                StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput,
            ),
            title: String::from(title),
            flags,
            menu: None,
            resize_move_mode: false,
            drag_status: DragStatus::None,
            title_max_width: 0,
            title_left_margin: 0,
            bar_items: Vec::with_capacity(4),
            current_bar_item: VectorIndex::invalid(),
        }
    }
    pub fn add<T>(&mut self, control: T)
    where
        T: Control + 'static,
    {
        // GDT: only for test --> not the final implementation
        let c = ControlManager::new(control);
        //let v = c.get_version();
        self.children.push(c);
    }
}
impl OnPaint for Window {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        let color_window = match () {
            _ if !self.has_focus() => theme.window.inactive,
            _ if self.flags.contains(WindowFlags::WarningWindow) => theme.window.warning,
            _ if self.flags.contains(WindowFlags::ErrorWindow) => theme.window.error,
            _ if self.flags.contains(WindowFlags::NotifyWindow) => theme.window.info,
            _ => theme.window.normal,
        };
        // set some colors
        let color_title: CharAttribute;
        let color_border: CharAttribute;
        let color_sep: CharAttribute;
        let line_type: LineType;

        // initialization
        if self.has_focus() {
            color_title = theme.text.focused;
            color_sep = theme.lines.normal;
            color_border = match self.drag_status {
                DragStatus::None => theme.border.focused,
                _ => theme.border.pressed_or_selectd
            };
            line_type = match self.drag_status {
                DragStatus::None => LineType::Double,
                _ => LineType::Single,
            };
        } else {
            color_title = theme.text.normal;
            color_sep = theme.lines.inactive;
            color_border = theme.border.normal;
            line_type = LineType::Single;
        }
    
        surface.clear(Character::with_attributes(' ', color_window));
        surface.draw_rect(Rect::with_size(0, 0, self.get_width(), self.get_height()), line_type, color_border);

        // paint bar items
        for (index,item) in self.bar_items.iter().enumerate() {
            item.paint(surface, theme);
        }

        // paint title
        if self.title_max_width >= 2 {
            let mut format = TextFormat::single_line(self.title_left_margin, 0, color_title, TextAlignament::Center);
            format.width = Some(self.title_max_width);
            surface.write_text(self.title.as_str(), &format);
        }
        // paint the menu
        if self.menu.is_some() {
            self.menu.as_ref().unwrap().paint(surface, theme);
        }
    }
}
