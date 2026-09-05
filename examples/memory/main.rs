//! Memory / Concentration as an `InputApp` example.
//!
//! This mode has **no tick/timer**. Handlers return `EventProcessStatus::Processed`
//! only when visible state changed (that triggers a repaint) and `Ignored` otherwise
//! (the app stays idle). Mismatched cards therefore flip back on the next select
//! action rather than after a delay — a timed auto-flip would belong in `frame_app`.
//! `on_paint` is `&self`; all mutation lives in the input handlers.

use appcui::prelude::*;

// Change these to try a quick 4×2 game, the default 4×4, or a larger 6×6.
const ROWS: u32 = 4;
const COLS: u32 = 4;

const CARD_W: i32 = 7;
const CARD_H: i32 = 3;
const GAP_X: i32 = 1;
const GAP_Y: i32 = 1;
const HEADER_H: i32 = 2;
const FOOTER_H: i32 = 1;

// First `pairs` entries are used; `pairs` is derived from the grid size, not hardcoded.
const SYMBOLS: &[char] = &[
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', // 4×4
    'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', // up to 6×6
];

#[derive(Copy, Clone, Eq, PartialEq)]
enum CardState {
    FaceDown,
    FaceUp,
    Found,
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Phase {
    SelectFirst,
    SelectSecond { first: usize },
    // InputApp has no tick/timer, so a mismatch stays visible until the next
    // select action. A timed auto-flip would belong in `frame_app`.
    ShowMismatch { a: usize, b: usize },
}

struct Card {
    symbol: char,
    state: CardState,
}

struct MemoryGame {
    rows: u32,
    cols: u32,
    cards: Vec<Card>,
    cursor_row: u32,
    cursor_col: u32,
    phase: Phase,
    moves: u32,
    won: bool,
    best: Option<u32>,
    term_size: Size,
}

impl MemoryGame {
    fn new(rows: u32, cols: u32) -> Self {
        assert!(rows > 0 && cols > 0, "grid must be at least 1×1");
        let total = rows.checked_mul(cols).expect("grid is too large");
        assert!(
            total.is_multiple_of(2),
            "grid must have an even number of cards (got {rows}×{cols} = {total})"
        );
        let pairs = (total / 2) as usize;
        assert!(
            pairs <= SYMBOLS.len(),
            "need {pairs} pair symbols for a {rows}×{cols} grid, but SYMBOLS has only {}",
            SYMBOLS.len()
        );

        let mut game = Self {
            rows,
            cols,
            cards: Vec::with_capacity(total as usize),
            cursor_row: 0,
            cursor_col: 0,
            phase: Phase::SelectFirst,
            moves: 0,
            won: false,
            best: None,
            term_size: Size::new(0, 0),
        };
        game.reset();
        game
    }

    fn reset(&mut self) {
        let pairs = (self.rows * self.cols / 2) as usize;
        self.cards.clear();
        for &symbol in SYMBOLS.iter().take(pairs) {
            self.cards.push(Card {
                symbol,
                state: CardState::FaceDown,
            });
            self.cards.push(Card {
                symbol,
                state: CardState::FaceDown,
            });
        }
        shuffle(&mut self.cards);
        self.cursor_row = 0;
        self.cursor_col = 0;
        self.phase = Phase::SelectFirst;
        self.moves = 0;
        self.won = false;
    }

    fn index(&self, row: u32, col: u32) -> usize {
        (row as usize) * (self.cols as usize) + (col as usize)
    }

    fn board_size(&self) -> (i32, i32) {
        let cols = self.cols as i32;
        let rows = self.rows as i32;
        let w = cols.saturating_mul(CARD_W).saturating_add(cols.saturating_sub(1).saturating_mul(GAP_X));
        let h = rows.saturating_mul(CARD_H).saturating_add(rows.saturating_sub(1).saturating_mul(GAP_Y));
        (w, h)
    }

    fn needed_size(&self) -> (i32, i32) {
        let (bw, bh) = self.board_size();
        (bw, bh.saturating_add(HEADER_H).saturating_add(FOOTER_H))
    }

    fn origin_for(&self, size: Size) -> (i32, i32) {
        let (bw, bh) = self.board_size();
        let sw = size.width as i32;
        let sh = size.height as i32;
        let x = sw.saturating_sub(bw) / 2;
        let avail_h = sh.saturating_sub(HEADER_H).saturating_sub(FOOTER_H);
        let y = HEADER_H.saturating_add(avail_h.saturating_sub(bh) / 2);
        (x.max(0), y.max(HEADER_H))
    }

    fn card_rect(&self, origin: (i32, i32), row: u32, col: u32) -> Rect {
        let (ox, oy) = origin;
        let x = ox.saturating_add((col as i32).saturating_mul(CARD_W.saturating_add(GAP_X)));
        let y = oy.saturating_add((row as i32).saturating_mul(CARD_H.saturating_add(GAP_Y)));
        Rect::with_size(x, y, CARD_W.max(1) as u16, CARD_H.max(1) as u16)
    }

    fn cell_at_pixel(&self, x: i32, y: i32) -> Option<(u32, u32)> {
        let origin = self.origin_for(self.term_size);
        for row in 0..self.rows {
            for col in 0..self.cols {
                if self.card_rect(origin, row, col).contains(Point::new(x, y)) {
                    return Some((row, col));
                }
            }
        }
        None
    }

    fn move_cursor(&mut self, d_row: i32, d_col: i32) -> EventProcessStatus {
        // No wrap: moving off the edge is a no-op, so we return Ignored and skip a repaint.
        let row = self.cursor_row as i32 + d_row;
        let col = self.cursor_col as i32 + d_col;
        if row < 0 || col < 0 || row >= self.rows as i32 || col >= self.cols as i32 {
            EventProcessStatus::Ignored
        } else {
            self.cursor_row = row as u32;
            self.cursor_col = col as u32;
            EventProcessStatus::Processed
        }
    }

    fn select_cell(&mut self, row: u32, col: u32) -> EventProcessStatus {
        if self.won {
            return EventProcessStatus::Ignored;
        }
        if matches!(self.phase, Phase::ShowMismatch { .. }) {
            return self.hide_mismatch();
        }

        let idx = self.index(row, col);
        match self.cards.get(idx).map(|c| c.state) {
            Some(CardState::FaceDown) => self.reveal(idx),
            _ => EventProcessStatus::Ignored,
        }
    }

    fn hide_mismatch(&mut self) -> EventProcessStatus {
        if let Phase::ShowMismatch { a, b } = self.phase {
            if let Some(card) = self.cards.get_mut(a) {
                card.state = CardState::FaceDown;
            }
            if let Some(card) = self.cards.get_mut(b) {
                card.state = CardState::FaceDown;
            }
            self.phase = Phase::SelectFirst;
            EventProcessStatus::Processed
        } else {
            EventProcessStatus::Ignored
        }
    }

    fn reveal(&mut self, idx: usize) -> EventProcessStatus {
        match self.phase {
            Phase::SelectFirst => {
                self.cards[idx].state = CardState::FaceUp;
                self.phase = Phase::SelectSecond { first: idx };
                EventProcessStatus::Processed
            }
            Phase::SelectSecond { first } => {
                if idx == first {
                    return EventProcessStatus::Ignored;
                }
                self.cards[idx].state = CardState::FaceUp;
                self.moves = self.moves.saturating_add(1);
                if self.cards[first].symbol == self.cards[idx].symbol {
                    self.cards[first].state = CardState::Found;
                    self.cards[idx].state = CardState::Found;
                    self.phase = Phase::SelectFirst;
                    if self.cards.iter().all(|c| c.state == CardState::Found) {
                        self.won = true;
                        self.best = Some(self.best.map_or(self.moves, |best| best.min(self.moves)));
                    }
                } else {
                    self.phase = Phase::ShowMismatch { a: first, b: idx };
                }
                EventProcessStatus::Processed
            }
            Phase::ShowMismatch { .. } => EventProcessStatus::Ignored,
        }
    }

    fn write_centered(&self, surface: &mut Surface, y: i32, text: &str, attr: CharAttribute) {
        let x = (surface.size().width as i32) / 2;
        let format = TextFormatBuilder::new()
            .position(x, y)
            .attribute(attr)
            .align(TextAlignment::Center)
            .build();
        surface.write_text(text, &format);
    }

    fn draw_card(surface: &mut Surface, rect: Rect, state: CardState, symbol: char, is_cursor: bool) {
        let (fill, glyph_fg, glyph_bg, border, line) = match (state, is_cursor) {
            (CardState::FaceDown, true) => (
                Character::new(SpecialChar::Block50, Color::Silver, Color::DarkBlue, CharFlags::None),
                Color::Silver,
                Color::DarkBlue,
                charattr!("yellow"),
                LineType::Double,
            ),
            (CardState::FaceDown, false) => (
                Character::new(SpecialChar::Block50, Color::Gray, Color::DarkBlue, CharFlags::None),
                Color::Gray,
                Color::DarkBlue,
                charattr!("silver"),
                LineType::Single,
            ),
            (CardState::FaceUp, true) => (
                Character::new(' ', Color::Yellow, Color::DarkBlue, CharFlags::None),
                Color::Yellow,
                Color::DarkBlue,
                charattr!("yellow"),
                LineType::Double,
            ),
            (CardState::FaceUp, false) => (
                Character::new(' ', Color::Yellow, Color::DarkBlue, CharFlags::None),
                Color::Yellow,
                Color::DarkBlue,
                charattr!("aqua"),
                LineType::Single,
            ),
            (CardState::Found, true) => (
                Character::new(' ', Color::Green, Color::Black, CharFlags::None),
                Color::Green,
                Color::Black,
                charattr!("yellow"),
                LineType::Double,
            ),
            (CardState::Found, false) => (
                Character::new(' ', Color::DarkGreen, Color::Black, CharFlags::None),
                Color::Green,
                Color::Black,
                charattr!("darkgreen"),
                LineType::Single,
            ),
        };

        surface.fill_rect(rect, fill);
        surface.draw_rect(rect, line, border);

        if state != CardState::FaceDown {
            surface.write_char(
                rect.center_x(),
                rect.center_y(),
                Character::new(symbol, glyph_fg, glyph_bg, CharFlags::None),
            );
        }
    }
}

// Fisher–Yates with a tiny LCG so the example stays dependency-free.
fn shuffle(cards: &mut [Card]) {
    let mut seed = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0xC0FFEE);
    if seed == 0 {
        seed = 0xC0FFEE;
    }
    for i in (1..cards.len()).rev() {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        let j = (seed as usize) % (i + 1);
        cards.swap(i, j);
    }
}

impl InputApp for MemoryGame {
    fn on_resize(&mut self, new_size: Size) {
        // The runtime always repaints after a resize, so this method has no return value.
        self.term_size = new_size;
    }

    fn on_key_event(&mut self, key: Key, _ch: char) -> EventProcessStatus {
        // Return Processed only when something visible changed. Ignored skips the
        // repaint, which is why an InputApp sits idle between meaningful inputs.
        if key.code == KeyCode::R {
            self.reset();
            return EventProcessStatus::Processed;
        }
        match key.value() {
            key!("Escape") => {
                App::close();
                EventProcessStatus::Processed
            }
            key!("Up") => self.move_cursor(-1, 0),
            key!("Down") => self.move_cursor(1, 0),
            key!("Left") => self.move_cursor(0, -1),
            key!("Right") => self.move_cursor(0, 1),
            key!("Enter") | key!("Space") => self.select_cell(self.cursor_row, self.cursor_col),
            _ => EventProcessStatus::Ignored,
        }
    }

    fn on_mouse_event(&mut self, ev: &MouseEvent) -> EventProcessStatus {
        match ev {
            MouseEvent::Pressed(data) => match self.cell_at_pixel(data.x, data.y) {
                Some((row, col)) => {
                    let cursor_moved = self.cursor_row != row || self.cursor_col != col;
                    self.cursor_row = row;
                    self.cursor_col = col;
                    if self.select_cell(row, col) == EventProcessStatus::Processed || cursor_moved {
                        EventProcessStatus::Processed
                    } else {
                        EventProcessStatus::Ignored
                    }
                }
                None => EventProcessStatus::Ignored,
            },
            // Mouse move / release / wheel never change visible state — skip the repaint.
            _ => EventProcessStatus::Ignored,
        }
    }

    // on_paint is &self: it only reads state and draws. All mutation lives in the handlers.
    fn on_paint(&self, surface: &mut Surface) {
        let size = surface.size();
        if size.width == 0 || size.height == 0 {
            return;
        }

        let (need_w, need_h) = self.needed_size();
        if (size.width as i32) < need_w || (size.height as i32) < need_h {
            self.write_centered(
                surface,
                (size.height as i32) / 2,
                "Terminal too small — enlarge the window.",
                charattr!("red"),
            );
            return;
        }

        let origin = self.origin_for(size);
        for row in 0..self.rows {
            for col in 0..self.cols {
                let idx = self.index(row, col);
                let card = &self.cards[idx];
                MemoryGame::draw_card(
                    surface,
                    self.card_rect(origin, row, col),
                    card.state,
                    card.symbol,
                    row == self.cursor_row && col == self.cursor_col,
                );
            }
        }

        let header = match self.best {
            Some(best) => format!("Memory — Moves: {}  Best: {}", self.moves, best),
            None => format!("Memory — Moves: {}", self.moves),
        };
        self.write_centered(surface, 0, &header, charattr!("aqua"));

        let footer = if self.won {
            "r = restart   Esc = quit"
        } else if matches!(self.phase, Phase::ShowMismatch { .. }) {
            "Enter/Space/click: hide mismatched cards   Esc: quit"
        } else {
            "Arrows: move   Enter/Space: flip   Click: select   R: restart   Esc: quit"
        };
        let footer_y = (size.height as i32).saturating_sub(1);
        self.write_centered(surface, footer_y, footer, charattr!("gray"));

        if self.won {
            let msg = format!("You won in {} moves!", self.moves);
            let hint = "r = restart, Esc = quit";
            let box_w = ((msg.len().max(hint.len()) as i32).saturating_add(4)).max(1) as u16;
            let box_rect = Rect::with_alignment((size.width as i32) / 2, (size.height as i32) / 2, box_w, 5, RectAlignment::Center);
            surface.fill_rect(box_rect, Character::new(' ', Color::Yellow, Color::DarkGreen, CharFlags::None));
            surface.draw_rect(box_rect, LineType::Double, charattr!("yellow,darkgreen"));
            self.write_centered(surface, box_rect.center_y().saturating_sub(1), &msg, charattr!("yellow,darkgreen"));
            self.write_centered(surface, box_rect.center_y().saturating_add(1), hint, charattr!("white,darkgreen"));
        }
    }
}

fn main() -> Result<(), appcui::system::Error> {
    App::input_app(MemoryGame::new(ROWS, COLS)).title("Memory").auto_close(false).run()
}
