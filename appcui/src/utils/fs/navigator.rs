use super::{Entry, Root};
use chrono::DateTime;
use chrono::NaiveDateTime;
use std::fs;

use std::os::windows::fs::MetadataExt;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};



pub(crate) struct Navigator {   
}

impl crate::utils::Navigator<Entry, Root> for Navigator {
    fn entries(&self, path: &str) -> Vec<Entry> {       
        Self::get_folder_listing(path).unwrap_or_default()        
    }

    fn roots(&self) -> Vec<Root> {
        super::get_os_roots()
    }
    fn new() -> Self {
        Self {  }
    }
}

impl Navigator {
    fn get_folder_listing(path: &str) -> std::io::Result<Vec<Entry>> {
        let mut result: Vec<Entry> = vec![];
        let dir = Path::new(path);
        // Read the directory entries
        for dir_entry in fs::read_dir(dir)? {
            let entry = dir_entry?;            
            let metadata = entry.metadata()?;
            if let Some(utf8path) = entry.path().to_str() {
                // TODO: push only filename/foldername and
                // add terminating char for folders
                let entry = Self::get_entry_from_metadata(utf8path, &metadata)?;
                result.push(entry);                
            } 
        }
        result.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(result)
    }

    fn get_entry_from_metadata(path: &str, metadata: &std::fs::Metadata) -> std::io::Result<Entry> {
        let creation = metadata.created()?;
        let datetime = Self::system_time_to_naive_datetime(creation)?;
        let size = match metadata.is_dir() {
            false => metadata.file_size(),
            _ => 0,
        };
        Ok(Entry::new(path, size, datetime, metadata.is_dir()))
    }

    fn system_time_to_naive_datetime(system_time: SystemTime) -> std::io::Result<NaiveDateTime> {
        if let Ok(ds) = system_time.duration_since(UNIX_EPOCH) {
            if let Some(date) = DateTime::from_timestamp(ds.as_secs() as i64, ds.subsec_nanos()) {
                return Ok(date.naive_utc());
            }
        }
        Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid timestamp"))
    }    
}
