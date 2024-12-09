use std::path::PathBuf;
use super::{Entry, Root};
pub(crate) struct NavSimulator {
    data: String,   
}
impl NavSimulator {
    pub(crate) fn with_csv(data: &str) -> Self {
        Self {
            data: data.to_string(),
        }
    }
}
impl crate::utils::Navigator<Entry,Root, PathBuf> for NavSimulator {
    fn entries(&self, path: &PathBuf) -> Vec<Entry> {
        let path = path.as_path().as_os_str().to_str().unwrap();
        let mut v = Vec::new();
        for line in self.data.lines() {
            if let Some(entry) = Entry::from_csv_line(line.trim(), path) {
                v.push(entry);
            }
        }  
        v 
    }
    fn roots(&self) -> Vec<Root> {
        let mut v = Vec::new();
        for line in self.data.lines() {
            if let Some(root) = Root::from_csv_line(line.trim()) {
                v.push(root);
            }
        }
        v
    }
    
    fn join(&self, path: &PathBuf, entry: &Entry) -> Option<PathBuf> {
        todo!()
    }
    
    fn exists(&self, path: &PathBuf) -> Option<bool> {
        todo!()
    }


}
