use crate::app::App;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    widgets::Paragraph,
    Frame,
};

pub fn render(_app: &mut App, frame: &mut Frame, main_area: Rect) {
    let layout =
        Layout::vertical(Constraint::from_ratios([(1, 3), (1, 3), (1, 3)])).split(main_area);

    frame.render_widget(
        Paragraph::new("foo0").black().on_red().centered(),
        layout[0],
    );
    frame.render_widget(
        Paragraph::new("foo1").black().on_green().centered(),
        layout[1],
    );
    frame.render_widget(
        Paragraph::new("foo2").black().on_blue().centered(),
        layout[2],
    );
}

pub fn handle_key_events(key_event: KeyEvent, app: &mut App) {
    match key_event.code {
        KeyCode::Esc => app.go_to_homepage(),
        KeyCode::Enter => {}
        _ => {}
    }
}
