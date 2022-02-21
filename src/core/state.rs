use crate::core::{Apple, Direction, Position, Scoreboard, SlitherResult, Snek, Walls};

#[derive(Debug)]
pub struct State {
    walls: Walls,
    scoreboard: Scoreboard,
    snek: Snek,
    apple: Apple,
}

impl State {
    #[tracing::instrument(level = "info")]
    pub fn new(width: usize, height: usize) -> State {
        let walls = Walls::new(width, height);
        let scoreboard = Scoreboard::new();
        let snek = Snek::default();
        let apple = Apple::default();
        State {
            apple,
            walls,
            snek,
            scoreboard,
        }
    }

    #[tracing::instrument(level = "debug")]
    pub fn tick(&mut self) -> SlitherResult {
        let slither_result = self.snek.try_slither(&self.walls, &self.apple);
        if let SlitherResult::Grew(_) | SlitherResult::AteTheWorld = slither_result {
            self.scoreboard.increment_score();
        }
        slither_result
    }

    #[tracing::instrument(level = "info")]
    pub fn turn_snek(&mut self, direction: Direction) -> bool {
        self.snek.turn(direction)
    }

    #[tracing::instrument(level = "trace")]
    pub fn get_snek(&self) -> &Snek {
        &self.snek
    }

    #[tracing::instrument(level = "trace")]
    pub fn get_score(&self) -> usize {
        self.scoreboard.get_score()
    }

    #[tracing::instrument(level = "trace")]
    pub fn get_apple(&self) -> &Apple {
        &self.apple
    }

    #[tracing::instrument(level = "debug")]
    fn get_empty_positions(&self) -> Vec<Position> {
        let mut filled_positions = Vec::new();
        filled_positions.extend(self.snek.get_segment_positions());
        filled_positions.push(self.apple.get_position());

        let mut possible_positions = Vec::new();
        for x in self.walls.left_wall()..self.walls.right_wall() {
            for y in self.walls.top_wall()..self.walls.bottom_wall() {
                let candidate_position = Position::new(x, y);
                if !filled_positions.contains(&candidate_position) {
                    possible_positions.push(candidate_position);
                }
            }
        }
        possible_positions
    }

    #[cfg(test)]
    pub(crate) fn plant_apple(&mut self, x: usize, y: usize) {
        self.apple = Apple::new(x, y);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_can_eat_an_apple() {
        let mut state = State::new(10, 10);
        // plant an apple to the right of the default snek
        state.plant_apple(1, 0);
        assert_eq!(state.get_score(), 0);
        assert_eq!(state.get_snek().get_direction(), Direction::Right);
        assert_eq!(state.get_snek().count_segments(), 1);

        // tick the game forward
        assert_eq!(state.tick(), SlitherResult::Grew(Direction::Right));
        assert_eq!(state.get_snek().count_segments(), 2);
        assert_eq!(state.get_score(), 1);
    }

    #[test]
    fn it_can_turn_and_eat_an_apple() {
        let mut state = State::new(10, 10);
        // plant an apple to the right and down from the default snek
        state.plant_apple(1, 1);
        assert_eq!(state.get_score(), 0);
        assert_eq!(state.get_snek().get_direction(), Direction::Right);
        assert_eq!(state.get_snek().count_segments(), 1);
        assert_eq!(state.tick(), SlitherResult::Slithered(Direction::Right));
        assert!(state.turn_snek(Direction::Down));
        assert_eq!(state.tick(), SlitherResult::Grew(Direction::Down));
        assert_eq!(state.get_snek().count_segments(), 2);
        assert_eq!(state.get_score(), 1);
    }

    #[test]
    fn it_can_eat_the_world() {
        let mut state = State::new(2, 2);
        // plant an apple to the right of the default snek
        state.plant_apple(1, 0);
        assert_eq!(state.get_score(), 0);
        assert_eq!(state.get_snek().get_direction(), Direction::Right);
        assert_eq!(state.get_snek().count_segments(), 1);

        // tick the game forward
        assert_eq!(state.tick(), SlitherResult::Grew(Direction::Right));
        assert_eq!(state.get_score(), 1);
        assert_eq!(state.get_snek().get_direction(), Direction::Right);
        assert_eq!(state.get_snek().count_segments(), 2);

        // plant another apple to the bottom
        state.plant_apple(1, 1);
        // turn towards the apple
        assert!(state.turn_snek(Direction::Down));
        assert_eq!(state.tick(), SlitherResult::Grew(Direction::Down));
        assert_eq!(state.get_score(), 2);
        assert_eq!(state.get_snek().get_direction(), Direction::Down);
        assert_eq!(state.get_snek().count_segments(), 3);
        // plant another apple to the left
        state.plant_apple(0, 1);
        // turn towards the apple
        assert!(state.turn_snek(Direction::Left));
        assert_eq!(state.tick(), SlitherResult::AteTheWorld);
        assert_eq!(state.get_score(), 3);
        assert_eq!(state.get_snek().get_direction(), Direction::Left);
        assert_eq!(state.get_snek().count_segments(), 4);
        let snek = state.get_snek();
        let first_segment = snek.get_segment(0).unwrap();
        let second_segment = snek.get_segment(1).unwrap();
        let third_segment = snek.get_segment(2).unwrap();
        let fourth_segment = snek.get_segment(3).unwrap();
        assert_eq!(first_segment.get_position(), Position::new(0, 0));
        assert_eq!(first_segment.get_direction(), Direction::Right);
        assert_eq!(second_segment.get_position(), Position::new(1, 0));
        assert_eq!(second_segment.get_direction(), Direction::Down);
        assert_eq!(third_segment.get_position(), Position::new(1, 1));
        assert_eq!(third_segment.get_direction(), Direction::Left);
        assert_eq!(fourth_segment.get_position(), Position::new(0, 1));
        assert_eq!(fourth_segment.get_direction(), Direction::Left);
    }
}
