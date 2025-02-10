use super::study_page;
use crate::{
    app::{App, Page},
    study::kana::{Kana, KanaRepresentation},
};
use crossterm::event::KeyEvent;
use ratatui::{
    layout::{Constraint, Layout, Margin, Rect},
    style::Stylize,
    text::{Line, Span, ToLine, ToSpan},
    widgets::{Paragraph, Wrap},
    Frame,
};

pub fn render(app: &mut App, frame: &mut Frame, main_area: Rect) {
    let page_data = PageData::extract(&app.current_page).unwrap();
    let [area_top, area_middle, area_bottom] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Fill(4),
        Constraint::Length(2),
    ])
    .areas(main_area.inner(Margin::new(2, 1)));

    let congratulations_line = Line::from("You finished! \u{1F44F}").bold().centered();
    frame.render_widget(congratulations_line, area_top);

    let kanas_count = page_data.good_guesses.len() + page_data.wrong_guesses.len();
    let kanas_count_line = Line::from(Vec::from([
        "Your study plan contained ".to_span(),
        Span::from(format!("{}", kanas_count)).bold(),
        Span::from(format!(" {}(s).", page_data.representation)),
    ]));
    let goods_line = Line::from(Vec::from([
        "> good guesses: ".to_span(),
        Span::from(format!("{}", page_data.good_guesses.len())),
        "/".to_span(),
        Span::from(format!("{}", kanas_count)),
    ]));
    let wrongs_line = Line::from(Vec::from([
        "> wrong guesses: ".to_span(),
        Span::from(format!("{}", page_data.wrong_guesses.len())),
        "/".to_span(),
        Span::from(format!("{}", kanas_count)),
    ]));
    let result_paragraph = Paragraph::new(Vec::from([
        kanas_count_line,
        "".to_line(),
        "You had:".to_line(),
        goods_line,
        wrongs_line,
    ]))
    .wrap(Wrap { trim: true });
    frame.render_widget(result_paragraph, area_middle);

    let info = Paragraph::new("Press any key to go to the homepage.")
        .wrap(Wrap { trim: true })
        .centered()
        .dim();
    frame.render_widget(info, area_bottom);
}

pub fn handle_key_events(_key_event: KeyEvent, app: &mut App) {
    // go to homepage, whatever the user inputs.
    app.go_to_homepage().call();
}

#[derive(Debug, Clone)]
pub struct PageData {
    representation: KanaRepresentation,
    good_guesses: Vec<Kana>,
    wrong_guesses: Vec<Kana>,
}

impl PageData {
    fn extract(page: &Page) -> Option<&Self> {
        match page {
            Page::ResultPage(page_data) => Some(page_data),
            _ => None,
        }
    }
}

impl From<study_page::PageData> for PageData {
    fn from(value: study_page::PageData) -> Self {
        Self {
            representation: value.representation,
            good_guesses: value.good_guesses,
            wrong_guesses: value.wrong_guesses,
        }
    }
}
