#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DeathCause {
    Wall,
    Tail,
    Resized,
}

impl DeathCause {
    pub fn display(&self) -> String {
        match self {
            DeathCause::Wall => "it ran into the wall",
            DeathCause::Tail => "it ran into its own tail",
            DeathCause::Resized => "you resized the terminal",
        }
        .to_string()
    }
}
