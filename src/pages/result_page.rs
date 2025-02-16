use crate::{
    app::{IPage, Page},
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

#[derive(Debug, Clone)]
pub struct ResultPage {
    representation: KanaRepresentation,
    good_guesses: Vec<Kana>,
    wrong_guesses: Vec<Kana>,
}

impl IPage for ResultPage {
    fn render(&mut self, frame: &mut Frame, main_area: Rect) {
        let [area_top, area_middle, area_bottom] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Fill(4),
            Constraint::Length(2),
        ])
        .areas(main_area.inner(Margin::new(2, 1)));

        let congratulations_line = Line::from("You finished! \u{1F44F}").bold().centered();
        frame.render_widget(congratulations_line, area_top);

        let kanas_count = self.good_guesses.len() + self.wrong_guesses.len();
        let kanas_count_line = Line::from(Vec::from([
            "Your study plan contained ".to_span(),
            Span::from(format!("{}", kanas_count)).bold(),
            Span::from(format!(" {}(s).", self.representation)),
        ]));
        let goods_line = Line::from(Vec::from([
            "> good guesses: ".to_span(),
            Span::from(format!("{}", self.good_guesses.len())),
            "/".to_span(),
            Span::from(format!("{}", kanas_count)),
        ]));
        let wrongs_line = Line::from(Vec::from([
            "> wrong guesses: ".to_span(),
            Span::from(format!("{}", self.wrong_guesses.len())),
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

    fn handle_key_events(&mut self, _: KeyEvent) -> Option<Page> {
        Page::go_home().call()
    }

    fn tick(&mut self) {}
}

impl From<super::study_page::StudyPage> for ResultPage {
    fn from(value: super::study_page::StudyPage) -> Self {
        Self {
            representation: value.representation,
            good_guesses: value.good_guesses,
            wrong_guesses: value.wrong_guesses,
        }
    }
}
