pub(super) mod small_blocks_renderer;
pub(super) mod large_blocks_renderer;
pub(super) mod braille_renderer;

#[derive(Copy,Clone)]
pub enum CharacterSet {
    SmallBlocks,
    LargeBlock,
    DitheredShades,
    Braille,
    AsciArt
}