use super::{Entry,Root};
use chrono::NaiveDateTime;  

pub(crate) struct Navigator {
}

impl crate::utils::Navigator<Entry,Root> for Navigator {
    fn entries(&self, _path: &str) -> Vec<Entry> {
        todo!()
    }

    fn roots(&self) -> Vec<Root> {
        todo!()
    }
}

