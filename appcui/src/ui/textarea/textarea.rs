use super::initialization_flags::Flags;

use crate::prelude::*;

struct Cursor {
    pos_x: usize,
    pos_y: usize,
}

struct Selection {
    pos_start: usize,
    pos_end: usize,
}

#[CustomControl(overwrite: [OnPaint, OnKeyPressed, OnMouseEvent, OnResize, OnFocus], internal=true)]
pub struct TextArea {
    flags: Flags,
    text: String,
    cursor: Cursor,
    selection: Selection,

    // Line sizes
    line_sizes: Vec<u32>,
    
    // Current position in the line
    row_offset: u32,

    // Current line
    line_offset: u32,
}

impl TextArea {

    fn get_absolute_position(&mut self) {
        let mut cursor_absolute_position = 0;
        for i in 0..self.line_offset + self.cursor.pos_y as u32 {
            cursor_absolute_position += self.line_sizes[i as usize];
        }
        cursor_absolute_position += self.row_offset + self.cursor.pos_x as u32;

        cursor_absolute_position;
    }

    pub fn new(text: &str, layout: Layout, flags: Flags) -> Self {
        let mut control = Self {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
            flags,
            text: text.to_string(),

            cursor: Cursor { pos_x: 0, pos_y: 0},
            selection: Selection {pos_start: 0, pos_end: 0},

            line_sizes: Vec::new(),
            row_offset: 0,
            line_offset: 0,
        };
        
        let mut current_offset = 0;
        for ch in control.text.as_bytes() {
            current_offset = current_offset + 1;
            if *ch == b'\n' {
                control.line_sizes.push(current_offset);
                current_offset = 0;
            }
        }
        if current_offset != 0 {
            control.line_sizes.push(current_offset);
        }

        for offset in &control.line_sizes {
            print!("Offset {offset}\n")
        }
        
        control
    }

    pub fn move_cursor_vertical(&mut self, no_of_rows: i32) {
        let new_position = self.cursor.pos_y as i32 + no_of_rows;
        
        if new_position < 0 {
            let update_offset = new_position + self.line_offset as i32; 

            self.cursor.pos_y = 0;

            if update_offset > 0 {
                self.line_offset = update_offset as u32;
            }
            else {
                self.line_offset = 0;
            }
        }
        else if new_position >= self.layout.get_height() as i32 {
            self.cursor.pos_y = self.layout.get_height() as usize - 1;
            
            let mut update_offset = new_position - self.layout.get_height() as i32 + 2 + self.line_offset as i32;
            
            if update_offset > self.line_sizes.len() as i32 - self.layout.get_height() as i32 {
                update_offset = self.line_sizes.len() as i32 - self.layout.get_height() as i32;
            }

            self.line_offset = update_offset as u32;
        }
        else {
            self.cursor.pos_y = new_position as usize;
            if self.line_offset + self.cursor.pos_y as u32 >= self.line_sizes.len() as u32 - 1 {
                self.cursor.pos_y = self.line_sizes.len() - self.line_offset as usize - 1;
            }
        }
    }

    pub fn move_cursor_horizontal(&mut self, no_collumns: i32) {
        let mut new_position = self.cursor.pos_x as i32 + self.row_offset as i32 + no_collumns;

        // The number of column left moves is bigger than the position in the line, needing to go upwards
        if new_position < 0 {
            // Current row, the offset of the first line printed + the row cursor position
            let mut current_row = self.line_offset + self.cursor.pos_y as u32;
            
            // Going up while we are still on a valid row
            while current_row > 0 && new_position < 0 {
                
                // Going a row above
                current_row -= 1;

                // Adding the row size of the current row 
                new_position = new_position + self.line_sizes[current_row as usize] as i32;
            }

            print!("New position ({row}, {pos})", row=current_row, pos=new_position);

            // Too much left move, setting to pos (0, 0) 
            if new_position < 0 {
                // Updating the cursor 
                self.cursor.pos_x = 0;
                self.cursor.pos_y = 0;
                
                // Updating the view as well
                self.line_offset = 0;
                self.row_offset = 0;
            }
            // We can change the position to pos (current_row, new_position)
            else {
                // We need to check if the line we landed on was already on the screen
                // If yes, we do not need to update the line_offset
                if current_row < self.line_offset {
                    self.line_offset = current_row;
                }
                // Updating the cursor position on vertical
                self.cursor.pos_y = current_row as usize - self.line_offset as usize;

                // We need to check if the row we landed on was already on the screen
                // The position is not on the screen, setting the cursor row_offset to the new position
                if !(new_position as u32 >= self.row_offset && new_position < self.layout.get_width() as i32 + self.row_offset as i32) {
                    if new_position < self.layout.get_width() as i32 {
                        self.row_offset = new_position as u32;
                    }
                    else {
                        self.row_offset = new_position as u32 - self.layout.get_width() as u32 + 1;
                    }
                }
                // Updating the cursor position on horizontal
                print!("Position {new}, Offset {off}", new=new_position, off=self.row_offset);
                self.cursor.pos_x = new_position as usize - self.row_offset as usize;
                print!("Cursor {c}", c=self.cursor.pos_x);
            }
        }
        // The number of column right moves will end up on a row below
        else if new_position as u32 >= self.line_sizes[self.line_offset as usize + self.cursor.pos_y as usize] {
            // Current row, the offset of the first line printed + the row cursor position
            let mut current_row = self.line_offset + self.cursor.pos_y as u32;

            // The number of position will needed, without the ones available in the current line which we know is not enough
            new_position = (new_position as u32 - self.line_sizes[current_row as usize]) as i32;
            
            // Going to the next row
            current_row += 1;
            // Going up while we are still on a valid row
            while current_row < self.line_sizes.len() as u32 {
                
                // Checking if the new position will land on the current row 
                if new_position <= self.line_sizes[current_row as usize] as i32 {
                    break;
                }
                // Otherwise, counting the positions before going a row below
                else {
                    new_position -= self.line_sizes[current_row as usize] as i32;
                }

                // Going a row below
                current_row += 1;
            }
            
            // Too much right move, setting to pos (last_line, last_char_in_line) 
            if current_row == self.line_sizes.len() as u32 {
                current_row = self.line_sizes.len() as u32 - 1;
                new_position = self.line_sizes[current_row as usize] as i32 - 1;
            }

            // Updating the line offset, which will be 0 if the text fits enterely on the screen, otherwise last_line - line_capacity 
            let tmp_line_offset = current_row as i32 - self.layout.get_height() as i32 + 1;
            if tmp_line_offset < 0 {
                self.line_offset = 0;
            }
            else {
                self.line_offset = tmp_line_offset as u32;
            }
            // Updating the cursor position vertically
            self.cursor.pos_y = current_row as usize - self.line_offset as usize;

            // Checking if th line fits enterely on the screen
            if (new_position as u32) < self.layout.get_width() as u32 {
                self.cursor.pos_x = new_position as usize;
                self.row_offset = 0;
            }
            // The line does not fit on the screen, we need to determine the offset for the line view
            else {
                let tmp_row_offset = new_position as i32 - self.layout.get_width() as i32;
                if tmp_row_offset < 0 {
                    self.row_offset = 0;
                }
                else {
                    self.row_offset = tmp_row_offset as u32;
                }
                self.cursor.pos_x = new_position as usize - self.row_offset as usize;
            }
        }
        // The line stays the same, updating only the row_offset
        else {
            
            // Checking if we need to update the row_offset
            // If the character is already on screen, no need to update

            // We consider a character a screen if row_offset <= new_position < row_offset + screen_width
            if !(self.row_offset <= new_position as u32 && (new_position as u32) < self.row_offset as u32 + self.layout.get_width() as u32) {
                
                // We need to update position, we will check where the direction on movement
                // Checking if the movement is to the left
                if self.row_offset > new_position as u32 {
                    let tmp_row_offset = new_position as i32 - self.layout.get_width() as i32;
                    if tmp_row_offset < 0 {
                        self.row_offset = new_position as u32;
                    }
                    else {
                        self.row_offset = tmp_row_offset as u32;
                    }
                }
                // The movement is to the right, we need to increase the offset
                else {
                    let tmp_row_offset = new_position as i32 - self.layout.get_width() as i32 + 1;
                    if tmp_row_offset < 0 {
                        self.row_offset = new_position as u32;
                    }
                    else {
                        self.row_offset = tmp_row_offset as u32;
                    }
                }
                
                print!("New offset {row_offset}, The new position is position is {pos} the width is {width}", row_offset=self.row_offset, pos=new_position, width=self.layout.get_width());
            }
            
            self.cursor.pos_x = new_position as usize - self.row_offset as usize;
        }
        print!("Row offset: {row}", row=self.row_offset);
    }

    pub fn insert_char(&mut self, character: char) {
        // First we need to calculate the absolute position in the text for the cursor
        // TODO: this seems slow to do considering text editing is the main purpose, we need to cache this information
        let mut cursor_absolute_position = 0;
        for i in 0..self.line_offset + self.cursor.pos_y as u32 {
            cursor_absolute_position += self.line_sizes[i as usize];
        }
        cursor_absolute_position += self.row_offset + self.cursor.pos_x as u32;
        // Inserting char, updating line size
        self.text.insert(cursor_absolute_position as usize, character);
        self.line_sizes[self.line_offset as usize + self.cursor.pos_y] += 1;
        
        // Move cursor to the newly added character
        self.move_cursor_horizontal(1);
    }

    pub fn remove_char_back(&mut self) {
        // First we need to calculate the absolute position in the text for the cursor
        let mut cursor_absolute_position = 0;
        for i in 0..self.line_offset + self.cursor.pos_y as u32 {
            cursor_absolute_position += self.line_sizes[i as usize];
        }
        cursor_absolute_position += self.row_offset + self.cursor.pos_x as u32;
        
        // If we press Backspace but we do not have any character to the left, we do nothing
        if cursor_absolute_position != 0 {
            // The position we need to remove
            cursor_absolute_position -= 1;

            // If its a newline, we need to merge 2 rows
            if self.text.as_bytes()[cursor_absolute_position as usize] == b'\n' {
                self.text.remove(cursor_absolute_position as usize);
                self.line_sizes[self.line_offset as usize + self.cursor.pos_y] -= 1;
                self.line_sizes[self.line_offset as usize + self.cursor.pos_y - 1] += self.line_sizes[self.line_offset as usize + self.cursor.pos_y];

                let tmp = self.line_sizes[self.line_offset as usize + self.cursor.pos_y];
                self.line_sizes.remove(self.line_offset as usize + self.cursor.pos_y);
                self.move_cursor_horizontal(-1 - tmp as i32);
            }
            else {
                self.text.remove(cursor_absolute_position as usize);
                self.line_sizes[self.line_offset as usize + self.cursor.pos_y] -= 1;
                self.move_cursor_horizontal(-1);
            }
        }
    }

    pub fn remove_char_front(&mut self) {
        let mut cursor_absolute_position = 0;
        for i in 0..self.line_offset + self.cursor.pos_y as u32 {
            cursor_absolute_position += self.line_sizes[i as usize];
        }
        cursor_absolute_position += self.row_offset + self.cursor.pos_x as u32;
        
        if cursor_absolute_position < self.text.len() as u32 - 1 {
            if self.text.as_bytes()[cursor_absolute_position as usize] == b'\n' {
                self.text.remove(cursor_absolute_position as usize);
                self.line_sizes[self.line_offset as usize + self.cursor.pos_y] -= 1;
                self.line_sizes[self.line_offset as usize + self.cursor.pos_y] += self.line_sizes[self.line_offset as usize + self.cursor.pos_y + 1];
                self.line_sizes.remove(self.line_offset as usize + self.cursor.pos_y + 1);
            }
            else {
                self.text.remove(cursor_absolute_position as usize);
                self.line_sizes[self.line_offset as usize + self.cursor.pos_y] -= 1;
            }
        }
    }

    pub fn insert_newline(&mut self) {
        // First we need to calculate the absolute position in the text for the cursor
        let mut cursor_absolute_position = 0;
        for i in 0..self.line_offset + self.cursor.pos_y as u32 {
            cursor_absolute_position += self.line_sizes[i as usize];
        }
        let position_in_line = self.row_offset + self.cursor.pos_x as u32;

        cursor_absolute_position += position_in_line;
        
        let line_index = self.line_offset as usize + self.cursor.pos_y;
        
        // Inserting the character for newline in the text, updating the line_size
        self.text.insert(cursor_absolute_position as usize, '\n');
        self.line_sizes[line_index] += 1;

        // Splitting the line into 2, [0, position] + (position, line_size]
        self.line_sizes.insert(line_index + 1, self.line_sizes[line_index] - position_in_line);
        self.line_sizes[line_index] = position_in_line;

        self.move_cursor_horizontal(1);
        self.move_cursor_horizontal(-1);
    }
}

impl OnPaint for TextArea {
    fn on_paint(&self, surface: &mut Surface, _theme: &Theme) {
        // TODO: to add scrollbar
        surface.clear(Character::new(' ', Color::Blue, Color::Green, CharFlags::None));
        
        let mut x = 0;
        let mut y = 0;


        let max_line_size = self.layout.get_width() as usize;
        let mut initial_offset = 0;
        
        for it in 0..self.line_sizes.len() {
            let current_offset = &self.line_sizes[it];

            // TODO: Replace layout with self.size()
            if it >= self.line_offset as usize + self.layout.get_height() as usize {
                break;
            }
            
            if it >= self.line_offset as usize {
                let current_line = &self.text[initial_offset as usize .. (initial_offset + *current_offset) as usize];
                
                if self.row_offset < current_line.len() as u32 {
                    let current_line_view = &current_line[self.row_offset as usize .. ];
                
                    let mut counter = 0;
                    for ch in current_line_view.chars() {
                        
                        if counter >= max_line_size {
                            break;
                        }

                        if ch == '\n' {
                            continue;
                        }

                        surface.write_char(x, y, Character::new(ch, Color::Blue, Color::Green, CharFlags::None));
                        x = x + 1;

                        counter = counter + 1;
                    }
                }

                y = y + 1;
                x = 0;
            }
            initial_offset = initial_offset + current_offset;
        }

        surface.set_cursor(self.cursor.pos_x as i32, self.cursor.pos_y as i32);
    }
}

impl OnFocus for TextArea {}

impl OnResize for TextArea {
    fn on_resize(&mut self, _old_size: Size, new_size: Size) {
        // TODO: cursor should always be on screen, when resize we should always focus on where the cursor is when key action happens
    }
}

impl OnKeyPressed for TextArea {
    fn on_key_pressed(&mut self, key: Key, character: char) -> EventProcessStatus {
        match key.value() {
            key!("Up") => {
                self.move_cursor_vertical(-1);
                return EventProcessStatus::Processed;
            }
            key!("Down") => {
                self.move_cursor_vertical(1);
                return EventProcessStatus::Processed;
            }
            key!("PageUp") => {
                self.move_cursor_vertical(1 - self.layout.get_height() as i32);
                return EventProcessStatus::Processed;
            }
            key!("PageDown") => {
                self.move_cursor_vertical(self.layout.get_height() as i32 - 1);
                return EventProcessStatus::Processed;
            }
            key!("Left") => {
                self.move_cursor_horizontal(-1);
                return EventProcessStatus::Processed;
            }
            key!("Right") => {
                self.move_cursor_horizontal(1);
                return EventProcessStatus::Processed;
            }
            
            // Test only for now
            key!("Ctrl+Left") | key!("Ctrl+Shift+Left") => {
                self.move_cursor_horizontal(-20);
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+Right") | key!("Ctrl+Shift+Right") => {
                self.move_cursor_horizontal(20);
                return EventProcessStatus::Processed;
            }

            key!("Back") => {
                self.remove_char_back();
                return EventProcessStatus::Processed;
            }
            key!("Delete") => {
                self.remove_char_front();
                return EventProcessStatus::Processed;
            }
            key!("Enter") => {
                self.insert_newline();
                return EventProcessStatus::Processed;
            }

            _ => {}
        }

        if (character as u32) > 0 {
            self.insert_char(character);
            return EventProcessStatus::Processed;
        }

        EventProcessStatus::Ignored
    }
}

impl OnMouseEvent for TextArea {}
