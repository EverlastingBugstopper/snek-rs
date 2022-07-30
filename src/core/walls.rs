use crate::core::Position;

#[derive(Debug, Clone)]
pub struct Walls {
    width: usize,
    height: usize,
    top_wall_y: usize,
    bottom_wall_y: usize,
    left_wall_x: usize,
    right_wall_x: usize,
    walls: Vec<Wall>,
}

impl Walls {
    pub fn new(width: usize, height: usize) -> Self {
        let mut walls = Vec::with_capacity(width * height);
        let top_wall_y = 0;
        let left_wall_x = 0;
        let bottom_wall_y = height - 1;
        let right_wall_x = width - 1;
        // create the top and bottom walls
        (left_wall_x..=right_wall_x).enumerate().for_each(|(i, x)| {
            let bottom_wall_position = Position::new(x, bottom_wall_y);
            let top_wall_position = Position::new(x, top_wall_y);
            let (top_wall_type, bottom_wall_type) = if i == left_wall_x {
                (WallType::TopLeftCorner, WallType::BottomLeftCorner)
            } else if i == right_wall_x {
                (WallType::TopRightCorner, WallType::BottomRightCorner)
            } else {
                (WallType::TopWall, WallType::BottomWall)
            };
            walls.push(Wall::new(bottom_wall_position, bottom_wall_type));
            walls.push(Wall::new(top_wall_position, top_wall_type));
        });

        (top_wall_y..=bottom_wall_y).enumerate().for_each(|(i, y)| {
            let left_wall_position = Position::new(left_wall_x, y);
            let right_wall_position = Position::new(right_wall_x, y);
            if i != top_wall_y && i != bottom_wall_y {
                walls.push(Wall::new(left_wall_position, WallType::LeftWall));
                walls.push(Wall::new(right_wall_position, WallType::RightWall));
            }
        });
        Self {
            walls,
            width,
            height,
            top_wall_y,
            bottom_wall_y,
            left_wall_x,
            right_wall_x,
        }
    }
    pub fn collides_with(&self, position: &Position) -> bool {
        !self.is_position_inside(position)
    }

    pub fn is_position_inside(&self, position: &Position) -> bool {
        let (x, y) = position.get_coordinates();
        self.is_x_inside(x) && self.is_y_inside(y)
    }

    pub fn perimeter(&self) -> Vec<Wall> {
        self.walls.clone()
    }

    pub fn interior(&self) -> Vec<Position> {
        let left = self.left_wall_x + 1;
        let right = self.right_wall_x - 1;
        let top = self.top_wall_y + 1;
        let bottom = self.bottom_wall_y - 1;
        let mut interior = Vec::with_capacity((right - left) * (bottom - top));
        for x in left..=right {
            for y in top..=bottom {
                interior.push(Position::new(x, y));
            }
        }
        interior
    }

    pub(crate) fn get_max_segments(&self) -> usize {
        (self.width - 2) * (self.height - 2)
    }

    fn is_x_inside(&self, x: usize) -> bool {
        x > self.left_wall_x && x < self.right_wall_x
    }

    fn is_y_inside(&self, y: usize) -> bool {
        y > self.top_wall_y && y < self.bottom_wall_y
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Wall {
    position: Position,
    wall_type: WallType,
}

impl Wall {
    fn new(position: Position, wall_type: WallType) -> Self {
        Self {
            position,
            wall_type,
        }
    }

    pub fn get_position(&self) -> Position {
        self.position
    }

    pub fn get_type(&self) -> WallType {
        self.wall_type
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WallType {
    TopWall,
    BottomWall,
    LeftWall,
    RightWall,
    TopLeftCorner,
    BottomLeftCorner,
    TopRightCorner,
    BottomRightCorner,
}

impl WallType {
    pub fn display(&self) -> String {
        match self {
            WallType::TopLeftCorner => "╭─",
            WallType::BottomLeftCorner => "╰─",
            WallType::BottomRightCorner => "╯",
            WallType::TopRightCorner => "╮",
            WallType::TopWall | WallType::BottomWall => "──",
            WallType::LeftWall | WallType::RightWall => "│",
        }
        .to_string()
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
