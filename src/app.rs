use crate::study::{create_study_plan, kana::Kana};
use std::error;

pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub struct App {
    pub running: bool,
    /// counter
    pub study_plan: Vec<Kana>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            study_plan: create_study_plan(),
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
}
