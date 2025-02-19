use crate::{
    app::{IPage, Page, ReturnedPage},
    models::{
        guess::GuessResultKind,
        kana::{Kana, KanaRepresentation},
        ValidateGuess,
    },
    tui,
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    layout::{Constraint, Flex, Layout, Margin, Rect},
    style::Stylize,
    text::{Line, ToLine},
    Frame,
};
use std::time::Instant;
use tui_popup::Popup;
use tui_prompts::{Prompt, State, TextPrompt, TextState};

#[derive(Debug, Clone)]
pub struct StudyPage {
    pub representation: KanaRepresentation,
    pub kanas: Vec<Kana>,
    pub guesses: Vec<(Kana, GuessResultKind)>,
    current_kana: Kana,
    indication: Option<Indication>,
    user_input: TextState<'static>,
    is_paused: bool,
    /// Timer should eventually be in a widget
    /// Contains our current timer. Is set to None, when the page is paused.
    current_timer: Option<Instant>,
    /// Is updated when our page is paused.
    memory_elapsed_ms: u128,
}

impl IPage for StudyPage {
    fn render(&mut self, frame: &mut Frame, main_area: Rect) {
        let [timer_area, kana_area, indication_area, input_area] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Fill(2),
            Constraint::Length(3),
            Constraint::Fill(3),
        ])
        .areas(main_area);

        let timer = Line::from(self.format_timer()).dim().centered();
        frame.render_widget(timer, timer_area.inner(Margin::new(0, 1)));

        let kana_title = Line::from(match self.representation {
            KanaRepresentation::Hiragana => self.current_kana.to_hiragana(),
            KanaRepresentation::Katakana => self.current_kana.to_katakana(),
        })
        .bold()
        .centered();
        frame.render_widget(
            kana_title,
            tui::flex(
                kana_area,
                (Flex::Center, Constraint::Length(6)),
                (Flex::End, Constraint::Length(1)),
            ),
        );

        if let Some(ref indication) = self.indication {
            let good_wrong_indication = indication.to_line().dim().centered();
            frame.render_widget(
                good_wrong_indication,
                tui::flex(
                    indication_area,
                    (Flex::Center, Constraint::Fill(1)),
                    (Flex::Center, Constraint::Length(1)),
                ),
            );
        }

        let user_input = TextPrompt::from("rōmaji");
        let user_input_layout = tui::flex(
            input_area,
            (Flex::Center, Constraint::Length(20)),
            (Flex::Start, Constraint::Length(1)),
        );
        user_input.draw(frame, user_input_layout, &mut self.user_input);

        if self.is_paused {
            let popup = Popup::new("Press any key to exit.").title("paused");
            frame.render_widget(&popup, frame.area());
        }
    }

    fn handle_key_events(&mut self, key_event: KeyEvent) -> ReturnedPage {
        if !self.is_paused && key_event.code == KeyCode::Esc {
            return Page::go_home().call();
        }

        if self.is_paused
            || key_event.code == KeyCode::Pause
            || (key_event.modifiers == KeyModifiers::CONTROL
                && key_event.code == KeyCode::Char('p'))
        {
            self.is_paused = !self.is_paused;
            self.reset_timer();
            // early return to prevent all other events
            return self.go_same_page();
        }

        // handle keyboard events
        match (key_event.modifiers, key_event.code) {
            (_, KeyCode::Enter) => {
                if self.is_input_valid() {
                    self.push_good_guess();
                    if !self.next_kana() {
                        self.finish_study_hook();
                        return Page::go_result().page(self.clone().into()).call();
                    }
                } else {
                    self.indication = Some(Indication::WrongGuess);
                    self.push_wrong_guess();
                    self.user_input.truncate();
                }
            }
            (_, KeyCode::Char(' ')) => {
                let help = Some(Indication::Help(self.current_kana.clone()));
                if self.indication.eq(&help) {
                    if !self.next_kana() {
                        self.finish_study_hook();
                        return Page::go_result().page(self.clone().into()).call();
                    }
                } else {
                    self.indication = help;
                    self.push_wrong_guess();
                }
            }
            _ => self.user_input.handle_key_event(key_event),
        };

        self.go_same_page()
    }
}

impl StudyPage {
    fn is_input_valid(&self) -> bool {
        self.current_kana.validate_guess(self.user_input.value())
    }

    /// Update [PageData] with next kana.
    /// If there are no kana left, return `false` and go to result page.
    fn next_kana(&mut self) -> bool {
        if let Some(next_kana) = self.kanas.pop() {
            self.current_kana = next_kana;
            self.indication = None;
            self.user_input = TextState::new().with_focus(tui_prompts::FocusState::Focused);
            return true;
        }

        false
    }

    fn push_good_guess(&mut self) {
        if self.guesses.iter().any(|(g, _)| g.eq(&self.current_kana)) {
            return;
        }
        self.guesses
            .push((self.current_kana.clone(), GuessResultKind::Good));
    }

    fn push_wrong_guess(&mut self) {
        if self.guesses.iter().any(|(g, _)| g.eq(&self.current_kana)) {
            return;
        }
        self.guesses
            .push((self.current_kana.clone(), GuessResultKind::Wrong));
    }

    fn go_same_page(&self) -> ReturnedPage {
        Page::go_study().page(self.clone()).call()
    }

    fn finish_study_hook(&mut self) {
        // only reset timer if we have one
        if self.current_timer.is_some() {
            self.reset_timer();
        }
    }

    /// Used when we pause our page, will save our last elapsed time in `self.memory_elapsed_time`
    /// and remove the timer. And when we restart, the timer is restarted.
    fn reset_timer(&mut self) {
        if let Some(last_start_time) = self.current_timer {
            self.memory_elapsed_ms += last_start_time.elapsed().as_millis();
            self.current_timer = None;
        } else {
            self.current_timer = Some(Instant::now());
        }
    }

    pub fn total_elapsed_time_ms(&self) -> u128 {
        let timer_elapsed_time = self
            .current_timer
            .map(|instant| instant.elapsed().as_millis())
            .unwrap_or(0);
        self.memory_elapsed_ms + timer_elapsed_time
    }

    fn format_timer(&self) -> String {
        let total_elapsed_time_ms = self.total_elapsed_time_ms();
        let seconds = (total_elapsed_time_ms / 1000) % 60;
        let minutes = (total_elapsed_time_ms / 60_000) % 60;
        format!("{:02}:{:02}", minutes, seconds)
    }
}

impl Default for StudyPage {
    fn default() -> Self {
        let mut kanas = crate::models::create_study_plan();
        let first_kana = kanas.pop().unwrap(); // panic should not happen
        Self {
            representation: KanaRepresentation::Hiragana,
            kanas,
            current_kana: first_kana,
            indication: None,
            guesses: Vec::new(),
            user_input: TextState::new().with_focus(tui_prompts::FocusState::Focused),
            is_paused: false,
            // start immediately
            current_timer: Some(Instant::now()),
            memory_elapsed_ms: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
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
