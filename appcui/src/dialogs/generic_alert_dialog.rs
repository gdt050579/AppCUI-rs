use super::DialogButtons;
use crate::prelude::*;

#[ModalWindow(events = ButtonEvents, internal: true)]
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
    const BUTTON_SIZE: u32 = 11;
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
    fn new(title: &str, caption: &str, buttons: DialogButtons) -> Self {
        let mut w = Self {
            base: ModalWindow::new(title, Layout::new("d:c,w:30,h:5"), window::Flags::NoCloseButton),
            b_ok: Handle::None,
            b_yes: Handle::None,
            b_no: Handle::None,
            b_cancel: Handle::None,
            b_retry: Handle::None,
        };

        let size = RuntimeManager::get().get_terminal_size();
        // the minimum size of the window should contain at least all buttons
        let min_window_width = buttons.count() * (GenericAlertDialog::BUTTON_SIZE + 1) + 3;
        // maximum size should not be bigger than 80% of the window
        // unless the number of buttons requires so
        let max_window_width = min_window_width.max(size.width * 8 / 10);

        // max len of a text can not be more that 80% of the screen.
        let (mut nr_lines, mut text_width) = GenericAlertDialog::compute_text_width_and_lines(caption);
        if text_width + 4 <= max_window_width {
            // all good --> it fits
        } else {
            text_width = max_window_width - 4;
            nr_lines = GenericAlertDialog::compute_text_lines(caption, text_width);
        }
        // a minimum o 8 chars (o line info + buttons) is required.
        let max_window_height = (size.height*8/10).max(8);
        nr_lines = nr_lines.min(max_window_height-7);

        // now that we have the nr_lines and the text_width --> we can create a window

        w
    }
}

impl ButtonEvents for GenericAlertDialog {
    fn on_pressed(&mut self, _handle: Handle<Button>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
