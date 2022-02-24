#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DeathCause {
    Wall,
    Tail,
}

impl DeathCause {
    pub fn describe(&self) -> &str {
        match self {
            DeathCause::Wall => "ran into the wall",
            DeathCause::Tail => "ran into its own tail",
        }
    }
}
