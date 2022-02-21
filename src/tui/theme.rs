use cursive::{Cursive, theme::{Theme as CursiveTheme, Color, PaletteColor}};

pub fn set(app: &mut Cursive) {
  let mut theme = CursiveTheme::default();
  theme.palette[PaletteColor::Background] = Color::TerminalDefault;
  app.set_theme(theme)
}