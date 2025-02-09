use crate::pages::{homepage, study_page};

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

#[bon::bon]
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

    #[builder]
    pub fn go_to_homepage(&mut self, data: Option<homepage::PageData>) {
        self.current_page = Page::Homepage(data.unwrap_or_default())
    }

    #[builder]
    pub fn go_to_study_page(&mut self, data: Option<study_page::PageData>) {
        self.current_page = Page::StudyPage(data.unwrap_or_default())
    }
}

#[derive(Debug)]
pub enum Page {
    Homepage(homepage::PageData),
    StudyPage(study_page::PageData),
}
