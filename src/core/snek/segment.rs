use crate::core::{DeathCause, Direction, Position};

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

    pub fn get_direction(&self) -> Direction {
        self.direction
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    pub fn inch(&mut self) -> Option<DeathCause> {
        self.position.nudge(self.direction)
    }

    pub fn get_char(&self) -> char {
        if self.is_head() {
            self.direction.get_head_char()
        } else {
            self.direction.get_tail_char()
        }
    }
}
