use crate::{
    app::{App, Page},
    tui,
};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Alignment, Constraint},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};
use std::fmt::Display;
use strum::{EnumCount, EnumIter, IntoEnumIterator};

pub fn render(app: &mut App, frame: &mut Frame) {
    let page_data = PageData::extract(&app.current_page).unwrap();

    let block = Block::default()
        .title(" kana-tui ")
        .title_alignment(Alignment::Center)
        .title_style(Style::default().bold().fg(Color::Red))
        .borders(Borders::all())
        .border_type(BorderType::Thick)
        .border_style(Style::default().fg(Color::DarkGray));

    frame.render_widget(block, frame.area());

    let menu = MenuOption::render(&page_data.current_option).centered();
    let area = tui::center(
        frame.area(),
        Constraint::Length(MenuOption::get_max_width()),
        Constraint::Length(MenuOption::get_max_height()),
    );
    frame.render_widget(menu, area);
}

pub fn handle_key_events(key_event: KeyEvent, app: &mut App) {
    let mut page_data = PageData::extract(&app.current_page).unwrap().clone();

    match (&page_data.current_option, key_event.code) {
        (MenuOption::Quit, KeyCode::Enter) => app.quit(),
        (MenuOption::Start, KeyCode::Enter) => {
            app.go_to_study_page();
            return;
        }
        (_, KeyCode::Left | KeyCode::Up) => page_data.previous_option(),
        (_, KeyCode::Right | KeyCode::Down) => page_data.next_option(),
        _ => {}
    }

    app.go_to_homepage(page_data);
}

#[derive(Debug, Clone, Default)]
pub struct PageData {
    current_option: MenuOption,
}

impl PageData {
    fn extract(page: &Page) -> Option<&Self> {
        match page {
            Page::Homepage(page_data) => Some(page_data),
            Page::StudyPage => None,
        }
    }

    fn next_option(&mut self) {
        match self.current_option {
            MenuOption::Start => self.current_option = MenuOption::Quit,
            MenuOption::Quit => self.current_option = MenuOption::Start,
        }
    }

    fn previous_option(&mut self) {
        match self.current_option {
            MenuOption::Start => self.current_option = MenuOption::Quit,
            MenuOption::Quit => self.current_option = MenuOption::Start,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, EnumIter, EnumCount)]
pub enum MenuOption {
    #[default]
    Start,
    Quit,
}

impl MenuOption {
    fn render(current_option: &MenuOption) -> Paragraph {
        let mut options: Vec<Line> = Vec::new();
        for (i, option) in MenuOption::iter().enumerate() {
            let l = if option.eq(current_option) {
                Line::from(option.to_string()).bold()
            } else {
                Line::from(option.to_string())
            };
            options.push(l);
            if i < MenuOption::COUNT - 1 {
                options.push(Line::from(""));
            }
        }
        Paragraph::new(options)
    }

    fn get_max_width() -> u16 {
        MenuOption::iter()
            .map(|o| o.to_string().len())
            .max()
            .unwrap_or(0) as u16
    }

    fn get_max_height() -> u16 {
        (MenuOption::COUNT as u16) * 2 - 1
    }
}

impl Display for MenuOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Start => write!(f, "Start"),
            Self::Quit => write!(f, "Quit"),
        }
    }
}
