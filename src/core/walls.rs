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
    pub fn collides_with(&self, position: &Position) -> bool {
        !self.is_position_inside(position)
    }

    pub fn is_position_inside(&self, position: &Position) -> bool {
        let (x, y) = position.get_coordinates();
        self.is_x_inside(x) && self.is_y_inside(y)
    }

    pub(crate) fn get_max_segments(&self) -> usize {
        (self.width - 2) * (self.height - 2)
    }

    fn is_x_inside(&self, x: usize) -> bool {
        x > self.left_wall() && x < self.right_wall()
    }

    fn is_y_inside(&self, y: usize) -> bool {
        y > self.top_wall() && y < self.bottom_wall()
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

    pub fn get_positions(&self) -> Vec<Position> {
        let mut positions = Vec::new();
        for x in self.left_wall()..=self.right_wall() {
            positions.push(Position::new(x, self.bottom_wall()));
            positions.push(Position::new(x, self.top_wall()));
        }
        for y in self.top_wall()..=self.bottom_wall() {
            positions.push(Position::new(self.left_wall(), y));
            positions.push(Position::new(self.right_wall(), y));
        }
        tracing::info!("{:?}", &positions);
        positions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn walls_know_valid_positions() {
        let walls = Walls::new(50, 50);
        let valid_position = Position::new(20, 20);
        assert!(walls.is_position_inside(&valid_position))
    }

    #[test]
    fn walls_know_invalid_x_and_y_positions() {
        let walls = Walls::new(50, 50);
        let invalid_position = Position::new(200, 200);
        assert!(!walls.is_position_inside(&invalid_position))
    }

    #[test]
    fn walls_are_not_susceptible_to_off_by_one() {
        let walls = Walls::new(5, 5);
        let valid_position = Position::new(1, 1);
        assert!(walls.is_position_inside(&valid_position));
        let valid_position = Position::new(3, 3);
        assert!(walls.is_position_inside(&valid_position));
        let invalid_position = Position::new(0, 0);
        assert!(!walls.is_position_inside(&invalid_position));
        let invalid_position = Position::new(4, 4);
        assert!(!walls.is_position_inside(&invalid_position))
    }

    #[test]
    fn walls_know_invalid_y_position() {
        let walls = Walls::new(50, 50);
        let valid_position = Position::new(20, 200);
        assert!(!walls.is_position_inside(&valid_position))
    }

    #[test]
    fn walls_know_invalid_x_position() {
        let walls = Walls::new(50, 50);
        let valid_position = Position::new(200, 20);
        assert!(!walls.is_position_inside(&valid_position))
    }

    #[test]
    fn walls_know_max_segments() {
        // [X,X,X,X,X]
        // [X,o,o,o,X]
        // [X,o,o,o,X]
        // [X,o,o,o,X]
        // [X,X,X,X,X]
        let walls = Walls::new(5, 5);
        assert_eq!(walls.get_max_segments(), 9)
    }

    #[test]
    fn walls_know_all_boundaries() {
        let walls = Walls::new(5, 5);
        let top_left_corner = Position::new(0, 0);
        let top_right_corner = Position::new(4, 0);
        let bottom_left_corner = Position::new(0, 4);
        let bottom_right_corner = Position::new(4, 4);
        assert!(walls.collides_with(&top_left_corner));
        assert!(walls.collides_with(&top_right_corner));
        assert!(walls.collides_with(&bottom_left_corner));
        assert!(walls.collides_with(&bottom_right_corner));
    }

    #[test]
    fn walls_are_sized_correctly_when_wide() {
        // [X,X,X,X,X,X]
        // [X,o,o,o,o,X]
        // [X,o,o,o,o,X]
        // [X,X,X,X,X,X]
        let walls = Walls::new(6, 4);
        for x in 2..=4 {
            for y in 1..=2 {
                assert!(walls.is_position_inside(&Position::new(x, y)));
                assert!(!walls.collides_with(&Position::new(x, y)));
            }
        }
        for x in 0..=6 {
            assert!(!walls.is_position_inside(&Position::new(x, 0)));
            assert!(walls.collides_with(&Position::new(x, 0)));
            assert!(!walls.is_position_inside(&Position::new(x, 3)));
            assert!(walls.collides_with(&Position::new(x, 3)));
        }
        for y in 0..=4 {
            assert!(!walls.is_position_inside(&Position::new(0, y)));
            assert!(walls.collides_with(&Position::new(0, y)));
            assert!(!walls.is_position_inside(&Position::new(5, y)));
            assert!(walls.collides_with(&Position::new(5, y)));
        }
    }

    #[test]
    fn walls_are_sized_correctly_when_tall() {
        // [X,X,X,X]
        // [X,o,o,X]
        // [X,o,o,X]
        // [X,o,o,X]
        // [X,o,o,X]
        // [X,X,X,X]
        let walls = Walls::new(4, 6);
        for x in 1..=2 {
            for y in 2..=4 {
                assert!(walls.is_position_inside(&Position::new(x, y)));
                assert!(!walls.collides_with(&Position::new(x, y)));
            }
        }
        for x in 0..=4 {
            assert!(!walls.is_position_inside(&Position::new(x, 0)));
            assert!(walls.collides_with(&Position::new(x, 0)));
            assert!(!walls.is_position_inside(&Position::new(x, 5)));
            assert!(walls.collides_with(&Position::new(x, 5)));
        }
        for y in 0..=6 {
            assert!(!walls.is_position_inside(&Position::new(0, y)));
            assert!(walls.collides_with(&Position::new(0, y)));
            assert!(!walls.is_position_inside(&Position::new(3, y)));
            assert!(walls.collides_with(&Position::new(3, y)));
        }
    }
}
