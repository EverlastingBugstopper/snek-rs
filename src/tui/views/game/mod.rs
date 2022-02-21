use crate::core::{Apple, Direction, Position, Segment, SlitherResult, State};

use cursive::event::{Event, EventResult};
use cursive::{
    view::View,
    views::{Dialog, LinearLayout, Panel},
    Cursive, Printer, Vec2,
};

pub fn new_game(app: &mut Cursive) {
    tracing::debug!("new game");
    let xy = app.screen_size();
    let options = Options::new(xy.x, xy.y);
    app.pop_layer();
    //     app.add__layer(Dialog::info(
    //         "Controls:
    // Move: WASD
    // Quit: q",
    //     ))
    //     app.pop_layer();
    app.add_layer(
        Dialog::new()
            .title("snek")
            .content(LinearLayout::horizontal().child(Panel::new(BoardView::new(options)))),
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
        let state = State::new(options.size.x, options.size.y);
        let mut board_view = BoardView {
            cells: vec![Cell::Free; cell_count],
            size: options.size,
            state,
        };

        board_view.update_cells();

        board_view
    }

    fn turn_snek(&mut self, direction: Direction) -> EventResult {
        self.state.turn_snek(direction);
        self.update_cells();
        EventResult::Consumed(None)
    }

    fn tick(&mut self) -> EventResult {
        let slither_result = self.state.tick();
        self.update_cells();
        match slither_result {
            SlitherResult::Died(death_cause) => {
                let text = death_cause.describe().to_string();
                EventResult::with_cb(move |s| {
                    s.set_autorefresh(false);
                    s.add_layer(Dialog::text(&text).button("Ok", |s| {
                        s.pop_layer();
                        s.pop_layer();
                        new_game(s);
                    }));
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
            SlitherResult::Grew(_) | SlitherResult::Slithered(_) => EventResult::Ignored,
        }
    }

    fn update_cells(&mut self) {
        self.cells = vec![Cell::Free; self.size.x * self.size.y];
        let apple = self.state.get_apple().to_owned();
        let apple_of_my_i = self.get_cell_idx_from_position(&apple.get_position());
        self.cells[apple_of_my_i] = Cell::Apple(apple);
        for segment in self.state.get_snek().get_segments() {
            let snek_eyes = self.get_cell_idx_from_position(&segment.get_position());
            self.cells[snek_eyes] = Cell::Snek(segment.to_owned())
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
        for (i, cell) in self.cells.iter().enumerate() {
            let position = self.get_position_from_cell_idx(i);
            let (x, y) = position.get_coordinates();

            let text = cell.get_char().to_string();
            printer.print((x, y), &text);
        }
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
            Event::Refresh => self.tick(),
            _ => EventResult::Ignored,
        }
    }
}

#[derive(Debug, Clone)]
enum Cell {
    Snek(Segment),
    Apple(Apple),
    Free,
}

impl Cell {
    fn get_char(&self) -> char {
        match self {
            Cell::Snek(segment) => segment.get_char(),
            Cell::Apple(_) => 'ó',
            Cell::Free => ' ',
        }
    }
}
