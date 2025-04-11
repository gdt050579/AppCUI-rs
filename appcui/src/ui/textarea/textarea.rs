use super::initialization_flags::Flags;

use crate::prelude::*;
use std::{cmp::Ordering, fmt, mem::swap};

#[derive(Debug)]
struct Cursor {
    pos_x: usize,
    pos_y: usize,

    pressed: bool,
}

#[derive(Debug, PartialEq, Eq)]
enum SelectionDirection {
    None,
    Right,
    Left
}
impl fmt::Display for SelectionDirection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SelectionDirection::Left => write!(f, "Left"),
            SelectionDirection::Right => write!(f, "Right"),
            SelectionDirection::None => write!(f, "None")
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum CharacterType {
    None,
    AlphaNumeric,
    NonAlphaNumeric
}
#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Left,
    Right
}

#[derive(Debug)]
struct Selection {
    pos_start: usize,
    pos_end: usize,
    direction: SelectionDirection,
}

#[CustomControl(overwrite: [OnPaint, OnKeyPressed, OnMouseEvent, OnResize, OnFocus], internal=true)]
pub struct TextArea {
    flags: Flags,
    text: String,
    cursor: Cursor,
    selection: Selection,

    // Line sizes in bytes
    line_sizes: Vec<u32>,
    // Line sizes in characters
    line_character_counts: Vec<u32>,
    // Max line size, right now in bytes, needs to be changed to character counts
    max_line_size: u32,

    // LineNumberBar size
    line_number_bar_size: u32,
    
    // Current position in the line
    row_offset: u32,

    // Current line
    line_offset: u32,

    scrollbars: ScrollBars,
    scrollbar_x: u32,
    scrollbar_y: u32,

    cursor_position_x_backup: u32,

    mouse_x: u32,
    mouse_y: u32,

    window_width: u32
}

impl TextArea {

    fn get_current_character(bytes: &[u8], position: usize) -> Option<(usize, char)> {
        if position >= bytes.len() {
            return None; // Out of bounds
        }
    
        let first_byte = bytes[position];
    
        // Determine the number of bytes in the character
        let char_len = if first_byte & 0x80 == 0x00 {
            1 // 1-byte (ASCII)
        } else if first_byte & 0xE0 == 0xC0 {
            2 // 2-byte character
        } else if first_byte & 0xF0 == 0xE0 {
            3 // 3-byte character
        } else if first_byte & 0xF8 == 0xF0 {
            4 // 4-byte character
        } else {
            return None; // Invalid UTF-8 sequence start
        };
    
        if position + char_len > bytes.len() {
            return None; // Avoid out-of-bounds access
        }
    
        // Extract the UTF-8 character
        std::str::from_utf8(&bytes[position..position + char_len])
            .ok()
            .and_then(|s| s.chars().next().map(|c| (position + char_len, c)))
    }
    fn get_previous_character(bytes: &[u8], position: usize) -> Option<(usize, char)> {
        if position == 0 {
            return None;
        }

        let mut start = position - 1;

        while start > 0 && (bytes[start] & 0xC0) == 0x80 {
            start -= 1;
        }

        std::str::from_utf8(&bytes[start..position]).ok().and_then(|s| s.chars().next().map(|c| (start, c)))
    }
    fn get_start_of_word(&mut self, absolute_position_start: usize) -> (usize, i32) {

        let mut current_position = absolute_position_start;
        let bytes = self.text.as_bytes();

        let mut previous_type = CharacterType::None;
        let mut character_count = 0;

        while let Some((prev_pos, prev_char)) = Self::get_previous_character(bytes, current_position) {
            if prev_char.is_alphanumeric() {
                if previous_type == CharacterType::None {
                    previous_type = CharacterType::AlphaNumeric;
                }
                else if previous_type == CharacterType::NonAlphaNumeric {
                    return (current_position, character_count);
                }
            }
            else if previous_type == CharacterType::None {
                previous_type = CharacterType::NonAlphaNumeric;
            }
            else if previous_type == CharacterType::AlphaNumeric {
                return (current_position, character_count);
            }

            current_position = prev_pos;
            character_count -= 1;
        }

        (current_position, character_count)
    }
    fn get_end_of_word(&mut self, absolute_position_start: usize) -> (usize, i32) {

        let mut current_position = absolute_position_start;
        let bytes = self.text.as_bytes();

        let mut previous_type = CharacterType::None;
        let mut character_count = 0;

        while let Some((prev_pos, prev_char)) = Self::get_current_character(bytes, current_position) {
            if prev_char.is_alphanumeric() {
                if previous_type == CharacterType::None {
                    previous_type = CharacterType::AlphaNumeric;
                }
                else if previous_type == CharacterType::NonAlphaNumeric {
                    return (current_position, character_count);
                }
            }
            else if previous_type == CharacterType::None {
                previous_type = CharacterType::NonAlphaNumeric;
            }
            else if previous_type == CharacterType::AlphaNumeric {
                return (current_position, character_count);
            }

            current_position = prev_pos;
            character_count += 1;
        }

        (current_position, character_count)
    }
    #[inline(always)]
    fn move_to_edge_of_word(&mut self, direction: Direction) -> (usize, usize) {
        let absolute_position_inital = self.get_absolute_position() as usize;
        let _absolute_position;
        let char_count;

        if direction == Direction::Right {
            (_absolute_position, char_count) = self.get_end_of_word(absolute_position_inital);
        }
        else {
            (_absolute_position, char_count) = self.get_start_of_word(absolute_position_inital);
        }

        self.move_cursor_horizontal(char_count);
        let absolute_position_new = self.get_absolute_position() as usize;

        (absolute_position_inital, absolute_position_new)
    }

    pub fn move_cursor_vertical(&mut self, no_of_rows: i32) {

        // We are already on the first line, moving left to the max
        if self.line_offset as i32 + self.cursor.pos_y as i32 + no_of_rows < 0 {
            return self.move_cursor_horizontal(i32::MIN);
        }
        // We are already on the last line, moving right to the max
        else if  self.line_offset as i32 + self.cursor.pos_y as i32 + no_of_rows >= self.line_character_counts.len() as i32 {
            return self.move_cursor_horizontal(i32::MAX);
        }

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
        else if new_position >= self.size().height as i32 {
            self.cursor.pos_y = self.size().height as usize - 1;
            
            let mut update_offset = new_position - self.size().height as i32 + 1 + self.line_offset as i32;
            
            if update_offset > self.line_character_counts.len() as i32 - self.size().height as i32 {
                update_offset = self.line_character_counts.len() as i32 - self.size().height as i32;
            }

            self.line_offset = update_offset as u32;
        }
        else {
            self.cursor.pos_y = new_position as usize;
            if self.line_offset + self.cursor.pos_y as u32 >= self.line_character_counts.len() as u32 - 1 {
                self.cursor.pos_y = self.line_character_counts.len() - self.line_offset as usize - 1;
            }
        }
        
        // Checking if the cursor points to a valid character in line
        let current_cursor_position_x = self.row_offset as usize + self.cursor.pos_x;
        let current_cursor_position_y = self.line_offset as usize + self.cursor.pos_y;

        let position_diff_x = self.line_character_counts[current_cursor_position_y] as i32 - current_cursor_position_x as i32 - 1; // -1 for the newline
        
        log!("TextArea", "Position diff: {}", position_diff_x);
        log!("TextArea", "Cursor position x: {}, y: {}", self.cursor.pos_x, self.cursor.pos_y);
        log!("TextArea", "Line length: {}", self.line_character_counts[current_cursor_position_y]);

        // Cursor is at invalid position, repositioning and remembering to restore position
        if position_diff_x < 0 {
            let tmp_cursor_position_x_backup = self.cursor_position_x_backup;
            self.reposition_cursor();
            log!("TextArea", "Cursor new position x: {}, y: {}", self.cursor.pos_x, self.cursor.pos_y);
            self.cursor_position_x_backup = tmp_cursor_position_x_backup;
            self.cursor_position_x_backup += -position_diff_x as u32;
        }
        else if self.cursor_position_x_backup > 0 {
            if (position_diff_x as u32) >= self.cursor_position_x_backup {
                self.move_cursor_horizontal(self.cursor_position_x_backup as i32);
            }
            else {
                let tmp_cursor_position_x_backup = self.cursor_position_x_backup;
                self.move_cursor_horizontal(position_diff_x);
                self.cursor_position_x_backup = tmp_cursor_position_x_backup;
                self.cursor_position_x_backup -= position_diff_x as u32;
            }
        }
        log!("TextArea", "Cursor new position x: {}, y: {}", self.cursor.pos_x, self.cursor.pos_y);
        log!("TextArea", "Datorie: {}", self.cursor_position_x_backup);

        self.update_scrollbar_pos();
    }

    pub fn move_cursor_horizontal(&mut self, no_collumns: i32) {
        self.cursor_position_x_backup = 0;

        let current_position_in_line = self.cursor.pos_x as i32 + self.row_offset as i32;
        let mut new_position = (current_position_in_line).saturating_add(no_collumns);

        // The number of column left moves is bigger than the position in the line, needing to go upwards
        if new_position < 0 {
            // Current row, the offset of the first line printed + the row cursor position
            let mut current_row = self.line_offset + self.cursor.pos_y as u32;
            
            // Going up while we are still on a valid row
            while current_row > 0 && new_position < 0 {
                
                // Going a row above
                current_row -= 1;

                // Adding the row size of the current row 
                new_position += self.line_character_counts[current_row as usize] as i32;
            }

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
                if !(new_position as u32 >= self.row_offset && new_position < self.window_width as i32 + self.row_offset as i32) {
                    if new_position < self.window_width as i32 {
                        self.row_offset = new_position as u32;
                    }
                    else {
                        self.row_offset = new_position as u32 - self.window_width + 1;
                    }
                }
                // Updating the cursor position on horizontal
                self.cursor.pos_x = new_position as usize - self.row_offset as usize;
            }
        }
        // The number of column right moves will end up on a row below
        else if new_position as u32 >= self.line_character_counts[self.line_offset as usize + self.cursor.pos_y] {
            // Current row, the offset of the first line printed + the row cursor position
            let mut current_row = self.line_offset + self.cursor.pos_y as u32;

            // The number of position will needed, without the ones available in the current line which we know is not enough
            new_position = (new_position as u32 - self.line_character_counts[current_row as usize]) as i32;
            
            // Going to the next row
            current_row += 1;
            // Going up while we are still on a valid row
            while current_row < self.line_character_counts.len() as u32 {
                
                // Checking if the new position will land on the current row 
                if new_position <= self.line_character_counts[current_row as usize] as i32 {
                    break;
                }
                // Otherwise, counting the positions before going a row below
                else {
                    new_position -= self.line_character_counts[current_row as usize] as i32;
                }

                // Going a row below
                current_row += 1;
            }
            
            // Too much right move, setting to pos (last_line, last_char_in_line) 
            if current_row == self.line_character_counts.len() as u32 {
                current_row = self.line_character_counts.len() as u32 - 1;
                new_position = self.line_character_counts[current_row as usize] as i32 - 1;
            }

            // Updating the line offset, which will be 0 if the text fits enterely on the screen, otherwise last_line - line_capacity 
            let tmp_line_offset = current_row as i32 - self.size().height as i32 + 1;
            if tmp_line_offset < 0 {
                self.line_offset = 0;
            }
            else {
                self.line_offset = tmp_line_offset as u32;
            }
            // Updating the cursor position vertically
            self.cursor.pos_y = current_row as usize - self.line_offset as usize;

            // Checking if th line fits enterely on the screen
            if (new_position as u32) < self.window_width {
                self.cursor.pos_x = new_position as usize;
                self.row_offset = 0;
            }
            // The line does not fit on the screen, we need to determine the offset for the line view
            else {
                let tmp_row_offset = new_position - self.window_width as i32;
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
            if !(self.row_offset <= new_position as u32 && (new_position as u32) < self.row_offset + self.window_width) {
                
                // We need to update position, we will check where the direction on movement
                // Checking if the movement is to the left
                if self.row_offset > new_position as u32 {
                    let tmp_row_offset = new_position;
                    if tmp_row_offset < 0 {
                        self.row_offset = new_position as u32;
                    }
                    else {
                        self.row_offset = tmp_row_offset as u32;
                    }
                }
                // The movement is to the right, we need to increase the offset
                else {
                    let tmp_row_offset = new_position - self.size().width as i32 + 1;
                    if tmp_row_offset < 0 {
                        self.row_offset = new_position as u32;
                    }
                    else {
                        self.row_offset = tmp_row_offset as u32;
                    }
                }
            }
            
            self.cursor.pos_x = new_position as usize - self.row_offset as usize;
        }
        
        self.update_scrollbar_pos();
    }

    fn get_cursor_position_in_line(line_text: &str, row_offset: usize, cursor_index: usize, linebar_size: usize) -> usize {
        let cursor_offset = row_offset + cursor_index - linebar_size;

        let char_index = line_text.char_indices().nth(cursor_offset).map(|(i, _)| i);
        if let Some(bytes_index) = char_index {
            bytes_index
        }
        else {
            usize::MAX
        }
    }
    fn get_absolute_position(&mut self) -> u32 {
        let cursor_absolute_position;
        let _byte_index_in_line;

        (cursor_absolute_position, _byte_index_in_line) = self.get_absolute_position_verbose();

        cursor_absolute_position
    }
    fn get_absolute_position_verbose(&mut self) -> (u32, u32) {
        let mut cursor_absolute_position = 0;

        // Here I count the byte sizes of that lines above
        for i in 0..self.line_offset + self.cursor.pos_y as u32 {
            cursor_absolute_position += self.line_sizes[i as usize];
        }

        // Here I need the absolute position of that character, therefore I need to slice 
        // the text and extract the absolute position
        let line_text = &self.text[cursor_absolute_position as usize .. (cursor_absolute_position + self.line_sizes[self.line_offset as usize + self.cursor.pos_y]) as usize];
        
        let byte_index_in_line = Self::get_cursor_position_in_line(line_text, self.row_offset as usize, self.cursor.pos_x, 0);
        if byte_index_in_line != usize::MAX {
            cursor_absolute_position += byte_index_in_line as u32;
        }

        (cursor_absolute_position, byte_index_in_line as u32)
    }

    fn reset_selection(&mut self) {
        self.selection.pos_start = 0;
        self.selection.pos_end = 0;
        self.selection.direction = SelectionDirection::None;
    }

    fn reposition_cursor(&mut self) {
        // If the cursor is outside the line its supposed to be
        if self.cursor.pos_x + self.row_offset as usize >= self.line_character_counts[self.cursor.pos_y + self.line_offset as usize] as usize {
            let diff = (self.cursor.pos_x + self.row_offset as usize) - self.line_character_counts[self.cursor.pos_y + self.line_offset as usize] as usize + 1;
            log!("TextArea", "Repositioning diff: {}", 0 - diff as i32);
            self.move_cursor_horizontal(0 - diff as i32);
        }

        self.update_scrollbar_pos();
    }

    #[inline(always)]
    fn parse_text_in_lines(&mut self, text: &str, line_sizes: &mut Vec<u32>, line_character_counts: &mut Vec<u32>) {
        for line in text.lines() {
            log!("Info", "Print line: {}", line);
            line_sizes.push(line.len() as u32 + 1); // +1 for the \n we need to keep in mind
            line_character_counts.push(line.chars().count() as u32 + 1); // +1 for the \n we need to keep in mind
        }

        if text.ends_with("\n") {
            line_sizes.push(0);
            line_character_counts.push(0);
        } else {
            let line_count = line_character_counts.len();
            
            line_sizes[line_count - 1] -= 1;
            line_character_counts[line_count - 1] -= 1;
        }
    }

    #[inline(always)]
    fn update_max_line_size(&mut self) {
        if let Some(max_line_size) = self.line_sizes.iter().max() {
            self.max_line_size = *max_line_size;
        }
    }

    #[inline(always)]
    fn update_line_number_tab_size(&mut self) {
        if !self.flags.contains(Flags::ShowLineNumber) {
            return;
        }

        let mut temp_size = 0;
        let mut temp_line_number = self.line_sizes.len();    
        while temp_line_number > 0 {
            temp_size += 1;
            temp_line_number /= 10;
        }
        temp_size += 1;

        if self.line_number_bar_size != temp_size {
            self.line_number_bar_size = temp_size;
            self.update_window_width();
        }
    }

    #[inline(always)]
    fn update_window_width(&mut self) {
        if self.size().width < self.line_number_bar_size {
            self.window_width = 0;
            return
        }
        self.window_width = self.size().width - self.line_number_bar_size;
    }

    pub fn new(text: &str, layout: Layout, flags: Flags) -> Self {
        let mut control = Self {
            base: ControlBase::with_status_flags(
                layout,
                (StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput)
                    | if flags.contains(Flags::ScrollBars) {
                        StatusFlags::IncreaseBottomMarginOnFocus | StatusFlags::IncreaseRightMarginOnFocus
                    } else {
                        StatusFlags::None
                    },
            ),
            flags,
            text: text.to_string().replace('\t', "    ").replace("\r\n", "\n"),

            cursor: Cursor { pos_x: 0, pos_y: 0, pressed: false},
            selection: Selection {pos_start: 0, pos_end: 0, direction: SelectionDirection::None},

            line_sizes: Vec::new(),
            line_character_counts: Vec::new(),
            max_line_size: 0,

            row_offset: 0,
            line_offset: 0,

            // LineNumber tab size
            line_number_bar_size: 0,

            scrollbars: ScrollBars::new(flags.contains(Flags::ScrollBars)),
            scrollbar_x: 0,
            scrollbar_y: 0,

            cursor_position_x_backup: 0,

            mouse_x: 0,
            mouse_y: 0,

            window_width: 0
        };

        if !flags.contains(Flags::ShowLineNumber) {
            control.line_number_bar_size = 0;
        }

        if !control.text.ends_with('\n') {
            control.text += "\n";
        }

        for line in text.lines() {
            control.line_sizes.push(line.len() as u32 + 1); // +1 for the \n we need to keep in mind
            control.line_character_counts.push(line.chars().count() as u32 + 1); // +1 for the \n we need to keep in mind
        }

        control.update_max_line_size();
        control.update_line_number_tab_size();
        control.update_window_width();
        
        control
    }

    #[inline(always)]
    fn update_scrollbar_data(&mut self) {
        // Setting the new scrollbar values as the current coordinates
        self.scrollbar_x = self.scrollbars.horizontal_index() as u32;
        self.scrollbar_y = self.scrollbars.vertical_index() as u32;
    }
    #[inline(always)]
    fn update_scrollbar_pos(&mut self) {
        self.scrollbars.set_indexes((self.row_offset as usize) as u64, (self.line_offset as usize) as u64);
        self.update_scrollbar_data();
    }
    fn update_view_from_scrollbars(&mut self, vertical: i32, horizontal: i32) {
        // In this function we need to change the horizontal and vertical view, 
        // without changing the position of the cursor, if possible. If the cursor is out of view,
        // it will be set on the first viewed character of the line if moving to right or the 
        // last if moving to left. Same applies for moving vertically

        if horizontal != 0 {
            let current_cursor_position = self.row_offset + self.cursor.pos_x as u32;

            // Moving to right
            if horizontal > 0 {

                // Updating the view
                self.row_offset += horizontal as u32;

                // Checking if the characer marked by cursor is out of screen right now
                if current_cursor_position < self.row_offset {
                    self.cursor.pos_x = 0;
                }
                // the characer marked by cursor is still on screen, needs repositioning
                else {
                    self.cursor.pos_x -= horizontal as usize;
                }
            }
            // Moving to left
            else {
                let tmp_moves_needed = (-horizontal) as u32;
                // Updating the view
                self.row_offset -= tmp_moves_needed;

                // Checking if the characer marked by cursor is out of screen right now
                if current_cursor_position >= self.row_offset + self.window_width {
                    self.cursor.pos_x = self.window_width as usize - 1;
                }
                // the characer marked by cursor is still on screen, needs repositioning
                else {
                    self.cursor.pos_x += tmp_moves_needed as usize;
                }
            }
        }
        if vertical != 0 {
            let current_cursor_position = self.line_offset + self.cursor.pos_y as u32;

            // Moving down
            if vertical > 0 {

                // Updating the view
                self.line_offset += vertical as u32;

                // Checking if the characer marked by cursor is out of screen right now
                if current_cursor_position < self.line_offset {
                    self.cursor.pos_y = 0;
                }
                // the characer marked by cursor is still on screen, needs repositioning
                else {
                    self.cursor.pos_y -= vertical as usize;
                }
            }
            // Moving up
            else {
                let tmp_moves_needed = (-vertical) as u32;
                // Updating the view
                self.line_offset -= tmp_moves_needed;

                // Checking if the characer marked by cursor is out of screen right now
                if current_cursor_position >= self.line_offset + self.size().height {
                    self.cursor.pos_y = self.size().height as usize - 1;
                }
                // the characer marked by cursor is still on screen, needs repositioning
                else {
                    self.cursor.pos_y += tmp_moves_needed as usize
                }
            }
        }
    }

    pub fn remove_char_back(&mut self) {
        // First we need to calculate the absolute position in the text for the cursor
        let cursor_absolute_position_initial = self.get_absolute_position() as usize;
        
        // If we press Backspace but we do not have any character to the left, we do nothing
        if cursor_absolute_position_initial != 0 {
            // The position we need to remove
            self.move_cursor_horizontal(-1);
            let cursor_absolute_position_new = self.get_absolute_position() as usize;
            
            log!("Info", "Removing data from index {} to {}", cursor_absolute_position_new, cursor_absolute_position_initial);

            let char_size = cursor_absolute_position_initial - cursor_absolute_position_new;

            log!("Info", "Removed text: {}", &self.text[cursor_absolute_position_new..cursor_absolute_position_initial]);

            // If its a newline, we need to merge 2 rows
            if self.text.as_bytes()[cursor_absolute_position_new] == b'\n' {   
                log!("Info", "Removing newline");

                self.line_sizes[self.line_offset as usize + self.cursor.pos_y] -= char_size as u32;
                self.line_sizes[self.line_offset as usize + self.cursor.pos_y] += self.line_sizes[self.line_offset as usize + self.cursor.pos_y + 1];
                self.line_sizes.remove(self.line_offset as usize + self.cursor.pos_y + 1);

                self.line_character_counts[self.line_offset as usize + self.cursor.pos_y] -= 1;
                self.line_character_counts[self.line_offset as usize + self.cursor.pos_y] += self.line_character_counts[self.line_offset as usize + self.cursor.pos_y + 1];
                self.line_character_counts.remove(self.line_offset as usize + self.cursor.pos_y + 1);
                
            }
            else {
                self.line_sizes[self.line_offset as usize + self.cursor.pos_y] -= char_size as u32;
                self.line_character_counts[self.line_offset as usize + self.cursor.pos_y] -= 1;
            }

            self.text.drain(cursor_absolute_position_new..cursor_absolute_position_initial);
            
            log!("Info", "Text after deletion: {}", self.text);
            
            self.update_line_number_tab_size();
            self.update_max_line_size();
            self.update_scrollbar_pos();
        }
    }

    pub fn remove_char_front(&mut self) {

        let current_line_number = self.line_offset as usize + self.cursor.pos_y;
        if current_line_number == self.line_sizes.len() - 1 {
            let position_in_line = self.row_offset as usize + self.cursor.pos_x;

            if self.line_character_counts[current_line_number] == 0 || position_in_line == self.line_character_counts[current_line_number] as usize - 1 {
                return;
            }
        }
        self.move_cursor_horizontal(1);
        self.remove_char_back();
    }

    fn update_scroll_pos_from_scrollbars(&mut self) {
        // Calculating the direction of movement as a difference between the current coordinates
        // and the new scrollbar values
        let new_pos_vertical = self.scrollbars.vertical_index() as i32 - self.scrollbar_y as i32;
        let new_pos_horizontal = self.scrollbars.horizontal_index() as i32 - self.scrollbar_x as i32;

        log!("Info", "Scrollbar data y: {}, x: {}", self.scrollbars.vertical_index(), self.scrollbars.horizontal_index());
        log!("Info", "Scrollbar update direction V={}, H={}", new_pos_vertical, new_pos_horizontal);

        // Updating the view based on the direction
        self.update_view_from_scrollbars(new_pos_vertical, new_pos_horizontal);
        self.update_scrollbar_data();
    }

    pub fn remove_text_selection(&mut self, pos_start: usize, pos_end: usize) {
        // Here we should reverse from absolut_position -> relative line position in order to modify our lines
        // If the remove is multiline, we should remove lines in between, cut and merge first and last line

        // aaaaaaaa         aaaaaaaa        aaaaa            aaaaabbbb
        // xxxxxxxx
        // xxxxxxxx     =>              =>              =>  
        // xxxxxxxx
        // bbbbbbbb         bbbbbbbb            bbbb

        // Return if range is invalid 
        if pos_start >= pos_end {
            return;
        }

        let mut position_start_x = 0;
        let mut position_start_y = 0;
        let mut position_end_x = 0;
        let mut position_end_y = 0;

        let mut counter = 0;
        let mut line_iterator = 0;

        while counter < pos_end && line_iterator < self.line_sizes.len() {
            
            if counter <= pos_start && pos_start < counter + self.line_sizes[line_iterator] as usize {
                position_start_y = line_iterator;
                position_start_x = pos_start - counter; 
            }

            if counter <= pos_end && pos_end < counter + self.line_sizes[line_iterator] as usize {
                position_end_y = line_iterator;
                position_end_x = pos_end - counter; 
            }

            counter += self.line_sizes[line_iterator] as usize;
            line_iterator += 1;
        }

        // If the deletion is requested on a single line
        if position_start_y == position_end_y {
            self.line_sizes[position_start_y] -= (position_end_x - position_start_x) as u32;
        }
        // The deletion request is multiline
        else {
            // Cut the first line to 0..position_start_x
            self.line_sizes[position_start_y] = position_start_x as u32;

            // Cut the last line to [position_end_x..]
            self.line_sizes[position_end_y] -= position_end_x as u32;

            // Remove the lines in between
            self.line_sizes.drain(position_start_y + 1..position_end_y);

            // Merge the two lines together
            self.line_sizes[position_start_y] += self.line_sizes[position_start_y + 1];
            self.line_sizes.remove(position_start_y + 1);
        }
        // Remove the selected text
        self.text.drain(pos_start..pos_end);
        
        // Reposition the cursor to the start of the selection
        
        log!("Info", "Starting position: ({}, {})", position_start_x, position_start_y);
        log!("Info", "Cursor position: ({}, {})", self.cursor.pos_x, self.cursor.pos_y);
        log!("Info", "line_offset: {}, row_offset: {}", self.line_offset, self.row_offset);
        // Check if cursor can be position in current view for horizontal
        if self.row_offset <= position_start_x as u32 && (position_start_x as u32) < self.row_offset + self.window_width {
            self.cursor.pos_x = position_start_x - self.row_offset as usize;
        }
        // It does not, updating the view
        else {
            self.cursor.pos_x = 0;
            self.row_offset = position_start_x as u32;
        }

        // Check if cursor can be position in current view for vertical
        if self.line_offset <= position_start_y as u32 && (position_start_y as u32) < self.line_offset + self.size().height {
            self.cursor.pos_y = position_start_y - self.line_offset as usize; 
        }
        // It does not, updating the view
        else {
            self.cursor.pos_y = 0;
            self.line_offset = position_start_y as u32;
        }

        self.update_scrollbar_pos();
    }

    #[inline(always)]
    fn update_lines_after_insert(destination_line_sizes: &mut Vec<u32>, line_index: usize, insert_index: u32, new_text_line_sizes: &[u32]) {
        // Splitting the line into 2, [0, position] + (position , line_size]
        destination_line_sizes.insert(line_index + 1, destination_line_sizes[line_index] - insert_index);
        destination_line_sizes[line_index] = insert_index;
        
        // Adding the first line of the inserted text to the original position
        destination_line_sizes[line_index] += new_text_line_sizes[0];

        // Adding the last line of the inserted text to the newly added line
        destination_line_sizes[line_index + 1] += new_text_line_sizes[new_text_line_sizes.len() - 1];
        
        // Inserting the new text in between the spliced line
        destination_line_sizes.splice(line_index + 1..line_index + 1, new_text_line_sizes[1 .. new_text_line_sizes.len() - 1].iter().cloned());
    }
    pub fn insert_text(&mut self, text: &str) {
        if text.contains('\n') {
            // We need to calculate the absolute position in the text for the cursor and the position in line
            let cursor_absolute_position;
            let byte_index_in_line;
            (cursor_absolute_position, byte_index_in_line) = self.get_absolute_position_verbose();

            let line_index = self.line_offset as usize + self.cursor.pos_y;
            let character_index = self.row_offset as usize + self.cursor.pos_x;

            log!("Info", "Insert text after: {}", &self.text[0 .. cursor_absolute_position as usize]);
            // Adding the text given as parameter
            self.text.insert_str(cursor_absolute_position as usize, text);

            // First we parse the text in lines
            let mut line_sizes : Vec<u32> = Vec::new();
            let mut line_character_counts : Vec<u32> = Vec::new();
            self.parse_text_in_lines(text, &mut line_sizes, &mut line_character_counts);

            for it in 0..line_sizes.len() {
                log!("Info", "Line size {}: {}", it, line_sizes[it]);
                log!("Info", "Line char count {}: {}", it, line_character_counts[it]);
            }

            TextArea::update_lines_after_insert(&mut self.line_sizes, line_index, byte_index_in_line, &line_sizes);
            TextArea::update_lines_after_insert(&mut self.line_character_counts, line_index, character_index as u32, &line_character_counts);
        }
        else {
            let cursor_absolute_position = self.get_absolute_position();
            let line_index = self.line_offset as usize + self.cursor.pos_y;

            // Adding the text given as parameter
            self.text.insert_str(cursor_absolute_position as usize, text);

            self.line_sizes[line_index] += text.len() as u32;
            self.line_character_counts[line_index] += text.chars().count() as u32;

            log!("Info", "Moving cursor by {} horizontal", text.chars().count());
        }

        self.update_max_line_size();
        self.update_line_number_tab_size();
        self.update_scrollbar_pos();
    }

    #[inline(always)]
    fn update_selection(&mut self, absolute_position_inital: usize, absolute_position_new: usize, direction: SelectionDirection) {
        if direction == SelectionDirection::None {
            return;
        }
        
        // Checking how the selection was initially started so we understand how to process it
                
        // the selection is already to the left, so we expand the selection
        if self.selection.direction == SelectionDirection::Left {
            self.selection.pos_start = absolute_position_new;

            // by moving our selection to the right, the selection is now sized at 0, 
            // so we change it to None, in order to be able to switch directions
            match self.selection.pos_start.cmp(&self.selection.pos_end) {
                Ordering::Equal => {
                    self.selection.direction = SelectionDirection::None;
                }
                Ordering::Greater => {
                    swap(&mut self.selection.pos_start, &mut self.selection.pos_end);
                    self.selection.direction = SelectionDirection::Right;
                }
                Ordering::Less => {}
            }
        }
        // the selection is to the right, so we must reduce our selection
        else if self.selection.direction == SelectionDirection::Right {
            self.selection.pos_end = absolute_position_new;

            // by moving our selection to the right, the selection is now sized at 0, 
            // so we change it to None, in order to be able to switch directions
            match self.selection.pos_start.cmp(&self.selection.pos_end) {
                Ordering::Equal => {
                    self.selection.direction = SelectionDirection::None;
                }
                Ordering::Greater => {
                    swap(&mut self.selection.pos_start, &mut self.selection.pos_end);
                    self.selection.direction = SelectionDirection::Left;
                }
                Ordering::Less => {}
            }
        }
        // there was no previous selection, so we create it now
        else {
            self.selection.direction = direction;

            if self.selection.direction == SelectionDirection::Left {
                self.selection.pos_end = absolute_position_inital;
                self.selection.pos_start = absolute_position_new;
            }
            else {
                self.selection.pos_start = absolute_position_inital;
                self.selection.pos_end = absolute_position_new;
            }
        }

        // print!("Selection start: {}, end: {}, direction: {}\n", self.selection.pos_start, self.selection.pos_end, self.selection.direction);              
    }

    #[inline(always)]
    fn save_mouse_data(&mut self, mouse_data: &MouseEventData) {
        // Updating the view based on the direction
        self.mouse_x = std::cmp::min(std::cmp::max(mouse_data.x, 0) as u32, self.window_width - 1);
        self.mouse_y = std::cmp::min(std::cmp::max(mouse_data.y, 0) as u32, self.size().height - 1);
    }

    fn set_cursor_pos_from_mouse(&mut self, mouse_data: &MouseEventData) {
        self.save_mouse_data(mouse_data);

        self.cursor.pos_x = self.mouse_x as usize;
        if self.flags.contains(Flags::ShowLineNumber) {
            self.cursor.pos_x = self.cursor.pos_x.saturating_sub(self.line_number_bar_size as usize);
        }
        self.cursor.pos_y = self.mouse_y as usize;

        self.reposition_cursor();
    }
    fn update_cursor_pos_from_mouse(&mut self, mouse_data: &MouseEventData) {
        // Calculating the direction of movement as a difference between the current coordinates
        let new_pos_vertical = mouse_data.y - self.mouse_y as i32;
        let mut new_pos_horizontal = mouse_data.x - self.mouse_x as i32;

        self.move_cursor_vertical(new_pos_vertical);
        let current_cursor_position_x = self.row_offset + self.cursor.pos_x as u32;
        let current_cursor_position_y = self.line_offset + self.cursor.pos_y as u32;
        if new_pos_horizontal < 0 {
            if -new_pos_horizontal > current_cursor_position_x as i32 {
                new_pos_horizontal = -(current_cursor_position_x as i32);
            }
        }
        else if new_pos_horizontal as u32 + current_cursor_position_x >= self.line_character_counts[current_cursor_position_y as usize] {
            new_pos_horizontal = (self.line_character_counts[current_cursor_position_y as usize] - current_cursor_position_x) as i32 - 1;
        }
        // Ignore mouse drag to the right over the linebar
        else if mouse_data.x <= self.line_number_bar_size as i32 {
            new_pos_horizontal = 0;
        }

        self.move_cursor_horizontal(new_pos_horizontal);

        self.save_mouse_data(mouse_data);
    }

    #[inline(always)]
    pub fn text(&self) -> &str {
        &self.text
    }

}

impl OnPaint for TextArea {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        if (self.has_focus()) && (self.flags.contains(Flags::ScrollBars)) {
            self.scrollbars.paint(surface, theme, self);
            surface.reduce_clip_by(0,0,1,1);
        }

        let (attr_text, attr_selection) = match (){
            _ if !self.is_enabled() => (theme.editor.inactive, theme.editor.inactive),
            _ if self.has_focus() => (theme.editor.focused, theme.editor.pressed_or_selectd),
            _ if self.is_mouse_over() => (theme.editor.hovered, theme.editor.hovered),
            _ => (theme.editor.normal, theme.editor.normal)
        };
        let attr_line_number = theme.editor.inactive;

        // surface.clear(Character::new(' ', Color::Black, Color::DarkBlue, CharFlags::None));
        surface.clear(Character::with_attributes(' ', attr_text));

        let mut x = 0;
        let mut y = 0;

        log!("TextArea - OnPaint", "line_offset: {}, row_offset: {}", self.line_offset, self.row_offset);
        
        // We reserve the necessary space for showing the line number, which will be dynamically calculated
        // using the number of lines the text has + 1 as a divider for the text
        if self.flags.contains(Flags::ShowLineNumber) {
            x = self.line_number_bar_size as i32;
        }

        let max_line_size = self.window_width as usize;
        let mut initial_offset = 0;
        
        for it in 0..self.line_sizes.len() {
            let current_offset = &self.line_sizes[it];

            if it >= self.line_offset as usize + self.size().height as usize {
                break;
            }

            if initial_offset + *current_offset > self.text.len() as u32 {
                log!("Error", "Attempted to access string data from initial {} + current {} while size: {}", initial_offset, *current_offset, self.text.len());
                break;
            }
            
            if it >= self.line_offset as usize {
                let current_line = &self.text[initial_offset as usize .. (initial_offset + *current_offset) as usize];
                
                if self.flags.contains(Flags::ShowLineNumber) {
                    let line_number_text = (it + 1).to_string();
                    let mut offset = self.line_number_bar_size as i32 - line_number_text.len() as i32 - 1;

                    for ch in line_number_text.chars() {
                        surface.write_char(offset, y, Character::with_attributes(ch, attr_line_number));
                        offset += 1;
                    }
                }
                

                if self.row_offset < current_line.len() as u32 {
                    let current_line_view = &current_line[self.row_offset as usize .. ];
                
                    let mut counter = x as usize;
                    for (ch_index, ch) in current_line_view.char_indices() {
                        
                        if counter >= max_line_size {
                            break;
                        }

                        if ch == '\n' {
                            continue;
                        }

                        let absolute_position = initial_offset + ch_index as u32;

                        if self.selection.pos_start <= absolute_position as usize && (absolute_position as usize) < self.selection.pos_end {
                            surface.write_char(x, y, Character::with_attributes(ch, attr_selection));
                        }                        
                        else if self.flags.contains(Flags::HighlightCursor) && y as usize == self.cursor.pos_y {
                            surface.write_char(x, y, Character::with_attributes(ch, attr_line_number));
                        }
                        else {
                            surface.write_char(x, y, Character::with_attributes(ch, attr_text));
                        }
                        
                        x += 1;

                        counter += 1;
                    }
                }

                y += 1;
                x = 0;
                if self.flags.contains(Flags::ShowLineNumber) {
                    x = self.line_number_bar_size as i32;
                }
            }
            initial_offset += current_offset;
        }

        surface.set_cursor(self.cursor.pos_x as i32 + self.line_number_bar_size as i32, self.cursor.pos_y as i32);
    }
}

impl OnFocus for TextArea {}

impl OnResize for TextArea {
    fn on_resize(&mut self, _old_size: Size, _new_size: Size) {
        self.scrollbars.resize(self.max_line_size as u64, self.line_sizes.len() as u64, &self.base);
        
        // TODO: cursor should always be on screen, when resize we should always focus on where the cursor is when key action happens
        let tmp_row_offset = self.row_offset as i32 + self.cursor.pos_x as i32;
        self.move_cursor_horizontal(0 - tmp_row_offset);
        self.move_cursor_horizontal(tmp_row_offset);

        let tmp_line_offset = self.line_offset as i32 + self.cursor.pos_y as i32;
        self.move_cursor_vertical(0 - tmp_line_offset);
        self.move_cursor_vertical(tmp_line_offset);

        self.update_window_width();
    }
}
  
impl OnKeyPressed for TextArea {
    fn on_key_pressed(&mut self, key: Key, character: char) -> EventProcessStatus {
        match key.value() {
            key!("Up") => {
                self.move_cursor_vertical(-1);

                self.reset_selection();

                return EventProcessStatus::Processed;
            }
            key!("Down") => {
                self.move_cursor_vertical(1);

                self.reset_selection();
                
                return EventProcessStatus::Processed;
            }
            key!("PageUp") => {
                self.move_cursor_vertical(1 - self.size().height as i32);

                self.reset_selection();

                return EventProcessStatus::Processed;
            }
            key!("PageDown") => {
                self.move_cursor_vertical(self.size().height as i32 - 1);

                self.reset_selection();

                return EventProcessStatus::Processed;
            }
            key!("Left") => {
                self.reposition_cursor();

                self.move_cursor_horizontal(-1);
                
                self.reset_selection();

                return EventProcessStatus::Processed;
            }
            key!("Right") => {
                self.reposition_cursor();

                self.move_cursor_horizontal(1);

                self.reset_selection();

                return EventProcessStatus::Processed;
            }
            
            key!("Shift+Right") => {
                // Making sure our cursor is in Focus
                self.reposition_cursor();
                let absolute_position_inital = self.get_absolute_position() as usize;
                self.move_cursor_horizontal(1);
                let absolute_position_new = self.get_absolute_position() as usize;
                
                self.update_selection(absolute_position_inital, absolute_position_new, SelectionDirection::Right);
                
                log!("Info", "Selection Start: {}, End: {}, Direction: {}", self.selection.pos_start, self.selection.pos_end, self.selection.direction);
                return EventProcessStatus::Processed;
            }
            key!("Shift+Left") => {
                // Making sure our cursor is in Focus
                self.reposition_cursor();
                let absolute_position_inital = self.get_absolute_position() as usize;
                self.move_cursor_horizontal(-1);
                let absolute_position_new = self.get_absolute_position() as usize;

                self.update_selection(absolute_position_inital, absolute_position_new, SelectionDirection::Left);

                log!("Info", "Selection Start: {}, End: {}, Direction: {}", self.selection.pos_start, self.selection.pos_end, self.selection.direction);
                return EventProcessStatus::Processed;
            }
            key!("Shift+Up") => {
                // Making sure our cursor is in Focus 
                self.move_cursor_vertical(0);
                let absolute_position_inital = self.get_absolute_position() as usize;
                self.move_cursor_vertical(-1);
                let absolute_position_new = self.get_absolute_position() as usize;

                self.update_selection(absolute_position_inital, absolute_position_new, SelectionDirection::Right);

                log!("Info", "Selection Start: {}, End: {}, Direction: {}", self.selection.pos_start, self.selection.pos_end, self.selection.direction);
                return EventProcessStatus::Processed;
            }
            key!("Shift+Down") => {
                // Making sure our cursor is in Focus 
                self.move_cursor_vertical(0);
                let absolute_position_inital = self.get_absolute_position() as usize;
                self.move_cursor_vertical(1);
                let absolute_position_new = self.get_absolute_position() as usize;

                self.update_selection(absolute_position_inital, absolute_position_new, SelectionDirection::Right);
     

                log!("Info", "Selection Start: {}, End: {}, Direction: {}", self.selection.pos_start, self.selection.pos_end, self.selection.direction);
                return EventProcessStatus::Processed;
            }
            
            key!("Ctrl+Left") => {
                // Making sure our cursor is in Focus
                self.reposition_cursor();
                
                let (_absolute_position_inital, _absolute_position_new) = self.move_to_edge_of_word(Direction::Left);

                return EventProcessStatus::Processed;
            }
            key!("Ctrl+Right") => {
                // Making sure our cursor is in Focus
                self.reposition_cursor();
                
                let (_absolute_position_inital, _absolute_position_new) = self.move_to_edge_of_word(Direction::Right);
                
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+Shift+Left") => {
                // Making sure our cursor is in Focus
                self.reposition_cursor();
                
                let (absolute_position_inital, absolute_position_new) = self.move_to_edge_of_word(Direction::Left);
                self.update_selection(absolute_position_inital, absolute_position_new, SelectionDirection::Left);
                
                log!("Info", "Selection Start: {}, End: {}, Direction: {}", self.selection.pos_start, self.selection.pos_end, self.selection.direction);
                
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+Shift+Right") => {
                // Making sure our cursor is in Focus
                self.reposition_cursor();
                
                let (absolute_position_inital, absolute_position_new) = self.move_to_edge_of_word(Direction::Right);
                self.update_selection(absolute_position_inital, absolute_position_new, SelectionDirection::Right);
                
                log!("Info", "Selection Start: {}, End: {}, Direction: {}", self.selection.pos_start, self.selection.pos_end, self.selection.direction);
                
                return EventProcessStatus::Processed;
            }

            key!("Ctrl+V") | key!("Ctrl+Shift+V") => {
                if !self.flags.contains(Flags::ReadOnly) {
                    self.reposition_cursor();

                    if let Some(clipboard_data) = RuntimeManager::get().terminal().get_clipboard_text() {
                        self.insert_text(&clipboard_data);
                        self.move_cursor_horizontal(clipboard_data.chars().count() as i32);
                    }
                    return EventProcessStatus::Processed;
                }
            }
            key!("Ctrl+C") | key!("Ctrl+Shift+C") => {
                if self.selection.direction != SelectionDirection::None && self.selection.pos_start != self.selection.pos_end {
                    RuntimeManager::get()
                            .terminal_mut()
                            .set_clipboard_text(&self.text[self.selection.pos_start..self.selection.pos_end]);
                }
                return EventProcessStatus::Processed;
            }

            key!("Back") => {
                if !self.flags.contains(Flags::ReadOnly) {
                    self.reposition_cursor();

                    if self.selection.direction != SelectionDirection::None && self.selection.pos_start != self.selection.pos_end {
                        self.remove_text_selection(self.selection.pos_start, self.selection.pos_end);
                        self.reset_selection();
                    }
                    else {
                        self.remove_char_back();
                    }

                    return EventProcessStatus::Processed;
                }
            }
            key!("Delete") => {
                if !self.flags.contains(Flags::ReadOnly) {
                    self.reposition_cursor();

                    if self.selection.direction != SelectionDirection::None && self.selection.pos_start != self.selection.pos_end {
                        self.remove_text_selection(self.selection.pos_start, self.selection.pos_end);
                        self.reset_selection();
                    }
                    else {
                        self.remove_char_front();
                    }
                    return EventProcessStatus::Processed;
                }
            }
            
            key!("Ctrl+Back") => {
                if !self.flags.contains(Flags::ReadOnly) {
                    self.reposition_cursor();

                    if self.selection.direction != SelectionDirection::None && self.selection.pos_start != self.selection.pos_end {
                        self.remove_text_selection(self.selection.pos_start, self.selection.pos_end);
                        self.reset_selection();
                    }
                    else {
                        let (absolute_position_inital, absolute_position_new) = self.move_to_edge_of_word(Direction::Left);
                        self.remove_text_selection(absolute_position_new, absolute_position_inital);
                    }

                    return EventProcessStatus::Processed;
                }
            }
            key!("Ctrl+Delete") => {
                if !self.flags.contains(Flags::ReadOnly) {
                    self.reposition_cursor();

                    if self.selection.direction != SelectionDirection::None && self.selection.pos_start != self.selection.pos_end {
                        self.remove_text_selection(self.selection.pos_start, self.selection.pos_end);
                        self.reset_selection();
                    }
                    else {
                        let (absolute_position_inital, absolute_position_new) = self.move_to_edge_of_word(Direction::Right);
                        self.remove_text_selection(absolute_position_inital, absolute_position_new);
                    }

                    return EventProcessStatus::Processed;
                }
            }

            key!("Enter") => {
                if !self.flags.contains(Flags::ReadOnly) {
                    self.reposition_cursor();
                    self.insert_text("\n");
                    self.move_cursor_horizontal(1);
                    
                    return EventProcessStatus::Processed;
                }
            }

            _ => {}
        }

        if (character as u32) > 0 && !self.flags.contains(Flags::ReadOnly) {
            self.reposition_cursor();

            self.reset_selection();

            self.insert_text(&character.to_string());
            self.move_cursor_horizontal(1);

            return EventProcessStatus::Processed;
        }

        EventProcessStatus::Ignored
    }
}

impl OnMouseEvent for TextArea {
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        if self.scrollbars.process_mouse_event(event) {
            self.update_scroll_pos_from_scrollbars();
            return EventProcessStatus::Processed;
        }

        match event {
            MouseEvent::Enter | MouseEvent::Leave => EventProcessStatus::Ignored,
            MouseEvent::Over(_) => EventProcessStatus::Ignored,
            MouseEvent::Pressed(data) => {
                // We need to change cursor position and reset selection
                self.set_cursor_pos_from_mouse(data);
                self.cursor.pressed = true;

                self.reset_selection();
                
                self.selection.pos_start = self.get_absolute_position() as usize;
                
                self.update_scrollbar_pos();

                EventProcessStatus::Processed
            }
            MouseEvent::Released(_) => {
                self.cursor.pressed = false;
                
                EventProcessStatus::Processed
            }
            MouseEvent::DoubleClick(_) => EventProcessStatus::Ignored,
            MouseEvent::Drag(data) => {
                if !self.cursor.pressed {
                    return EventProcessStatus::Ignored;
                }

                log!("TextArea - Mouse Drag", "Previous Mouse position x: {}, y: {}", self.mouse_x, self.mouse_y);
                log!("TextArea - Mouse Drag", "New Mouse position x: {}, y: {}", data.x, data.y);

                let cursor_absolute_position = self.get_absolute_position();

                self.update_cursor_pos_from_mouse(data);

                let absolute_position_new = self.get_absolute_position() as usize;

                if self.selection.direction == SelectionDirection::None {
                    match (cursor_absolute_position as usize).cmp(&absolute_position_new) {
                        Ordering::Less => {
                            self.update_selection(cursor_absolute_position as usize, absolute_position_new, SelectionDirection::Right);
                        }
                        
                        Ordering::Greater =>    {
                            self.update_selection(cursor_absolute_position as usize, absolute_position_new, SelectionDirection::Left);
                        }
                        Ordering::Equal => {}
                    }
                }
                else if self.selection.direction == SelectionDirection::Right {
                    self.update_selection(cursor_absolute_position as usize, absolute_position_new, SelectionDirection::Right);
                }
                else {
                    self.update_selection(cursor_absolute_position as usize, absolute_position_new, SelectionDirection::Left);
                }

                EventProcessStatus::Processed
            }
            MouseEvent::Wheel(_) => EventProcessStatus::Ignored,
        }
    }
}
