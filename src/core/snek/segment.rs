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

    pub fn dead_head(&mut self) {
        self.segment_type = SegmentType::DeadHead;
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
                SegmentType::TopLeftSegment
            }
            (Direction::Left, Direction::Up) | (Direction::Down, Direction::Right) => {
                SegmentType::BottomLeftSegment
            }
            (Direction::Right, Direction::Down) | (Direction::Up, Direction::Left) => {
                SegmentType::TopRightSegment
            }
            (Direction::Right, Direction::Up) | (Direction::Down, Direction::Left) => {
                SegmentType::BottomRightSegment
            }
            (_, direction) => SegmentType::from(*direction),
        };
        self.direction = *direction;
    }

    pub fn opposite_neighbor(&self, direction: &Direction) -> Option<Position> {
        self.position.neighbor(direction.opposite())
    }

    pub fn display(&self) -> String {
        if self.is_head() {
            if let SegmentType::DeadHead = self.segment_type {
                "💀"
            } else {
                match self.direction {
                    Direction::Up => "△",
                    Direction::Down => "▽",
                    Direction::Left => "◁",
                    Direction::Right => " ▷",
                }
            }
        } else {
            match self.segment_type {
                SegmentType::DownSegment | SegmentType::UpSegment => "│",
                SegmentType::LeftSegment | SegmentType::RightSegment => "──",
                SegmentType::TopLeftSegment => "╭─",
                SegmentType::BottomLeftSegment => "╰─",
                SegmentType::TopRightSegment => "╮",
                SegmentType::BottomRightSegment => "╯",
                _ => "",
            }
        }
        .to_string()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SegmentType {
    DeadHead,
    LeftSegment,
    RightSegment,
    UpSegment,
    DownSegment,
    BottomLeftSegment,
    TopLeftSegment,
    BottomRightSegment,
    TopRightSegment,
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
