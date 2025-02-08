use crate::pages::homepage;

#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub current_page: Page,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            current_page: Page::Homepage(homepage::PageData::default()),
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn go_to_homepage(&mut self, page_data: homepage::PageData) {
        self.current_page = Page::Homepage(page_data)
    }

    pub fn go_to_study_page(&mut self) {
        self.current_page = Page::StudyPage
    }
}

#[derive(Debug)]
pub enum Page {
    Homepage(homepage::PageData),
    StudyPage,
}
