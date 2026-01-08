#[allow(clippy::upper_case_acronyms)]
pub(crate) type HANDLE = usize;
#[allow(clippy::upper_case_acronyms)]
pub(crate) type BOOL = u32;

#[repr(C)]
#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Default, Copy, Clone, Debug)]
pub(crate) struct SIZE {
    pub width: u16,
    pub height: u16,
}

#[repr(C)]
#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Default, Copy, Clone, Debug)]
pub(crate) struct COORD {
    pub x: i16,
    pub y: i16,
}

#[repr(C)]
#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Default, Copy, Clone, Debug)]
pub(crate) struct SMALL_RECT {
    pub left: i16,
    pub top: i16,
    pub right: i16,
    pub bottom: i16,
}
// #[repr(C)]
// #[allow(non_camel_case_types)]
// #[allow(clippy::upper_case_acronyms)]
// #[derive(Default, Copy, Clone, Debug)]
// pub(crate) struct CONSOLE_SCREEN_BUFFER_INFO {
//     pub size: COORD,
//     pub cursor_pos: COORD,
//     pub attributes: u16,
//     pub window: SMALL_RECT,
//     pub max_size: COORD,
// }

#[repr(C)]
#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Default, Copy, Clone, Debug)]
pub(crate) struct CONSOLE_SCREEN_BUFFER_INFOEX {
    pub structure_size: u32,
    pub size: COORD,
    pub cursor_pos: COORD,
    pub attributes: u16,
    pub window: SMALL_RECT,
    pub max_size: COORD,
    pub popup_attr: u16,
    pub supports_full_screen: BOOL,
    pub color_table: [u32; 16],
}

#[repr(C)]
#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Default, Copy, Clone, Debug)]
pub(crate) struct CONSOLE_CURSOR_INFO {
    pub size: u32,
    pub visible: BOOL,
}

#[repr(C)]
#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Default, Copy, Clone, Debug)]
pub(crate) struct CHAR_INFO {
    pub code: u16,
    pub attr: u16,
}

#[repr(C)]
#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Copy, Clone, Debug)]
pub(crate) struct KEY_EVENT_RECORD {
    pub key_down: BOOL,
    pub repeat_count: u16,
    pub virtual_key_code: u16,
    pub virtual_scan_code: u16,
    pub unicode_char: u16,
    pub control_key_state: u32,
}

#[repr(C)]
#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Copy, Clone, Debug)]
pub(crate) struct MOUSE_EVENT_RECORD {
    pub mouse_position: COORD,
    pub button_state: u32,
    pub control_key_state: u32,
    pub event_flags: u32,
}

#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub(crate) union WindowsTerminalEvent {
    pub key_event: KEY_EVENT_RECORD,
    pub mouse_event: MOUSE_EVENT_RECORD,
    pub window_buffer_size_event: SIZE,
    pub extra: u32,
}

#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub(crate) struct INPUT_RECORD {
    pub event_type: u16,
    pub event: WindowsTerminalEvent,
}

/// Console font information structure for GetCurrentConsoleFontEx
#[repr(C)]
#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Copy, Clone, Debug)]
pub(crate) struct CONSOLE_FONT_INFOEX {
    pub cb_size: u32,
    pub n_font: u32,
    pub dw_font_size: COORD,
    pub font_family: u32,
    pub font_weight: u32,
    pub face_name: [u16; 32], // LF_FACESIZE = 32
}

impl Default for CONSOLE_FONT_INFOEX {
    fn default() -> Self {
        Self {
            cb_size: std::mem::size_of::<CONSOLE_FONT_INFOEX>() as u32,
            n_font: 0,
            dw_font_size: COORD::default(),
            font_family: 0,
            font_weight: 0,
            face_name: [0; 32],
        }
    }
}
