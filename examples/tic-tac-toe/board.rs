use std::time::{SystemTime, UNIX_EPOCH};

use appcui::prelude::*;

#[derive(Copy, Clone, PartialEq, Eq)]
enum Piece {
    X,
    O,
}
impl Piece {
    fn switch(self) -> Self {
        match self {
            Piece::X => Piece::O,
            Piece::O => Piece::X,
        }
    }
}

const WIN_POSITIONS: [(usize, usize, usize); 8] = [
    // Rows
    (0, 1, 2),
    (3, 4, 5),
    (6, 7, 8),
    // Columns
    (0, 3, 6),
    (1, 4, 7),
    (2, 5, 8),
    // Diagonals
    (0, 4, 8),
    (2, 4, 6),
];

#[derive(Copy, Clone)]
enum GameResult {
    WinnerX,
    WinnerO,
    Draw,
}

#[CustomControl(overwrite: OnPaint+OnKeyPressed+OnMouseEvent, emit: GameOver+Exit)]
pub struct Board {
    cells: Vec<Option<Piece>>,
    current_cell_index: usize,
    clicked: bool,
    piece: Piece,
    computer: bool,
}

impl Board {
    pub fn new() -> Self {
        Self {
            base: ControlBase::new(Layout::new("x:2,y:1,w:34,h:19"), true),
            cells: vec![None; 9],
            current_cell_index: usize::MAX,
            clicked: false,
            piece: Piece::X,
            computer: false,
        }
    }
    pub fn reset_game(&mut self) {
        for c in self.cells.iter_mut() {
            *c = None;
        }
        self.current_cell_index = usize::MAX;
        self.clicked = false;
    }
    fn paint_x(&self, x: i32, y: i32, surface: &mut Surface) {
        let ch = char!("' ',back:red");
        for i in 0..5 {
            surface.write_char(x + i * 2, y + i, ch);
            surface.write_char(x + i * 2 + 1, y + i, ch);
            surface.write_char(x + 8 - i * 2, y + i, ch);
            surface.write_char(x + 9 - i * 2, y + i, ch);
        }
    }
    fn paint_o(&self, x: i32, y: i32, surface: &mut Surface) {
        let ch = char!("' ',back:green");
        surface.write_string(x + 2, y, "      ", CharAttribute::with_color(Color::Black, Color::Green), false);
        surface.write_string(x + 2, y + 4, "      ", CharAttribute::with_color(Color::Black, Color::Green), false);
        for i in 1..4 {
            surface.write_char(x, y + i, ch);
            surface.write_char(x + 1, y + i, ch);
            surface.write_char(x + 8, y + i, ch);
            surface.write_char(x + 9, y + i, ch);
        }
    }
    fn mouse_pos_to_cell_index(&self, x: i32, y: i32) -> usize {
        if !(1..=32).contains(&x) || !(1..=17).contains(&y) {
            return usize::MAX;
        }
        if (x == 6) || (x == 12) || (y == 1) || (y == 22) {
            return usize::MAX;
        }
        let new_poz = (((x - 1) / 11) + ((y - 1) / 6) * 3) as usize;
        if self.cells[new_poz].is_some() {
            usize::MAX
        } else {
            new_poz
        }
    }
    fn next_valid(&mut self) {
        let poz = if self.current_cell_index < 9 { self.current_cell_index } else { 0 };
        for i in 1..9 {
            let new_poz = (poz + i) % 9;
            if self.cells[new_poz].is_none() {
                self.current_cell_index = new_poz;
                return;
            }
        }
        self.current_cell_index = usize::MAX;
    }
    fn previous_valid(&mut self) {
        let poz = if self.current_cell_index < 9 { self.current_cell_index } else { 8 };
        for i in 1..9 {
            let new_poz = (poz + 9 - i) % 9;
            if self.cells[new_poz].is_none() {
                self.current_cell_index = new_poz;
                return;
            }
        }
        self.current_cell_index = usize::MAX;
    }
    fn game_result(&self) -> Option<GameResult> {
        for &(pos_1, pos_2, pos_3) in WIN_POSITIONS.iter() {
            if let (Some(Piece::X), Some(Piece::X), Some(Piece::X)) = (self.cells[pos_1], self.cells[pos_2], self.cells[pos_3]) {
                return Some(GameResult::WinnerX);
            }
            if let (Some(Piece::O), Some(Piece::O), Some(Piece::O)) = (self.cells[pos_1], self.cells[pos_2], self.cells[pos_3]) {
                return Some(GameResult::WinnerO);
            }
        }
        for elem in self.cells.iter() {
            if elem.is_none() {
                return None;
            }
        }
        Some(GameResult::Draw)
    }
    fn computer_move(&mut self) {
        let mut available: [usize; 9] = [usize::MAX; 9];
        let mut count = 0;
        for (index, elem) in self.cells.iter().enumerate() {
            if elem.is_some() {
                available[count] = index;
                count += 1;
            }
        }
        if count == 0 {
            return;
        }
        // pseudo random value
        let index = (SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos() as usize) % count;
        self.cells[index] = Some(self.piece);
    }
    fn place_piece(&mut self) {
        if self.current_cell_index >= 9 {
            return;
        }
        self.cells[self.current_cell_index] = Some(self.piece);
        if let Some(result) = self.game_result() {
            self.show_result(result);
            return;
        }
        self.piece = self.piece.switch();
        if self.computer {
            self.computer_move();
            if let Some(result) = self.game_result() {
                self.show_result(result);
            }
        }
        self.next_valid();
    }
    fn show_result(&mut self, result: GameResult) {
        match result {
            GameResult::WinnerX => dialogs::message("Game over", "Player (X) wins !"),
            GameResult::WinnerO => dialogs::message("Game over", "Player (O) wins !"),
            GameResult::Draw => dialogs::message("Game over", "Draw game !"),
        }
        self.raise_event(board::Events::GameOver);
    }
}

impl OnPaint for Board {
    fn on_paint(&self, surface: &mut Surface, _theme: &Theme) {
        surface.clear(char!("' ',black,black"));
        surface.draw_rect(
            Rect::new(0, 0, 33, 18),
            LineType::Single,
            CharAttribute::with_color(Color::White, Color::Black),
        );
        surface.fill_horizontal_line(1, 6, 32, char!("BoxHorizontalSingleLine,white,black"));
        surface.fill_horizontal_line(1, 12, 32, char!("BoxHorizontalSingleLine,white,black"));
        surface.fill_vertical_line(11, 1, 17, char!("BoxVerticalSingleLine,white,black"));
        surface.fill_vertical_line(22, 1, 17, char!("BoxVerticalSingleLine,white,black"));
        surface.write_char(11, 6, char!("BoxCrossSingleLine,white,black"));
        surface.write_char(11, 12, char!("BoxCrossSingleLine,white,black"));
        surface.write_char(22, 6, char!("BoxCrossSingleLine,white,black"));
        surface.write_char(22, 12, char!("BoxCrossSingleLine,white,black"));

        for (index, cell) in self.cells.iter().enumerate() {
            let x = ((index % 3) as i32) * 11 + 1;
            let y = ((index / 3) as i32) * 6 + 1;
            match cell {
                Some(Piece::X) => self.paint_x(x, y, surface),
                Some(Piece::O) => self.paint_o(x, y, surface),
                None => {}
            }
            if index == self.current_cell_index {
                surface.draw_rect(
                    Rect::with_size(x - 1, y - 1, 12, 7),
                    LineType::Border,
                    if self.clicked {
                        CharAttribute::with_color(Color::Blue, Color::Black)
                    } else {
                        CharAttribute::with_color(Color::Yellow, Color::Black)
                    },
                );
            }
        }
    }
}
impl OnKeyPressed for Board {
    fn on_key_pressed(&mut self, key: Key, _character: char) -> EventProcessStatus {
        match key.get_compact_code() {
            key!("Left") => {
                self.previous_valid();
                EventProcessStatus::Processed
            }
            key!("Right") => {
                self.next_valid();
                EventProcessStatus::Processed
            }
            _ => EventProcessStatus::Ignored,
        }
    }
}
impl OnMouseEvent for Board {
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        match event {
            MouseEvent::Enter | MouseEvent::Leave => {
                self.current_cell_index = usize::MAX;
                self.clicked = false;
                EventProcessStatus::Processed
            }
            MouseEvent::Over(data) => {
                self.current_cell_index = self.mouse_pos_to_cell_index(data.x, data.y);
                EventProcessStatus::Processed
            }
            MouseEvent::Pressed(data) => {
                self.current_cell_index = self.mouse_pos_to_cell_index(data.x, data.y);
                self.clicked = true;
                EventProcessStatus::Processed
            }
            MouseEvent::Released(data) => {
                self.current_cell_index = self.mouse_pos_to_cell_index(data.x, data.y);
                self.clicked = false;
                self.place_piece();
                EventProcessStatus::Processed
            }
            _ => EventProcessStatus::Ignored,
        }
    }
}
