use super::Homepage;
use crate::{
    app::{IPage, PageEvent},
    config::Config,
    widgets::Button,
};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Constraint, Flex, Layout, Margin, Rect},
    style::Stylize,
    text::{Line, Span},
    widgets::List,
    Frame,
};

#[derive(Debug, Clone, Default)]
pub struct ConfigPage {
    focused_field: ConfigField,
}

impl IPage for ConfigPage {
    fn render(&mut self, frame: &mut Frame, main_area: Rect, _: &Config) {
        let [title_area, middle_area, bottom_area] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .areas(main_area.inner(Margin::new(1, 0)));

        let page_title = Line::from("\u{1F527} Configuration").bold().centered();
        frame.render_widget(page_title, title_area);

        let writing_system_field = self.writing_system_field(
            middle_area.width as usize,
            self.focused_field == ConfigField::StudyModeField,
        );
        let field_list = List::new(Vec::from([writing_system_field]));
        frame.render_widget(field_list, middle_area);

        self.render_bottom_actions(frame, bottom_area);
    }

    fn handle_key_events(&mut self, key_event: KeyEvent, _: &mut Config) -> PageEvent {
        if key_event.code == KeyCode::Esc {
            return PageEvent::Navigate(Homepage::default().into());
        }

        match (&self.focused_field, key_event.code) {
            (ConfigField::Action(BottomAction::Cancel), KeyCode::Enter | KeyCode::Char(' ')) => {
                return PageEvent::Navigate(Homepage::default().into());
            }
            (ConfigField::Action(BottomAction::Save), KeyCode::Enter | KeyCode::Char(' ')) => {
                // TODO should save config
                return PageEvent::Navigate(Homepage::default().into());
            }
            // Handle arrows and tab button updating current field. (Should always be last?)
            (c, key_code) => {
                if let Some(new_field) = c.arrows_and_tab(&key_code) {
                    self.focused_field = new_field;
                }
            }
        }

        PageEvent::Nothing
    }
}

impl ConfigPage {
    fn writing_system_field(&self, width: usize, is_focused: bool) -> Line {
        let label = Span::from("Writing system");
        let mut input = Span::from("\u{2BC7} Hiragana \u{2BC8}").bold();
        if is_focused {
            input = input.black().on_light_red();
        } else {
            input = input.white().on_dark_gray();
        }

        let space_available = width
            .saturating_sub(label.width())
            .saturating_sub(input.width());
        Line::from(Vec::from([
            label,
            Span::from(" ".repeat(space_available)),
            input,
        ]))
    }

    fn render_bottom_actions(&self, frame: &mut Frame, area: Rect) {
        let [left_button, _, right_button] = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Length(3),
            Constraint::Fill(1),
        ])
        .flex(Flex::Center)
        .areas(area);
        frame.render_widget(
            Button::new(
                "Cancel",
                self.focused_field == ConfigField::Action(BottomAction::Cancel),
            )
            .right_aligned(),
            left_button,
        );
        frame.render_widget(
            Button::new(
                "Save",
                self.focused_field == ConfigField::Action(BottomAction::Save),
            ),
            right_button,
        );
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
enum ConfigField {
    #[default]
    StudyModeField,
    Action(BottomAction),
}

/// Handle moves on ConfigField, a bit verbose, but I want it to be customizable.
impl ConfigField {
    fn arrows_and_tab(&self, key_code: &KeyCode) -> Option<Self> {
        match (self, key_code) {
            (c, KeyCode::Up) => c.up(),
            (c, KeyCode::Right) => c.right(),
            (c, KeyCode::Down) => c.down(),
            (c, KeyCode::Left) => c.left(),
            (c, KeyCode::Tab) => c.tab(),
            _ => None,
        }
    }

    fn up(&self) -> Option<Self> {
        match self {
            ConfigField::StudyModeField => None,
            ConfigField::Action(_) => Some(ConfigField::StudyModeField),
        }
    }

    fn right(&self) -> Option<Self> {
        match self {
            ConfigField::StudyModeField => Some(ConfigField::Action(BottomAction::Save)),
            ConfigField::Action(BottomAction::Cancel) => {
                Some(ConfigField::Action(BottomAction::Save))
            }
            ConfigField::Action(BottomAction::Save) => {
                Some(ConfigField::Action(BottomAction::Cancel))
            }
        }
    }

    fn down(&self) -> Option<Self> {
        match self {
            ConfigField::StudyModeField => Some(ConfigField::Action(BottomAction::Save)),
            ConfigField::Action(_) => None,
        }
    }

    fn left(&self) -> Option<Self> {
        match self {
            ConfigField::StudyModeField => None,
            ConfigField::Action(_) => self.right(),
        }
    }

    fn tab(&self) -> Option<Self> {
        match self {
            ConfigField::StudyModeField => self.down(),
            ConfigField::Action(BottomAction::Cancel) => Some(ConfigField::StudyModeField),
            ConfigField::Action(BottomAction::Save) => self.left(),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
enum BottomAction {
    Cancel,
    #[default]
    Save,
}
