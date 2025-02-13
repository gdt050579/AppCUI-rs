use super::initialization_flags::Flags;

use crate::prelude::*;

use std::{fs::File, io::Write, mem::swap};


// TODO: implement a second Cursor that is used as absolute position on screen and is discarded whenever we
// move horizontally
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

    // Line sizes
    line_sizes: Vec<u32>,
    max_line_size: u32,

    // LineNumberBar size
    line_number_bar_size: u32,
    
    // Current position in the line
    row_offset: u32,

    // Current line
    line_offset: u32,

    file: File,

    scrollbars: ScrollBars

}

impl TextArea {

    fn get_absolute_position(&mut self) -> u32 {
        let mut cursor_absolute_position = 0;
        for i in 0..self.line_offset + self.cursor.pos_y as u32 {
            cursor_absolute_position += self.line_sizes[i as usize];
        }
        cursor_absolute_position += self.row_offset + self.cursor.pos_x as u32;

        // We need to remove the LineNumberBar size from Cursor Position
        if self.flags.contains(Flags::ShowLineNumber) {
            cursor_absolute_position -= self.line_number_bar_size;
        }

        cursor_absolute_position
    }

    fn reset_selection(&mut self) {
        self.selection.pos_start = 0;
        self.selection.pos_end = 0;
        self.selection.direction = SelectionDirection::None;
    }

    fn reposition_cursor(&mut self) {
        // If the cursor is outside the line its supposed to be
        if self.cursor.pos_x + self.line_offset as usize >= self.line_sizes[self.cursor.pos_y + self.line_offset as usize] as usize {
            let diff = (self.cursor.pos_x + self.line_offset as usize) - self.line_sizes[self.cursor.pos_y + self.line_offset as usize] as usize + 1;
            self.move_cursor_horizontal(0 - diff as i32);
        }
    }

    fn parse_text_in_lines(&mut self, text: &str, line_sizes: &mut Vec<i32>) {
        let mut current_offset = 0;
        for ch in text.as_bytes() {
            current_offset = current_offset + 1;
            if *ch == b'\n' {
                line_sizes.push(current_offset);
                current_offset = 0;
            }
        }
        if current_offset != 0 {
            line_sizes.push(current_offset);
        }

        for line in line_sizes {
            _ = self.file.write_all((format!("Line size: {line}")).as_bytes());
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

        self.line_number_bar_size = temp_size + 1;
    }

    pub fn new(text: &str, layout: Layout, flags: Flags) -> Self {

        if flags.contains(Flags::ShowLineNumber) {

        }

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
            text: text.to_string().replace('\t', "  "),

            cursor: Cursor { pos_x: 0, pos_y: 0, pressed: false},
            selection: Selection {pos_start: 0, pos_end: 0, direction: SelectionDirection::None},

            line_sizes: Vec::new(),
            max_line_size: 0,

            row_offset: 0,
            line_offset: 0,

            // LineNumber tab size
            line_number_bar_size: 0,

            file: File::create("debug.txt").unwrap(),

            scrollbars: ScrollBars::new(flags.contains(Flags::ScrollBars))
        };

        if !control.text.ends_with('\n') {
            control.text += "\n";
        }

        if !flags.contains(Flags::ShowLineNumber) {
            control.line_number_bar_size = 0;
        }

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
        control.update_max_line_size();
        control.update_line_number_tab_size();
        
        control
    }

    #[inline(always)]
    fn update_scrollbar_pos(&mut self) {
        self.scrollbars.set_indexes((self.row_offset as usize) as u64, (self.cursor.pos_y + self.line_offset as usize) as u64);
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
        else if new_position >= self.size().height as i32 {
            self.cursor.pos_y = self.size().height as usize - 1;
            
            let mut update_offset = new_position - self.size().height as i32 + 1 + self.line_offset as i32;
            
            if update_offset > self.line_sizes.len() as i32 - self.size().height as i32 {
                update_offset = self.line_sizes.len() as i32 - self.size().height as i32;
            }

            self.line_offset = update_offset as u32;
        }
        else {
            self.cursor.pos_y = new_position as usize;
            if self.line_offset + self.cursor.pos_y as u32 >= self.line_sizes.len() as u32 - 1 {
                self.cursor.pos_y = self.line_sizes.len() - self.line_offset as usize - 1;
            }
        }
        self.update_scrollbar_pos();
    }

    pub fn move_cursor_horizontal(&mut self, no_collumns: i32) {
        let mut new_position = self.cursor.pos_x as i32 + self.row_offset as i32 + no_collumns;

        // The number of column left moves is bigger than the position in the line, needing to go upwards
        if new_position < self.line_number_bar_size as i32 {
            // Current row, the offset of the first line printed + the row cursor position
            let mut current_row = self.line_offset + self.cursor.pos_y as u32;
            
            // Going up while we are still on a valid row
            while current_row > 0 && new_position < self.line_number_bar_size as i32 {
                
                // Going a row above
                current_row -= 1;

                // Adding the row size of the current row 
                new_position = new_position + self.line_sizes[current_row as usize] as i32;
            }

            // Too much left move, setting to pos (0, 0) 
            if new_position < self.line_number_bar_size as i32 {
                // Updating the cursor 
                self.cursor.pos_x = self.line_number_bar_size as i32 as usize;
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
                if !(new_position as u32 >= self.row_offset && new_position < self.size().width as i32 + self.row_offset as i32) {
                    if new_position < self.size().width as i32 {
                        self.row_offset = new_position as u32;
                    }
                    else {
                        self.row_offset = new_position as u32 - self.size().width as u32 + 1;
                    }
                }
                // Updating the cursor position on horizontal
                self.cursor.pos_x = new_position as usize - self.row_offset as usize;
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
            if (new_position as u32) < self.size().width as u32 {
                self.cursor.pos_x = new_position as usize;
                self.row_offset = 0;
            }
            // The line does not fit on the screen, we need to determine the offset for the line view
            else {
                let tmp_row_offset = new_position as i32 - self.size().width as i32;
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
            if !(self.row_offset <= new_position as u32 && (new_position as u32) < self.row_offset as u32 + self.size().width as u32) {
                
                // We need to update position, we will check where the direction on movement
                // Checking if the movement is to the left
                if self.row_offset > new_position as u32 {
                    let tmp_row_offset = new_position as i32 - self.size().width as i32;
                    if tmp_row_offset < 0 {
                        self.row_offset = new_position as u32;
                    }
                    else {
                        self.row_offset = tmp_row_offset as u32;
                    }
                }
                // The movement is to the right, we need to increase the offset
                else {
                    let tmp_row_offset = new_position as i32 - self.size().width as i32 + 1;
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

        // if self.flags.contains(Flags::ShowLineNumber) {
        //     self.cursor.pos_x += self.line_number_bar_size as usize;
        // }

        self.update_scrollbar_pos();

        let _ = self.file.write_all(format!("Cursor {:?}\n", self.cursor).as_bytes());
    }

    pub fn insert_char(&mut self, character: char) {
        // First we need to calculate the absolute position in the text for the cursor
        // TODO: this seems slow to do considering text editing is the main purpose, we need to cache this information
        let cursor_absolute_position = self.get_absolute_position();
        
        // Inserting char, updating line size
        self.text.insert(cursor_absolute_position as usize, character);
        self.line_sizes[self.line_offset as usize + self.cursor.pos_y] += 1;
        
        // Move cursor to the newly added character
        self.move_cursor_horizontal(1);

        self.update_max_line_size();
    }

    pub fn remove_char_back(&mut self) {
        // First we need to calculate the absolute position in the text for the cursor
        let mut cursor_absolute_position = self.get_absolute_position();
        
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

                self.update_line_number_tab_size();
            }
            else {
                self.text.remove(cursor_absolute_position as usize);
                self.line_sizes[self.line_offset as usize + self.cursor.pos_y] -= 1;
                self.move_cursor_horizontal(-1);
            }

            self.update_max_line_size();
        }
    }

    pub fn remove_char_front(&mut self) {
        let cursor_absolute_position = self.get_absolute_position();
        
        if cursor_absolute_position < self.text.len() as u32 - 1 {
            if self.text.as_bytes()[cursor_absolute_position as usize] == b'\n' {
                self.text.remove(cursor_absolute_position as usize);
                self.line_sizes[self.line_offset as usize + self.cursor.pos_y] -= 1;
                self.line_sizes[self.line_offset as usize + self.cursor.pos_y] += self.line_sizes[self.line_offset as usize + self.cursor.pos_y + 1];
                self.line_sizes.remove(self.line_offset as usize + self.cursor.pos_y + 1);

                self.update_line_number_tab_size();
            }
            else {
                self.text.remove(cursor_absolute_position as usize);
                self.line_sizes[self.line_offset as usize + self.cursor.pos_y] -= 1;
            }

            self.update_max_line_size();
        }
    }

    fn update_scroll_pos_from_scrollbars(&mut self) {
        let current_pos = self.line_offset as i32 + self.cursor.pos_y as i32;
        let new_pos = self.scrollbars.vertical_index() as i32;

        self.move_cursor_vertical(new_pos - current_pos);
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

        // print!("{position_start_x}, {position_start_y}, {position_end_x}, {position_end_y}");

        // this is bad, but this is my level for now in Rust
        if position_start_y == position_end_y {

            for _it in 0..position_end_x-position_start_x {
                self.text.remove(pos_start);
            }

            self.line_sizes[position_start_y] -= (position_end_x - position_start_x) as u32;
        }
        else {
            
        }

        self.move_cursor_horizontal(pos_start as i32 - pos_end as i32);
    }

    pub fn insert_newline(&mut self) {
        // First we need to calculate the absolute position in the text for the cursor
        let cursor_absolute_position = self.get_absolute_position();

        let mut position_in_line = self.row_offset + self.cursor.pos_x as u32;
        if self.flags.contains(Flags::ShowLineNumber) {
            position_in_line -= self.line_number_bar_size;
        }
        
        let line_index = self.line_offset as usize + self.cursor.pos_y;
        
        // Inserting the character for newline in the text, updating the line_size
        self.text.insert(cursor_absolute_position as usize, '\n');
        self.line_sizes[line_index] += 1;

        // Splitting the line into 2, [0, position + 1] + (position + 1, line_size]
        // The + 1 is in fact the '\n' added
        self.line_sizes.insert(line_index + 1, self.line_sizes[line_index] - position_in_line - 1);
        self.line_sizes[line_index] = position_in_line + 1;

        self.move_cursor_horizontal(1);

        self.update_max_line_size();
        self.update_line_number_tab_size();
    }

    pub fn insert_text(&mut self, text: &str) {
        if text.contains('\n') {

            // First we parse the text in lines
            let mut line_sizes : Vec<i32> = Vec::new();
            self.parse_text_in_lines(text, &mut line_sizes);

            // We need to calculate the absolute position in the text for the cursor and the position in line
            let mut cursor_absolute_position = 0;
            for i in 0..self.line_offset + self.cursor.pos_y as u32 {
                cursor_absolute_position += self.line_sizes[i as usize];
            }
            let position_in_line = self.row_offset + self.cursor.pos_x as u32;
            cursor_absolute_position += position_in_line;

            let line_index = self.line_offset as usize + self.cursor.pos_y;

            // Adding the text given as parameter
            self.text.insert_str(cursor_absolute_position as usize, text);

            // Splitting the line into 2, [0, position] + (position , line_size]
            self.line_sizes.insert(line_index + 1, self.line_sizes[line_index] - position_in_line);
            self.line_sizes[line_index] = position_in_line;
            
            self.line_sizes[line_index] += line_sizes[0] as u32;
            self.line_sizes[line_index + 1] += line_sizes[line_sizes.len() - 1] as u32;
            
            for i in 1..line_sizes.len() - 1 {
                self.line_sizes.insert(line_index + i, line_sizes[i] as u32);
            }

            self.update_line_number_tab_size();
        }
        else {
            // We need to calculate the absolute position in the text for the cursor and the position in line
            let mut cursor_absolute_position = 0;
            for i in 0..self.line_offset + self.cursor.pos_y as u32 {
                cursor_absolute_position += self.line_sizes[i as usize];
            }
            let position_in_line = self.row_offset + self.cursor.pos_x as u32;
            cursor_absolute_position += position_in_line;

            let line_index = self.line_offset as usize + self.cursor.pos_y;

            // Adding the text given as parameter
            self.text.insert_str(cursor_absolute_position as usize, text);

            self.line_sizes[line_index] += text.len() as u32;
        }
    
        self.update_max_line_size();

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
        
        // We reserve the necessary space for showing the line number, which will be dynamically calculated
        // using the number of lines the text has + 1 as a divider for the text
        if self.flags.contains(Flags::ShowLineNumber) {
            x = self.line_number_bar_size as i32;
        }

        let max_line_size = self.size().width as usize;
        let mut initial_offset = 0;
        
        for it in 0..self.line_sizes.len() {
            let current_offset = &self.line_sizes[it];

            if it >= self.line_offset as usize + self.size().height as usize {
                break;
            }
            
            if it >= self.line_offset as usize {
                let current_line = &self.text[initial_offset as usize .. (initial_offset + *current_offset) as usize];
                
                if self.flags.contains(Flags::ShowLineNumber) {
                    let line_number_text = (it + 1).to_string();
                    let mut offset = self.line_number_bar_size as i32 - line_number_text.len() as i32 - 1;

                    for ch in line_number_text.chars() {
                        surface.write_char(offset, y, Character::with_attributes(ch, attr_line_number));
                        offset = offset + 1;
                    }
                }
                

                if self.row_offset < current_line.len() as u32 {
                    let current_line_view = &current_line[self.row_offset as usize .. ];
                
                    let mut counter = x as usize;
                    for ch in current_line_view.chars() {
                        
                        if counter >= max_line_size {
                            break;
                        }

                        if ch == '\n' {
                            continue;
                        }

                        let absolute_position = initial_offset + self.row_offset + counter as u32;

                        if self.selection.pos_start <= absolute_position as usize && (absolute_position as usize) < self.selection.pos_end {
                            surface.write_char(x, y, Character::with_attributes(ch, attr_selection));
                        }                        
                        else {
                            if self.flags.contains(Flags::HighlightCursor) && y as usize == self.cursor.pos_y {
                                surface.write_char(x, y, Character::with_attributes(ch, attr_text));

                            }
                            else {
                                surface.write_char(x, y, Character::with_attributes(ch, attr_text));
                            }
                        }
                        x = x + 1;

                        counter += 1;
                    }
                }

                y = y + 1;
                x = 0;
                if self.flags.contains(Flags::ShowLineNumber) {
                    x = self.line_number_bar_size as i32;
                }
            }
            initial_offset = initial_offset + current_offset;
        }

        surface.set_cursor(self.cursor.pos_x as i32, self.cursor.pos_y as i32);
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

                // The text is already at the end, so no action is performed
                if absolute_position_inital == self.text.len() - 1 {
                    return EventProcessStatus::Processed;
                }
                
                // Checking how the selction was initially started so we understand how to process it
                
                // the selection is already to the right, so we expand the selection
                if self.selection.direction == SelectionDirection::Right {
                    let absolute_position_final = absolute_position_inital + 1;
                    
                    self.selection.pos_end = absolute_position_final;
                }
                // the selection is to the left, so we must reduce our selection
                else if self.selection.direction == SelectionDirection::Left {
                    let absolute_position_final = absolute_position_inital + 1;

                    self.selection.pos_start = absolute_position_final;

                    // by moving our selection to the right, the selection is now sized at 0, 
                    // so we change it to None, in order to be able to switch directions
                    if self.selection.pos_start == self.selection.pos_end {
                        self.selection.direction = SelectionDirection::None;
                    }
                }
                // there was no previous selection, so we create it now
                else {
                    let absolute_position_final = absolute_position_inital + 1;
                    
                    self.selection.pos_start = absolute_position_inital;
                    self.selection.pos_end = absolute_position_final;
                    self.selection.direction = SelectionDirection::Right;
                }                
                
                let _ = self.file.write_all(format!("Selection {:?}\n", self.selection).as_bytes());

                self.move_cursor_horizontal(1);
                return EventProcessStatus::Processed;
            }
            key!("Shift+Left") => {
                // Making sure our cursor is in Focus
                self.reposition_cursor();
                
                let absolute_position_inital = self.get_absolute_position() as usize;

                // The text is already at the start, so no action is performed
                if absolute_position_inital == 0 {
                    return EventProcessStatus::Processed;
                }

                // Checking how the selection was initially started so we understand how to process it
                
                // the selection is already to the left, so we expand the selection
                if self.selection.direction == SelectionDirection::Left {
                    let absolute_position_final = absolute_position_inital - 1;
                    
                    self.selection.pos_start = absolute_position_final;
                }
                // the selection is to the right, so we must reduce our selection
                else if self.selection.direction == SelectionDirection::Right {
                    let absolute_position_final = absolute_position_inital - 1;

                    self.selection.pos_end = absolute_position_final;

                    // by moving our selection to the right, the selection is now sized at 0, 
                    // so we change it to None, in order to be able to switch directions
                    if self.selection.pos_start == self.selection.pos_end {
                        self.selection.direction = SelectionDirection::None;
                    }
                }
                // there was no previous selection, so we create it now
                else {
                    let absolute_position_final = absolute_position_inital - 1;
                    
                    self.selection.pos_end = absolute_position_inital;
                    self.selection.pos_start = absolute_position_final;
                    self.selection.direction = SelectionDirection::Left;
                }

                let _ = self.file.write_all(format!("Selection {:?}\n", self.selection).as_bytes());

                self.move_cursor_horizontal(-1);
                return EventProcessStatus::Processed;
            }
            key!("Shift+Up") => {
                // We can consider Shift+Up as just a Shift+Left that moves from position (x, y) to (x, y-1)
                // Which in theory should mean moving Left with 
                // cursor.position_x (for current line) + min(line_sizes[cursor.position_y - 1] - cursor.position_x, 0)
                // which equates to exactly line_sizes[cursor.position_y - 1] if the position in the line is smaller than the line above
                // or cursor.position_x otherwise 

                // Making sure our cursor is in Focus 
                self.move_cursor_vertical(0);

                let absolute_position_inital = self.get_absolute_position() as usize;

                let positions_to_move;

                // If we are at line 0, we just need to move to the beginning of it
                if self.cursor.pos_y + self.line_offset as usize == 0 {
                    if self.cursor.pos_x + self.row_offset as usize == 0 {
                        // Cannot select, therefor exiting without changes 
                        return EventProcessStatus::Processed;
                    }
                    else {
                        // The number of positions becomes the row_offset + the cursor position
                        positions_to_move = self.cursor.pos_x + self.row_offset as usize;
                    }
                }
                else if self.row_offset as usize + self.cursor.pos_x < self.line_sizes[self.line_offset as usize + self.cursor.pos_y - 1] as usize {
                    positions_to_move = self.line_sizes[self.line_offset as usize + self.cursor.pos_y - 1] as usize;
                }
                else {
                    positions_to_move = self.row_offset as usize + self.cursor.pos_x;  
                }

                // Checking how the selection was initially started so we understand how to process it
                
                // the selection is already to the left, so we expand the selection
                if self.selection.direction == SelectionDirection::Left {
                    let absolute_position_final = absolute_position_inital - positions_to_move;
                    
                    self.selection.pos_start = absolute_position_final;
                }
                // the selection is to the right, so we must reduce our selection
                else if self.selection.direction == SelectionDirection::Right {
                    let absolute_position_final = absolute_position_inital - positions_to_move;

                    self.selection.pos_end = absolute_position_final;

                    // by moving our selection to the right, the selection is now sized at 0, 
                    // so we change it to None, in order to be able to switch directions
                    if self.selection.pos_start == self.selection.pos_end {
                        self.selection.direction = SelectionDirection::None;
                    }
                }
                // there was no previous selection, so we create it now
                else {
                    let absolute_position_final = absolute_position_inital - positions_to_move;
                    
                    self.selection.pos_end = absolute_position_inital;
                    self.selection.pos_start = absolute_position_final;
                    self.selection.direction = SelectionDirection::Left;
                }

                let _ = self.file.write_all(format!("Selection {:?}\n", self.selection).as_bytes());

                self.move_cursor_horizontal(0 - positions_to_move as i32);
                return EventProcessStatus::Processed;
            }
            key!("Shift+Down") => {
                // Making sure our cursor is in Focus 
                self.move_cursor_vertical(0);

                let absolute_position_inital = self.get_absolute_position() as usize;

                let positions_to_move;
                
                let current_line_position = self.cursor.pos_y as usize + self.line_offset as usize;
                let current_row_position = self.cursor.pos_x + self.row_offset as usize;

                // If we are at the last line, we just need to move to the end of it
                if current_line_position == self.line_sizes.len() - 1 {
                    if current_row_position as usize == self.line_sizes[current_line_position] as usize - 1 {
                        // Cannot select, therefor exiting without changes 
                        return EventProcessStatus::Processed;
                    }
                    else {
                        // The number of positions becomes the row_offset + the cursor position
                        positions_to_move = self.line_sizes[current_line_position] as usize - current_row_position  - 1; // -1 for the newline 
                    }
                }
                else {
                    if current_row_position < self.line_sizes[current_line_position + 1] as usize {
                        positions_to_move = self.line_sizes[current_line_position] as usize;
                    }
                    else {
                        positions_to_move = self.line_sizes[current_line_position] as usize - current_row_position + self.line_sizes[current_line_position + 1] as usize; 
                    }
                }

                // Checking how the selection was initially started so we understand how to process it
                
                // the selection is already to the right, so we expand the selection
                if self.selection.direction == SelectionDirection::Right {
                    let absolute_position_final = absolute_position_inital + positions_to_move;
                    
                    self.selection.pos_end = absolute_position_final;
                }
                // the selection is to the left, so we must reduce our selection
                else if self.selection.direction == SelectionDirection::Left {
                    let absolute_position_final = absolute_position_inital + positions_to_move;

                    self.selection.pos_start = absolute_position_final;

                    // by moving our selection to the right, the selection is now sized at 0, 
                    // so we change it to None, in order to be able to switch directions
                    if self.selection.pos_start == self.selection.pos_end {
                        self.selection.direction = SelectionDirection::None;
                    }
                    // selection changed direction, we need to update our selection parameters
                    if self.selection.pos_start > self.selection.pos_end {
                        swap(&mut self.selection.pos_start, &mut self.selection.pos_end);
                        self.selection.direction = SelectionDirection::Right;
                    }
                }
                // there was no previous selection, so we create it now
                else {
                    let absolute_position_final = absolute_position_inital + positions_to_move;
                    
                    self.selection.pos_start = absolute_position_inital;
                    self.selection.pos_end = absolute_position_final;
                    self.selection.direction = SelectionDirection::Right;
                }               

                let _ = self.file.write_all(format!("Selection {:?}\n", self.selection).as_bytes());

                self.move_cursor_horizontal(positions_to_move as i32);
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

            key!("Ctrl+V") | key!("Ctrl+Shift+V") => {
                if !self.flags.contains(Flags::ReadOnly) {
                    self.reposition_cursor();

                    if let Some(clipboard_data) = RuntimeManager::get().terminal().get_clipboard_text() {
                        self.insert_text(&clipboard_data);
                        self.move_cursor_horizontal(clipboard_data.len() as i32);
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
                // TODO: update scrollbars when deleting \n
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
            // TODO: Add Ctrl+Delete and Ctrl+Back as commands

            key!("Enter") => {
                if !self.flags.contains(Flags::ReadOnly) {
                    self.reposition_cursor();

                    self.insert_newline();
                    return EventProcessStatus::Processed;
                }
            }

            _ => {}
        }

        if (character as u32) > 0 {
            if !self.flags.contains(Flags::ReadOnly) {
                self.reposition_cursor();

                self.reset_selection();

                self.insert_char(character);
                return EventProcessStatus::Processed;
            }
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

                self.cursor.pos_x = data.x as usize;
                self.cursor.pos_y = data.y as usize;
                self.cursor.pressed = true;

                self.reset_selection();
                
                let mut cursor_absolute_position = 0;
                for i in 0..self.line_offset + self.cursor.pos_y as u32 {
                    cursor_absolute_position += self.line_sizes[i as usize];
                }
                cursor_absolute_position += self.row_offset + self.cursor.pos_x as u32;

                self.selection.pos_start = cursor_absolute_position as usize;
                
                self.update_scrollbar_pos();

                EventProcessStatus::Processed
            }
            MouseEvent::Released(_) => {
                self.cursor.pressed = false;
                
                EventProcessStatus::Processed
            }
            MouseEvent::DoubleClick(_) => EventProcessStatus::Ignored,
            MouseEvent::Drag(data) => {
                if self.cursor.pressed == false {
                    return EventProcessStatus::Ignored;
                }

                self.cursor.pos_x = data.x as usize;
                self.cursor.pos_y = data.y as usize;

                let mut cursor_absolute_position = 0;
                for i in 0..self.line_offset + self.cursor.pos_y as u32 {
                    cursor_absolute_position += self.line_sizes[i as usize];
                }
                cursor_absolute_position += self.row_offset + self.cursor.pos_x as u32;

                if self.selection.direction == SelectionDirection::None {
                    self.selection.pos_end = cursor_absolute_position as usize;

                    if self.selection.pos_start < self.selection.pos_end {
                        self.selection.direction = SelectionDirection::Right;
                    }
                    else if self.selection.pos_start > self.selection.pos_end {

                        swap(&mut self.selection.pos_start, &mut self.selection.pos_end);
                        self.selection.direction = SelectionDirection::Left;
                    }
                }
                else if self.selection.direction == SelectionDirection::Right {
                    self.selection.pos_end = cursor_absolute_position as usize;

                    if self.selection.pos_start == self.selection.pos_end {
                        self.selection.direction = SelectionDirection::None;
                    }
                    else if self.selection.pos_start > self.selection.pos_end {

                        swap(&mut self.selection.pos_start, &mut self.selection.pos_end);
                        self.selection.direction = SelectionDirection::Left;
                    }
                }
                else if self.selection.direction == SelectionDirection::Left {
                    self.selection.pos_start = cursor_absolute_position as usize;

                    if self.selection.pos_start == self.selection.pos_end {
                        self.selection.direction = SelectionDirection::None;
                    }
                    else if self.selection.pos_start > self.selection.pos_end {

                        swap(&mut self.selection.pos_start, &mut self.selection.pos_end);
                        self.selection.direction = SelectionDirection::Right;
                    }
                }

                EventProcessStatus::Processed
            }
            MouseEvent::Wheel(_) => EventProcessStatus::Ignored,
        }
    }
}
