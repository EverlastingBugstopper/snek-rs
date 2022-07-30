use crate::core::{
    Cell, Direction, DrawEvent, EventStream, Position, Segment, SlitherEvent, SlitherResultType,
    State, UserEvent,
};

use std::sync::{Arc, Mutex, MutexGuard};
use std::thread::JoinHandle;
use std::time::Duration;

#[derive(Debug)]
pub struct Board {
    state: Arc<Mutex<State>>,
    event_stream: EventStream,
    height: usize,
    width: usize,
    tick_interval: u64,
    event_loop: Arc<Mutex<Option<JoinHandle<()>>>>,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        let board = Self {
            state: Arc::new(Mutex::new(State::new(width, height))),
            event_stream: EventStream::new(),
            height,
            width,
            tick_interval: 500,
            event_loop: Arc::new(Mutex::new(None)),
        };
        board.draw_walls();
        board.draw_apple();
        board.draw_snek();
        board
    }

    pub fn start(&mut self) {
        tracing::info!("starting");
        if self.event_loop.lock().unwrap().is_none() {
            let state = self.state.clone();
            let tick_interval = self.get_tick_interval();
            state.lock().unwrap().start();
            tracing::info!("started slithering");
            let handle = std::thread::spawn(move || {
                tracing::info!("creating ticker");
                let ticker = crossbeam_channel::tick(Duration::from_millis(tick_interval));
                loop {
                    tracing::info!("waiting for tick");
                    ticker.recv().unwrap();
                    tracing::info!("received tick, modifying state.");
                    state.lock().unwrap().tick();
                    tracing::info!("modified state");
                }
            });
            self.event_loop = Arc::new(Mutex::new(Some(handle)));
        }
    }

    pub fn set_width(&mut self, width: usize) {
        self.width = width;
    }

    pub fn set_height(&mut self, height: usize) {
        self.height = height;
    }

    pub fn get_state_mut(&self) -> MutexGuard<State> {
        self.state.lock().unwrap()
    }

    pub fn get_tick_interval(&self) -> u64 {
        self.tick_interval
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn is_paused(&self) -> bool {
        !self.get_state_mut().is_slithering()
    }

    pub fn pause(&self) {
        self.get_state_mut().stop();
    }

    pub fn unpause(&self) {
        self.get_state_mut().start();
    }

    fn apply_direction(&self, direction: Direction) {
        let mut state = self.get_state_mut();
        if state.turn_snek(direction) {
            let mut wormy_head = *state.get_snek().get_head();
            wormy_head.set_direction(&direction);
            self.draw_segment(&wormy_head);
        }
    }

    pub fn user_directional_input(&mut self, direction: Direction) {
        self.event_stream.turn(direction)
    }

    pub fn get_score(&self) -> usize {
        self.get_state_mut().get_score()
    }

    pub fn maybe_draw_events(&self) -> Vec<DrawEvent> {
        self.event_stream.maybe_draw_events()
    }

    pub fn maybe_slither_event(&self) -> Option<SlitherEvent> {
        self.event_stream.maybe_slither_event()
    }

    pub fn tick(&self) {
        for user_event in self.event_stream.maybe_user_events() {
            match user_event {
                UserEvent::Pause => self.pause(),
                UserEvent::Unpause => self.unpause(),
                UserEvent::Turn(direction) => self.apply_direction(direction),
            }
        }

        if !self.is_paused() {
            let slither_result = self.get_state_mut().tick();
            if let Some(slime_trail) = slither_result.get_slime_trail() {
                self.draw_empty(slime_trail);
            }
            if let Some(old_apple) = slither_result.get_old_apple() {
                self.draw_empty(old_apple);
            }
            self.draw_head();
            match slither_result.get_type() {
                SlitherResultType::Died { death_cause } => self.event_stream.die(death_cause),
                SlitherResultType::Grew => self.event_stream.score(self.get_score()),
                _ => (),
            }
        }
    }

    fn draw_cell_at(&self, position: Position, cell: Cell) {
        self.event_stream.draw(cell, position);
    }

    fn draw_snek(&self) {
        self.get_state_mut()
            .get_snek()
            .get_segments()
            .iter()
            .for_each(|s| self.draw_segment(s))
    }

    fn draw_head(&self) {
        let head = *self.get_state_mut().get_snek().get_head();
        self.draw_segment(&head);
    }

    fn draw_segment(&self, segment: &Segment) {
        let position = segment.get_position();
        let cell = Cell::Snek(*segment);
        self.draw_cell_at(position, cell);
    }

    fn draw_empty(&self, slime_trail: Position) {
        self.draw_cell_at(slime_trail, Cell::Free)
    }

    fn draw_apple(&self) {
        let apple = *self.get_state_mut().get_apple();
        self.draw_cell_at(apple.get_position(), Cell::Apple(apple));
    }

    fn draw_walls(&self) {
        for wall in self.get_state_mut().perimeter() {
            self.draw_cell_at(wall.get_position(), Cell::Wall(wall.get_type()));
        }
    }
}
