use super::{Navigator, NavigatorEntry, NavigatorRoot};
use chrono::NaiveDateTime;  

pub(crate) struct FSNavigator {
}

#[derive(Debug)]
pub(crate) struct FSEntry {
    pub(crate) name: String,
    pub(crate) size: u64,
    pub(crate) created: NaiveDateTime,
    pub(crate) folder: bool,
}

pub(crate) struct FSRoot {
    pub(crate) name: String,
    pub(crate) size: u64,
    pub(crate) free_space: u64,
}

impl Navigator<FSEntry,FSRoot> for FSNavigator {
    fn entries(&self, _path: &str) -> Vec<FSEntry> {
        todo!()
    }

    fn roots(&self) -> Vec<FSRoot> {
        todo!()
    }
}
impl NavigatorEntry for FSEntry {
    fn name(&self) -> &str {
        &self.name
    }
    fn is_container(&self) -> bool {
        self.folder
    }
}
impl NavigatorRoot for FSRoot {
    fn name(&self) -> &str {
        &self.name
    }
}