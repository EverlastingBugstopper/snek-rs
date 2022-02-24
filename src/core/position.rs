use crate::core::Direction;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub struct Position {
    x: usize,
    y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    /// gets the neighboring position
    /// iff it is a valid position.
    /// if the computed neighbor would result
    /// in a negative position, there is no neighbor
    pub fn neighbor(&self, direction: Direction) -> Option<Position> {
        match direction {
            Direction::Up => {
                if self.y == 0 {
                    None
                } else {
                    Some(Position::new(self.x, self.y - 1))
                }
            }
            Direction::Down => Some(Position::new(self.x, self.y + 1)),
            Direction::Left => {
                if self.x == 0 {
                    None
                } else {
                    Some(Position::new(self.x - 1, self.y))
                }
            }
            Direction::Right => Some(Position::new(self.x + 1, self.y)),
        }
    }

    pub fn set(&mut self, position: Position) {
        *self = position;
    }

    pub fn set_x(&mut self, x: usize) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: usize) {
        self.y = y;
    }

    pub fn get_coordinates(&self) -> (usize, usize) {
        (self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_knows_negatives_are_not_the_answer() {
        let position = Position::new(0, 0);
        assert!(position.neighbor(Direction::Up).is_none());
        assert!(position.neighbor(Direction::Left).is_none());
    }

    #[test]
    fn it_can_nudge_left() {
        let mut old_position = Position::new(10, 10);
        assert_eq!((10, 10), old_position.get_coordinates());
        let new_position = old_position.neighbor(Direction::Left).unwrap();
        assert_eq!((9, 10), new_position.get_coordinates());
        old_position.set(new_position);
        assert_eq!((9, 10), old_position.get_coordinates());
    }

    #[test]
    fn it_can_nudge_right() {
        let mut old_position = Position::new(10, 10);
        assert_eq!((10, 10), old_position.get_coordinates());
        let new_position = old_position.neighbor(Direction::Right).unwrap();
        assert_eq!((11, 10), new_position.get_coordinates());
        old_position.set(new_position);
        assert_eq!((11, 10), old_position.get_coordinates());
    }

    #[test]
    fn it_can_nudge_down() {
        let mut old_position = Position::new(10, 10);
        assert_eq!((10, 10), old_position.get_coordinates());
        let new_position = old_position.neighbor(Direction::Down).unwrap();
        assert_eq!((10, 11), new_position.get_coordinates());
        old_position.set(new_position);
        assert_eq!((10, 11), old_position.get_coordinates());
    }

    #[test]
    fn it_can_nudge_up() {
        let mut old_position = Position::new(10, 10);
        assert_eq!((10, 10), old_position.get_coordinates());
        let new_position = old_position.neighbor(Direction::Up).unwrap();
        assert_eq!((10, 9), new_position.get_coordinates());
        old_position.set(new_position);
        assert_eq!((10, 9), old_position.get_coordinates());
    }
}
