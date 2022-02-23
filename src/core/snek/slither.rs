use crate::core::{DeathCause, Direction};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SlitherResult {
    Died(DeathCause),
    Grew(Direction),
    Slithered(Direction),
    AteTheWorld,
}

impl SlitherResult {
    pub fn describe(&self) -> String {
        match self {
            SlitherResult::Died(death_cause) => {
                format!("snek died because it {}", death_cause.describe())
            }
            SlitherResult::Grew(direction) => format!("snek grew {}", direction.describe()),
            SlitherResult::Slithered(direction) => {
                format!("snek slithered {}", direction.describe())
            }
            SlitherResult::AteTheWorld => "snek ate the world".to_string(),
        }
    }
}
