use super::Surface;
use std::ops::{Deref, DerefMut};

pub(crate) struct SurfaceTester {
    surface: Surface,
}
impl SurfaceTester {
    pub(crate) fn new(width: u32, height: u32) -> SurfaceTester {
        SurfaceTester {
            surface: Surface::new(width, height),
        }
    }
    // #[allow(dead_code)]
    // pub(crate) fn print(&mut self, show_just_hash: bool) {
    //     use crate::backend::utils::{AnsiFlags, AnsiFormatter};
    //     if show_just_hash {
    //         println!("Hash = 0x{:X} - Dec: {}", self.compute_hash(), self.compute_hash());
    //     } else {
    //         let mut tmp = AnsiFormatter::new(1024, AnsiFlags::None);
    //         let mut x = 0u32;
    //         tmp.write_string(format!("======| Hash: 0x{:X} |======", self.compute_hash()).as_str());
    //         tmp.move_to_next_line();
    //         for ch in &self.surface.chars {
    //             tmp.set_color(ch.foreground, ch.background);
    //             tmp.set_char_flags(ch.flags);
    //             tmp.write_char(ch.code);
    //             x += 1;
    //             if x == self.surface.size.width {
    //                 tmp.reset_color();
    //                 tmp.move_to_next_line();
    //                 x = 0;
    //             }
    //         }
    //         tmp.reset_color();
    //         for _ in 0..self.surface.size.width {
    //             tmp.write_char('-');
    //         }
    //         tmp.move_to_next_line();
    //         println!("{}", tmp.text());
    //     }
    // }

    pub(crate) fn compute_hash(&self) -> u64 {
        // use FNV algorithm ==> https://en.wikipedia.org/wiki/Fowler%E2%80%93Noll%E2%80%93Vo_hash_function
        let mut hash = 0xcbf29ce484222325u64;
        let mut buf = [0u8; 8];
        for ch in &self.chars {
            buf[0] = ((ch.code as u32) & 0xFF) as u8;
            buf[1] = (((ch.code as u32) >> 8) & 0xFF) as u8;
            buf[2] = (((ch.code as u32) >> 16) & 0xFF) as u8;
            buf[3] = (((ch.code as u32) >> 24) & 0xFF) as u8;
            buf[4] = ch.foreground.as_color_index();
            buf[5] = ch.background.as_color_index();
            buf[6] = ((ch.flags.get_value() >> 8) & 0xFF) as u8;
            buf[7] = (ch.flags.get_value() & 0xFF) as u8;
            for b in buf {
                hash ^= b as u64;
                hash = hash.wrapping_mul(0x00000100000001B3u64);
            }
            if let Some((r, g, b)) = ch.foreground.rgb() {
                hash ^= r as u64;
                hash = hash.wrapping_mul(0x00000100000001B3u64);
                hash ^= g as u64;
                hash = hash.wrapping_mul(0x00000100000001B3u64);
                hash ^= b as u64;
                hash = hash.wrapping_mul(0x00000100000001B3u64);
            }
            if let Some((r, g, b)) = ch.background.rgb() {
                hash ^= r as u64;
                hash = hash.wrapping_mul(0x00000100000001B3u64);
                hash ^= g as u64;
                hash = hash.wrapping_mul(0x00000100000001B3u64);
                hash ^= b as u64;
                hash = hash.wrapping_mul(0x00000100000001B3u64);
            }
        }
        hash
    }
}
impl Deref for SurfaceTester {
    type Target = Surface;

    fn deref(&self) -> &Self::Target {
        &self.surface
    }
}
impl DerefMut for SurfaceTester {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.surface
    }
}
