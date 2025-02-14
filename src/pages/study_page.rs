use crate::{
    app::{IPage, Page},
    study::{
        kana::{Kana, KanaRepresentation},
        ValidateGuess,
    },
    tui,
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    layout::{Constraint, Flex, Layout, Rect},
    style::Stylize,
    text::{Line, ToLine},
    Frame,
};
use tui_popup::Popup;
use tui_prompts::{Prompt, State, TextPrompt, TextState};

#[derive(Debug, Clone)]
pub struct StudyPage {
    pub representation: KanaRepresentation,
    pub kanas: Vec<Kana>,
    pub good_guesses: Vec<Kana>,
    pub wrong_guesses: Vec<Kana>,
    current_kana: Kana,
    indication: Option<Indication>,
    user_input: TextState<'static>,
    is_paused: bool,
}

impl IPage for StudyPage {
    fn render(&mut self, frame: &mut Frame, main_area: Rect) {
        let [area_top, area_middle, area_bottom] = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(3),
            Constraint::Fill(1),
        ])
        .areas(main_area);

        let kana_title = Line::from(match self.representation {
            KanaRepresentation::Hiragana => self.current_kana.to_hiragana(),
            KanaRepresentation::Katakana => self.current_kana.to_katakana(),
        })
        .bold()
        .centered();
        frame.render_widget(
            kana_title,
            tui::flex(
                area_top,
                (Flex::Center, Constraint::Length(6)),
                (Flex::End, Constraint::Length(1)),
            ),
        );

        if let Some(ref indication) = self.indication {
            let good_wrong_indication = indication.to_line().dim().centered();
            frame.render_widget(
                good_wrong_indication,
                tui::flex(
                    area_middle,
                    (Flex::Center, Constraint::Fill(1)),
                    (Flex::Center, Constraint::Length(1)),
                ),
            );
        }

        let user_input = TextPrompt::from("rōmaji");
        let user_input_layout = tui::flex(
            area_bottom,
            (Flex::Center, Constraint::Length(20)),
            (Flex::Start, Constraint::Length(1)),
        );
        user_input.draw(frame, user_input_layout, &mut self.user_input);

        if self.is_paused {
            let popup = Popup::new("Press any key to exit").title("paused");
            frame.render_widget(&popup, frame.area());
        }
    }

    fn handle_key_events(&mut self, key_event: KeyEvent) -> Option<Page> {
        if key_event.code == KeyCode::Esc {
            return Page::go_home().call();
        }

        if self.is_paused
            || key_event.code == KeyCode::Pause
            || (key_event.modifiers == KeyModifiers::CONTROL
                && key_event.code == KeyCode::Char('p'))
        {
            self.is_paused = !self.is_paused;
            // early return to prevent all other events
            return self.go_same_page();
        }

        match (key_event.modifiers, key_event.code) {
            (_, KeyCode::Enter) => {
                if self.is_input_valid() {
                    self.good_guesses.push(self.current_kana.clone());
                    if !self.next_kana() {
                        return Page::go_result().page(self.clone().into()).call();
                    }
                } else {
                    self.trigger_wrong_input();
                }
            }
            (_, KeyCode::Char(' ')) => {
                let help = Some(Indication::Help(self.current_kana.clone()));
                if self.indication.eq(&help) {
                    if !self.next_kana() {
                        return Page::go_result().page(self.clone().into()).call();
                    }
                } else {
                    self.indication = help;
                    self.wrong_guesses.push(self.current_kana.clone());
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

    fn trigger_wrong_input(&mut self) {
        self.indication = Some(Indication::WrongGuess);
        self.wrong_guesses.push(self.current_kana.clone());
        self.user_input.truncate();
    }

    fn go_same_page(&self) -> Option<Page> {
        Page::go_study().page(self.clone()).call()
    }
}

impl Default for StudyPage {
    fn default() -> Self {
        let mut kanas = crate::study::create_study_plan();
        let first_kana = kanas.pop().unwrap(); // panic should not happen
        Self {
            representation: KanaRepresentation::Hiragana,
            kanas,
            current_kana: first_kana,
            indication: None,
            good_guesses: Vec::new(),
            wrong_guesses: Vec::new(),
            user_input: TextState::new().with_focus(tui_prompts::FocusState::Focused),
            is_paused: false,
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
