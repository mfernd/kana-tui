use crossterm::event::KeyEvent;
use ratatui::{layout::Rect, Frame};

use crate::app::{IPage, ReturnedPage};

#[derive(Debug, Clone)]
pub struct ConfigPage;

impl IPage for ConfigPage {
    fn render(&mut self, _: &mut Frame, _: Rect) {
        todo!()
    }

    fn handle_key_events(&mut self, _: KeyEvent) -> ReturnedPage {
        todo!()
    }
}
