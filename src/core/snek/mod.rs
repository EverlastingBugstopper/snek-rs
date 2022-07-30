mod death;
mod direction;
mod segment;
mod slither;

use std::collections::VecDeque;

pub use death::DeathCause;
pub use direction::Direction;
pub use segment::Segment;
pub use slither::{SlitherAction, SlitherResult, SlitherResultType};

use crate::core::Position;

use rayon::prelude::*;

#[derive(Debug, Clone)]
pub struct Snek {
    segments: VecDeque<Segment>,
    alive: bool,
}

impl Default for Snek {
    fn default() -> Self {
        Snek::baby_snek(Position::new(1, 1), Direction::Right)
    }
}

impl Snek {
    pub fn baby_snek(start: Position, direction: Direction) -> Self {
        Snek {
            segments: vec![Segment::new_head(start, direction)].into(),
            alive: true,
        }
    }

    pub fn line_snek(start: Position, direction: Direction, len: usize) -> Self {
        if len == 0 {
            panic!("snek must have a length of at least 1")
        }
        let mut segments = VecDeque::new();
        let mut position = start;
        for i in 0..len {
            if i == len - 1 {
                segments.push_back(Segment::new_head(position, direction));
            } else {
                segments.push_back(Segment::new_tail(position, direction));
            }
            position = position.neighbor(direction).unwrap_or_else(|| panic!(
                "invalid snek configuration. starting the tail at {:?} pointing {:?} with len {} makes segment {} out of bounds",
                start, direction, len, i
            ));
        }
        Snek {
            segments,
            alive: true,
        }
    }

    #[cfg(test)]
    pub fn get_segment(&self, index: usize) -> Option<&Segment> {
        self.segments.get(index)
    }

    pub fn get_segments(&self) -> VecDeque<Segment> {
        self.segments.clone()
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

    pub fn is_dead(&self) -> bool {
        !self.alive
    }

    pub fn get_head(&self) -> &Segment {
        self.segments.back().unwrap()
    }

    pub(crate) fn get_head_mut(&mut self) -> &mut Segment {
        self.segments.back_mut().unwrap()
    }

    fn get_tail(&self) -> Vec<Segment> {
        let mut tail = Vec::with_capacity(self.segments.len() - 1);
        for segment in self.segments.iter() {
            tail.push(*segment)
        }
        tail
    }

    pub fn grow(&mut self, direction: &Direction) {
        let mut new_head = self.get_head().to_owned();
        let old_head = self.get_head_mut();
        old_head.make_tail();
        old_head.set_direction(direction);
        // my thinking is that it's safe to unwrap here because we wouldn't
        // be growing if there wasn't an apple, and an apple couldn't have
        // an invalid position. right?
        new_head.set_position(&new_head.get_position().neighbor(*direction).unwrap());
        new_head.set_direction(direction);
        self.segments.push_back(new_head);
    }

    /// slithers the snek in a direction
    /// returning the slime trail position
    pub fn slither(&mut self, direction: &Direction) -> Position {
        self.grow(direction);
        let slime_trail = *self.segments.front().unwrap();
        self.segments.pop_front().unwrap();
        slime_trail.get_position()
    }

    pub(crate) fn will_i_run_into_myssself(
        &self,
        potential_head: &Position,
        will_grow: bool,
    ) -> bool {
        let mut will_i_run_into_myself = false;
        let skip_tail = if will_grow { 1 } else { 0 };
        self.get_tail().iter().skip(skip_tail).for_each(|s| {
            if &s.get_position() == potential_head {
                will_i_run_into_myself = true;
            }
        });
        will_i_run_into_myself
    }

    pub(crate) fn kill(&mut self) {
        self.get_head_mut().dead_head();
        self.alive = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_snek_position() -> Position {
        Position::new(10, 10)
    }

    fn direction_snek(direction: Direction) -> Snek {
        Snek::baby_snek(default_snek_position(), direction)
    }

    #[test]
    fn snek_can_grow() {
        let mut snek = direction_snek(Direction::Right);
        assert_eq!(snek.count_segments(), 1);
        assert_eq!(snek.get_head().get_position(), default_snek_position());
        snek.grow(&Direction::Right);
        for segment in snek.get_tail() {
            assert!(segment.is_tail());
        }
        assert!(snek.get_head().is_head());
        assert_eq!(snek.count_segments(), 2);
        assert_eq!(
            snek.get_head().get_position(),
            default_snek_position().neighbor(Direction::Right).unwrap()
        );
    }

    #[test]
    fn snek_can_slither() {
        let mut snek = direction_snek(Direction::Right);
        assert_eq!(snek.count_segments(), 1);
        assert_eq!(snek.get_head().get_position(), default_snek_position());
        snek.slither(&Direction::Right);
        assert!(snek.get_head().is_head());
        assert_eq!(snek.count_segments(), 1);
        assert_eq!(
            snek.get_head().get_position(),
            default_snek_position().neighbor(Direction::Right).unwrap()
        );
    }

    #[test]
    fn snek_can_die() {
        let mut snek = direction_snek(Direction::Right);
        assert!(snek.is_alive());
        snek.kill();
        assert!(snek.is_dead())
    }
}
