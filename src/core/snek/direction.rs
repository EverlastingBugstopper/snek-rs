#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn get_tail_char(&self) -> char {
        match self {
            Direction::Up | Direction::Down => '|',
            Direction::Left | Direction::Right => '-',
        }
    }

    pub fn describe(&self) -> &str {
        match self {
            Direction::Up => "up",
            Direction::Down => "down",
            Direction::Left => "left",
            Direction::Right => "right",
        }
    }

    pub fn is_on_a_dime(&self, attempted_direction: &Direction) -> bool {
        self.opposite().eq(&attempted_direction)
    }

    pub fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}
