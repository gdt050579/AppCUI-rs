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

pub(crate) const ALL_MOUSE_EVENTS: i32 = 268435455;
pub(crate) const REPORT_MOUSE_POSITION: i32 = 268435456;

pub(crate) type chtype = u64;
pub(crate) type c_bool = libc::c_uchar;
pub(crate) type winttype = u32;
pub(crate) type mmask_t = chtype;
pub(crate) type attr_t = chtype;
pub(crate) type NCURSES_ATTR_T = attr_t;

pub(crate) const LC_CTYPE: libc::c_int = 0;
pub(crate) const LC_NUMERIC: libc::c_int = 1;
pub(crate) const LC_TIME: libc::c_int = 2;
pub(crate) const LC_COLLATE: libc::c_int = 3;
pub(crate) const LC_MONETARY: libc::c_int = 4;
pub(crate) const LC_MESSAGES: libc::c_int = 5;
pub(crate) const LC_ALL: libc::c_int = 6;

pub(crate) const ERR: i32 = -1;
pub(crate) const OK: i32 = 0;
pub(crate) const TRUE: c_bool = 1;
pub(crate) const FALSE: c_bool = 0;
pub(crate) const NCURSES_ATTR_SHIFT: u32 = 8;
pub(crate) const NR_COLORS: i16 = 16;
pub(crate) const COLOR_BLACK: i16 = 0;
pub(crate) const COLOR_RED: i16 = 1;
pub(crate) const COLOR_GREEN: i16 = 2;
pub(crate) const COLOR_YELLOW: i16 = 3;
pub(crate) const COLOR_BLUE: i16 = 4;
pub(crate) const COLOR_MAGENTA: i16 = 5;
pub(crate) const COLOR_CYAN: i16 = 6;
pub(crate) const COLOR_WHITE: i16 = 7;
pub(crate) const _SUBWIN: i32 = 1;
pub(crate) const _ENDLINE: i32 = 2;
pub(crate) const _FULLWIN: i32 = 4;
pub(crate) const _SCROLLWIN: i32 = 8;
pub(crate) const _ISPAD: i32 = 16;
pub(crate) const _HASMOVED: i32 = 32;
pub(crate) const _WRAPPED: i32 = 64;
pub(crate) const _NOCHANGE: i32 = -1;
pub(crate) const _NEWINDEX: i32 = -1;
pub(crate) const KEY_CODE_YES: i32 = 256;
pub(crate) const KEY_MIN: i32 = 257;
pub(crate) const KEY_BREAK: i32 = 257;
pub(crate) const KEY_SRESET: i32 = 344;
pub(crate) const KEY_RESET: i32 = 345;
pub(crate) const KEY_DOWN: i32 = 258;
pub(crate) const KEY_UP: i32 = 259;
pub(crate) const KEY_LEFT: i32 = 260;
pub(crate) const KEY_RIGHT: i32 = 261;
pub(crate) const KEY_HOME: i32 = 262;
pub(crate) const KEY_BACKSPACE: i32 = 263;
pub(crate) const KEY_F0: i32 = 264;
pub(crate) const KEY_DL: i32 = 328;
pub(crate) const KEY_IL: i32 = 329;
pub(crate) const KEY_DC: i32 = 330;
pub(crate) const KEY_IC: i32 = 331;
pub(crate) const KEY_EIC: i32 = 332;
pub(crate) const KEY_CLEAR: i32 = 333;
pub(crate) const KEY_EOS: i32 = 334;
pub(crate) const KEY_EOL: i32 = 335;
pub(crate) const KEY_SF: i32 = 336;
pub(crate) const KEY_SR: i32 = 337;
pub(crate) const KEY_NPAGE: i32 = 338;
pub(crate) const KEY_PPAGE: i32 = 339;
pub(crate) const KEY_STAB: i32 = 340;
pub(crate) const KEY_CTAB: i32 = 341;
pub(crate) const KEY_CATAB: i32 = 342;
pub(crate) const KEY_ENTER: i32 = 343;
pub(crate) const KEY_PRINT: i32 = 346;
pub(crate) const KEY_LL: i32 = 347;
pub(crate) const KEY_BTAB: i32 = 353;
pub(crate) const KEY_BEG: i32 = 354;
pub(crate) const KEY_CANCEL: i32 = 355;
pub(crate) const KEY_CLOSE: i32 = 356;
pub(crate) const KEY_COMMAND: i32 = 357;
pub(crate) const KEY_COPY: i32 = 358;
pub(crate) const KEY_CREATE: i32 = 359;
pub(crate) const KEY_END: i32 = 360;
pub(crate) const KEY_EXIT: i32 = 361;
pub(crate) const KEY_FIND: i32 = 362;
pub(crate) const KEY_HELP: i32 = 363;
pub(crate) const KEY_MARK: i32 = 364;
pub(crate) const KEY_MESSAGE: i32 = 365;
pub(crate) const KEY_MOVE: i32 = 366;
pub(crate) const KEY_NEXT: i32 = 367;
pub(crate) const KEY_OPEN: i32 = 368;
pub(crate) const KEY_OPTIONS: i32 = 369;
pub(crate) const KEY_PREVIOUS: i32 = 370;
pub(crate) const KEY_REDO: i32 = 371;
pub(crate) const KEY_REFERENCE: i32 = 372;
pub(crate) const KEY_REFRESH: i32 = 373;
pub(crate) const KEY_REPLACE: i32 = 374;
pub(crate) const KEY_RESTART: i32 = 375;
pub(crate) const KEY_RESUME: i32 = 376;
pub(crate) const KEY_SAVE: i32 = 377;
pub(crate) const KEY_SBEG: i32 = 378;
pub(crate) const KEY_SCANCEL: i32 = 379;
pub(crate) const KEY_SCOMMAND: i32 = 380;
pub(crate) const KEY_SCOPY: i32 = 381;
pub(crate) const KEY_SCREATE: i32 = 382;
pub(crate) const KEY_SDC: i32 = 383;
pub(crate) const KEY_SDL: i32 = 384;
pub(crate) const KEY_SELECT: i32 = 385;
pub(crate) const KEY_SEND: i32 = 386;
pub(crate) const KEY_SEOL: i32 = 387;
pub(crate) const KEY_SEXIT: i32 = 388;
pub(crate) const KEY_SFIND: i32 = 389;
pub(crate) const KEY_SHELP: i32 = 390;
pub(crate) const KEY_SHOME: i32 = 391;
pub(crate) const KEY_SIC: i32 = 392;
pub(crate) const KEY_SLEFT: i32 = 393;
pub(crate) const KEY_SMESSAGE: i32 = 394;
pub(crate) const KEY_SMOVE: i32 = 395;
pub(crate) const KEY_SNEXT: i32 = 396;
pub(crate) const KEY_SOPTIONS: i32 = 397;
pub(crate) const KEY_SPREVIOUS: i32 = 398;
pub(crate) const KEY_SPRINT: i32 = 399;
pub(crate) const KEY_SREDO: i32 = 400;
pub(crate) const KEY_SREPLACE: i32 = 401;
pub(crate) const KEY_SRIGHT: i32 = 402;
pub(crate) const KEY_SRSUME: i32 = 403;
pub(crate) const KEY_SSAVE: i32 = 404;
pub(crate) const KEY_SSUSPEND: i32 = 405;
pub(crate) const KEY_SUNDO: i32 = 406;
pub(crate) const KEY_SUSPEND: i32 = 407;
pub(crate) const KEY_UNDO: i32 = 408;
pub(crate) const KEY_MOUSE: i32 = 409;
pub(crate) const KEY_RESIZE: i32 = 410;
pub(crate) const KEY_MAX: i32 = 511;
pub(crate) const NCURSES_MOUSE_VERSION: i32 = 2;
pub(crate) const NCURSES_BUTTON_RELEASED: i32 = 1;
pub(crate) const NCURSES_BUTTON_PRESSED: i32 = 2;
pub(crate) const NCURSES_BUTTON_CLICKED: i32 = 4;
pub(crate) const NCURSES_DOUBLE_CLICKED: i32 = 8;
pub(crate) const NCURSES_TRIPLE_CLICKED: i32 = 16;
pub(crate) const NCURSES_RESERVED_EVENT: i32 = 32;
pub(crate) const BUTTON1_RELEASED: i32 = 1;
pub(crate) const BUTTON1_PRESSED: i32 = 2;
pub(crate) const BUTTON1_CLICKED: i32 = 4;
pub(crate) const BUTTON1_DOUBLE_CLICKED: i32 = 8;
pub(crate) const BUTTON1_TRIPLE_CLICKED: i32 = 16;
pub(crate) const BUTTON2_RELEASED: i32 = 256;
pub(crate) const BUTTON2_PRESSED: i32 = 512;
pub(crate) const BUTTON2_CLICKED: i32 = 1024;
pub(crate) const BUTTON2_DOUBLE_CLICKED: i32 = 2048;
pub(crate) const BUTTON2_TRIPLE_CLICKED: i32 = 4096;
pub(crate) const BUTTON3_RELEASED: i32 = 65536;
pub(crate) const BUTTON3_PRESSED: i32 = 131072;
pub(crate) const BUTTON3_CLICKED: i32 = 262144;
pub(crate) const BUTTON3_DOUBLE_CLICKED: i32 = 524288;
pub(crate) const BUTTON3_TRIPLE_CLICKED: i32 = 1048576;
pub(crate) const BUTTON4_RELEASED: i32 = 16777216;
pub(crate) const BUTTON4_PRESSED: i32 = 33554432;
pub(crate) const BUTTON4_CLICKED: i32 = 67108864;
pub(crate) const BUTTON4_DOUBLE_CLICKED: i32 = 134217728;
pub(crate) const BUTTON4_TRIPLE_CLICKED: i32 = 268435456;
pub(crate) const BUTTON_CTRL: i32 = 536870912;
pub(crate) const BUTTON_SHIFT: i32 = 1073741824;
pub(crate) const BUTTON_ALT: i32 = -2147483648;
pub(crate) const WHEEL_DOWN: i32 = 2097152;
pub(crate) const WHEEL_UP: i32 = 65536;
pub(crate) const WHEEL_LEFT: i32 = 134283264;
pub(crate) const WHEEL_RIGHT: i32 = 136314880;
// pub(crate) const NCURSES_MOUSE_MASK: i32 = 4294967295;

pub(crate) const A_NORMAL: u64 = 0;//1 - 1;
pub(crate) const A_ATTRIBUTES: u64 = !(0); //not(1-1)
pub(crate) const A_CHARTEXT: u64 = 0;//(1 << 0) - 1;
pub(crate) const A_COLOR: u64 = (1 << 8) - 1;
pub(crate) const A_STANDOUT: u64 = 1 << 8;
pub(crate) const A_UNDERLINE: u64 = 1 << 9;
pub(crate) const A_REVERSE: u64 = 1 << 10;
pub(crate) const A_BLINK: u64 = 1 << 11;
pub(crate) const A_DIM: u64 = 1 << 12;
pub(crate) const A_BOLD: u64 = 1 << 13;
pub(crate) const A_ALTCHARSET: u64 = 1 << 14;
pub(crate) const A_INVIS: u64 = 1 << 15;
pub(crate) const A_PROTECT: u64 = 1 << 16;
pub(crate) const A_HORIZONTAL: u64 = 1 << 17;
pub(crate) const A_LEFT: u64 = 1 << 18;
pub(crate) const A_LOW: u64 = 1 << 19;
pub(crate) const A_RIGHT: u64 = 1 << 20;
pub(crate) const A_TOP: u64 = 1 << 21;
pub(crate) const A_VERTICAL: u64 = 1 << 22;