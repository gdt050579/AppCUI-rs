use super::DialogButtons;
use super::DialogResult;
use crate::prelude::*;

#[ModalWindow(events = ButtonEvents, internal: true, response: DialogResult)]
pub struct GenericAlertDialog {
    b_ok: Handle<Button>,
    b_yes: Handle<Button>,
    b_no: Handle<Button>,
    b_cancel: Handle<Button>,
    b_retry: Handle<Button>,
}

impl GenericAlertDialog {
    // min size has to be 10
    // <Yes> <No> <  Cancel  >
    // 11 chars per button (12 chars + one space) => minim 12*3 = 36 chars + 2 = minim 38 width+1 => 39 chars min for window
    const BUTTON_SIZE: u32 = 12;
    fn compute_text_width_and_lines(text: &str) -> (u32, u32) {
        let mut nr_lines = 0;
        let mut max_length = 0;
        let mut current_line_len = 0;
        for c in text.chars() {
            if c == '\n' {
                max_length = max_length.max(current_line_len);
                current_line_len = 0;
                nr_lines += 1;
            } else {
                current_line_len += 1;
            }
        }
        if current_line_len > 0 {
            max_length = max_length.max(current_line_len);
            nr_lines += 1;
        }
        nr_lines = nr_lines.max(1);
        (nr_lines, max_length)
    }
    fn compute_text_lines(text: &str, width: u32) -> u32 {
        let mut nr_lines = 0;
        let mut current_line_len = 0;
        for c in text.chars() {
            if c == '\n' {
                current_line_len = 0;
                nr_lines += 1;
            } else {
                current_line_len += 1;
                if current_line_len == width {
                    current_line_len = 0;
                    nr_lines += 1;
                }
            }
        }
        if current_line_len > 0 {
            nr_lines += 1;
        }
        nr_lines = nr_lines.max(1);
        nr_lines
    }
    fn add_button(text: &str, x: i32, y: i32) -> Button {
        let s = format!("x:{x},y:{y},w:11");
        Button::new(text, Layout::new(&s), button::Type::Normal)
    }
    pub(super) fn new(title: &str, caption: &str, buttons: DialogButtons, win_type: window::Type) -> Self {
        let size = RuntimeManager::get().get_terminal_size();
        // the minimum size of the window should contain at least all buttons
        let min_window_width = buttons.count() * GenericAlertDialog::BUTTON_SIZE + 3;
        // maximum size should not be bigger than 80% of the window
        // unless the number of buttons requires so
        let max_window_width = min_window_width.max(size.width * 8 / 10);

        // max len of a text can not be more that 80% of the screen.
        let (mut nr_lines, mut text_width) = GenericAlertDialog::compute_text_width_and_lines(caption);
        if text_width + 4 <= max_window_width {
            // all good --> it fits
            text_width = text_width.max(min_window_width - 4);
        } else {
            text_width = (max_window_width - 4).max(min_window_width - 4);
            nr_lines = GenericAlertDialog::compute_text_lines(caption, text_width);
        }
        // a minimum o 8 chars (o line info + buttons) is required.
        let max_window_height = (size.height * 8 / 10).max(8);
        nr_lines = nr_lines.min(max_window_height - 7);

        // now that we have the nr_lines and the text_width --> we can create a window
        let window_width = text_width + 4;
        let window_height = nr_lines + 7;

        // lets create the window
        let w_format = format!("d:c,w:{window_width},h:{window_height}");
        let mut w = Self {
            base: ModalWindow::with_type(title, Layout::new(&w_format), window::Flags::NoCloseButton, win_type),
            b_ok: Handle::None,
            b_yes: Handle::None,
            b_no: Handle::None,
            b_cancel: Handle::None,
            b_retry: Handle::None,
        };

        // create the label
        let mut lb_info = label!("'',l:1,t:1,r:1,b:3");
        lb_info.set_caption(caption);
        w.add(lb_info);

        // add the buttons
        let mut x = ((window_width - (buttons.count() * GenericAlertDialog::BUTTON_SIZE)) >> 1) as i32;
        let y = (window_height - 4) as i32;
        match buttons {
            DialogButtons::Ok => {
                w.b_ok = w.add(GenericAlertDialog::add_button("&Ok", x, y));
            }
            DialogButtons::YesNo => {
                w.b_yes = w.add(GenericAlertDialog::add_button("&Yes", x, y));
                x += GenericAlertDialog::BUTTON_SIZE as i32;
                w.b_no = w.add(GenericAlertDialog::add_button("&No", x, y));
            }
            DialogButtons::YesNoCancel => {
                w.b_yes = w.add(GenericAlertDialog::add_button("&Yes", x, y));
                x += GenericAlertDialog::BUTTON_SIZE as i32;
                w.b_no = w.add(GenericAlertDialog::add_button("&No", x, y));
                x += GenericAlertDialog::BUTTON_SIZE as i32;
                w.b_cancel = w.add(GenericAlertDialog::add_button("&Cancel", x, y));
            }
            DialogButtons::RetryCancel => {
                w.b_retry = w.add(GenericAlertDialog::add_button("&Retry", x, y));
                x += GenericAlertDialog::BUTTON_SIZE as i32;
                w.b_cancel = w.add(GenericAlertDialog::add_button("&Cancel", x, y));
            }
        }

        w
    }
}

impl ButtonEvents for GenericAlertDialog {
    fn on_pressed(&mut self, handle: Handle<Button>) -> EventProcessStatus {
        match () {
            _ if handle == self.b_ok => {
                self.exit_with(DialogResult::Ok);
            }
            _ if handle == self.b_yes => {
                self.exit_with(DialogResult::Yes);
            }
            _ if handle == self.b_no => {
                self.exit_with(DialogResult::No);
            }
            _ if handle == self.b_cancel => {
                self.exit_with(DialogResult::Cancel);
            }
            _ if handle == self.b_retry => {
                self.exit_with(DialogResult::Retry);
            }
            _ => {
                return EventProcessStatus::Ignored;
            }
        }
        EventProcessStatus::Processed
    }
}
