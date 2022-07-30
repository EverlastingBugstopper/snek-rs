use crate::core::{DeathCause, Direction, Position};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SlitherAction {
    Die {
        death_cause: DeathCause,
        direction: Direction,
    },
    Grow(Direction),
    Slither(Direction),
}

#[derive(Debug, Clone, PartialEq)]
pub struct SlitherResult {
    direction: Direction,
    result_type: SlitherResultType,
    slime_trail: Option<Position>,
    old_apple: Option<Position>,
}

impl SlitherResult {
    pub fn grew(direction: &Direction, old_apple: Position) -> Self {
        Self {
            direction: *direction,
            result_type: SlitherResultType::Grew,
            slime_trail: None,
            old_apple: Some(old_apple),
        }
    }

    pub fn ate_the_world(direction: &Direction, old_apple: Position) -> Self {
        Self {
            direction: *direction,
            result_type: SlitherResultType::AteTheWorld,
            slime_trail: None,
            old_apple: Some(old_apple),
        }
    }

    pub fn slithered(direction: &Direction, slime_trail: Position) -> Self {
        Self {
            direction: *direction,
            result_type: SlitherResultType::Slithered,
            slime_trail: Some(slime_trail),
            old_apple: None,
        }
    }

    pub fn died(direction: &Direction, slime_trail: Position, death_cause: &DeathCause) -> Self {
        Self {
            direction: *direction,
            result_type: SlitherResultType::Died {
                death_cause: *death_cause,
            },
            slime_trail: Some(slime_trail),
            old_apple: None,
        }
    }

    pub fn describe(&self) -> String {
        let action = match self.result_type {
            SlitherResultType::Died { death_cause } => {
                format!("snek died because {}", death_cause.display())
            }
            SlitherResultType::Grew => format!("snek grew {}", self.direction.describe()),
            SlitherResultType::Slithered => format!("snek slithered {}", self.direction.describe()),
            SlitherResultType::AteTheWorld => "snek ate the world".to_string(),
        };
        if let Some(slime_trail) = self.slime_trail {
            let (x, y) = slime_trail.get_coordinates();
            format!("{}, leaving a slime trail at ({}, {})", action, x, y)
        } else {
            action
        }
    }

    pub fn get_direction(&self) -> Direction {
        self.direction
    }

    pub fn get_type(&self) -> SlitherResultType {
        self.result_type
    }

    pub fn get_slime_trail(&self) -> Option<Position> {
        self.slime_trail
    }

    pub fn get_old_apple(&self) -> Option<Position> {
        self.old_apple
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SlitherResultType {
    Died { death_cause: DeathCause },
    Grew,
    Slithered,
    AteTheWorld,
}
