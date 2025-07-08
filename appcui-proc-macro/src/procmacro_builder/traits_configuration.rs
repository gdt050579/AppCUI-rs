use super::appcui_traits::{AppCUITrait, TraitType};

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub(crate) enum TraitImplementation {
    None,
    Default,
    BaseFallback,
    DefaultNonOverwritable,
    BaseFallbackNonOverwritable,
}
impl TraitImplementation {
    #[inline(always)]
    pub(crate) fn can_be_overwritten(&self) -> bool {
        match self {
            TraitImplementation::None => true,
            TraitImplementation::Default => true,
            TraitImplementation::BaseFallback => true,
            TraitImplementation::DefaultNonOverwritable => false,
            TraitImplementation::BaseFallbackNonOverwritable => false,
        }
    }
}
pub(crate) struct TraitsConfig {
    list: [TraitImplementation; TraitsConfig::MAX_ITEMS],
    name: &'static str,
}
pub(crate) struct TraitConfigIterator<'a> {
    config: &'a TraitsConfig,
    index: u8,
}
impl TraitsConfig {
    const MAX_ITEMS: usize = 64;
    pub(crate) fn new(name: &'static str) -> Self {
        Self {
            list: [TraitImplementation::None; TraitsConfig::MAX_ITEMS],
            name,
        }
    }
    pub(crate) fn set(&mut self, appcui_trait: AppCUITrait, implementation_mode: TraitImplementation) {
        let idx = (appcui_trait as u8) as usize;
        if idx > TraitsConfig::MAX_ITEMS {
            panic!(
                "Invalid trait ({:?}) - index is over {} -> {}. This is an internal error and shoul be fixed on AppCUIProcMacro side !",
                appcui_trait,
                TraitsConfig::MAX_ITEMS,
                idx
            );
        }
        if self.list[idx] != TraitImplementation::None {
            panic!(
                "Trait {appcui_trait:?} with index {idx} was already set up. Check to see if there are two traits with the same index !"
            );
        }
        self.list[idx] = implementation_mode;
    }
    pub(crate) fn clear(&mut self, appcui_trait: AppCUITrait) {
        let idx = (appcui_trait as u8) as usize;
        if idx > TraitsConfig::MAX_ITEMS {
            panic!(
                "Invalid trait ({:?}) - index is over {} -> {}. This is an internal error and shoul be fixed on AppCUIProcMacro side !",
                appcui_trait,
                TraitsConfig::MAX_ITEMS,
                idx
            );
        }
        self.list[idx] = TraitImplementation::None;
    }
    pub(crate) fn get(&self, appcui_trait: AppCUITrait) -> TraitImplementation {
        let idx = (appcui_trait as u8) as usize;
        if idx > TraitsConfig::MAX_ITEMS {
            panic!(
                "Invalid trait ({:?}) - index is over {} -> {}. This is an internal error and shoul be fixed on AppCUIProcMacro side !",
                appcui_trait,
                TraitsConfig::MAX_ITEMS,
                idx
            );
        }
        self.list[idx]
    }
    pub(crate) fn iter(&self) -> TraitConfigIterator {
        TraitConfigIterator { config: self, index: 0 }
    }
    #[inline(always)]
    pub(crate) fn get_name(&self) -> &'static str {
        self.name
    }
    pub(crate) fn traits_of_type(&self, trait_type: TraitType) -> String {
        let mut res = String::with_capacity(256);
        for i in 0u8..255u8 {
            if let Some(appcui_trait) = AppCUITrait::with_discriminant(i) {
                if (appcui_trait.trait_type() == trait_type) && (self.get(appcui_trait).can_be_overwritten()) {
                    if !res.is_empty() {
                        res.push(',');
                    }
                    res.push_str(appcui_trait.name());
                }
            }
        }
        res
    }
}
impl Iterator for TraitConfigIterator<'_> {
    type Item = (AppCUITrait, TraitImplementation);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(appcui_trait) = AppCUITrait::with_discriminant(self.index) {
            self.index += 1;
            Some((appcui_trait, self.config.get(appcui_trait)))
        } else {
            None
        }
    }
}
