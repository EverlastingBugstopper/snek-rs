mod theme;
mod views;

use cursive::{Cursive, CursiveExt};

pub struct Tui {
    app: Cursive,
}

impl Tui {
    pub fn new() -> Self {
        let mut app = Cursive::default();
        app.add_global_callback('q', Cursive::quit);
        theme::set(&mut app);
        views::title_screen(&mut app);
        Tui { app }
    }

    pub fn run(&mut self) {
        self.app.run();
    }
}
