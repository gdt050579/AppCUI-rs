use super::{CommandBar, InitializationData, InitializationFlags, Theme, ToolTip};
use crate::controls::control_manager::ParentLayout;
use crate::controls::events::Control;
use crate::controls::menu::{Menu, MenuBar};
use crate::controls::ControlManager;
use crate::controls::*;
use crate::graphics::{Rect, Size, Surface};
use crate::terminal::*;
use crate::utils::Caption;

pub(crate) struct RuntimeManager {
    theme: Theme,
    terminal: Box<dyn Terminal>,
    surface: Surface,
    root: ControlManager,
    tooltip: ToolTip,
    commandbar: Option<CommandBar>,
    menubar: Option<MenuBar>,
    recompute_layout: bool,
    repaint: bool,
}

static mut RUNTIME_MANAGER: Option<RuntimeManager> = None;

impl RuntimeManager {
    pub(super) fn create(data: InitializationData) -> Result<(), super::Error> {
        let term = TerminalType::new(&data)?;
        let width = term.get_width();
        let height = term.get_height();
        let surface = Surface::new(width, height);
        let manager = RuntimeManager {
            theme: Theme::new(),
            terminal: term,
            surface: surface,
            root: ControlManager::new(Desktop::new()),
            tooltip: ToolTip::new(),
            recompute_layout: true,
            repaint: true,
            commandbar: if data.flags.contains(InitializationFlags::CommandBar) {
                Some(CommandBar::new(width, height))
            } else {
                None
            },
            menubar: if data.flags.contains(InitializationFlags::Menu) {
                None
            } else {
                None
            },
        };
        unsafe {
            RUNTIME_MANAGER = Some(manager);
        }
        Ok(())
    }
    pub(crate) fn get() -> &'static mut RuntimeManager {
        unsafe { RUNTIME_MANAGER.as_mut().unwrap() }
    }
    pub(crate) fn get_terminal_size(&self) -> Size {
        Size {
            width: self.terminal.get_width(),
            height: self.terminal.get_height(),
        }
    }
    pub(crate) fn get_desktop_rect(&self) -> Rect {
        Rect::new(
            0,
            if self.menubar.is_some() { 1 } else { 0 },
            (self.terminal.get_width() as i32) - 1,
            if self.commandbar.is_some() {
                (self.terminal.get_height() as i32) - 2
            } else {
                (self.terminal.get_height() as i32) - 1
            },
        )
    }
    pub(crate) fn request_repaint(&mut self) {
        self.repaint = true;
    }
    pub(crate) fn request_recompute_layout(&mut self) {
        self.recompute_layout = true;
    }
    pub(crate) fn show_tooltip(&mut self, txt: &str, rect: &Rect) {
        self.tooltip.show(
            txt,
            &rect,
            self.terminal.get_width(),
            self.terminal.get_height(),
            &self.theme,
        );
    }
    pub(crate) fn hide_tooltip(&mut self) {
        self.tooltip.hide();
    }
    pub(crate) fn add<T>(&mut self, obj: T) -> ControlHandle<T>
    where
        T: Control + 'static,
    {
        let c = ControlManager::new(obj);
        let v = c.get_version();
        self.root.get_base_mut().children.push(c);
        return ControlHandle::new(0, v);
    }
    pub(crate) fn add_menu(&mut self, menu: Menu, caption: Caption) {
        if self.menubar.is_some() {
            self.menubar.as_mut().unwrap().add(menu, caption);
        }
    }
    pub(crate) fn run(&mut self) {
        // must pe self so that after a run a second call will not be possible
        if self.recompute_layout {
            self.recompute_layouts();
        }
        if self.repaint | self.recompute_layout {
            self.paint();
        }
        self.recompute_layout = false;
        self.repaint = false;
        let sys_event = self.terminal.get_system_event();
        match sys_event {
            SystemEvent::None => {}
            SystemEvent::AppClose => todo!(),
            SystemEvent::KeyPressed(event) => self.process_keypressed_event(event),
            SystemEvent::KeyModifierChanged(_) => todo!(),
            SystemEvent::Resize(new_size) => self.process_terminal_resize_event(new_size),
            SystemEvent::MouseButtonDown(_) => todo!(),
            SystemEvent::MouseButtonUp(_) => todo!(),
            SystemEvent::MouseDoubleClick(_) => todo!(),
            SystemEvent::MouseMove(_) => todo!(),
            SystemEvent::MouseWheel(_) => todo!(),
        }
    }

    fn recompute_layouts(&mut self) {
        let term_layout = ParentLayout::from(&self.terminal);
        self.root.update_layout(&term_layout);
    }

    fn paint(&mut self) {
        self.root.paint(&mut self.surface, &self.theme);
        self.surface.reset();
        if self.commandbar.is_some() {
            self.commandbar
                .as_ref()
                .unwrap()
                .paint(&mut self.surface, &self.theme);
        }
        if self.menubar.is_some() {
            self.menubar
                .as_ref()
                .unwrap()
                .paint(&mut self.surface, &self.theme);
        }
        if self.tooltip.is_visible() {
            self.tooltip.paint(&mut self.surface, &self.theme);
        }
        self.terminal.update_screen(&self.surface);
    }

    fn process_keypressed_event(&mut self, event: KeyPressedEvent) {
        self.root
            .process_keypressed_event(event.key, event.character);
    }
    fn process_terminal_resize_event(&mut self, new_size: Size) {
        // sanity checks
        if (new_size.width == 0) || (new_size.height == 0) {
            return;
        }
        if (new_size.width == self.surface.get_width())
            && (new_size.height == self.surface.get_height())
        {
            return;
        }
        self.surface.resize(new_size);
        self.terminal.on_resize(new_size);
        if let Some(commandbar) = self.commandbar.as_mut() {
            commandbar.set_desktop_size(new_size);
        }
        if let Some(menubar) = self.menubar.as_mut() {
            menubar.set_position(0, 0, new_size.width);
        }
        self.recompute_layout = true;
    }
}
