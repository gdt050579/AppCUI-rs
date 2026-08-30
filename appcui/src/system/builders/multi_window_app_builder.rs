use super::impl_internal_builder_methods;
use super::InternalBuilder;

/// Builder for a multi-window AppCUI application.
pub struct MultiWindowAppBuilder {
    builder: InternalBuilder,
}

impl MultiWindowAppBuilder {
    pub(crate) fn new() -> Self {
        Self {
            builder: InternalBuilder::new(),
        }
    }

    pub(crate) fn from_internal(builder: InternalBuilder) -> Self {
        Self { builder }
    }

    impl_internal_builder_methods!();
}
