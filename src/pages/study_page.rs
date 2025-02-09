use crate::{
    app::{App, Page},
    study::kana::{Kana, KanaRepresentation},
    tui,
};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Constraint, Flex, Layout, Rect},
    style::Stylize,
    widgets::Paragraph,
    Frame,
};

pub fn render(app: &mut App, frame: &mut Frame, main_area: Rect) {
    let page_data = PageData::extract(&app.current_page).unwrap();
    let [area_top, area_middle, area_bottom] = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(3),
        Constraint::Fill(1),
    ])
    .areas(main_area);

    let kana_title = Paragraph::new(match page_data.representation {
        KanaRepresentation::Hiragana => page_data.current_kana.to_hiragana(),
        KanaRepresentation::Katakana => page_data.current_kana.to_katakana(),
    })
    .bold()
    .black()
    .on_red()
    .centered();
    frame.render_widget(
        kana_title,
        tui::flex(
            area_top,
            (Flex::Center, Constraint::Length(4)),
            (Flex::End, Constraint::Length(1)),
        ),
    );

    if let Some(ref indication) = page_data.indication {
        let good_wrong_indication = Paragraph::new(indication.to_string()).black().on_green();
        frame.render_widget(
            good_wrong_indication,
            tui::flex(
                area_middle,
                (Flex::Center, Constraint::Length(2)),
                (Flex::Center, Constraint::Length(1)),
            ),
        );
    }

    let user_input = Paragraph::new("test")
        .underlined()
        .black()
        .on_blue()
        .centered();
    let user_input_layout = tui::flex(
        area_bottom,
        (Flex::Center, Constraint::Length(6)),
        (Flex::Start, Constraint::Length(1)),
    );
    frame.render_widget(user_input, user_input_layout);
}

pub fn handle_key_events(key_event: KeyEvent, app: &mut App) {
    match key_event.code {
        KeyCode::Esc => app.go_to_homepage().call(),
        KeyCode::Enter => {}
        _ => {}
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PageData {
    representation: KanaRepresentation,
    kanas: Vec<Kana>,
    current_kana: Kana,
    indication: Option<Indication>,
    good_guesses: Vec<Kana>,
    wrong_guesses: Vec<Kana>,
    is_paused: bool,
}

impl PageData {
    fn extract(page: &Page) -> Option<&Self> {
        match page {
            Page::Homepage(_) => None,
            Page::StudyPage(page_data) => Some(page_data),
        }
    }
}

impl Default for PageData {
    fn default() -> Self {
        let mut kanas = crate::study::create_study_plan();
        let first_kana = kanas.pop().unwrap(); // panic should not happen
        Self {
            representation: KanaRepresentation::Hiragana,
            kanas,
            current_kana: first_kana,
            indication: Some(Indication::WrongGuess),
            good_guesses: Vec::new(),
            wrong_guesses: Vec::new(),
            is_paused: false,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
enum Indication {
    WrongGuess,
    Help(Kana),
}

impl std::fmt::Display for Indication {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WrongGuess => write!(f, "\u{274C}"),
            Self::Help(kana) => write!(f, "{}", kana),
        }
    }
}
