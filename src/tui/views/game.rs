use crate::core::{Board, DeathEvent, Direction, ScoreEvent, SlitherEvent};

use cursive::{
    direction::Orientation,
    event::{Event as CursiveEvent, EventResult},
    view::View,
    views::{Dialog, LinearLayout, NamedView, TextView, ViewRef},
    Cursive, Printer, Vec2,
};

use std::sync::{Arc, Mutex, MutexGuard};

pub fn new_game(app: &mut Cursive) {
    tracing::debug!("new game");
    let board = Board::new(6, 6);
    tracing::info!("creating board view");
    let board_view = BoardView::new(Arc::new(Mutex::new(board)));
    tracing::info!("created board view");
    let score_view = TextView::new(board_view.get_score_content(0)).center();
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
    app.set_fps(30);
}

struct ScoreView {
    has_resized: bool,
    pub size: Vec2,
    pub score: usize,
}

struct BoardView {
    has_resized: bool,
    pub size: Vec2,
    pub board: Arc<Mutex<Board>>,
    offset: usize,
}

impl BoardView {
    pub fn new(board: Arc<Mutex<Board>>) -> Self {
        let this_board = board.clone();
        let mut b = this_board.lock().unwrap();
        let (width, height) = (b.get_width(), b.get_height());
        b.start();
        let board = this_board.clone();
        let view = BoardView {
            size: Vec2::new(width, height),
            board,
            has_resized: false,
            offset: 2,
        };

        view
    }

    fn get_board(&self) -> MutexGuard<Board> {
        self.board.lock().unwrap()
    }

    fn turn_snek(&mut self, direction: Direction) -> EventResult {
        self.get_board().user_directional_input(direction);
        EventResult::Consumed(None)
    }

    fn pause(&self) -> EventResult {
        self.get_board().pause();
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

    fn update_score(&self, score_event: ScoreEvent) -> EventResult {
        let score_content = self.get_score_content(score_event.get_score());
        EventResult::with_cb(move |s| {
            let mut score_view: ViewRef<TextView> = s.find_name("score").unwrap();
            score_view.set_content(&score_content);
        })
    }

    fn resize(&mut self, constraints: Vec2) {
        if !self.has_resized {
            if constraints > self.size {
                self.size = constraints;
                let mut board = self.get_board();
                board.set_width(constraints.x);
                board.set_height(constraints.y);
                self.get_board().start();
            }
            self.has_resized = true;
            self.size = constraints;
        }
    }

    fn die_alog(&self, death_event: DeathEvent) -> EventResult {
        let text = death_event.display();
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

    fn permanent_win(&self) -> EventResult {
        EventResult::with_cb(|s| {
            s.set_autorefresh(false);
            s.add_layer(Dialog::text("snek ate the world!").button("Ok", |s| {
                s.pop_layer();
                s.pop_layer();
                new_game(s);
            }));
        })
    }

    fn get_score_content(&self, score: usize) -> String {
        format!("ssscore: {}", score)
    }
}

impl View for ScoreView {
    fn draw(&self, printer: &Printer) {
        printer.print_line(
            Orientation::Horizontal,
            self.size,
            self.size.x,
            &format!("ssscore: {}", &self.score),
        )
    }

    fn required_size(&mut self, constraints: Vec2) -> Vec2 {
        Vec2 {
            x: (constraints.x),
            y: 2,
        }
    }

    fn needs_relayout(&self) -> bool {
        !self.has_resized
    }
}

impl View for BoardView {
    fn draw(&self, printer: &Printer) {
        self.get_board()
            .maybe_draw_events()
            .iter()
            .for_each(|draw_event| {
                let (x, y) = draw_event.get_coordinates();
                printer.print((x + self.offset, y), &draw_event.display());
            })
    }
    fn required_size(&mut self, constraints: Vec2) -> Vec2 {
        if !self.has_resized {
            let new_size = Vec2 {
                x: (constraints.x / 2) - self.offset,
                y: constraints.y,
            };
            self.resize(new_size);
        }
        constraints
    }

    fn needs_relayout(&self) -> bool {
        !self.has_resized
    }

    fn on_event(&mut self, event: CursiveEvent) -> EventResult {
        match event {
            CursiveEvent::Char('w') => self.turn_snek(Direction::Up),
            CursiveEvent::Char('a') => self.turn_snek(Direction::Left),
            CursiveEvent::Char('s') => self.turn_snek(Direction::Down),
            CursiveEvent::Char('d') => self.turn_snek(Direction::Right),
            CursiveEvent::Char('p') | CursiveEvent::FocusLost => self.pause(),
            CursiveEvent::Refresh => {
                let board = self.get_board();
                let event_result = if board.is_paused() {
                    Some(self.pause())
                } else {
                    board
                        .maybe_slither_event()
                        .map(|slither_event| match slither_event {
                            SlitherEvent::Die(death_event) => self.die_alog(death_event),
                            SlitherEvent::Score(score_event) => self.update_score(score_event),
                            SlitherEvent::Win => self.permanent_win(),
                        })
                };
                event_result.unwrap_or(EventResult::Ignored)
            }
            _ => EventResult::Ignored,
        }
    }
}
