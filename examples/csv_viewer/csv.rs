use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use appcui::prelude::*;

pub struct CSVEntry {
    pub data: Vec<String>,
}

impl ListItem for CSVEntry {
    fn render_method(&self, column_index: u16) -> Option<listview::RenderMethod> {
        if column_index < self.data.len() as u16 {
            Some(listview::RenderMethod::Text(&self.data[column_index as usize]))
        } else {
            None
        }
    }
}

pub struct CSVFile {
    pub headers: Vec<String>,
    pub entries: Vec<CSVEntry>,
}

impl CSVFile {
    fn split_line(line: &str) -> Vec<String> {
        let mut result = Vec::new();
        let mut current = String::new();
        let mut quotes = false;

        for c in line.chars() {
            match c {
                '"' => quotes = !quotes,
                ',' if !quotes => {
                    result.push(current);
                    current = String::new();
                }
                _ => {
                    current.push(c);
                }
            }
        }
        result.push(current);
        result
    }
    pub fn from_file(file_path: &Path) -> Option<Self> {
        let file = File::open(file_path).ok()?;
        let reader = BufReader::new(file);
        let mut headers = Vec::new();
        let mut entries = Vec::new();
        let mut columns_count = 0;

        for line in reader.lines().map_while(Result::ok) {
            if line.trim().is_empty() {
                continue;
            }
            if columns_count == 0 {
                headers = Self::split_line(&line);
                if headers.is_empty() {
                    return None;
                }
                columns_count = headers.len();
            } else {
                let result = Self::split_line(&line);
                if result.len() != columns_count {
                    return None;
                }
                entries.push(CSVEntry { data: result });
            }
        }

        Some(Self { headers, entries })
    }
}
