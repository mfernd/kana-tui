use crate::app::{IPage, PageEvent};
use crossterm::event::KeyEvent;
use ratatui::{layout::Rect, Frame};

#[derive(Debug, Clone)]
pub struct ConfigPage;

impl IPage for ConfigPage {
    fn render(&mut self, _: &mut Frame, _: Rect) {
        todo!()
    }

    fn handle_key_events(&mut self, _: KeyEvent) -> PageEvent {
        PageEvent::Nothing
    }
}
