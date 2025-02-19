use crate::pages;
use bon::bon;
use crossterm::event::KeyEvent;
use enum_dispatch::enum_dispatch;
use ratatui::{layout::Rect, Frame};

#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub current_page: Page,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            current_page: Page::Homepage(pages::Homepage::default()),
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn go_to(&mut self, new_page: Page) {
        self.current_page = new_page;
    }
}

pub type ReturnedPage = Option<Page>;

#[enum_dispatch]
pub trait IPage: std::fmt::Debug {
    /// Used when rendering the page.
    fn render(&mut self, frame: &mut Frame, main_area: Rect);
    /// To update the content of the page with key events.
    fn handle_key_events(&mut self, key_event: KeyEvent) -> ReturnedPage;
}

#[derive(Debug)]
#[enum_dispatch(IPage)]
pub enum Page {
    Homepage(pages::Homepage),
    StudyPage(pages::StudyPage),
    ResultPage(pages::ResultPage),
}

// These implementations are juste here to simplify my life (or be more readable) when changing/updating page...
// Instead of doing `Some(Homepage::default().into())`, `Some(Page::from(self.clone()))`...
#[bon]
impl Page {
    #[builder]
    pub fn go_home(page: Option<pages::Homepage>) -> ReturnedPage {
        Some(page.unwrap_or_default().into())
    }

    #[builder]
    pub fn go_study(page: Option<pages::StudyPage>) -> ReturnedPage {
        Some(page.unwrap_or_default().into())
    }

    #[builder]
    pub fn go_result(page: pages::ResultPage) -> ReturnedPage {
        Some(page.into())
    }
}
