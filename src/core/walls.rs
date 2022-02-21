use crate::core::Position;

#[derive(Debug, Clone)]
pub struct Walls {
    width: usize,
    height: usize,
}

impl Walls {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }
    pub fn contains_position(&self, position: &Position) -> bool {
        let (x, y) = position.get_coordinates();
        self.contains_x(x) && self.contains_y(y)
    }

    pub(crate) fn get_max_segments(&self) -> usize {
        self.width * self.height
    }

    fn contains_x(&self, x_position: usize) -> bool {
        x_position >= self.left_wall() && x_position <= self.right_wall()
    }

    fn contains_y(&self, y_position: usize) -> bool {
        y_position >= self.top_wall() && y_position <= self.bottom_wall()
    }

    pub(crate) fn top_wall(&self) -> usize {
        0
    }

    pub(crate) fn left_wall(&self) -> usize {
        0
    }

    pub(crate) fn bottom_wall(&self) -> usize {
        self.height - 1
    }

    pub(crate) fn right_wall(&self) -> usize {
        self.width - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn walls_know_valid_positions() {
        let walls = Walls::new(50, 50);
        let valid_position = Position::new(20, 20);
        assert!(walls.contains_position(&valid_position))
    }

    #[test]
    fn walls_know_invalid_x_and_y_positions() {
        let walls = Walls::new(50, 50);
        let invalid_position = Position::new(200, 200);
        assert!(!walls.contains_position(&invalid_position))
    }

    #[test]
    fn walls_are_not_susceptible_to_off_by_one() {
        let walls = Walls::new(5, 5);
        let valid_position = Position::new(4, 4);
        assert!(walls.contains_position(&valid_position));
        let invalid_position = Position::new(5, 5);
        assert!(!walls.contains_position(&invalid_position))
    }

    #[test]
    fn walls_know_invalid_y_position() {
        let walls = Walls::new(50, 50);
        let valid_position = Position::new(20, 200);
        assert!(!walls.contains_position(&valid_position))
    }

    #[test]
    fn walls_know_invalid_x_position() {
        let walls = Walls::new(50, 50);
        let valid_position = Position::new(200, 20);
        assert!(!walls.contains_position(&valid_position))
    }

    #[test]
    fn walls_know_max_segments() {
        // [_,_,_,_,_]
        // [_,_,_,_,_]
        // [_,_,_,_,_]
        // [_,_,_,_,_]
        // [_,_,_,_,_]
        let walls = Walls::new(5, 5);
        assert_eq!(walls.get_max_segments(), 25)
    }
}
