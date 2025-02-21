use crate::{config::Config, pages};
use crossterm::event::KeyEvent;
use enum_dispatch::enum_dispatch;
use ratatui::{layout::Rect, Frame};

#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub current_page: Page,
    pub config: Config,
}

impl App {
    pub fn new(config: Config) -> Self {
        Self {
            running: true,
            current_page: Page::Homepage(pages::Homepage::default()),
            config,
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}

#[derive(Debug)]
pub enum PageEvent {
    Nothing,
    Navigate(Page),
    QuitApp,
}

#[enum_dispatch]
pub trait IPage: std::fmt::Debug {
    /// Used when rendering the page.
    fn render(&mut self, frame: &mut Frame, main_area: Rect);
    /// To update the content of the page with key events.
    fn handle_key_events(&mut self, key_event: KeyEvent) -> PageEvent;
}

#[derive(Debug)]
#[enum_dispatch(IPage)]
pub enum Page {
    Homepage(pages::Homepage),
    ConfigPage(pages::ConfigPage),
    StudyPage(pages::StudyPage),
    ResultPage(pages::ResultPage),
}
