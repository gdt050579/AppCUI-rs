use super::{CharacterSet, ColorSchema, Scale};

pub struct RenderOptions {
    pub(super) scale: Scale,
    pub(super) char_set: CharacterSet,
    pub(super) color_schema: ColorSchema,
    pub(super) luminance_threshold: u8,
}
pub struct RenderOptionsBuilder {
    inner: RenderOptions,
}
impl RenderOptionsBuilder {
    pub fn new() -> Self {
        Self {
            inner: RenderOptions {
                scale: Scale::NoScale,
                char_set: CharacterSet::SmallBlocks,
                color_schema: ColorSchema::Auto,
                luminance_threshold: 128, 
            },
        }
    }
    #[inline(always)]
    pub fn build(self) -> RenderOptions {
        self.inner
    }
    #[inline(always)]
    pub fn luminance_threshold(mut self, value: f64) -> Self {
        self.inner.luminance_threshold = (value.clamp(0.0f64, 1.0f64) * 255f64) as u8;
        self
    }
    #[inline(always)]
    pub fn scale(mut self, scale: Scale) -> Self {
        self.inner.scale = scale;
        self
    }
    #[inline(always)]
    pub fn color_schema(mut self, color_schema: ColorSchema) -> Self {
        self.inner.color_schema = color_schema;
        self
    }
    #[inline(always)]
    pub fn character_set(mut self, char_set: CharacterSet) -> Self {
        self.inner.char_set = char_set;
        self
    }
}
