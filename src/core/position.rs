use crate::core::{DeathCause, Direction};

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Position {
    x: usize,
    y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn nudge(&mut self, direction: Direction) -> Option<DeathCause> {
        match direction {
            Direction::Up => {
                if self.y == 0 {
                    return Some(DeathCause::Wall);
                }
                self.y -= 1;
            }
            Direction::Down => self.y += 1,
            Direction::Right => self.x += 1,
            Direction::Left => {
                if self.x == 0 {
                    return Some(DeathCause::Wall);
                }
                self.x -= 1;
            }
        }
        None
    }
    pub fn get_coordinates(&self) -> (usize, usize) {
        (self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_nudge_left() {
        let mut position = Position::new(10, 10);
        assert_eq!((10, 10), position.get_coordinates());
        position.nudge(Direction::Left);
        assert_eq!((9, 10), position.get_coordinates());
    }

    #[test]
    fn it_can_nudge_right() {
        let mut position = Position::new(10, 10);
        assert_eq!((10, 10), position.get_coordinates());
        position.nudge(Direction::Right);
        assert_eq!((11, 10), position.get_coordinates());
    }

    #[test]
    fn it_can_nudge_down() {
        let mut position = Position::new(10, 10);
        assert_eq!((10, 10), position.get_coordinates());
        position.nudge(Direction::Down);
        assert_eq!((10, 11), position.get_coordinates());
    }

    #[test]
    fn it_an_nudge_up() {
        let mut position = Position::new(10, 10);
        assert_eq!((10, 10), position.get_coordinates());
        position.nudge(Direction::Up);
        assert_eq!((10, 9), position.get_coordinates());
    }
}
