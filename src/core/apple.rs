use crate::core::Position;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Apple {
    position: Position,
}

impl Apple {
    #[tracing::instrument(level = "info")]
    pub fn new(position: Position) -> Self {
        Self { position }
    }
    #[tracing::instrument(level = "trace")]
    pub fn get_position(&self) -> Position {
        self.position
    }

    pub fn will_be_eaten_by(&self, potential_head: &Position) -> bool {
        potential_head == &self.get_position()
    }
}
