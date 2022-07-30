use crate::core::{Apple, Segment, WallType};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Cell {
    Snek(Segment),
    Apple(Apple),
    Wall(WallType),
    Free,
}
impl Cell {
    pub fn display(&self) -> String {
        match self {
            Cell::Snek(segment) => segment.display(),
            Cell::Apple(_) => "🍎".to_string(),
            Cell::Wall(wall_type) => wall_type.display(),
            Cell::Free => "  ".to_string(),
        }
    }
}
