use crate::core::Position;

#[derive(Debug, Clone, Copy)]
pub struct Apple {
    position: Position,
}

impl Apple {
    #[tracing::instrument(level = "info")]
    pub fn random(possible_positions: &[&Position]) -> Self {
        // TODO: add random apple function
        let x = 5;
        let y = 5;
        let position = Position::new(x, y);
        if possible_positions.contains(&&position) {
            Self {
                position: Position::new(x, y),
            }
        } else {
            panic!("can't create an apple here")
        }
    }
    #[tracing::instrument(level = "trace")]
    pub fn get_position(&self) -> Position {
        self.position
    }

    pub fn new(x: usize, y: usize) -> Self {
        Self {
            position: Position::new(x, y),
        }
    }
}

impl Default for Apple {
    fn default() -> Self {
        Apple::new(5, 5)
    }
}
