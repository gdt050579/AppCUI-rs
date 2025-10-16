use super::file_mask::FileMask;
use crate::prelude::*;

#[ModalWindow(internal: true, response: String, events: ButtonEvents)]
pub(super) struct ExtensionSelectionDialog {
    combo: Handle<ComboBox>,
    btn_ok: Handle<Button>,
    btn_cancel: Handle<Button>,
}

impl ExtensionSelectionDialog {
    pub(super) fn new(file_mask: &FileMask) -> Self {
        let mut me = Self {
            base: ModalWindow::new("Select Extension", layout!("a:c,w:50,h:9"), window::Flags::NoCloseButton),
            combo: Handle::None,
            btn_ok: Handle::None,
            btn_cancel: Handle::None,
        };
        
        me.add(Label::new("Please select an extension to add to the file:", layout!("l:1,t:1,r:1,h:1")));
        
        let mut combo = ComboBox::new(layout!("l:1,t:3,r:1,h:1"), combobox::Flags::None);        
        for i in 0..file_mask.extensions_count() {
            combo.add(file_mask.extension(i));
        }
        combo.add("Do Nothing (keep the file as it is)");
        combo.set_index(0);
        
        me.combo = me.add(combo);
        
        me.btn_ok = me.add(Button::with_type("&OK", layout!("l:12,b:0,w:13"), button::Type::Normal));
        me.btn_cancel = me.add(Button::with_type("&Cancel", layout!("l:26,b:0,w:13"), button::Type::Normal));
        
        let combo_handle = me.combo;
        me.request_focus_for_control(combo_handle);
        
        me
    }
    
    fn return_result(&mut self) {
        if let Some(combo) = self.control(self.combo) {
            if let Some(index) = combo.index() {
                if index == combo.count() - 1 {
                    self.exit();
                } else {
                    self.exit_with(combo.value().to_string());
                }
            } else {
                self.exit();
            }
        } else {
            self.exit();
        }
    }
}

impl ButtonEvents for ExtensionSelectionDialog {
    fn on_pressed(&mut self, handle: Handle<Button>) -> EventProcessStatus {
        if handle == self.btn_ok {
            self.return_result();
        } else if handle == self.btn_cancel {
            self.exit();
        }
        EventProcessStatus::Processed
    }
}
