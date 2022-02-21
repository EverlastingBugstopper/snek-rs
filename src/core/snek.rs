use crate::core::{Apple, Position, Walls};

#[derive(Debug)]
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
    pub(crate) fn get_segment(&self, index: usize) -> Option<&Segment> {
        self.segments.get(index)
    }

    pub fn get_segments(&self) -> &[Segment] {
        &self.segments
    }

    pub fn get_segment_positions(&self) -> Vec<Position> {
        self.segments.iter().map(|s| s.get_position()).collect()
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
        let can_turn = match self.direction {
            Direction::Right | Direction::Left => {
                attempted_direction != Direction::Right && attempted_direction != Direction::Left
            }
            Direction::Up | Direction::Down => {
                attempted_direction != Direction::Up && attempted_direction != Direction::Down
            }
        };
        if can_turn {
            self.direction = attempted_direction;
        }
        can_turn
    }

    pub fn try_slither(&mut self, walls: &Walls, apple: &Apple) -> SlitherResult {
        self.get_head_mut().direction = self.direction;
        if let Some(death_cause) = self.inch_head() {
            return SlitherResult::Died(death_cause);
        } else if self.did_eat_apple____mmmm(apple) {
            if self.segments.len() == walls.get_max_segments() {
                SlitherResult::AteTheWorld
            } else {
                SlitherResult::Grew(self.direction)
            }
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
        let mut new_head = self.get_head().clone();
        self.get_head_mut().make_tail();
        if let Some(death_cause) = new_head.inch() {
            self.inch_tail();
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
        self.check_wall_kill(walls).or(self.check_tail_kill())
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
        for segment in self.get_tail() {
            if head_position == segment.get_position() {
                self.kill();
                return Some(DeathCause::Tail);
            }
        }
        None
    }

    fn kill(&mut self) {
        self.alive = false;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Segment {
    position: Position,
    direction: Direction,
    head: bool,
}

impl Segment {
    pub fn new_head(position: Position, direction: Direction) -> Self {
        Self {
            position,
            direction,
            head: true,
        }
    }

    pub fn make_tail(&mut self) {
        self.head = false;
    }

    pub fn is_head(&self) -> bool {
        self.head
    }

    pub fn is_tail(&self) -> bool {
        !self.head
    }

    pub fn get_position(&self) -> Position {
        self.position
    }

    pub fn get_direction(&self) -> Direction {
        self.direction
    }

    fn inch(&mut self) -> Option<DeathCause> {
        self.position.nudge(self.direction)
    }

    pub fn get_char(&self) -> char {
        if self.is_head() {
            self.direction.get_head_char()
        } else {
            self.direction.get_tail_char()
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn get_head_char(&self) -> char {
        match self {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
        }
    }

    pub fn get_tail_char(&self) -> char {
        match self {
            Direction::Up | Direction::Down => '|',
            Direction::Left | Direction::Right => '-',
        }
    }

    pub fn describe(&self) -> &str {
        match self {
            Direction::Up => "up",
            Direction::Down => "down",
            Direction::Left => "left",
            Direction::Right => "right",
        }
    }
}

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
