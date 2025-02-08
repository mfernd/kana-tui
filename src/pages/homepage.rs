use crate::app::{App, Page};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Margin, Rect},
    style::Stylize,
    text::Line,
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use std::fmt::Display;
use strum::{EnumIter, IntoEnumIterator};

pub fn render(app: &mut App, frame: &mut Frame) {
    let page_data = PageData::extract(&app.current_page).unwrap();
    let frame_area = frame.area();

    let block = Block::default()
        .title("kana-tui")
        .borders(Borders::all())
        .title_alignment(ratatui::layout::Alignment::Center);
    let menu = MenuOption::render_paragraph(&page_data.current_option);

    let width = 55;
    let height = 30;
    let area = Rect {
        x: frame_area.width.saturating_sub(width) / 2,
        y: frame_area.height.saturating_sub(height) / 2,
        width,
        height,
    };

    frame.render_widget(block, area);
    frame.render_widget(
        menu,
        area.inner(Margin {
            vertical: 4,
            horizontal: 12,
        }),
    );
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

#[derive(Debug, Clone, Default, PartialEq, EnumIter)]
pub enum MenuOption {
    #[default]
    Start,
    Quit,
}

impl MenuOption {
    fn render_paragraph(current_option: &MenuOption) -> Paragraph {
        let options: Vec<Line> = MenuOption::iter()
            .map(|option| {
                let mut l = Line::from(option.to_string());
                if option.eq(current_option) {
                    l = l.underlined();
                }

                l
            })
            .collect();
        Paragraph::new(options)
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
