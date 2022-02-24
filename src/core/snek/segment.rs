use crate::core::{Direction, Position};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Segment {
    position: Position,
    direction: Direction,
    head: bool,
}

impl Segment {
    pub fn new_head(position: Position, direction: Direction) -> Self {
        Self {
            position,
            direction,
            head: true,
        }
    }

    pub fn new_tail(position: Position, direction: Direction) -> Self {
        Self {
            position,
            direction,
            head: false,
        }
    }

    pub fn make_tail(&mut self) {
        self.head = false;
    }

    pub fn is_head(&self) -> bool {
        self.head
    }

    pub fn is_tail(&self) -> bool {
        !self.head
    }

    pub fn get_position(&self) -> Position {
        self.position
    }

    pub fn set_position(&mut self, position: &Position) {
        self.position = *position
    }

    pub fn get_direction(&self) -> Direction {
        self.direction
    }

    pub fn set_direction(&mut self, direction: &Direction) {
        self.direction = *direction;
    }

    pub fn opposite_neighbor(&self, direction: &Direction) -> Option<Position> {
        match direction {
            Direction::Up => self.position.neighbor(Direction::Down),
            Direction::Down => self.position.neighbor(Direction::Up),
            Direction::Left => self.position.neighbor(Direction::Right),
            Direction::Right => self.position.neighbor(Direction::Left),
        }
    }

    pub fn display(&self) -> &str {
        if self.is_head() {
            match self.direction {
                Direction::Up => "^",
                Direction::Down => "v",
                Direction::Left => "<",
                Direction::Right => ">",
            }
        } else {
            match self.direction {
                Direction::Down | Direction::Up => "|",
                Direction::Left | Direction::Right => "-",
            }
        }
    }
}
