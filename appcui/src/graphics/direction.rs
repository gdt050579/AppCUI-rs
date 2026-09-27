use super::Point;

#[derive(Copy, Clone)]
pub(super) enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub(super) fn from_one_unit_step(dx: i32, dy: i32) -> Option<Self> {
        match (dx, dy) {
            (1, 0) => Some(Self::Right),
            (-1, 0) => Some(Self::Left),
            (0, 1) => Some(Self::Down),
            (0, -1) => Some(Self::Up),
            _ => None,
        }
    }
    pub(super) fn from_points(start: Point, end: Point) -> Option<Self> {
        let dx = end.x - start.x;
        let dy = end.y - start.y;
        if dx == 0 && dy == 0 {
            return None;
        }

        Some(if dx.abs() >= dy.abs() {
            if dx < 0 {
                Direction::Left
            } else {
                Direction::Right
            }
        } else if dy < 0 {
            Direction::Up
        } else {
            Direction::Down
        })
    }
}
