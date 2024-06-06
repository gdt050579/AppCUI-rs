/*
    Copyright (c) 2016 Jesse 'Jeaye' Wilkerson

    Permission is hereby granted, free of charge, to any person obtaining a copy
    of this software and associated documentation files (the "Software"), to deal
    in the Software without restriction, including without limitation the rights
    to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
    copies of the Software, and to permit persons to whom the Software is
    furnished to do so, subject to the following conditions:

    The above copyright notice and this permission notice shall be included in all
    copies or substantial portions of the Software.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
    AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
    OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
    SOFTWARE.

    Except as contained in this notice, the name(s) of the above copyright
    holders shall not be used in advertising or otherwise to promote the
    sale, use or other dealings in this Software without prior written
    authorization.
*/

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

pub const ALL_MOUSE_EVENTS: i32 = 268435455;
pub const REPORT_MOUSE_POSITION: i32 = 268435456;

pub type chtype = u64;
pub type c_bool = libc::c_uchar;
pub type winttype = u32;
pub type mmask_t = chtype;
pub type attr_t = chtype;
pub type NCURSES_ATTR_T = attr_t;

pub const LC_CTYPE: libc::c_int = 0;
pub const LC_NUMERIC: libc::c_int = 1;
pub const LC_TIME: libc::c_int = 2;
pub const LC_COLLATE: libc::c_int = 3;
pub const LC_MONETARY: libc::c_int = 4;
pub const LC_MESSAGES: libc::c_int = 5;
pub const LC_ALL: libc::c_int = 6;

pub const ERR: i32 = -1;
pub const OK: i32 = 0;
pub const TRUE: c_bool = 1;
pub const FALSE: c_bool = 0;
pub const NCURSES_ATTR_SHIFT: u32 = 8;
pub const NR_COLORS: i16 = 16;
pub const COLOR_BLACK: i16 = 0;
pub const COLOR_RED: i16 = 1;
pub const COLOR_GREEN: i16 = 2;
pub const COLOR_YELLOW: i16 = 3;
pub const COLOR_BLUE: i16 = 4;
pub const COLOR_MAGENTA: i16 = 5;
pub const COLOR_CYAN: i16 = 6;
pub const COLOR_WHITE: i16 = 7;
pub const _SUBWIN: i32 = 1;
pub const _ENDLINE: i32 = 2;
pub const _FULLWIN: i32 = 4;
pub const _SCROLLWIN: i32 = 8;
pub const _ISPAD: i32 = 16;
pub const _HASMOVED: i32 = 32;
pub const _WRAPPED: i32 = 64;
pub const _NOCHANGE: i32 = -1;
pub const _NEWINDEX: i32 = -1;
pub const KEY_CODE_YES: i32 = 256;
pub const KEY_MIN: i32 = 257;
pub const KEY_BREAK: i32 = 257;
pub const KEY_SRESET: i32 = 344;
pub const KEY_RESET: i32 = 345;
pub const KEY_DOWN: i32 = 258;
pub const KEY_UP: i32 = 259;
pub const KEY_LEFT: i32 = 260;
pub const KEY_RIGHT: i32 = 261;
pub const KEY_HOME: i32 = 262;
pub const KEY_BACKSPACE: i32 = 263;
pub const KEY_F0: i32 = 264;
pub const KEY_DL: i32 = 328;
pub const KEY_IL: i32 = 329;
pub const KEY_DC: i32 = 330;
pub const KEY_IC: i32 = 331;
pub const KEY_EIC: i32 = 332;
pub const KEY_CLEAR: i32 = 333;
pub const KEY_EOS: i32 = 334;
pub const KEY_EOL: i32 = 335;
pub const KEY_SF: i32 = 336;
pub const KEY_SR: i32 = 337;
pub const KEY_NPAGE: i32 = 338;
pub const KEY_PPAGE: i32 = 339;
pub const KEY_STAB: i32 = 340;
pub const KEY_CTAB: i32 = 341;
pub const KEY_CATAB: i32 = 342;
pub const KEY_ENTER: i32 = 343;
pub const KEY_PRINT: i32 = 346;
pub const KEY_LL: i32 = 347;
pub const KEY_BTAB: i32 = 353;
pub const KEY_BEG: i32 = 354;
pub const KEY_CANCEL: i32 = 355;
pub const KEY_CLOSE: i32 = 356;
pub const KEY_COMMAND: i32 = 357;
pub const KEY_COPY: i32 = 358;
pub const KEY_CREATE: i32 = 359;
pub const KEY_END: i32 = 360;
pub const KEY_EXIT: i32 = 361;
pub const KEY_FIND: i32 = 362;
pub const KEY_HELP: i32 = 363;
pub const KEY_MARK: i32 = 364;
pub const KEY_MESSAGE: i32 = 365;
pub const KEY_MOVE: i32 = 366;
pub const KEY_NEXT: i32 = 367;
pub const KEY_OPEN: i32 = 368;
pub const KEY_OPTIONS: i32 = 369;
pub const KEY_PREVIOUS: i32 = 370;
pub const KEY_REDO: i32 = 371;
pub const KEY_REFERENCE: i32 = 372;
pub const KEY_REFRESH: i32 = 373;
pub const KEY_REPLACE: i32 = 374;
pub const KEY_RESTART: i32 = 375;
pub const KEY_RESUME: i32 = 376;
pub const KEY_SAVE: i32 = 377;
pub const KEY_SBEG: i32 = 378;
pub const KEY_SCANCEL: i32 = 379;
pub const KEY_SCOMMAND: i32 = 380;
pub const KEY_SCOPY: i32 = 381;
pub const KEY_SCREATE: i32 = 382;
pub const KEY_SDC: i32 = 383;
pub const KEY_SDL: i32 = 384;
pub const KEY_SELECT: i32 = 385;
pub const KEY_SEND: i32 = 386;
pub const KEY_SEOL: i32 = 387;
pub const KEY_SEXIT: i32 = 388;
pub const KEY_SFIND: i32 = 389;
pub const KEY_SHELP: i32 = 390;
pub const KEY_SHOME: i32 = 391;
pub const KEY_SIC: i32 = 392;
pub const KEY_SLEFT: i32 = 393;
pub const KEY_SMESSAGE: i32 = 394;
pub const KEY_SMOVE: i32 = 395;
pub const KEY_SNEXT: i32 = 396;
pub const KEY_SOPTIONS: i32 = 397;
pub const KEY_SPREVIOUS: i32 = 398;
pub const KEY_SPRINT: i32 = 399;
pub const KEY_SREDO: i32 = 400;
pub const KEY_SREPLACE: i32 = 401;
pub const KEY_SRIGHT: i32 = 402;
pub const KEY_SRSUME: i32 = 403;
pub const KEY_SSAVE: i32 = 404;
pub const KEY_SSUSPEND: i32 = 405;
pub const KEY_SUNDO: i32 = 406;
pub const KEY_SUSPEND: i32 = 407;
pub const KEY_UNDO: i32 = 408;
pub const KEY_MOUSE: i32 = 409;
pub const KEY_RESIZE: i32 = 410;
pub const KEY_MAX: i32 = 511;
pub const NCURSES_MOUSE_VERSION: i32 = 2;
pub const NCURSES_BUTTON_RELEASED: i32 = 1;
pub const NCURSES_BUTTON_PRESSED: i32 = 2;
pub const NCURSES_BUTTON_CLICKED: i32 = 4;
pub const NCURSES_DOUBLE_CLICKED: i32 = 8;
pub const NCURSES_TRIPLE_CLICKED: i32 = 16;
pub const NCURSES_RESERVED_EVENT: i32 = 32;
pub const BUTTON1_RELEASED: i32 = 1;
pub const BUTTON1_PRESSED: i32 = 2;
pub const BUTTON1_CLICKED: i32 = 4;
pub const BUTTON1_DOUBLE_CLICKED: i32 = 8;
pub const BUTTON1_TRIPLE_CLICKED: i32 = 16;
pub const BUTTON2_RELEASED: i32 = 256;
pub const BUTTON2_PRESSED: i32 = 512;
pub const BUTTON2_CLICKED: i32 = 1024;
pub const BUTTON2_DOUBLE_CLICKED: i32 = 2048;
pub const BUTTON2_TRIPLE_CLICKED: i32 = 4096;
pub const BUTTON3_RELEASED: i32 = 65536;
pub const BUTTON3_PRESSED: i32 = 131072;
pub const BUTTON3_CLICKED: i32 = 262144;
pub const BUTTON3_DOUBLE_CLICKED: i32 = 524288;
pub const BUTTON3_TRIPLE_CLICKED: i32 = 1048576;
pub const BUTTON4_RELEASED: i32 = 16777216;
pub const BUTTON4_PRESSED: i32 = 33554432;
pub const BUTTON4_CLICKED: i32 = 67108864;
pub const BUTTON4_DOUBLE_CLICKED: i32 = 134217728;
pub const BUTTON4_TRIPLE_CLICKED: i32 = 268435456;
pub const BUTTON_CTRL: i32 = 536870912;
pub const BUTTON_SHIFT: i32 = 1073741824;
pub const BUTTON_ALT: i32 = -2147483648;
pub const WHEEL_DOWN: i32 = 2097152;
pub const WHEEL_UP: i32 = 65536;
pub const WHEEL_LEFT: i32 = 134283264;
pub const WHEEL_RIGHT: i32 = 136314880;
// pub const NCURSES_MOUSE_MASK: i32 = 4294967295;

pub const A_NORMAL: u64 = 1 - 1;
pub const A_ATTRIBUTES: u64 = !(1 - 1);
pub const A_CHARTEXT: u64 = (1 << 0) - 1;
pub const A_COLOR: u64 = ((1 << 8) - 1) << 0;
pub const A_STANDOUT: u64 = 1 << 8;
pub const A_UNDERLINE: u64 = 1 << 9;
pub const A_REVERSE: u64 = 1 << 10;
pub const A_BLINK: u64 = 1 << 11;
pub const A_DIM: u64 = 1 << 12;
pub const A_BOLD: u64 = 1 << 13;
pub const A_ALTCHARSET: u64 = 1 << 14;
pub const A_INVIS: u64 = 1 << 15;
pub const A_PROTECT: u64 = 1 << 16;
pub const A_HORIZONTAL: u64 = 1 << 17;
pub const A_LEFT: u64 = 1 << 18;
pub const A_LOW: u64 = 1 << 19;
pub const A_RIGHT: u64 = 1 << 20;
pub const A_TOP: u64 = 1 << 21;
pub const A_VERTICAL: u64 = 1 << 22;