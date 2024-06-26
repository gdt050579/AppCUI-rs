use super::Column;

pub struct ColumnsHeader {
    columns: Vec<Column>,
}
impl ColumnsHeader {
    pub fn add(&mut self, column: Column) {
        self.columns.push(column);
        if self.columns.len() == 1 {
            self.columns[0].x = 0;
        } else {
            let last = self.columns.len() - 1;
            self.columns[last].x = self.columns[last - 1].x + 1 + self.columns[last - 1].width as i32;
        }
    }
}
