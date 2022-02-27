use crate::core::{Apple, DeathCause, Direction, Position, Segment, SlitherResultType, State};

use crossbeam_channel::unbounded;

use cursive::{
    event::{Event, EventResult},
    view::View,
    views::{Dialog, LinearLayout, NamedView, TextView, ViewRef},
    Cursive, Printer, Vec2,
};

use rayon::prelude::*;

pub fn new_game(app: &mut Cursive) {
    tracing::debug!("new game");
    let board_view = BoardView::new();
    let score_view = TextView::new(board_view.get_score_content()).center();
    let named_board_view = NamedView::new("board", board_view);
    let named_score_view = NamedView::new("score", score_view);
    app.pop_layer();
    // let wrapped_view; // replae linear layout with wrapped view
    // implement required_size on the score view
    app.add_fullscreen_layer(
        LinearLayout::vertical()
            .child(named_score_view)
            .child(named_board_view),
    );
    app.focus_name("board").unwrap();
    app.set_fps(6);
}

struct BoardView {
    has_resized: bool,
    pub size: Vec2,
    pub cells: Vec<Cell>,
    state: State,
    offset: usize,
}

impl BoardView {
    pub fn new() -> Self {
        let min_width = 6;
        let min_height = 6;
        BoardView {
            size: Vec2::new(min_width, min_height),
            cells: vec![Cell::Free; min_width * min_height],
            state: State::new(min_width, min_height),
            has_resized: false,
            offset: 2,
        }
    }

    fn turn_snek(&mut self, direction: Direction) -> EventResult {
        if self.state.turn_snek(direction) {
            let mut wormy_head = *self.state.get_snek().get_head();
            wormy_head.set_direction(&direction);
            self.draw_segment(&wormy_head);
        }
        EventResult::Consumed(None)
    }

    fn pause(&mut self) -> EventResult {
        EventResult::with_cb(|s| {
            let old_fps = s.fps();
            let controls_dialog = Dialog::text(
                "  ~~~ controlsss ~~~

ssslither ~~~> wasssd
  paussse ~~~> p
   ssstop ~~~> q",
            );
            s.set_fps(0);
            s.add_layer(NamedView::new(
                "pause",
                controls_dialog.button("unpausss", move |s| {
                    s.pop_layer();
                    if let Some(old_fps) = old_fps {
                        s.set_fps(old_fps.get());
                    }
                }),
            ))
        })
    }

    fn tick(&mut self) -> EventResult {
        let old_apple = self.state.get_apple().get_position();
        let slither_result = self.state.tick();
        if let Some(slime_trail) = slither_result.get_slime_trail() {
            self.free_cell(slime_trail);
        }
        self.update_apple(Some(old_apple));
        self.state
            .get_snek()
            .get_segments()
            .iter()
            .for_each(|s| self.draw_segment(s));
        match slither_result.get_type() {
            SlitherResultType::Died { death_cause } => self.die_alog(death_cause),
            SlitherResultType::AteTheWorld => EventResult::with_cb(|s| {
                s.set_autorefresh(false);
                s.add_layer(Dialog::text("snek ate the world!").button("Ok", |s| {
                    s.pop_layer();
                    s.pop_layer();
                    new_game(s);
                }));
            }),
            SlitherResultType::Grew => {
                let score_content = self.get_score_content();
                EventResult::with_cb(move |s| {
                    let mut score_view: ViewRef<TextView> = s.find_name("score").unwrap();
                    score_view.set_content(&score_content);
                })
            }
            SlitherResultType::Slithered => EventResult::Consumed(None),
        }
    }

    fn resize(&mut self, constraints: Vec2) {
        if !self.has_resized {
            if constraints > self.size {
                self.size = constraints;
                self.state = State::new(constraints.x, constraints.y);
                self.cells = vec![Cell::Free; constraints.x * constraints.y];
            }
            self.update_walls();
            assert_eq!(
                self.cells.last().unwrap(),
                &Cell::Wall(WallType::BottomRightCorner)
            );
            self.update_apple(None);
            self.state
                .get_snek()
                .get_segments()
                .iter()
                .for_each(|s| self.draw_segment(s));
            self.has_resized = true;
            self.size = constraints;
        }
    }

    fn die_alog(&mut self, death_cause: DeathCause) -> EventResult {
        let text = death_cause.describe().to_string();
        EventResult::with_cb(move |s| {
            s.add_layer(
                Dialog::text(&text)
                    .button("play again", |s| {
                        s.pop_layer();
                        s.pop_layer();
                        new_game(s);
                    })
                    .button("quit", |s| s.quit()),
            );
            s.set_autorefresh(false);
        })
    }

    fn update_apple(&mut self, old_apple: Option<Position>) {
        let apple = self.state.get_apple().to_owned();
        if let Some(old_apple) = old_apple {
            self.update_cell(old_apple, Cell::Free);
        }
        self.update_cell(apple.get_position(), Cell::Apple(apple));
    }

    fn draw_segment(&mut self, segment: &Segment) {
        let position = segment.get_position();
        let cell = Cell::Snek(*segment);
        self.update_cell(position, cell);
    }

    fn free_cell(&mut self, position: Position) {
        self.update_cell(position, Cell::Free)
    }

    fn update_cell(&mut self, position: Position, cell: Cell) {
        let i = self.get_cell_idx_from_position(&position);
        self.cells[i] = cell;
    }

    fn update_walls(&mut self) {
        let walls = self.state.get_walls().clone();
        let left_wall = walls.left_wall();
        let right_wall = walls.right_wall();
        let top_wall = walls.top_wall();
        let bottom_wall = walls.bottom_wall();
        for position in walls.get_positions() {
            let (x, y) = position.get_coordinates();
            let wall_type = match (
                (x == left_wall, x == right_wall),
                (y == top_wall, y == bottom_wall),
            ) {
                ((true, false), (true, false)) => WallType::TopLeftCorner,
                ((false, true), (true, false)) => WallType::TopRightCorner,
                ((true, false), (false, true)) => WallType::BottomLeftCorner,
                ((false, true), (false, true)) => WallType::BottomRightCorner,
                ((true, false), (false, false)) => WallType::LeftWall,
                ((false, true), (false, false)) => WallType::RightWall,
                ((false, false), (true, false)) => WallType::TopWall,
                ((false, false), (false, true)) => WallType::BottomWall,
                _ => {
                    unreachable!("{:?} is a bad wall segment", position)
                }
            };
            self.update_cell(position, Cell::Wall(wall_type));
        }
    }

    fn get_cell_idx_from_position(&self, position: &Position) -> usize {
        let (x, y) = position.get_coordinates();
        x + (self.size.x * y)
    }

    fn get_position_from_cell_idx(&self, cell_idx: usize) -> Position {
        let x = (cell_idx % self.size.x) * 2;
        let y = cell_idx / self.size.x;
        Position::new(x, y)
    }

    fn get_score_content(&self) -> String {
        format!("ssscore: {}", self.state.get_score())
    }

    fn user_resized(&mut self) -> EventResult {
        self.die_alog(DeathCause::Resized)
    }
}

impl View for BoardView {
    fn draw(&self, printer: &Printer) {
        let (sender, receiver) = unbounded();
        self.cells
            .par_iter()
            .enumerate()
            .for_each_with(sender, |sender, (i, cell)| {
                let position = self.get_position_from_cell_idx(i);
                if let Cell::Snek(segment) = cell {
                    tracing::info!("{}", segment.display());
                }
                if let Cell::Free = cell {
                } else {
                    sender
                        .send((position.get_coordinates(), cell.display()))
                        .unwrap();
                }
            });
        receiver.iter().for_each(|((x, y), text)| {
            printer.print((x + self.offset, y), &text);
        });
    }
    fn required_size(&mut self, constraints: Vec2) -> Vec2 {
        if !self.has_resized {
            let (width, height) = term_size::dimensions().unwrap();
            tracing::info!("resizing to width: {}, height: {}", width, height);
            let new_size = Vec2 {
                x: (width / 2) - self.offset,
                y: height - 1,
            };
            self.resize(new_size);
        }
        constraints
    }

    fn needs_relayout(&self) -> bool {
        !self.has_resized
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        match event {
            Event::Char('w') => self.turn_snek(Direction::Up),
            Event::Char('a') => self.turn_snek(Direction::Left),
            Event::Char('s') => self.turn_snek(Direction::Down),
            Event::Char('d') => self.turn_snek(Direction::Right),
            Event::Char('p') | Event::FocusLost => self.pause(),
            Event::Refresh => self.tick(),
            Event::WindowResize => self.user_resized(),
            _ => EventResult::Ignored,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    Snek(Segment),
    Apple(Apple),
    Wall(WallType),
    Free,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum WallType {
    TopWall,
    BottomWall,
    LeftWall,
    RightWall,
    TopLeftCorner,
    BottomLeftCorner,
    TopRightCorner,
    BottomRightCorner,
}

impl Cell {
    fn display(&self) -> String {
        match self {
            Cell::Snek(segment) => segment.display().to_string(),
            Cell::Apple(_) => "🍎".to_string(),
            Cell::Wall(wall_type) => match wall_type {
                WallType::TopLeftCorner => "╭─",
                WallType::BottomLeftCorner => "╰─",
                WallType::BottomRightCorner => "╯",
                WallType::TopRightCorner => "╮",
                WallType::TopWall | WallType::BottomWall => "──",
                WallType::LeftWall | WallType::RightWall => "│",
            }
            .to_string(),
            Cell::Free => "  ".to_string(),
        }
    }
}
