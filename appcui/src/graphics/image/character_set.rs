pub(super) mod small_blocks_renderer;
pub(super) mod large_blocks_renderer;
pub(super) mod braille_renderer;
pub(super) mod ascii_art_renderer;
pub(super) mod dithered_shades_renderer;

/// How an [`Image`](super::Image) is mapped onto terminal characters when painted on a [`Surface`](crate::graphics::Surface).
///
/// Small and large blocks trade resolution for color fidelity, dithering and ASCII art
/// encode brightness as glyphs, and Braille packs a 2×4 pixel grid into each cell.
#[derive(Copy,Clone)]
pub enum CharacterSet {
    /// Half-block characters (`▀`), packing two vertical pixels into one cell.
    SmallBlocks,
    /// Two-wide spaces filled with background color, one pixel per pair of cells.
    LargeBlocks,
    /// Shade characters `░▒▓█` (and spaces) to approximate brightness.
    DitheredShades,
    /// Braille dots (`⣿`), packing a 2×4 pixel grid into one cell.
    Braille,
    /// ASCII art using a luminance ramp such as `` .':;+*#@$ ``.
    AsciiArt
}