use cursive::{
    theme::{Color, PaletteColor, Theme as CursiveTheme},
    Cursive,
};

pub fn set(app: &mut Cursive) {
    let mut theme = CursiveTheme::default();
    theme.palette[PaletteColor::Background] = Color::TerminalDefault;
    app.set_theme(theme)
}
