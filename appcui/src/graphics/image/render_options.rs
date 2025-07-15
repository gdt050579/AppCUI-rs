use super::{CharacterSet, ColorSchema, Scale};

#[derive(Clone)]
pub struct RenderOptions {
    pub(super) scale: Scale,
    pub(super) char_set: CharacterSet,
    pub(super) color_schema: ColorSchema,
    pub(super) luminance_threshold: u8,
}

impl RenderOptions {
    /// Returns the scale of the render options.
    #[inline(always)]
    pub fn scale(&self) -> Scale {
        self.scale
    }
    /// Returns the character set of the render options.
    #[inline(always)]
    pub fn character_set(&self) -> CharacterSet {
        self.char_set
    }
    /// Returns the color schema of the render options.
    #[inline(always)]
    pub fn color_schema(&self) -> ColorSchema {
        self.color_schema
    }
    /// Returns the luminance threshold of the render options.
    #[inline(always)]
    pub fn luminance_threshold(&self) -> f64 {
        self.luminance_threshold as f64 / 255.0
    }
    /// Sets the character set of the render options.
    #[inline(always)]
    pub fn set_character_set(&mut self, char_set: CharacterSet) {
        self.char_set = char_set;
    }
    /// Sets the color schema of the render options.
    #[inline(always)]
    pub fn set_color_schema(&mut self, color_schema: ColorSchema) {
        self.color_schema = color_schema;
    }
    /// Sets the luminance threshold of the render options.
    /// The value is a float between 0.0 and 1.0.
    #[inline(always)]
    pub fn set_luminance_threshold(&mut self, luminance_threshold: f64) {
        self.luminance_threshold = (luminance_threshold.clamp(0.0, 1.0) * 255.0) as u8;
    }
    /// Sets the scale of the render options.
    #[inline(always)]
    pub fn set_scale(&mut self, scale: Scale) {
        self.scale = scale;
    }
}

pub struct RenderOptionsBuilder {
    inner: RenderOptions,
}
impl RenderOptionsBuilder {
    /// Creates a new render options builder.
    #[inline(always)]
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
    /// Builds the render options.
    pub fn build(self) -> RenderOptions {
        self.inner
    }
    #[inline(always)]
    /// Sets the luminance threshold of the render options.
    /// The value is a float between 0.0 and 1.0.
    pub fn luminance_threshold(mut self, value: f64) -> Self {
        self.inner.luminance_threshold = (value.clamp(0.0f64, 1.0f64) * 255f64) as u8;
        self
    }
    #[inline(always)]
    /// Sets the scale of the render options.
    pub fn scale(mut self, scale: Scale) -> Self {
        self.inner.scale = scale;
        self
    }
    #[inline(always)]
    /// Sets the color schema of the render options.
    pub fn color_schema(mut self, color_schema: ColorSchema) -> Self {
        self.inner.color_schema = color_schema;
        self
    }
    #[inline(always)]
    /// Sets the character set of the render options.
    pub fn character_set(mut self, char_set: CharacterSet) -> Self {
        self.inner.char_set = char_set;
        self
    }
}
