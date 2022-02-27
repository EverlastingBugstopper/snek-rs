use crate::core::{DeathCause, Direction, Position, Segment};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SlitherAction {
    Die {
        cause: DeathCause,
        direction: Direction,
    },
    Grow(Direction),
    Slither(Direction),
}

#[derive(Debug, Clone, PartialEq)]
pub enum SlitherResult {
    Died(DeathCause),
    Grew {
        direction: Direction,
        segments: Vec<Segment>,
        slime_trail: Position,
    },
    Slithered {
        direction: Direction,
        segments: Vec<Segment>,
        slime_trail: Position,
    },
    AteTheWorld,
}

impl SlitherResult {
    pub fn describe(&self) -> String {
        match self {
            SlitherResult::Died(death_cause) => {
                format!("snek died because {}", death_cause.describe())
            }
            SlitherResult::Grew {
                direction,
                slime_trail,
                segments: _,
            } => format!(
                "snek grew {} and left a slime trail at {:?}",
                direction.describe(),
                slime_trail
            ),
            SlitherResult::Slithered {
                direction,
                slime_trail,
                segments: _,
            } => {
                format!(
                    "snek slithered {} and left a slime trail at {:?}",
                    direction.describe(),
                    slime_trail
                )
            }
            SlitherResult::AteTheWorld => "snek ate the world".to_string(),
        }
    }

    pub fn get_direction(&self) -> Option<Direction> {
        match self {
            SlitherResult::Grew {
                direction,
                segments: _,
                slime_trail: _,
            }
            | SlitherResult::Slithered {
                direction,
                segments: _,
                slime_trail: _,
            } => Some(*direction),
            _ => None,
        }
    }
}
