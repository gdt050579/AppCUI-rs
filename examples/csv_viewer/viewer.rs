use std::path::Path;
use appcui::prelude::*;
use crate::CSVFile;
use crate::CSVEntry;


#[Window()]
pub struct Viewer {
   
}
impl Viewer {
    pub fn new(path: &Path, csv: CSVFile) -> Self {
        let mut w = Self {
            base: Window::new(path.to_str().unwrap_or("???"), layout!("a:c,w:50%,h:50%"), window::Flags::Sizeable),
        };
        let mut lv = listview!("CSVEntry,a:c,w:100%,h:100%,flags: SearchBar+ScrollBars,lsm:2");
        for h in &csv.headers {
            let column_width = (h.len() as u8).min(20);
            lv.add_column(Column::new(h, column_width + 2, TextAlignment::Left));
        }
        lv.add_items(csv.entries);
        w.add(lv);
        w
    }
}