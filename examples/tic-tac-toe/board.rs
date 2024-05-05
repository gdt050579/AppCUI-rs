use appcui::prelude::*;

#[derive(Copy, Clone)]
enum Player {
    X,
    O,
}

#[derive(Copy, Clone)]
enum GameState {
    InProgress,
    Draw,
    Winner(Player),
}

#[CustomControl(overwrite: OnPaint+OnKeyPressed+OnMouseEvent)]
pub struct Board {
    cells: Vec<Option<Player>>,
    status: GameState,
}

impl Board {
    pub fn new() -> Self {
        Self {
            base: ControlBase::new(Layout::new("x:2,y:1,w:34,h:19"), true),
            cells: vec![None; 9],
            status: GameState::InProgress
        }
    }
    pub fn reset_game(&mut self) {
        for c in self.cells.iter_mut() {
            *c = None;
        }
        self.status = GameState::InProgress;
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
                Some(Player::X) => self.paint_x(x, y, surface),
                Some(Player::O) => self.paint_o(x, y, surface),
                None => {}
            }
        }
    }
}
impl OnKeyPressed for Board {}
impl OnMouseEvent for Board {}
