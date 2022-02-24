use crate::core::{
    Apple, DeathCause, Direction, Position, Scoreboard, SlitherAction, SlitherResult, Snek, Walls,
};

use rand::{seq::SliceRandom, thread_rng};

#[derive(Debug)]
pub struct State {
    walls: Walls,
    scoreboard: Scoreboard,
    snek: Snek,
    apple: Apple,
    direction: Direction,
}

impl State {
    #[tracing::instrument(level = "info")]
    pub fn new(width: usize, height: usize) -> State {
        if width * height < 16 {
            panic!("width * height must be at least 16");
        }
        let walls = Walls::new(width, height);
        let scoreboard = Scoreboard::new();
        let direction = Direction::Right;
        // let snek = Snek::line_snek(Position::new(1, 1), Direction::Right, 2);
        let snek = Snek::default();
        // dummy apple
        let apple = Apple::new(Position::new(2, 1));
        let mut state = State {
            apple,
            walls,
            snek,
            scoreboard,
            direction,
        };
        state.new_apple();
        state
    }

    pub fn new_apple(&mut self) {
        let mut possible_positions = Vec::new();
        for x in self.walls.left_wall()..=self.walls.right_wall() {
            for y in self.walls.top_wall()..=self.walls.bottom_wall() {
                let position = Position::new(x, y);
                let mut position_is_valid = true;
                for segment in self.snek.get_segments() {
                    if segment.get_position() == position {
                        position_is_valid = false;
                    }
                }
                for wall_position in self.get_walls().get_positions() {
                    if wall_position == position {
                        position_is_valid = false;
                    }
                }
                if position_is_valid {
                    possible_positions.push(Position::new(x, y));
                }
            }
        }
        let mut rng = thread_rng();
        self.apple = Apple::new(*possible_positions.choose(&mut rng).unwrap());
    }

    #[tracing::instrument(level = "debug")]
    pub fn tick(&mut self) -> SlitherResult {
        let slither_action = self.get_slither_action();
        self.take_slither_action(&slither_action)
    }

    pub fn get_direction(&self) -> Direction {
        self.direction
    }

    fn get_slither_action(&self) -> SlitherAction {
        if let Some(potential_head) = self
            .snek
            .get_head()
            .get_position()
            .neighbor(self.get_direction())
        {
            if self.walls.collides_with(&potential_head) {
                SlitherAction::Die(DeathCause::Wall)
            } else if self.apple.will_be_eaten_by(&potential_head) {
                if self.snek.will_i_run_into_myssself(&potential_head, true) {
                    SlitherAction::Die(DeathCause::Tail)
                } else {
                    SlitherAction::Grow(self.get_direction())
                }
            } else if self.snek.will_i_run_into_myssself(&potential_head, false) {
                SlitherAction::Die(DeathCause::Tail)
            } else {
                SlitherAction::Slither(self.get_direction())
            }
        } else {
            SlitherAction::Die(DeathCause::Wall)
        }
    }
    #[tracing::instrument(level = "info")]
    pub fn turn_snek(&mut self, attempted_direction: Direction) -> bool {
        // check the head's current direction
        let can_turn = match self.snek.get_head().get_direction() {
            // if you're going right or left
            Direction::Right | Direction::Left => {
                // you can turn up or down
                attempted_direction == Direction::Up || attempted_direction == Direction::Down
            }
            // if you're going up or down
            Direction::Up | Direction::Down => {
                // you can turn left or right
                attempted_direction == Direction::Left || attempted_direction == Direction::Right
            }
        };
        if can_turn {
            // update the snek's _overall_ direction
            // this will only update the heads direction in Snek::grow
            // when we are actually ready to move
            // this prevents someone from going right, then up, then left, causing death, all before the next tick even occurred
            self.direction = attempted_direction;
        }
        can_turn
    }

    fn take_slither_action(&mut self, slither_action: &SlitherAction) -> SlitherResult {
        match slither_action {
            SlitherAction::Die(death_cause) => {
                self.snek.kill();
                tracing::info!("snek died");
                SlitherResult::Died(*death_cause)
            }
            SlitherAction::Grow(direction) => {
                let slime_trail = self.snek.get_segments().first().unwrap().get_position();
                self.snek.grow(direction);
                self.scoreboard.increment_score();
                tracing::info!("ate an apple, new score: {}", self.scoreboard.get_score());
                if self.snek.count_segments() == self.walls.get_max_segments() {
                    SlitherResult::AteTheWorld
                } else {
                    self.new_apple();
                    SlitherResult::Grew {
                        direction: *direction,
                        segments: self.snek.get_segments(),
                        slime_trail,
                    }
                }
            }
            SlitherAction::Slither(direction) => {
                let slime_trail = self.snek.get_segments().first().unwrap().get_position();
                self.snek.slither(direction);
                tracing::info!("slithered {:?}", direction);
                SlitherResult::Slithered {
                    direction: *direction,
                    segments: self.get_snek().get_segments(),
                    slime_trail,
                }
            }
        }
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

    #[tracing::instrument(level = "trace")]
    pub fn get_walls(&self) -> &Walls {
        &self.walls
    }

    #[tracing::instrument(level = "trace")]
    pub fn is_wall(&self, position: &Position) -> bool {
        self.walls.collides_with(position)
    }

    #[cfg(test)]
    pub(crate) fn plant_apple(&mut self, x: usize, y: usize) {
        self.apple = Apple::new(Position::new(x, y));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{DeathCause, Position};

    #[test]
    fn it_can_eat_an_apple() {
        let mut state = State::new(10, 10);
        // plant an apple to the right of the default snek
        state.plant_apple(2, 1);
        assert_eq!(state.get_score(), 0);
        assert_eq!(state.get_direction(), Direction::Right);
        assert_eq!(state.get_snek().count_segments(), 1);

        // tick the game forward
        assert_eq!(state.tick().get_direction().unwrap(), Direction::Right);
        assert_eq!(state.get_snek().count_segments(), 2);
        assert_eq!(state.get_score(), 1);
    }

    #[test]
    fn it_can_turn_and_eat_an_apple() {
        let mut state = State::new(10, 10);
        // plant an apple to the right and down from the default snek
        state.plant_apple(2, 2);
        assert_eq!(state.get_score(), 0);
        assert_eq!(state.get_direction(), Direction::Right);
        assert_eq!(state.get_snek().count_segments(), 1);
        assert_eq!(state.tick().get_direction().unwrap(), Direction::Right);
        assert!(state.turn_snek(Direction::Down));
        assert_eq!(state.tick().get_direction().unwrap(), Direction::Down);
        assert_eq!(state.get_snek().count_segments(), 2);
        assert_eq!(state.get_score(), 1);
    }

    #[test]
    fn it_cannot_turn_into_itself() {
        let mut state = State::new(10, 10);
        // plant an apple to the right of the default snek
        assert_eq!(state.get_score(), 0);
        assert_eq!(state.get_direction(), Direction::Right);
        assert_eq!(state.get_snek().count_segments(), 1);

        // tick the game forward
        assert_eq!(state.tick().get_direction().unwrap(), Direction::Right);

        // turn down
        assert!(state.turn_snek(Direction::Down));

        // make sure we cannot turn left before the next tick (into itself)
        assert!(!state.turn_snek(Direction::Left));

        // tick the game forward, we should continue down since we couldn't turn left earlier
        assert_eq!(state.tick().get_direction().unwrap(), Direction::Down);

        // now we can turn left
        assert!(state.turn_snek(Direction::Left));
    }

    #[test]
    fn it_can_eat_the_world() {
        let mut state = State::new(4, 4);
        // plant an apple to the right of the default snek
        state.plant_apple(2, 1);
        assert_eq!(state.get_score(), 0);
        assert_eq!(state.get_direction(), Direction::Right);
        assert_eq!(state.get_snek().count_segments(), 1);

        // tick the game forward
        assert_eq!(state.tick().get_direction().unwrap(), Direction::Right);
        assert_eq!(state.get_score(), 1);
        assert_eq!(state.get_direction(), Direction::Right);
        assert_eq!(state.get_snek().count_segments(), 2);

        // plant another apple to the bottom
        state.plant_apple(2, 2);
        // turn towards the apple
        assert!(state.turn_snek(Direction::Down));
        assert_eq!(state.tick().get_direction().unwrap(), Direction::Down);
        assert_eq!(state.get_score(), 2);
        assert_eq!(state.get_direction(), Direction::Down);
        assert_eq!(state.get_snek().count_segments(), 3);
        // plant another apple to the left
        state.plant_apple(1, 2);
        // turn towards the apple
        assert!(state.turn_snek(Direction::Left));
        assert_eq!(state.tick(), SlitherResult::AteTheWorld);
        assert_eq!(state.get_score(), 3);
        assert_eq!(state.get_direction(), Direction::Left);
        assert_eq!(state.get_snek().count_segments(), 4);
        let snek = state.get_snek();
        let first_segment = snek.get_segment(0).unwrap();
        let second_segment = snek.get_segment(1).unwrap();
        let third_segment = snek.get_segment(2).unwrap();
        let fourth_segment = snek.get_segment(3).unwrap();
        assert_eq!(first_segment.get_position(), Position::new(1, 1));
        assert_eq!(first_segment.get_direction(), Direction::Right);
        assert_eq!(second_segment.get_position(), Position::new(2, 1));
        assert_eq!(second_segment.get_direction(), Direction::Down);
        assert_eq!(third_segment.get_position(), Position::new(2, 2));
        assert_eq!(third_segment.get_direction(), Direction::Left);
        assert_eq!(fourth_segment.get_position(), Position::new(1, 2));
        assert_eq!(fourth_segment.get_direction(), Direction::Left);
    }

    #[test]
    fn it_can_die_by_hitting_top_wall() {
        let mut state = State::new(4, 4);
        assert!(state.turn_snek(Direction::Up));
        assert_eq!(state.tick(), SlitherResult::Died(DeathCause::Wall));
        assert!(!state.get_snek().is_alive())
    }

    #[test]
    fn it_can_die_by_hitting_bottom_wall() {
        let mut state = State::new(4, 4);
        assert!(state.turn_snek(Direction::Down));
        assert_eq!(state.tick().get_direction().unwrap(), Direction::Down);
        assert_eq!(state.tick(), SlitherResult::Died(DeathCause::Wall));
        assert!(!state.get_snek().is_alive())
    }

    #[test]
    fn it_can_die_by_hitting_left_wall() {
        let mut state = State::new(4, 4);
        assert!(state.turn_snek(Direction::Down));
        assert_eq!(state.tick().get_direction().unwrap(), Direction::Down);
        assert!(state.turn_snek(Direction::Left));
        assert_eq!(state.tick(), SlitherResult::Died(DeathCause::Wall));
        assert!(!state.get_snek().is_alive())
    }

    #[test]
    fn it_can_die_by_hitting_right_wall() {
        let mut state = State::new(4, 4);
        assert_eq!(state.tick().get_direction().unwrap(), Direction::Right);
        assert_eq!(state.tick(), SlitherResult::Died(DeathCause::Wall));
        assert!(!state.get_snek().is_alive())
    }

    #[test]
    fn it_can_die_by_hitting_tail() {
        let mut state = State::new(5, 5);
        // plant an apple to the right of the default snek
        state.plant_apple(2, 1);
        assert_eq!(state.tick().get_direction().unwrap(), Direction::Right);
        assert_eq!(state.get_score(), 1);
        assert_eq!(state.get_snek().count_segments(), 2);

        state.plant_apple(3, 1);
        assert_eq!(state.tick().get_direction().unwrap(), Direction::Right);
        assert_eq!(state.get_snek().count_segments(), 3);

        state.plant_apple(3, 2);
        assert!(state.turn_snek(Direction::Down));
        assert_eq!(state.tick().get_direction().unwrap(), Direction::Down);
        assert_eq!(state.get_score(), 3);
        assert_eq!(state.get_snek().count_segments(), 4);

        state.plant_apple(2, 2);
        assert!(state.turn_snek(Direction::Left));
        assert_eq!(state.tick().get_direction().unwrap(), Direction::Left);
        assert_eq!(state.get_score(), 4);
        assert_eq!(state.get_snek().count_segments(), 5);

        state.plant_apple(1, 2);
        assert_eq!(state.tick().get_direction().unwrap(), Direction::Left);
        assert_eq!(state.get_score(), 5);
        assert_eq!(state.get_snek().count_segments(), 6);

        assert!(state.turn_snek(Direction::Up));
        assert_eq!(
            state
                .get_snek()
                .get_head()
                .get_position()
                .neighbor(Direction::Up)
                .unwrap(),
            Position::new(1, 1)
        );
        assert_eq!(
            state
                .get_snek()
                .get_segments()
                .first()
                .unwrap()
                .get_position(),
            Position::new(1, 1)
        );
        assert_eq!(state.tick(), SlitherResult::Died(DeathCause::Tail));
        assert!(!state.get_snek().is_alive())
    }

    fn direction_state(direction: Direction) -> State {
        let mut state = State::new(10, 10);
        state.direction = direction;
        state.snek.get_head_mut().set_direction(&direction);
        state
    }

    #[test]
    fn snek_can_turn_from_up_to_left() {
        let mut state = direction_state(Direction::Up);
        assert_eq!(state.get_direction(), Direction::Up);
        assert!(state.turn_snek(Direction::Left));
        state.tick();
        assert_eq!(state.get_direction(), Direction::Left);
    }

    #[test]
    fn snek_can_turn_from_up_to_right() {
        let mut state = direction_state(Direction::Up);
        assert_eq!(state.get_direction(), Direction::Up);
        assert!(state.turn_snek(Direction::Right));
        state.tick();
        assert_eq!(state.get_direction(), Direction::Right);
    }

    #[test]
    fn snek_cant_turn_from_up_to_down() {
        let mut state = direction_state(Direction::Up);
        assert_eq!(state.get_direction(), Direction::Up);
        assert!(!state.turn_snek(Direction::Down));
        state.tick();
        assert_eq!(state.get_direction(), Direction::Up);
    }

    #[test]
    fn snek_cant_turn_from_up_to_up() {
        let mut state = direction_state(Direction::Up);
        assert_eq!(state.get_direction(), Direction::Up);
        assert!(!state.turn_snek(Direction::Up));
        state.tick();
        assert_eq!(state.get_direction(), Direction::Up);
    }

    #[test]
    fn snek_can_turn_from_down_to_left() {
        let mut state = direction_state(Direction::Down);
        assert_eq!(state.get_direction(), Direction::Down);
        assert!(state.turn_snek(Direction::Left));
        state.tick();
        assert_eq!(state.get_direction(), Direction::Left);
    }

    #[test]
    fn snek_can_turn_from_down_to_right() {
        let mut state = direction_state(Direction::Down);
        assert_eq!(state.get_direction(), Direction::Down);
        assert!(state.turn_snek(Direction::Right));
        state.tick();
        assert_eq!(state.get_direction(), Direction::Right);
    }

    #[test]
    fn snek_cant_turn_from_down_to_down() {
        let mut state = direction_state(Direction::Down);
        assert_eq!(state.get_direction(), Direction::Down);
        assert!(!state.turn_snek(Direction::Down));
        state.tick();
        assert_eq!(state.get_direction(), Direction::Down);
    }

    #[test]
    fn snek_cant_turn_from_down_to_up() {
        let mut state = direction_state(Direction::Down);
        assert_eq!(state.get_direction(), Direction::Down);
        assert!(!state.turn_snek(Direction::Up));
        state.tick();
        assert_eq!(state.get_direction(), Direction::Down);
    }

    #[test]
    fn snek_can_turn_from_right_to_up() {
        let mut state = direction_state(Direction::Right);
        assert_eq!(state.get_direction(), Direction::Right);
        assert!(state.turn_snek(Direction::Up));
        state.tick();
        assert_eq!(state.get_direction(), Direction::Up);
    }

    #[test]
    fn snek_can_turn_from_right_to_down() {
        let mut state = direction_state(Direction::Right);
        assert_eq!(state.get_direction(), Direction::Right);
        assert!(state.turn_snek(Direction::Down));
        state.tick();
        assert_eq!(state.get_direction(), Direction::Down);
    }

    #[test]
    fn snek_cant_turn_from_right_to_right() {
        let mut state = direction_state(Direction::Right);
        assert_eq!(state.get_direction(), Direction::Right);
        assert!(!state.turn_snek(Direction::Right));
        state.tick();
        assert_eq!(state.get_direction(), Direction::Right);
    }

    #[test]
    fn snek_cant_turn_from_right_to_left() {
        let mut state = direction_state(Direction::Right);
        assert_eq!(state.get_direction(), Direction::Right);
        assert!(!state.turn_snek(Direction::Left));
        state.tick();
        assert_eq!(state.get_direction(), Direction::Right);
    }

    #[test]
    fn snek_can_turn_from_left_to_up() {
        let mut state = direction_state(Direction::Left);
        assert_eq!(state.get_direction(), Direction::Left);
        assert!(state.turn_snek(Direction::Up));
        state.tick();
        assert_eq!(state.get_direction(), Direction::Up);
    }

    #[test]
    fn snek_can_turn_from_left_to_down() {
        let mut state = direction_state(Direction::Left);
        assert_eq!(state.get_direction(), Direction::Left);
        assert!(state.turn_snek(Direction::Down));
        state.tick();
        assert_eq!(state.get_direction(), Direction::Down);
    }

    #[test]
    fn snek_cant_turn_from_left_to_left() {
        let mut state = direction_state(Direction::Left);
        assert_eq!(state.get_direction(), Direction::Left);
        assert!(!state.turn_snek(Direction::Left));
        state.tick();
        assert_eq!(state.get_direction(), Direction::Left);
    }

    #[test]
    fn snek_cant_turn_from_left_to_right() {
        let mut state = direction_state(Direction::Left);
        assert_eq!(state.get_direction(), Direction::Left);
        assert!(!state.turn_snek(Direction::Right));
        state.tick();
        assert_eq!(state.get_direction(), Direction::Left);
    }
}
