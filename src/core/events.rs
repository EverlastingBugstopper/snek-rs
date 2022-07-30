use crossbeam_channel::{unbounded, Receiver, Sender};

use crate::core::{Cell, DeathCause, Direction, Position};

#[derive(Debug, Clone)]
pub struct EventStream {
    draw_event_sender: Sender<DrawEvent>,
    draw_event_receiver: Receiver<DrawEvent>,
    slither_event_sender: Sender<SlitherEvent>,
    slither_event_receiver: Receiver<SlitherEvent>,
    user_event_sender: Sender<UserEvent>,
    user_event_receiver: Receiver<UserEvent>,
}

impl EventStream {
    pub fn new() -> Self {
        let (draw_event_sender, draw_event_receiver) = unbounded();
        let (user_event_sender, user_event_receiver) = unbounded();
        let (slither_event_sender, slither_event_receiver) = unbounded();
        Self {
            slither_event_sender,
            slither_event_receiver,
            draw_event_sender,
            draw_event_receiver,
            user_event_sender,
            user_event_receiver,
        }
    }

    pub fn draw(&self, cell: Cell, position: Position) {
        self.draw_event_sender
            .send(DrawEvent { cell, position })
            .unwrap();
    }

    pub fn turn(&self, direction: Direction) {
        self.user_event_sender
            .send(UserEvent::Turn(direction))
            .unwrap();
    }

    pub fn die(&self, death_cause: DeathCause) {
        let slither_event = SlitherEvent::Die(DeathEvent::new(death_cause));
        self.slither_event_sender.send(slither_event).unwrap();
    }

    pub fn score(&self, new_score: usize) {
        let slither_event = SlitherEvent::Score(ScoreEvent::new(new_score));
        self.slither_event_sender.send(slither_event).unwrap();
    }

    pub fn pause(&self) {
        self.user_event_sender.send(UserEvent::Pause).unwrap();
    }

    pub fn unpause(&self) {
        self.user_event_sender.send(UserEvent::Unpause).unwrap();
    }

    pub fn maybe_draw_events(&self) -> Vec<DrawEvent> {
        self.draw_event_receiver.try_iter().collect()
    }

    pub fn maybe_user_events(&self) -> Vec<UserEvent> {
        self.user_event_receiver.try_iter().collect()
    }

    pub fn maybe_slither_event(&self) -> Option<SlitherEvent> {
        self.slither_event_receiver.try_recv().ok()
    }
}

pub struct DrawEvent {
    cell: Cell,
    position: Position,
}

impl DrawEvent {
    pub fn get_coordinates(&self) -> (usize, usize) {
        self.position.get_coordinates()
    }

    pub fn display(&self) -> String {
        self.cell.display()
    }
}

#[derive(Debug, Clone)]
pub enum SlitherEvent {
    Score(ScoreEvent),
    Die(DeathEvent),
    Win,
}

#[derive(Debug, Clone)]
pub struct ScoreEvent {
    score: usize,
}

impl ScoreEvent {
    pub fn new(score: usize) -> Self {
        Self { score }
    }
}

impl ScoreEvent {
    pub fn get_score(&self) -> usize {
        self.score
    }
}

#[derive(Debug, Clone)]
pub struct DeathEvent {
    death_cause: DeathCause,
}

impl DeathEvent {
    pub fn new(death_cause: DeathCause) -> Self {
        Self { death_cause }
    }
    pub fn display(&self) -> String {
        self.death_cause.display()
    }
}

pub enum UserEvent {
    Pause,
    Unpause,
    Turn(Direction),
}
