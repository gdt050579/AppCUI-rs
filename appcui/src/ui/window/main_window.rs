use self::{common::StatusFlags, keyselector::events::KeySelectorEvents};

use super::{Flags, Type, Window};
use crate::prelude::*;
use std::ops::{Deref, DerefMut};

#[repr(C)]
pub struct MainWindow {
    base: Window,
}
impl Deref for MainWindow {
    type Target = Window;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
impl DerefMut for MainWindow {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
impl MainWindow {
    pub fn new() -> Self {
        Self {
            base: Window::internal_create(
                "",
                Layout::fill(),
                Flags::NoCloseButton | Flags::NoTitle | Flags::FixedPosition,
                Some(Type::Borderless),
                window::Background::Normal,
                StatusFlags::MainWindow | StatusFlags::ThemeType,
            ),
        }
    }
}
impl Control for MainWindow {}
impl OnThemeChanged for MainWindow {}
impl GenericMenuEvents for MainWindow {}
impl AppBarEvents for MainWindow {}
impl DesktopEvents for MainWindow {}
impl ToolBarEvents for MainWindow {}
impl WindowEvents for MainWindow {}
impl GenericCommandBarEvents for MainWindow {}
impl CheckBoxEvents for MainWindow {}
impl RadioBoxEvents for MainWindow {}
impl PasswordEvents for MainWindow {}
impl KeySelectorEvents for MainWindow {}
impl TextFieldEvents for MainWindow {}
impl MarkdownComposerEvents for MainWindow {}
impl RichTextFieldEvents for MainWindow {}
impl ButtonEvents for MainWindow {}
impl HyperLinkEvents for MainWindow {}
impl ToggleButtonEvents for MainWindow {}
impl ColorPickerEvents for MainWindow {}
impl CharPickerEvents for MainWindow {}
impl ComboBoxEvents for MainWindow {}
impl DatePickerEvents for MainWindow {}
impl TimePickerEvents for MainWindow {}
impl ListBoxEvents for MainWindow {}
impl CustomEvents for MainWindow {}
impl GenericSelectorEvents for MainWindow {}
impl GenericDropDownListEvents for MainWindow {}
impl GenericNumericSelectorEvents for MainWindow {}
impl GenericHSliderEvents for MainWindow {}
impl GenericListViewEvents for MainWindow {}
impl GenericBufferViewEvents for MainWindow {}
impl GenericTreeViewEvents for MainWindow {}
impl GenericBackgroundTaskEvents for MainWindow {}
impl GenericGraphViewEvents for MainWindow {}
impl GenericVBarChartEvents for MainWindow {}
impl GenericHBarChartEvents for MainWindow {}
impl PieChartEvents for MainWindow {}
impl GenericLineChartEvents for MainWindow {}
impl UsageChartEvents for MainWindow {}
impl GenericHeatMapEvents for MainWindow {}
impl GenericVStackedBarChartEvents for MainWindow {}
impl OnDefaultAction for MainWindow {}
impl WindowControl for MainWindow {}
impl OnExpand for MainWindow {}
impl ThreeStateBoxEvents for MainWindow {}
impl OnSiblingSelected for MainWindow {}
impl PathFinderEvents for MainWindow {}
impl TimerEvents for MainWindow {}
impl MarkdownEvents for MainWindow {}
impl AccordionEvents for MainWindow {}
impl TabEvents for MainWindow {}

// events routed to base window
impl OnFocus for MainWindow {
    fn on_focus(&mut self) {
        OnFocus::on_focus(&mut self.base);
    }

    fn on_lose_focus(&mut self) {
        OnFocus::on_lose_focus(&mut self.base);
    }
}
impl OnPaint for MainWindow {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        OnPaint::on_paint(&self.base, surface, theme);
    }
}
impl OnResize for MainWindow {
    fn on_resize(&mut self, old_size: Size, new_size: Size) {
        OnResize::on_resize(&mut self.base, old_size, new_size);
    }
}
impl OnKeyPressed for MainWindow {
    fn on_key_pressed(&mut self, key: Key, character: char) -> EventProcessStatus {
        if !self.base.is_in_resize_mode() {
            match key.value() {
                key!("Enter") => {
                    if let Some(interface) = self.interface_mut() {
                        WindowEvents::on_accept(interface);
                    }
                    EventProcessStatus::Processed
                }
                key!("Escape") => {
                    if let Some(interface) = self.interface_mut() {
                        let result = WindowEvents::on_cancel(interface);
                        if result == ActionRequest::Allow {
                            // force the exit with None
                            RuntimeManager::get().exit_execution_loop();
                        }
                    }
                    EventProcessStatus::Processed
                }
                _ => OnKeyPressed::on_key_pressed(&mut self.base, key, character),
            }
        } else {
            OnKeyPressed::on_key_pressed(&mut self.base, key, character)
        }
    }
}
impl OnMouseEvent for MainWindow {
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        OnMouseEvent::on_mouse_event(&mut self.base, event)
    }
}
impl OnWindowRegistered for MainWindow {
    fn on_registered(&mut self) {
        self.base.on_registered();
    }
}
impl MainWindowControl for MainWindow {}
