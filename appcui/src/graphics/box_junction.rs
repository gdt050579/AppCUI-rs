pub const NONE: u8 = 0;
pub const SINGLE: u8 = 1;
pub const DOUBLE: u8 = 2;
pub const HEAVY: u8 = 3;
const NO_GLYPH: u8 = 0xFF;
const BLOCK_START: u32 = 0x2500;
const BLOCK_END: u32 = 0x257F;

#[inline]
pub(super) const fn mask(n: u8, e: u8, s: u8, w: u8) -> u8 {
    (n << 6) | (e << 4) | (s << 2) | w
}
#[inline]
pub(super) const fn north(m: u8) -> u8 {
    (m >> 6) & 3
}
#[inline]
pub(super) const fn east(m: u8) -> u8 {
    (m >> 4) & 3
}
#[inline]
pub(super) const fn south(m: u8) -> u8 {
    (m >> 2) & 3
}
#[inline]
pub(super) const fn west(m: u8) -> u8 {
    m & 3
}

pub(super) const BOX_JUNCTION_DEFS: &[(u8, u8, u8, u8, u8)] = &[
    // -- single --
    (0x00, NONE, SINGLE, NONE, SINGLE),     // ─
    (0x02, SINGLE, NONE, SINGLE, NONE),     // │
    (0x0C, NONE, SINGLE, SINGLE, NONE),     // ┌
    (0x10, NONE, NONE, SINGLE, SINGLE),     // ┐
    (0x14, SINGLE, SINGLE, NONE, NONE),     // └
    (0x18, SINGLE, NONE, NONE, SINGLE),     // ┘
    (0x1C, SINGLE, SINGLE, SINGLE, NONE),   // ├
    (0x24, SINGLE, NONE, SINGLE, SINGLE),   // ┤
    (0x2C, NONE, SINGLE, SINGLE, SINGLE),   // ┬
    (0x34, SINGLE, SINGLE, NONE, SINGLE),   // ┴
    (0x3C, SINGLE, SINGLE, SINGLE, SINGLE), // ┼
    // -- heavy --
    (0x01, NONE, HEAVY, NONE, HEAVY),   // ━
    (0x03, HEAVY, NONE, HEAVY, NONE),   // ┃
    (0x0F, NONE, HEAVY, HEAVY, NONE),   // ┏
    (0x13, NONE, NONE, HEAVY, HEAVY),   // ┓
    (0x17, HEAVY, HEAVY, NONE, NONE),   // ┗
    (0x1B, HEAVY, NONE, NONE, HEAVY),   // ┛
    (0x23, HEAVY, HEAVY, HEAVY, NONE),  // ┣
    (0x2B, HEAVY, NONE, HEAVY, HEAVY),  // ┫
    (0x33, NONE, HEAVY, HEAVY, HEAVY),  // ┳
    (0x3B, HEAVY, HEAVY, NONE, HEAVY),  // ┻
    (0x4B, HEAVY, HEAVY, HEAVY, HEAVY), // ╋
    // -- single/heavy transitions --
    (0x0D, NONE, HEAVY, SINGLE, NONE),     // ┍
    (0x0E, NONE, SINGLE, HEAVY, NONE),     // ┎
    (0x11, NONE, NONE, SINGLE, HEAVY),     // ┑
    (0x12, NONE, NONE, HEAVY, SINGLE),     // ┒
    (0x15, SINGLE, HEAVY, NONE, NONE),     // ┕
    (0x16, HEAVY, SINGLE, NONE, NONE),     // ┖
    (0x19, SINGLE, NONE, NONE, HEAVY),     // ┙
    (0x1A, HEAVY, NONE, NONE, SINGLE),     // ┚
    (0x1D, SINGLE, HEAVY, SINGLE, NONE),   // ┝
    (0x1E, HEAVY, SINGLE, SINGLE, NONE),   // ┞
    (0x1F, SINGLE, SINGLE, HEAVY, NONE),   // ┟
    (0x20, HEAVY, SINGLE, HEAVY, NONE),    // ┠
    (0x21, HEAVY, HEAVY, SINGLE, NONE),    // ┡
    (0x22, SINGLE, HEAVY, HEAVY, NONE),    // ┢
    (0x25, SINGLE, NONE, SINGLE, HEAVY),   // ┥
    (0x26, HEAVY, NONE, SINGLE, SINGLE),   // ┦
    (0x27, SINGLE, NONE, HEAVY, SINGLE),   // ┧
    (0x28, HEAVY, NONE, HEAVY, SINGLE),    // ┨
    (0x29, HEAVY, NONE, SINGLE, HEAVY),    // ┩
    (0x2A, SINGLE, NONE, HEAVY, HEAVY),    // ┪
    (0x2D, NONE, SINGLE, SINGLE, HEAVY),   // ┭
    (0x2E, NONE, HEAVY, SINGLE, SINGLE),   // ┮
    (0x2F, NONE, HEAVY, SINGLE, HEAVY),    // ┯
    (0x30, NONE, SINGLE, HEAVY, SINGLE),   // ┰
    (0x31, NONE, SINGLE, HEAVY, HEAVY),    // ┱
    (0x32, NONE, HEAVY, HEAVY, SINGLE),    // ┲
    (0x35, SINGLE, SINGLE, NONE, HEAVY),   // ┵
    (0x36, SINGLE, HEAVY, NONE, SINGLE),   // ┶
    (0x37, SINGLE, HEAVY, NONE, HEAVY),    // ┷
    (0x38, HEAVY, SINGLE, NONE, SINGLE),   // ┸
    (0x39, HEAVY, SINGLE, NONE, HEAVY),    // ┹
    (0x3A, HEAVY, HEAVY, NONE, SINGLE),    // ┺
    (0x3D, SINGLE, SINGLE, SINGLE, HEAVY), // ┽
    (0x3E, SINGLE, HEAVY, SINGLE, SINGLE), // ┾
    (0x3F, SINGLE, HEAVY, SINGLE, HEAVY),  // ┿
    (0x40, HEAVY, SINGLE, SINGLE, SINGLE), // ╀
    (0x41, SINGLE, SINGLE, HEAVY, SINGLE), // ╁
    (0x42, HEAVY, SINGLE, HEAVY, SINGLE),  // ╂
    (0x43, HEAVY, SINGLE, SINGLE, HEAVY),  // ╃
    (0x44, HEAVY, HEAVY, SINGLE, SINGLE),  // ╄
    (0x45, SINGLE, SINGLE, HEAVY, HEAVY),  // ╅
    (0x46, SINGLE, HEAVY, HEAVY, SINGLE),  // ╆
    (0x47, HEAVY, SINGLE, HEAVY, HEAVY),   // ╇
    (0x48, SINGLE, HEAVY, HEAVY, HEAVY),   // ╈
    (0x49, HEAVY, HEAVY, SINGLE, HEAVY),   // ╉
    (0x4A, HEAVY, HEAVY, HEAVY, SINGLE),   // ╊
    // -- double --
    (0x50, NONE, DOUBLE, NONE, DOUBLE),     // ═
    (0x51, DOUBLE, NONE, DOUBLE, NONE),     // ║
    (0x54, NONE, DOUBLE, DOUBLE, NONE),     // ╔
    (0x57, NONE, NONE, DOUBLE, DOUBLE),     // ╗
    (0x5A, DOUBLE, DOUBLE, NONE, NONE),     // ╚
    (0x5D, DOUBLE, NONE, NONE, DOUBLE),     // ╝
    (0x60, DOUBLE, DOUBLE, DOUBLE, NONE),   // ╠
    (0x63, DOUBLE, NONE, DOUBLE, DOUBLE),   // ╣
    (0x66, NONE, DOUBLE, DOUBLE, DOUBLE),   // ╦
    (0x69, DOUBLE, DOUBLE, NONE, DOUBLE),   // ╩
    (0x6C, DOUBLE, DOUBLE, DOUBLE, DOUBLE), // ╬
    // -- single/double transitions --
    (0x52, NONE, DOUBLE, SINGLE, NONE),     // ╒
    (0x53, NONE, SINGLE, DOUBLE, NONE),     // ╓
    (0x55, NONE, NONE, SINGLE, DOUBLE),     // ╕
    (0x56, NONE, NONE, DOUBLE, SINGLE),     // ╖
    (0x58, SINGLE, DOUBLE, NONE, NONE),     // ╘
    (0x59, DOUBLE, SINGLE, NONE, NONE),     // ╙
    (0x5B, SINGLE, NONE, NONE, DOUBLE),     // ╛
    (0x5C, DOUBLE, NONE, NONE, SINGLE),     // ╜
    (0x5E, SINGLE, DOUBLE, SINGLE, NONE),   // ╞
    (0x5F, DOUBLE, SINGLE, DOUBLE, NONE),   // ╟
    (0x61, SINGLE, NONE, SINGLE, DOUBLE),   // ╡
    (0x62, DOUBLE, NONE, DOUBLE, SINGLE),   // ╢
    (0x64, NONE, DOUBLE, SINGLE, DOUBLE),   // ╤
    (0x65, NONE, SINGLE, DOUBLE, SINGLE),   // ╥
    (0x67, SINGLE, DOUBLE, NONE, DOUBLE),   // ╧
    (0x68, DOUBLE, SINGLE, NONE, SINGLE),   // ╨
    (0x6A, SINGLE, DOUBLE, SINGLE, DOUBLE), // ╪
    (0x6B, DOUBLE, SINGLE, DOUBLE, SINGLE), // ╫
    // -- half lines --
    (0x74, NONE, NONE, NONE, SINGLE), // ╴
    (0x75, SINGLE, NONE, NONE, NONE), // ╵
    (0x76, NONE, SINGLE, NONE, NONE), // ╶
    (0x77, NONE, NONE, SINGLE, NONE), // ╷
    (0x78, NONE, NONE, NONE, HEAVY),  // ╸
    (0x79, HEAVY, NONE, NONE, NONE),  // ╹
    (0x7A, NONE, HEAVY, NONE, NONE),  // ╺
    (0x7B, NONE, NONE, HEAVY, NONE),  // ╻
];

const fn coerce_to_dominant(m: u8) -> u8 {
    let n = north(m);
    let e = east(m);
    let s = south(m);
    let w = west(m);

    let mut counts = [0u8; 4];
    counts[n as usize] += 1;
    counts[e as usize] += 1;
    counts[s as usize] += 1;
    counts[w as usize] += 1;

    let mut dominant = SINGLE;
    if counts[DOUBLE as usize] > counts[dominant as usize] {
        dominant = DOUBLE;
    }
    if counts[HEAVY as usize] > counts[dominant as usize] {
        dominant = HEAVY;
    }

    coerce_to_weight(m, dominant)
}
const fn coerce_to_weight(m: u8, weight: u8) -> u8 {
    let n = if north(m) == 0 { 0 } else { weight };
    let e = if east(m) == 0 { 0 } else { weight };
    let s = if south(m) == 0 { 0 } else { weight };
    let w = if west(m) == 0 { 0 } else { weight };
    mask(n, e, s, w)
}

#[inline(always)]
const fn connection_count(m: u8) -> u32 {
    // Fold each 2-bit field down to 1 if either bit is set, then popcount.
    let folded = (m | (m >> 1)) & 0b0101_0101;
    folded.count_ones()
}

pub struct BoxJunction {
    to_mask: [u8; 256],
    to_char: [u8; 256],
}

impl BoxJunction {
    pub const fn new() -> Self {
        let mut t = BoxJunction {
            to_mask: [0; 256],
            to_char: [NO_GLYPH; 256],
        };
        let mut i = 0;
        while i < BOX_JUNCTION_DEFS.len() {
            let (idx, n, e, s, w) = BOX_JUNCTION_DEFS[i];
            let m = mask(n, e, s, w);
            t.to_mask[idx as usize] = m;
            t.to_char[m as usize] = idx;
            i += 1;
        }
        let mut m = 1usize;
        while m < 256 {
            if t.to_char[m] == NO_GLYPH {
                let mut candidate = coerce_to_dominant(m as u8);
                if t.to_char[candidate as usize] == NO_GLYPH {
                    candidate = coerce_to_weight(m as u8, SINGLE);
                }
                if t.to_char[candidate as usize] == NO_GLYPH {
                    candidate = coerce_to_weight(m as u8, HEAVY);
                }
                if t.to_char[candidate as usize] == NO_GLYPH {
                    candidate = coerce_to_weight(m as u8, DOUBLE);
                }
                t.to_char[m] = t.to_char[candidate as usize];
            }
            m += 1;
        }

        t
    }
    #[inline(always)]
    pub const fn mask_of(&self, c: char) -> u8 {
        let cp = c as u32;
        if cp >= BLOCK_START && cp <= BLOCK_END {
            self.to_mask[(cp - BLOCK_START) as usize]
        } else {
            0
        }
    }
    #[inline(always)]
    pub(super) const fn resolve(&self, left: char, up: char, right: char, down: char) -> Option<char> {
        let m = mask(
            south(self.mask_of(up)),
            west(self.mask_of(right)),
            north(self.mask_of(down)),
            east(self.mask_of(left)),
        );
        if connection_count(m) < 2 {
            return None;
        }
        let idx = self.to_char[m as usize];
        if idx == NO_GLYPH {
            None
        } else {
            Some(unsafe { char::from_u32_unchecked(BLOCK_START + idx as u32) })
        }
    }
}
pub(super) static BOX_JUNCTION: BoxJunction = BoxJunction::new();
