use cursive::{views::Dialog, Cursive};

pub fn start(app: &mut Cursive) {
    tracing::info_span!("entering high score screen");
    app.add_layer(Dialog::info("Not yet implemented!"))
}
