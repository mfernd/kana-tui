use crate::{
    app::{App, Page},
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
    widgets::Paragraph,
    Frame,
};
use tui_popup::Popup;
use tui_prompts::{Prompt, State, TextPrompt, TextState};

pub fn render(app: &mut App, frame: &mut Frame, main_area: Rect) {
    let mut page_data = PageData::extract(&app.current_page).unwrap().clone();
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
    .centered();
    frame.render_widget(
        kana_title,
        tui::flex(
            area_top,
            (Flex::Center, Constraint::Length(6)),
            (Flex::End, Constraint::Length(1)),
        ),
    );

    if let Some(ref indication) = page_data.indication {
        let good_wrong_indication = Paragraph::new(indication.to_string()).centered();
        frame.render_widget(
            good_wrong_indication,
            tui::flex(
                area_middle,
                (Flex::Center, Constraint::Length(6)),
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
    user_input.draw(frame, user_input_layout, &mut page_data.user_input);

    if page_data.is_paused {
        let popup = Popup::new("Press any key to exit").title("paused");
        frame.render_widget(&popup, frame.area());
    }

    app.go_to_study_page().data(page_data).call();
}

pub fn handle_key_events(key_event: KeyEvent, app: &mut App) {
    let mut page_data = PageData::extract(&app.current_page).unwrap().clone();

    match (page_data.is_paused, key_event.modifiers, key_event.code) {
        (_, _, KeyCode::Esc) => {
            app.go_to_homepage().call();
            return;
        }
        (false, _, KeyCode::Pause)
        | (false, KeyModifiers::CONTROL, KeyCode::Char('p'))
        | (true, _, _) => page_data.is_paused = !page_data.is_paused,
        (false, _, KeyCode::Enter) => {
            if page_data.is_input_valid() {
                page_data.good_guesses.push(page_data.current_kana.clone());
                page_data.next_kana();
            } else {
                page_data.indication = Some(Indication::WrongGuess);
                page_data.wrong_guesses.push(page_data.current_kana.clone());
                page_data.user_input.truncate();
            }
        }
        (false, _, KeyCode::Char(' ')) => {
            let help = Some(Indication::Help(page_data.current_kana.clone()));
            if page_data.indication.eq(&help) {
                page_data.next_kana();
            } else {
                page_data.indication = help;
                page_data.wrong_guesses.push(page_data.current_kana.clone());
            }
        }
        (false, _, _) => page_data.user_input.handle_key_event(key_event),
    }

    app.go_to_study_page().data(page_data).call();
}

#[derive(Debug, Clone)]
pub struct PageData {
    representation: KanaRepresentation,
    kanas: Vec<Kana>,
    current_kana: Kana,
    indication: Option<Indication>,
    good_guesses: Vec<Kana>,
    wrong_guesses: Vec<Kana>,
    user_input: TextState<'static>,
    is_paused: bool,
}

impl PageData {
    fn extract(page: &Page) -> Option<&Self> {
        match page {
            Page::Homepage(_) => None,
            Page::StudyPage(page_data) => Some(page_data),
        }
    }

    fn is_input_valid(&self) -> bool {
        self.current_kana.validate_guess(self.user_input.value())
    }

    fn next_kana(&mut self) {
        if let Some(next_kana) = self.kanas.pop() {
            self.current_kana = next_kana;
            self.indication = None;
            self.user_input = TextState::new().with_focus(tui_prompts::FocusState::Focused);
        } else {
            // TODO: go to result page
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
