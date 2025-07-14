pub(super) mod small_blocks_renderer;
pub(super) mod large_blocks_renderer;
pub(super) mod braille_renderer;
pub(super) mod ascii_art_renderer;
pub(super) mod dithered_shades_renderer;

#[derive(Copy,Clone)]
pub enum CharacterSet {
    SmallBlocks,
    LargeBlocks,
    DitheredShades,
    Braille,
    AsciArt
}