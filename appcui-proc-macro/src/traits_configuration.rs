use crate::appcui_traits::AppCUITraits;

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub(crate) enum TraitImplementation {
    None,
    Default,
    BaseFallback,
    DefaultNonOverwritable,
    BaseFallbackNonOverwritable,
}
pub(crate) struct TraitsConfig {
    list: [TraitImplementation; TraitsConfig::MAX_ITEMS],
}
impl TraitsConfig {
    const MAX_ITEMS: usize = 64;
    pub(crate) fn new() -> Self {
        Self {
            list: [TraitImplementation::None; TraitsConfig::MAX_ITEMS],
        }
    }
    pub(crate) fn set(
        &mut self,
        appcui_trait: AppCUITraits,
        implementation_mode: TraitImplementation,
    ) {
        let idx = (appcui_trait as u8) as usize;
        if idx > TraitsConfig::MAX_ITEMS {
            panic!("Invalid trait ({:?}) - index is over {} -> {}. This is an internal error and shoul be fixed on AppCUIProcMacro side !", appcui_trait,TraitsConfig::MAX_ITEMS, idx);
        }
        if self.list[idx] != TraitImplementation::None {
            panic!("Trait {:?} with index {} was already set up. Check to see if there are two traits with the same index !", appcui_trait, idx);
        }
        self.list[idx] = implementation_mode;
    }
    pub(crate) fn clear(&mut self, appcui_trait: AppCUITraits) {
        let idx = (appcui_trait as u8) as usize;
        if idx > TraitsConfig::MAX_ITEMS {
            panic!("Invalid trait ({:?}) - index is over {} -> {}. This is an internal error and shoul be fixed on AppCUIProcMacro side !", appcui_trait, TraitsConfig::MAX_ITEMS, idx);
        }
        self.list[idx] = TraitImplementation::None;
    }
    pub(crate) fn get(&self, appcui_trait: AppCUITraits)-> TraitImplementation {
        let idx = (appcui_trait as u8) as usize;
        if idx > TraitsConfig::MAX_ITEMS {
            panic!("Invalid trait ({:?}) - index is over {} -> {}. This is an internal error and shoul be fixed on AppCUIProcMacro side !", appcui_trait, TraitsConfig::MAX_ITEMS, idx);
        }    
        self.list[idx]   
    }
}
