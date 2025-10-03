use appcui::prelude::*;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum PieceColor {
    White,
    Black,
}

impl PieceColor {
    fn opposite(&self) -> PieceColor {
        match self {
            PieceColor::White => PieceColor::Black,
            PieceColor::Black => PieceColor::White,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Piece {
    piece_type: PieceType,
    color: PieceColor,
}

impl Piece {
    fn new(piece_type: PieceType, color: PieceColor) -> Self {
        Self { piece_type, color }
    }

    fn symbol(&self) -> char {
        match self.piece_type {
            PieceType::Pawn => 'P',
            PieceType::Rook => 'R',
            PieceType::Knight => 'N',
            PieceType::Bishop => 'B',
            PieceType::Queen => 'Q',
            PieceType::King => 'K',
        }
    }

    fn color(&self) -> Color {
        match self.color {
            PieceColor::White => Color::Yellow,  // Use bright yellow for white pieces
            PieceColor::Black => Color::Red,    // Use red for black pieces
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CellState {
    Empty,
    Occupied(Piece),
}

impl CellState {
    fn is_empty(&self) -> bool {
        matches!(self, CellState::Empty)
    }

    fn piece(&self) -> Option<Piece> {
        match self {
            CellState::Empty => None,
            CellState::Occupied(piece) => Some(*piece),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum GameState {
    Playing,
    Checkmate,
    Stalemate,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SelectionState {
    None,
    Selected((usize, usize)),
}

const BOARD_SIZE: usize = 8;
const CELL_WIDTH: i32 = 9;
const CELL_HEIGHT: i32 = 4;

#[CustomControl(overwrite = OnPaint+OnKeyPressed+OnMouseEvent)]
pub struct ChessLogic {
    board: [[CellState; BOARD_SIZE]; BOARD_SIZE],
    current_player: PieceColor,
    game_state: GameState,
    selection_state: SelectionState,
    possible_moves: Vec<(usize, usize)>,
}

impl ChessLogic {
    pub fn new() -> Self {
        let mut game = Self {
            base: ControlBase::new(layout!("d:f"), true),
            board: [[CellState::Empty; BOARD_SIZE]; BOARD_SIZE],
            current_player: PieceColor::White,
            game_state: GameState::Playing,
            selection_state: SelectionState::None,
            possible_moves: Vec::new(),
        };

        game.setup_initial_board();
        game
    }

    fn setup_initial_board(&mut self) {
        // Setup pawns
        for col in 0..BOARD_SIZE {
            self.board[1][col] = CellState::Occupied(Piece::new(PieceType::Pawn, PieceColor::Black));
            self.board[6][col] = CellState::Occupied(Piece::new(PieceType::Pawn, PieceColor::White));
        }

        // Setup other pieces for black (top row)
        self.board[0][0] = CellState::Occupied(Piece::new(PieceType::Rook, PieceColor::Black));
        self.board[0][1] = CellState::Occupied(Piece::new(PieceType::Knight, PieceColor::Black));
        self.board[0][2] = CellState::Occupied(Piece::new(PieceType::Bishop, PieceColor::Black));
        self.board[0][3] = CellState::Occupied(Piece::new(PieceType::Queen, PieceColor::Black));
        self.board[0][4] = CellState::Occupied(Piece::new(PieceType::King, PieceColor::Black));
        self.board[0][5] = CellState::Occupied(Piece::new(PieceType::Bishop, PieceColor::Black));
        self.board[0][6] = CellState::Occupied(Piece::new(PieceType::Knight, PieceColor::Black));
        self.board[0][7] = CellState::Occupied(Piece::new(PieceType::Rook, PieceColor::Black));

        // Setup other pieces for white (bottom row)
        self.board[7][0] = CellState::Occupied(Piece::new(PieceType::Rook, PieceColor::White));
        self.board[7][1] = CellState::Occupied(Piece::new(PieceType::Knight, PieceColor::White));
        self.board[7][2] = CellState::Occupied(Piece::new(PieceType::Bishop, PieceColor::White));
        self.board[7][3] = CellState::Occupied(Piece::new(PieceType::Queen, PieceColor::White));
        self.board[7][4] = CellState::Occupied(Piece::new(PieceType::King, PieceColor::White));
        self.board[7][5] = CellState::Occupied(Piece::new(PieceType::Bishop, PieceColor::White));
        self.board[7][6] = CellState::Occupied(Piece::new(PieceType::Knight, PieceColor::White));
        self.board[7][7] = CellState::Occupied(Piece::new(PieceType::Rook, PieceColor::White));
    }

    fn draw_piece(&self, surface: &mut Surface, rect: Rect, piece: Piece) {
        let center_x = rect.left() + rect.width() as i32 / 2;
        let center_y = rect.top() + rect.height() as i32 / 2;
        
        let bg_color = if (rect.left() / CELL_WIDTH + rect.top() / CELL_HEIGHT) % 2 == 0 {
            Color::DarkBlue
        } else {
            Color::Gray
        };
        
        surface.fill_rect(rect, Character::new(' ', Color::White, bg_color, CharFlags::None));
        
        surface.write_char(center_x, center_y, Character::new(
            piece.symbol(),
            piece.color(),
            bg_color,
            CharFlags::None
        ));
    }

    fn get_possible_moves(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut moves = Vec::new();
        
        if let Some(piece) = self.board[row][col].piece() {
            if piece.color != self.current_player {
                return moves;
            }

            match piece.piece_type {
                PieceType::Pawn => self.get_pawn_moves(row, col, piece, &mut moves),
                PieceType::Rook => self.get_rook_moves(row, col, piece, &mut moves),
                PieceType::Knight => self.get_knight_moves(row, col, piece, &mut moves),
                PieceType::Bishop => self.get_bishop_moves(row, col, piece, &mut moves),
                PieceType::Queen => {
                    self.get_rook_moves(row, col, piece, &mut moves);
                    self.get_bishop_moves(row, col, piece, &mut moves);
                },
                PieceType::King => self.get_king_moves(row, col, piece, &mut moves),
            }
        }
        
        moves
    }

    fn get_pawn_moves(&self, row: usize, col: usize, piece: Piece, moves: &mut Vec<(usize, usize)>) {
        let direction = match piece.color {
            PieceColor::White => -1,
            PieceColor::Black => 1,
        };
        
        let start_row = match piece.color {
            PieceColor::White => 6,
            PieceColor::Black => 1,
        };

        let new_row_i32 = row as i32 + direction;
        if new_row_i32 >= 0 && new_row_i32 < BOARD_SIZE as i32 {
            let new_row = new_row_i32 as usize;
            if self.board[new_row][col].is_empty() {
                moves.push((new_row, col));
                
                if row == start_row {
                    let double_row_i32 = row as i32 + 2 * direction;
                    if double_row_i32 >= 0 && double_row_i32 < BOARD_SIZE as i32 {
                        let double_row = double_row_i32 as usize;
                        if self.board[double_row][col].is_empty() {
                            moves.push((double_row, col));
                        }
                    }
                }
            }
        }

        let new_row_i32 = row as i32 + direction;
        if new_row_i32 >= 0 && new_row_i32 < BOARD_SIZE as i32 {
            let new_row = new_row_i32 as usize;
            for &dc in &[-1, 1] {
                let new_col_i32 = col as i32 + dc;
                if new_col_i32 >= 0 && new_col_i32 < BOARD_SIZE as i32 {
                    let new_col = new_col_i32 as usize;
                    if let Some(target_piece) = self.board[new_row][new_col].piece() {
                        if target_piece.color != piece.color {
                            moves.push((new_row, new_col));
                        }
                    }
                }
            }
        }
    }

    fn get_rook_moves(&self, row: usize, col: usize, piece: Piece, moves: &mut Vec<(usize, usize)>) {
        for &dc in &[-1, 1] {
            let mut new_col = (col as i32 + dc) as usize;
            while new_col < BOARD_SIZE {
                if self.board[row][new_col].is_empty() {
                    moves.push((row, new_col));
                } else if let Some(target_piece) = self.board[row][new_col].piece() {
                    if target_piece.color != piece.color {
                        moves.push((row, new_col));
                    }
                    break;
                } else {
                    break;
                }
                new_col = (new_col as i32 + dc) as usize;
            }
        }

        for &dr in &[-1, 1] {
            let mut new_row = (row as i32 + dr) as usize;
            while new_row < BOARD_SIZE {
                if self.board[new_row][col].is_empty() {
                    moves.push((new_row, col));
                } else if let Some(target_piece) = self.board[new_row][col].piece() {
                    if target_piece.color != piece.color {
                        moves.push((new_row, col));
                    }
                    break;
                } else {
                    break;
                }
                new_row = (new_row as i32 + dr) as usize;
            }
        }
    }

    fn get_knight_moves(&self, row: usize, col: usize, piece: Piece, moves: &mut Vec<(usize, usize)>) {
        let knight_moves = [
            (-2, -1), (-2, 1), (-1, -2), (-1, 2),
            (1, -2), (1, 2), (2, -1), (2, 1)
        ];

        for &(dr, dc) in &knight_moves {
            let new_row_i32 = row as i32 + dr;
            let new_col_i32 = col as i32 + dc;
            
            if new_row_i32 >= 0 && new_row_i32 < BOARD_SIZE as i32 && 
               new_col_i32 >= 0 && new_col_i32 < BOARD_SIZE as i32 {
                let new_row = new_row_i32 as usize;
                let new_col = new_col_i32 as usize;
                
                if self.board[new_row][new_col].is_empty() {
                    moves.push((new_row, new_col));
                } else if let Some(target_piece) = self.board[new_row][new_col].piece() {
                    if target_piece.color != piece.color {
                        moves.push((new_row, new_col));
                    }
                }
            }
        }
    }

    fn get_bishop_moves(&self, row: usize, col: usize, piece: Piece, moves: &mut Vec<(usize, usize)>) {
        let directions = [(-1, -1), (-1, 1), (1, -1), (1, 1)];

        for &(dr, dc) in &directions {
            let mut new_row = (row as i32 + dr) as usize;
            let mut new_col = (col as i32 + dc) as usize;
            
            while new_row < BOARD_SIZE && new_col < BOARD_SIZE {
                if self.board[new_row][new_col].is_empty() {
                    moves.push((new_row, new_col));
                } else if let Some(target_piece) = self.board[new_row][new_col].piece() {
                    if target_piece.color != piece.color {
                        moves.push((new_row, new_col));
                    }
                    break;
                } else {
                    break;
                }
                new_row = (new_row as i32 + dr) as usize;
                new_col = (new_col as i32 + dc) as usize;
            }
        }
    }

    fn get_king_moves(&self, row: usize, col: usize, piece: Piece, moves: &mut Vec<(usize, usize)>) {
        let king_moves = [
            (-1, -1), (-1, 0), (-1, 1),
            (0, -1),           (0, 1),
            (1, -1),  (1, 0),  (1, 1)
        ];

        for &(dr, dc) in &king_moves {
            let new_row_i32 = row as i32 + dr;
            let new_col_i32 = col as i32 + dc;
            
            if new_row_i32 >= 0 && new_row_i32 < BOARD_SIZE as i32 && 
               new_col_i32 >= 0 && new_col_i32 < BOARD_SIZE as i32 {
                let new_row = new_row_i32 as usize;
                let new_col = new_col_i32 as usize;
                
                if self.board[new_row][new_col].is_empty() {
                    moves.push((new_row, new_col));
                } else if let Some(target_piece) = self.board[new_row][new_col].piece() {
                    if target_piece.color != piece.color {
                        moves.push((new_row, new_col));
                    }
                }
            }
        }
    }

    fn handle_click(&mut self, x: i32, y: i32) {
        let board_x = 2;
        let board_y = 2;
        
        if x < board_x || y < board_y {
            return;
        }
        
        let relative_x = x - board_x;
        let relative_y = y - board_y;
        
        let col = (relative_x / CELL_WIDTH) as usize;
        let row = (relative_y / CELL_HEIGHT) as usize;
        
        if row >= BOARD_SIZE || col >= BOARD_SIZE {
            return;
        }

        match self.selection_state {
            SelectionState::None => {
                if let Some(piece) = self.board[row][col].piece() {
                    if piece.color == self.current_player {
                        self.selection_state = SelectionState::Selected((row, col));
                        self.possible_moves = self.get_possible_moves(row, col);
                    }
                }
            },
            SelectionState::Selected((selected_row, selected_col)) => {
                if (row, col) == (selected_row, selected_col) {
                    self.selection_state = SelectionState::None;
                    self.possible_moves.clear();
                } else if self.possible_moves.contains(&(row, col)) {
                    self.make_move(selected_row, selected_col, row, col);
                    self.selection_state = SelectionState::None;
                    self.possible_moves.clear();
                    self.current_player = self.current_player.opposite();
                } else if let Some(piece) = self.board[row][col].piece() {
                    if piece.color == self.current_player {
                        self.selection_state = SelectionState::Selected((row, col));
                        self.possible_moves = self.get_possible_moves(row, col);
                    }
                }
            }
        }
    }

    fn make_move(&mut self, from_row: usize, from_col: usize, to_row: usize, to_col: usize) {
        if let Some(piece) = self.board[from_row][from_col].piece() {
            self.board[to_row][to_col] = CellState::Occupied(piece);
            self.board[from_row][from_col] = CellState::Empty;
        }
    }
}

impl OnPaint for ChessLogic {
    fn on_paint(&self, surface: &mut Surface, _theme: &Theme) {
        surface.clear(char!("' ',black,black"));
        
        let board_x = 2;
        let board_y = 2;
        
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                let x = board_x + (col as i32 * CELL_WIDTH);
                let y = board_y + (row as i32 * CELL_HEIGHT);
                let rect = Rect::with_size(x, y, CELL_WIDTH as u16, CELL_HEIGHT as u16);
                
                let bg_color = if (row + col) % 2 == 0 {
                    Color::DarkBlue
                } else {
                    Color::Gray
                };
                
                let cell_bg = match self.selection_state {
                    SelectionState::Selected((selected_row, selected_col)) 
                        if (row, col) == (selected_row, selected_col) => Color::Yellow,
                    _ => bg_color,
                };
                
                let final_bg = if self.possible_moves.contains(&(row, col)) {
                    Color::Green
                } else {
                    cell_bg
                };
                
                surface.fill_rect(rect, Character::new(' ', Color::White, final_bg, CharFlags::None));
                
                if let Some(piece) = self.board[row][col].piece() {
                    self.draw_piece(surface, rect, piece);
                }
            }
        }
        
        let current_player_text = match self.current_player {
            PieceColor::White => "White Player's Turn",
            PieceColor::Black => "Black Player's Turn",
        };
        surface.write_string(2, 0, current_player_text, charattr!("white"), false);
    }
}

impl OnMouseEvent for ChessLogic {
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        match event {
            MouseEvent::Pressed(data) => {
                self.handle_click(data.x, data.y);
                EventProcessStatus::Processed
            },
            _ => EventProcessStatus::Ignored,
        }
    }
}

impl OnKeyPressed for ChessLogic {
    fn on_key_pressed(&mut self, key: Key, _character: char) -> EventProcessStatus {
        match key.value() {
            key!("Escape") => {
                self.selection_state = SelectionState::None;
                self.possible_moves.clear();
                EventProcessStatus::Processed
            },
            key!("R") => {
                self.board = [[CellState::Empty; BOARD_SIZE]; BOARD_SIZE];
                self.current_player = PieceColor::White;
                self.game_state = GameState::Playing;
                self.selection_state = SelectionState::None;
                self.possible_moves.clear();
                self.setup_initial_board();
                EventProcessStatus::Processed
            },
            _ => EventProcessStatus::Ignored,
        }
    }
}
