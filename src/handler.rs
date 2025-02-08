use crate::{
    app::{App, Page},
    pages::homepage,
    AppResult,
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match (&app.current_page, key_event.modifiers, key_event.code) {
        (_, _, KeyCode::Esc | KeyCode::Char('q'))
        | (_, KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => app.quit(),
        (Page::Homepage(_), _, _) => homepage::handle_key_events(key_event, app),
        _ => {}
    }
    Ok(())
}
