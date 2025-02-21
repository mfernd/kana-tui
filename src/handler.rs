use crate::{
    app::{App, IPage, PageEvent},
    AppResult,
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match (key_event.modifiers, key_event.code) {
        (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => app.quit(),
        _ => match app.current_page.handle_key_events(key_event) {
            PageEvent::Nothing => { /* do nothing :) */ }
            PageEvent::Navigate(new_page) => app.current_page = new_page,
            PageEvent::QuitApp => app.quit(),
        },
    }

    Ok(())
}
