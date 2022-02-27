use crate::core::{Direction, Position};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Segment {
    position: Position,
    direction: Direction,
    segment_type: SegmentType,
    head: bool,
}

impl Segment {
    pub fn new_head(position: Position, direction: Direction) -> Self {
        Self {
            position,
            direction,
            head: true,
            segment_type: SegmentType::from(direction),
        }
    }

    pub fn new_tail(position: Position, direction: Direction) -> Self {
        Self {
            position,
            direction,
            head: false,
            segment_type: SegmentType::from(direction),
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
        self.segment_type = match (self.direction, direction) {
            (Direction::Left, Direction::Down) | (Direction::Up, Direction::Right) => {
                SegmentType::LeftDownSegment
            }
            (Direction::Left, Direction::Up) | (Direction::Down, Direction::Right) => {
                SegmentType::LeftUpSegment
            }
            (Direction::Right, Direction::Down) | (Direction::Up, Direction::Left) => {
                SegmentType::RightDownSegment
            }
            (Direction::Right, Direction::Up) | (Direction::Down, Direction::Left) => {
                SegmentType::RightUpSegment
            }
            (_, direction) => SegmentType::from(*direction),
        };
        self.direction = *direction;
    }

    pub fn opposite_neighbor(&self, direction: &Direction) -> Option<Position> {
        self.position.neighbor(direction.opposite())
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
            match self.segment_type {
                SegmentType::DownSegment | SegmentType::UpSegment => "│",
                SegmentType::LeftSegment | SegmentType::RightSegment => "──",
                SegmentType::LeftDownSegment => "┌─",
                SegmentType::LeftUpSegment => "└─",
                SegmentType::RightDownSegment => "┐",
                SegmentType::RightUpSegment => "┘",
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SegmentType {
    LeftSegment,
    RightSegment,
    UpSegment,
    DownSegment,
    LeftUpSegment,
    LeftDownSegment,
    RightUpSegment,
    RightDownSegment,
}

impl From<Direction> for SegmentType {
    fn from(d: Direction) -> SegmentType {
        match d {
            Direction::Down => SegmentType::DownSegment,
            Direction::Up => SegmentType::UpSegment,
            Direction::Left => SegmentType::LeftSegment,
            Direction::Right => SegmentType::RightSegment,
        }
    }
}
