use appcui::prelude::*;
use super::images;
use std::str::FromStr;

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



    fn color(&self) -> Color {
        match self.color {
            PieceColor::White => Color::White,  
            PieceColor::Black => Color::Red, 
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
    // This is a demo (not a full game) - the purpose is to demonstrate how it could work
    //Checkmate,
    //Stalemate,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SelectionState {
    None,
    Selected((usize, usize)),
}

const BOARD_SIZE: usize = 8;
const CELL_WIDTH: i32 = 8;
const CELL_HEIGHT: i32 = 4;
const BOARD_LEFT_POS: i32 = 2;
const BOARD_TOP_POS: i32 = 0;

#[CustomControl(overwrite = OnPaint+OnKeyPressed+OnMouseEvent)]
pub struct ChessLogic {
    board: [[CellState; BOARD_SIZE]; BOARD_SIZE],
    current_player: PieceColor,
    game_state: GameState,
    selection_state: SelectionState,
    possible_moves: Vec<(usize, usize)>,
    queen_image: BitTile<256>,
    pawn_image: BitTile<256>,
    rook_image: BitTile<256>,
    knight_image: BitTile<256>,
    bishop_image: BitTile<256>,
    king_image: BitTile<256>,
    hovered: Option<(usize, usize)>,
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
            queen_image: BitTile::from_str(images::QUEEN_IMAGE).unwrap(),
            pawn_image: BitTile::from_str(images::PAWN_IMAGE).unwrap(),
            rook_image: BitTile::from_str(images::ROOK_IMAGE).unwrap(),
            knight_image: BitTile::from_str(images::KNIGHT_IMAGE).unwrap(),
            bishop_image: BitTile::from_str(images::BISHOP_IMAGE).unwrap(),
            king_image: BitTile::from_str(images::KING_IMAGE).unwrap(),
            hovered: None,
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
        let tile = match piece.piece_type {
            PieceType::Queen => self.queen_image,
            PieceType::Pawn => self.pawn_image,
            PieceType::Rook => self.rook_image,
            PieceType::Knight => self.knight_image,
            PieceType::Bishop => self.bishop_image,
            PieceType::King => self.king_image,
        };

        surface.draw_tile(rect.left(), rect.top(), &tile, piece.color(), Color::Transparent, BitTileRenderMethod::Braille);
        
        
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

    fn mouse_pos_to_board_pos(&self, x: i32, y: i32) -> Option<(usize, usize)> {        
        if x < BOARD_LEFT_POS || y < BOARD_TOP_POS {
            return None;
        }
        let col = ((x - BOARD_LEFT_POS) / CELL_WIDTH) as usize;
        let row = ((y - BOARD_TOP_POS) / CELL_HEIGHT) as usize;
        
        if row >= BOARD_SIZE || col >= BOARD_SIZE {
            return None;
        }
        
        Some((row, col))
    }

    fn click_on_board(&mut self, col: usize, row: usize) {
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
        
        let (hovered_row, hovered_col) = self.hovered.unwrap_or((usize::MAX, usize::MAX));
        
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                let x = BOARD_LEFT_POS + (col as i32 * CELL_WIDTH);
                let y = BOARD_TOP_POS + (row as i32 * CELL_HEIGHT);
                let rect = Rect::with_size(x, y, CELL_WIDTH as u16, CELL_HEIGHT as u16);
                
                let bg_color = if (row + col) % 2 == 0 {
                    Color::Black
                } else {
                    Color::from_rgb(32, 32, 32)
                    //Color::Olive
                };
                
                let cell_bg = match self.selection_state {
                    SelectionState::Selected((selected_row, selected_col)) 
                        if (row, col) == (selected_row, selected_col) => Color::Yellow,
                    _ => bg_color,
                };
                
                let final_bg = if row == hovered_row && col == hovered_col {
                    Color::DarkRed
                } else if self.possible_moves.contains(&(row, col)) {
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
                if let Some((row, col)) = self.mouse_pos_to_board_pos(data.x, data.y) {
                    self.click_on_board(col, row);
                }
                EventProcessStatus::Processed
            },
            MouseEvent::Over(data) => {
                let new_hovered = self.mouse_pos_to_board_pos(data.x, data.y);
                if new_hovered != self.hovered {
                    self.hovered = new_hovered;
                    EventProcessStatus::Processed
                } else {
                    EventProcessStatus::Ignored
                }
            },
            MouseEvent::Leave | MouseEvent::Enter => {
                self.hovered = None;
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
