use super::{Entry,Root};
use std::path::PathBuf;
use chrono::NaiveDateTime;  

pub(crate) struct Navigator {
}

impl crate::utils::Navigator<Entry,Root, PathBuf> for Navigator {
    fn entries(&self, _path: &PathBuf) -> Vec<Entry> {
        todo!()
    }

    fn roots(&self) -> Vec<Root> {
        todo!()
    }
    
    fn join(&self, path: &PathBuf, entry: &Entry) -> Option<PathBuf> {
        todo!()
    }
    
    fn exists(&self, path: &PathBuf) -> Option<bool> {
        todo!()
    }
    
}

