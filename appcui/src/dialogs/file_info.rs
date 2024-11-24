use chrono::NaiveDate;
use crate::prelude::*;

#[derive(ListViewItem)]
pub(super) struct FileInfo {
    #[Column(name = "&Name", width = 20)]
    name: String,
    #[Column(name = "&Size", width = 12, align = right, render = size, format = auto)]
    size: u64,
    #[Column(name = "&Created", width = 12, align = center, render = date, format = YearMonthDay)]
    created: NaiveDate,
}