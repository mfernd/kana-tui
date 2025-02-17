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
    total_elapsed_time: u128,
}

impl IPage for ResultPage {
    fn render(&mut self, frame: &mut Frame, main_area: Rect) {
        let [area_top, area_middle, area_bottom] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Fill(4),
            Constraint::Length(2),
        ])
        .areas(main_area.inner(Margin::new(1, 1)));

        let congratulations_line = Line::from("You finished! \u{1F44F}").bold().centered();
        frame.render_widget(congratulations_line, area_top);

        let kanas_count = self.good_guesses.len() + self.wrong_guesses.len();
        let correct_percent = (self.good_guesses.len() as f64 / kanas_count as f64) * 100_f64;
        let kanas_count_line = Line::from(Vec::from([
            "You have completed your study plan of ".to_span(),
            kanas_count.to_span().bold(),
            Span::from(format!(" {}(s) in ", self.representation)),
            self.format_time().bold(),
            ".".to_span(),
        ]));
        let goods_line = Line::from(Vec::from([
            "> good guesses: ".to_span(),
            Span::from(format!("{}", self.good_guesses.len())),
            "/".to_span(),
            kanas_count.to_span(),
        ]));
        let wrongs_line = Line::from(Vec::from([
            "> wrong guesses: ".to_span(),
            Span::from(format!("{}", self.wrong_guesses.len())),
            "/".to_span(),
            kanas_count.to_span(),
        ]));
        let result_paragraph = Paragraph::new(Vec::from([
            kanas_count_line,
            "".to_line(),
            "You had:".to_line(),
            goods_line,
            wrongs_line,
            format!("Total: {:.0}% correct answers.", correct_percent).into(),
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

impl ResultPage {
    fn format_time(&self) -> String {
        let seconds = (self.total_elapsed_time / 1000) % 60;
        let minutes = (self.total_elapsed_time / 60_000) % 60;
        if minutes == 0 {
            return format!("{}s", seconds);
        } else if seconds == 0 {
            return format!("{}min", seconds);
        }
        format!("{}min and {}s", minutes, seconds)
    }
}

impl From<super::study_page::StudyPage> for ResultPage {
    fn from(value: super::study_page::StudyPage) -> Self {
        Self {
            total_elapsed_time: value.total_elapsed_time_ms(),
            representation: value.representation,
            good_guesses: value.good_guesses,
            wrong_guesses: value.wrong_guesses,
        }
    }
}
