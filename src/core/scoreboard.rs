#[derive(Debug, Clone, Copy)]
pub struct Scoreboard {
    score: usize,
}

impl Scoreboard {
    pub fn new() -> Self {
        Self { score: 0 }
    }

    pub fn get_score(&self) -> usize {
        self.score
    }

    pub fn increment_score(&mut self) {
        self.score += 1;
    }
}

impl Default for Scoreboard {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_starts_at_zero() {
        let scoreboard = Scoreboard::new();
        assert_eq!(scoreboard.get_score(), 0);
    }

    #[test]
    fn it_increments_one_at_a_time() {
        let mut scoreboard = Scoreboard::new();
        assert_eq!(scoreboard.get_score(), 0);
        scoreboard.increment_score();
        assert_eq!(scoreboard.get_score(), 1);
        scoreboard.increment_score();
        assert_eq!(scoreboard.get_score(), 2);
    }
}
