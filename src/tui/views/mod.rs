mod game;
mod high_scores;

use cursive::{views::Dialog, Cursive};

pub fn title_screen(app: &mut Cursive) {
    tracing::info_span!("entering title screen");
    app.pop_layer();
    app.add_layer(
        Dialog::text("\n\n   sso u want to play ssnek?..\n\n")
            .title("snek")
            .button("sstart", game::new_game)
            .button("high sscores", high_scores::start)
            .button("sstop", |app| app.quit()),
    );
}
