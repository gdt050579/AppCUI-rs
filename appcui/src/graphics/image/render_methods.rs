pub(super) mod small_blocks_renderer;
pub(super) mod large_blocks_renderer;

#[derive(Copy,Clone)]
pub enum RenderMethod {
    SmallBlocks,
    LargeBlock,
    DitheredShades,
    Braille,
    AsciArt
}