mod death;
mod direction;
mod segment;
mod slither;

pub use death::DeathCause;
pub use direction::Direction;
pub use segment::Segment;
pub use slither::SlitherResult;

use crate::core::{Apple, Position, Walls};

use rayon::prelude::*;

#[derive(Debug, Clone)]
pub struct Snek {
    segments: Vec<Segment>,
    direction: Direction,
    alive: bool,
}

impl Default for Snek {
    fn default() -> Self {
        Snek::new(Position::new(0, 0), Direction::Right)
    }
}

impl Snek {
    pub fn new(position: Position, direction: Direction) -> Self {
        Snek {
            segments: vec![Segment::new_head(position, direction)],
            direction,
            alive: true,
        }
    }

    #[cfg(test)]
    pub fn get_segment(&self, index: usize) -> Option<&Segment> {
        self.segments.get(index)
    }

    pub fn get_segments(&self) -> &[Segment] {
        &self.segments
    }

    pub fn get_segment_positions(&self) -> Vec<Position> {
        self.segments.par_iter().map(|s| s.get_position()).collect()
    }

    pub fn count_segments(&self) -> usize {
        self.segments.len()
    }

    pub fn is_alive(&self) -> bool {
        self.alive
    }

    pub fn get_direction(&self) -> Direction {
        self.direction
    }

    pub fn get_head(&self) -> &Segment {
        self.segments.last().unwrap()
    }

    fn get_head_mut(&mut self) -> &mut Segment {
        self.segments.last_mut().unwrap()
    }

    pub fn get_tail(&self) -> &[Segment] {
        &self.segments[0..self.segments.len() - 1]
    }

    pub fn turn(&mut self, attempted_direction: Direction) -> bool {
        // check the head's current direction
        let can_turn = match self.get_head().get_direction() {
            Direction::Right | Direction::Left => {
                attempted_direction != Direction::Right && attempted_direction != Direction::Left
            }
            Direction::Up | Direction::Down => {
                attempted_direction != Direction::Up && attempted_direction != Direction::Down
            }
        };
        if can_turn {
            // update the snek's _overall_ direction
            // this will only udpate the heads direction in try_slither
            // when we are actually ready to move
            // this prevents someone from going right, then up, then left, causing death, all before the next tick even occurred
            self.direction = attempted_direction;
        }
        can_turn
    }

    pub fn try_slither(&mut self, walls: &Walls, apple: &Apple) -> SlitherResult {
        // get the snek's _overall_ direction
        let direction = self.get_direction();
        // set the head to the new direction
        self.get_head_mut().set_direction(direction);

        // move the head forward and check if it killed the snek
        if let Some(death_cause) = self.inch_head() {
            SlitherResult::Died(death_cause)
        // check if the inch forward caused us to eat an apple
        } else if self.did_eat_apple____mmmm(apple) {
            if self.segments.len() == walls.get_max_segments() {
                SlitherResult::AteTheWorld
            } else {
                SlitherResult::Grew(self.direction)
            }
        // if we didn't eat an apple we have to inch its tail forward as well
        // and then check if we ran into ourselves or a wall
        } else {
            self.inch_tail();
            if let Some(death_cause) = self.face_the_reaper(walls) {
                SlitherResult::Died(death_cause)
            } else {
                SlitherResult::Slithered(self.direction)
            }
        }
    }

    /// inches the snek's head forward
    /// without removing the tail
    /// after calling, it's possible
    /// the snek is tooooooooooooooooooooooooooooooooooo long
    /// so make sure to call inch_tail along with it         W
    /// if it did not eat an apple when it inches its head   A
    /// also the snek could die if it inches its head into thL
    /// so be careful.                                       L
    fn inch_head(&mut self) -> Option<DeathCause> {
        let mut new_head = self.get_head().to_owned();
        self.get_head_mut().make_tail();
        if let Some(death_cause) = new_head.inch() {
            self.inch_tail();
            self.kill();
            Some(death_cause)
        } else {
            self.segments.push(new_head);
            None
        }
    }

    /// inches along the tail, removing it from the segments
    fn inch_tail(&mut self) {
        self.segments.remove(0);
    }

    // slumber, ye ol' paperclip.
    // it is not a snake case,
    // it is the case of the snek
    // mmmmm?
    #[allow(non_snake_case)]
    fn did_eat_apple____mmmm(&self, apple: &Apple) -> bool {
        apple.get_position() == self.get_head().get_position()
    }

    fn face_the_reaper(&mut self, walls: &Walls) -> Option<DeathCause> {
        self.check_wall_kill(walls)
            .or_else(|| self.check_tail_kill())
    }

    fn check_wall_kill(&mut self, walls: &Walls) -> Option<DeathCause> {
        if walls.contains_position(&self.get_head().get_position()) {
            None
        } else {
            self.kill();
            Some(DeathCause::Wall)
        }
    }

    fn check_tail_kill(&mut self) -> Option<DeathCause> {
        let head_position = self.get_head().get_position();
        let should_kill = false;
        self.segments[0..self.segments.len() - 1]
            .par_iter()
            .for_each_with(should_kill, |sk, segment| {
                if head_position == segment.get_position() {
                    *sk = true;
                }
            });
        if should_kill {
            self.kill();
            Some(DeathCause::Tail)
        } else {
            None
        }
    }

    fn kill(&mut self) {
        self.alive = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_snek_position() -> Position {
        Position::new(10, 10)
    }

    fn new_up_snek() -> Snek {
        Snek::new(default_snek_position(), Direction::Up)
    }
    fn new_down_snek() -> Snek {
        Snek::new(default_snek_position(), Direction::Down)
    }
    fn new_left_snek() -> Snek {
        Snek::new(default_snek_position(), Direction::Left)
    }

    fn new_right_snek() -> Snek {
        Snek::new(default_snek_position(), Direction::Right)
    }

    #[test]
    fn snek_can_make_head_tail() {
        let mut snek = Snek::new(Position::new(0, 0), Direction::Right);
        assert!(snek.inch_head().is_none());
        assert_eq!(snek.segments.len(), 2);
        assert_eq!(&snek.segments[1], snek.get_head());
        assert!(!snek.segments[0].is_head());
        assert!(snek.segments[0].is_tail());
        assert!(snek.segments[1].is_head());
        assert!(!snek.segments[1].is_tail());
        assert_eq!(snek.segments[1].get_position(), Position::new(1, 0));
        assert_eq!(snek.segments[1].get_direction(), Direction::Right);
        snek.inch_tail();
        assert_eq!(snek.segments.len(), 1);
        assert!(snek.segments[0].is_head());
    }

    #[test]
    fn snek_can_turn_from_up_to_left() {
        let mut snek = new_up_snek();
        assert_eq!(snek.get_direction(), Direction::Up);
        assert!(snek.turn(Direction::Left));
        assert_eq!(snek.get_direction(), Direction::Left);
    }

    #[test]
    fn snek_can_turn_from_up_to_right() {
        let mut snek = new_up_snek();
        assert_eq!(snek.get_direction(), Direction::Up);
        assert!(snek.turn(Direction::Right));
        assert_eq!(snek.get_direction(), Direction::Right);
    }

    #[test]
    fn snek_cant_turn_from_up_to_down() {
        let mut snek = new_up_snek();
        assert_eq!(snek.get_direction(), Direction::Up);
        assert!(!snek.turn(Direction::Down));
        assert_eq!(snek.get_direction(), Direction::Up);
    }

    #[test]
    fn snek_cant_turn_from_up_to_up() {
        let mut snek = new_up_snek();
        assert_eq!(snek.get_direction(), Direction::Up);
        assert!(!snek.turn(Direction::Up));
        assert_eq!(snek.get_direction(), Direction::Up);
    }

    #[test]
    fn snek_can_turn_from_down_to_left() {
        let mut snek = new_down_snek();
        assert_eq!(snek.get_direction(), Direction::Down);
        assert!(snek.turn(Direction::Left));
        assert_eq!(snek.get_direction(), Direction::Left);
    }

    #[test]
    fn snek_can_turn_from_down_to_right() {
        let mut snek = new_down_snek();
        assert_eq!(snek.get_direction(), Direction::Down);
        assert!(snek.turn(Direction::Right));
        assert_eq!(snek.get_direction(), Direction::Right);
    }

    #[test]
    fn snek_cant_turn_from_down_to_down() {
        let mut snek = new_down_snek();
        assert_eq!(snek.get_direction(), Direction::Down);
        assert!(!snek.turn(Direction::Down));
        assert_eq!(snek.get_direction(), Direction::Down);
    }

    #[test]
    fn snek_cant_turn_from_down_to_up() {
        let mut snek = new_down_snek();
        assert_eq!(snek.get_direction(), Direction::Down);
        assert!(!snek.turn(Direction::Up));
        assert_eq!(snek.get_direction(), Direction::Down);
    }

    #[test]
    fn snek_can_turn_from_right_to_up() {
        let mut snek = new_right_snek();
        assert_eq!(snek.get_direction(), Direction::Right);
        assert!(snek.turn(Direction::Up));
        assert_eq!(snek.get_direction(), Direction::Up);
    }

    #[test]
    fn snek_can_turn_from_right_to_down() {
        let mut snek = new_right_snek();
        assert_eq!(snek.get_direction(), Direction::Right);
        assert!(snek.turn(Direction::Down));
        assert_eq!(snek.get_direction(), Direction::Down);
    }

    #[test]
    fn snek_cant_turn_from_right_to_right() {
        let mut snek = new_right_snek();
        assert_eq!(snek.get_direction(), Direction::Right);
        assert!(!snek.turn(Direction::Right));
        assert_eq!(snek.get_direction(), Direction::Right);
    }

    #[test]
    fn snek_cant_turn_from_right_to_left() {
        let mut snek = new_right_snek();
        assert_eq!(snek.get_direction(), Direction::Right);
        assert!(!snek.turn(Direction::Left));
        assert_eq!(snek.get_direction(), Direction::Right);
    }

    #[test]
    fn snek_can_turn_from_left_to_up() {
        let mut snek = new_left_snek();
        assert_eq!(snek.get_direction(), Direction::Left);
        assert!(snek.turn(Direction::Up));
        assert_eq!(snek.get_direction(), Direction::Up);
    }

    #[test]
    fn snek_can_turn_from_left_to_down() {
        let mut snek = new_left_snek();
        assert_eq!(snek.get_direction(), Direction::Left);
        assert!(snek.turn(Direction::Down));
        assert_eq!(snek.get_direction(), Direction::Down);
    }

    #[test]
    fn snek_cant_turn_from_left_to_left() {
        let mut snek = new_left_snek();
        assert_eq!(snek.get_direction(), Direction::Left);
        assert!(!snek.turn(Direction::Left));
        assert_eq!(snek.get_direction(), Direction::Left);
    }

    #[test]
    fn snek_cant_turn_from_left_to_right() {
        let mut snek = new_left_snek();
        assert_eq!(snek.get_direction(), Direction::Left);
        assert!(!snek.turn(Direction::Right));
        assert_eq!(snek.get_direction(), Direction::Left);
    }
}
