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
impl crate::utils::Navigator<Entry,Root> for NavSimulator {
    fn entries(&self, path: &str) -> Vec<Entry> {
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
}
