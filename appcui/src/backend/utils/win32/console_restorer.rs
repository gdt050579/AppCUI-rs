use crate::backend::utils::win32::api;
use crate::backend::RestoreReason;
use crate::backend::TerminalRestorer;

use super::constants;
use super::structs;
use super::OriginalScreen;

pub(crate) struct ConsoleRestorer {
    pub(super) stdin: structs::HANDLE,
    pub(super) stdout: structs::HANDLE,
    pub(super) stdin_original_mode_flags: u32,
    pub(super) stdout_original_mode_flags: u32,
    pub(super) orig_screen: Option<OriginalScreen>,
    pub(super) orig_color_table: [u32; 16],
    pub(super) color_table_changed: bool,
}

impl TerminalRestorer for ConsoleRestorer {
    fn restore(mut self: Box<Self>, reason: RestoreReason) {
        unsafe {
            let _ = api::SetConsoleMode(self.stdin, self.stdin_original_mode_flags);
            let _ = api::SetConsoleMode(self.stdout, self.stdout_original_mode_flags);
            // revert the console color table (read the current info first, so that
            // only the color table is changed and not the size, cursor, etc.)
            if self.color_table_changed {
                let mut cbuf = structs::CONSOLE_SCREEN_BUFFER_INFOEX {
                    structure_size: 0x60,
                    ..Default::default()
                };
                if api::GetConsoleScreenBufferInfoEx(self.stdout, &mut cbuf) == constants::TRUE {
                    cbuf.color_table = self.orig_color_table;
                    let _ = api::SetConsoleScreenBufferInfoEx(self.stdout, &cbuf);
                }
            }
        }
        match reason {
            RestoreReason::Normal => {
                // restore the original screen
                if let Some(os) = self.orig_screen.take() {
                    os.restore();
                }
            }
            RestoreReason::Panic => {
                // move the cursor at the list line of the screen (top, bottom-most)
                unsafe {
                    let mut cbuf = structs::CONSOLE_SCREEN_BUFFER_INFOEX {
                        structure_size: 0x60,
                        ..Default::default()
                    };
                    if api::GetConsoleScreenBufferInfoEx(self.stdout, &mut cbuf) == constants::TRUE {
                        api::SetConsoleCursorPosition(
                            self.stdout,
                            structs::COORD {
                                x: 0,
                                y: cbuf.size.y.saturating_sub(1),
                            },
                        );
                    }
                    println!(); // write an enter to the terminal to move to the next line
                }
            }
        }
    }
}
