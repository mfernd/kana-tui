use super::Homepage;
use crate::{
    app::{IPage, PageEvent},
    config::Config,
    models::{answer::AnswerResult, kana::KanaRepresentation},
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
    good_answers_count: usize,
    wrong_answers_count: usize,
    total_elapsed_time: u128,
}

impl IPage for ResultPage {
    fn render(&mut self, frame: &mut Frame, main_area: Rect, config: &Config) {
        let [area_top, area_middle, area_bottom] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Fill(1),
            Constraint::Length(2),
        ])
        .areas(main_area.inner(Margin::new(1, 1)));

        let congratulations_line = Line::from("You finished! \u{1F44F}").bold().centered();
        frame.render_widget(congratulations_line, area_top);

        let kana_representation = KanaRepresentation::from(config.writing_system.clone());
        let kanas_count = self.good_answers_count + self.wrong_answers_count;
        let correct_percent = (self.good_answers_count as f64 / kanas_count as f64) * 100_f64;
        let kanas_count_line = Line::from(Vec::from([
            "You have completed your study plan of ".to_span(),
            kanas_count.to_span().bold(),
            Span::from(format!(" {}(s) in ", kana_representation)),
            self.format_time().bold(),
            ".".to_span(),
        ]));
        let goods_line = Line::from(Vec::from([
            "> correct answers: ".to_span(),
            self.good_answers_count.to_span(),
            "/".to_span(),
            kanas_count.to_span(),
        ]));
        let wrongs_line = Line::from(Vec::from([
            "> wrong answers: ".to_span(),
            self.wrong_answers_count.to_span(),
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

    fn handle_key_events(&mut self, _: KeyEvent, _: &mut Config) -> PageEvent {
        PageEvent::Navigate(Homepage::default().into())
    }
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
            good_answers_count: value.get_count_by_result(&AnswerResult::Good),
            wrong_answers_count: value.get_count_by_result(&AnswerResult::Wrong),
        }
    }
}
