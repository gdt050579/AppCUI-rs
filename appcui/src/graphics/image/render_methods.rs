pub(super) mod small_blocks_renderer;

#[derive(Copy,Clone)]
pub enum RenderMethod {
    SmallBlocks,
    LargeBlock,
    Braille,
    AsciArt
}