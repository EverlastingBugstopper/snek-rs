use crate::core::{Apple, Direction, Position, Segment, SlitherResult, State};

use crossbeam_channel::unbounded;
use cursive::event::{Event, EventResult};
use cursive::{
    view::View,
    views::{Dialog, LinearLayout, NamedView},
    Cursive, Printer, Vec2,
};

use rayon::prelude::*;

pub fn new_game(app: &mut Cursive) {
    tracing::debug!("new game");
    let xy = app.screen_size();
    let options = Options::new(xy.x, xy.y);
    app.pop_layer();
    app.add_layer(
        Dialog::new()
            .title("snek")
            .content(LinearLayout::horizontal().child(BoardView::new(options))),
    );
    app.set_fps(3);
}

pub(crate) struct Options {
    pub size: Vec2,
}

impl Options {
    pub fn new(width: usize, height: usize) -> Options {
        Options {
            size: Vec2 {
                x: width,
                y: height,
            },
        }
    }
}

#[derive(Debug)]
struct BoardView {
    pub cells: Vec<Cell>,
    pub size: Vec2,
    state: State,
}

impl BoardView {
    pub fn new(options: Options) -> Self {
        let cell_count = options.size.x * options.size.y;
        let state = State::new(options.size.x - 80, options.size.y - 5);
        let mut board_view = BoardView {
            cells: vec![Cell::Free; cell_count],
            size: options.size,
            state,
        };

        board_view.init();

        board_view
    }

    fn turn_snek(&mut self, direction: Direction) -> EventResult {
        if self.state.turn_snek(direction) {
            let mut wormy_head = *self.state.get_snek().get_head();
            wormy_head.set_direction(&direction);
            self.draw_segment(wormy_head);
        }
        EventResult::Consumed(None)
    }

    fn pause(&mut self) -> EventResult {
        EventResult::with_cb(|s| {
            let old_fps = s.fps();
            s.set_fps(0);
            s.add_layer(NamedView::new(
                "pause",
                Dialog::text(
                    "Controls:
            ssslither: wasssd
            pausss: p
            ssstop: q",
                )
                .button("unpausss", move |s| {
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
        match slither_result {
            SlitherResult::Died(death_cause) => {
                let text = death_cause.describe().to_string();
                EventResult::with_cb(move |s| {
                    s.set_autorefresh(false);
                    s.add_layer(
                        Dialog::text(&text)
                            .button("play again", |s| {
                                s.pop_layer();
                                s.pop_layer();
                                new_game(s);
                            })
                            .button("quit", |s| s.quit()),
                    );
                })
            }
            SlitherResult::AteTheWorld => EventResult::with_cb(|s| {
                s.set_autorefresh(false);
                s.add_layer(Dialog::text("snek ate the world!").button("Ok", |s| {
                    s.pop_layer();
                    s.pop_layer();
                    new_game(s);
                }));
            }),
            SlitherResult::Grew {
                direction: _,
                segments,
                slime_trail,
            } => {
                self.free_cell(slime_trail);
                self.update_apple(Some(old_apple));
                for segment in segments {
                    self.draw_segment(segment);
                }

                EventResult::Consumed(None)
            }
            SlitherResult::Slithered {
                direction: _,
                segments,
                slime_trail,
            } => {
                self.free_cell(slime_trail);
                for segment in segments {
                    self.draw_segment(segment);
                }
                self.update_apple(Some(old_apple));
                EventResult::Consumed(None)
            }
        }
    }

    fn init(&mut self) {
        self.update_walls();
        self.update_apple(None);
        for segment in self.state.get_snek().get_segments() {
            self.draw_segment(segment);
        }
    }

    fn update_apple(&mut self, old_apple: Option<Position>) {
        let apple = self.state.get_apple().to_owned();
        if let Some(old_apple) = old_apple {
            self.update_cell(old_apple, Cell::Free);
        }
        self.update_cell(apple.get_position(), Cell::Apple(apple));
    }

    fn draw_segment(&mut self, segment: Segment) {
        let position = segment.get_position();
        let cell = Cell::Snek(segment);
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
        for position in self.state.get_walls().get_positions() {
            self.update_cell(position, Cell::Wall)
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
}

impl View for BoardView {
    fn draw(&self, printer: &Printer) {
        let (sender, receiver) = unbounded();
        self.cells
            .par_iter()
            .enumerate()
            .for_each_with(sender, |sender, (i, c)| {
                let position = self.get_position_from_cell_idx(i);
                let (x, y) = position.get_coordinates();

                let text = c.display().to_string();
                sender.send(((x, y), text)).unwrap();
            });
        receiver.iter().for_each(|((x, y), text)| {
            printer.print((x, y), &text);
        });
    }

    fn required_size(&mut self, _: Vec2) -> Vec2 {
        self.size.map_x(|x| 2 * x)
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        match event {
            Event::Char('w') => self.turn_snek(Direction::Up),
            Event::Char('a') => self.turn_snek(Direction::Left),
            Event::Char('s') => self.turn_snek(Direction::Down),
            Event::Char('d') => self.turn_snek(Direction::Right),
            Event::Char('p') => self.pause(),
            Event::Refresh => self.tick(),
            _ => EventResult::Ignored,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Cell {
    Snek(Segment),
    Apple(Apple),
    Wall,
    Free,
}

impl Cell {
    fn display(&self) -> &str {
        match self {
            Cell::Snek(segment) => segment.display(),
            Cell::Apple(_) => "🍎",
            Cell::Wall => "X",
            Cell::Free => " ",
        }
    }
}
